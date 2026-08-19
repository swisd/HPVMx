use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use core::future::Future;
use core::pin::Pin;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use crate::env::SpinLock;

/// Unique identifier for an asynchronous task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId(pub usize);

impl TaskId {
    /// Generates a new unique `TaskId`.
    pub fn new() -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

/// Standalone task representation holding a pinned boxed future.
pub struct Task {
    pub id: TaskId,
    pub future: Pin<Box<dyn Future<Output = ()> + Send>>,
}

impl Task {
    /// Creates a new `Task` from a Send + 'static Future.
    pub fn new(future: impl Future<Output = ()> + Send + 'static) -> Self {
        Self {
            id: TaskId::new(),
            future: Box::pin(future),
        }
    }

    /// Polls the task future with the given context.
    pub fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}

/// Handle to an asynchronous task managed by `MultiCoreExecutor`.
#[derive(Clone)]
pub struct TaskHandle {
    pub id: TaskId,
    pub node: Arc<TaskNode>,
}

impl TaskHandle {
    /// Returns the unique `TaskId`.
    pub fn id(&self) -> TaskId {
        self.id
    }

    /// Returns `true` if the background task has completed execution.
    pub fn is_done(&self) -> bool {
        self.node.is_completed.load(Ordering::SeqCst)
    }
}

impl Future for TaskHandle {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.is_done() {
            Poll::Ready(())
        } else {
            // Task is still running on an AP or ready queue.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// Internal node representing a task managed by `MultiCoreExecutor`.
pub struct TaskNode {
    pub id: TaskId,
    pub future: SpinLock<Option<Pin<Box<dyn Future<Output = ()> + Send>>>>,
    pub task_queue: Arc<SpinLock<VecDeque<Arc<TaskNode>>>>,
    pub is_queued: AtomicBool,
    pub is_completed: AtomicBool,
}

impl TaskNode {
    /// Wakes the task by reference, re-enqueuing it into the shared executor ready queue.
    pub fn wake_by_ref(self: &Arc<Self>) {
        if self.is_completed.load(Ordering::SeqCst) {
            return;
        }
        // Avoid duplicate enqueues if already queued
        if !self.is_queued.swap(true, Ordering::SeqCst) {
            let mut q = self.task_queue.lock();
            q.push_back(Arc::clone(self));
        }
    }
}

// ---------------------------------------------------------------------------
// ArcWaker implementation using RawWakerVTable
// ---------------------------------------------------------------------------

unsafe fn clone_task_waker(ptr: *const ()) -> RawWaker {
    let arc = Arc::from_raw(ptr as *const TaskNode);
    let cloned = Arc::clone(&arc);
    core::mem::forget(arc); // Keep original arc intact
    RawWaker::new(Arc::into_raw(cloned) as *const (), &TASK_WAKER_VTABLE)
}

unsafe fn wake_task(ptr: *const ()) {
    let arc = Arc::from_raw(ptr as *const TaskNode);
    arc.wake_by_ref();
    drop(arc);
}

unsafe fn wake_task_by_ref(ptr: *const ()) {
    let arc = Arc::from_raw(ptr as *const TaskNode);
    arc.wake_by_ref();
    core::mem::forget(arc);
}

unsafe fn drop_task_waker(ptr: *const ()) {
    drop(Arc::from_raw(ptr as *const TaskNode));
}

static TASK_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    clone_task_waker,
    wake_task,
    wake_task_by_ref,
    drop_task_waker,
);

/// Creates a `Waker` for a specific `TaskNode`.
pub fn task_waker(task: Arc<TaskNode>) -> Waker {
    let raw_waker = RawWaker::new(Arc::into_raw(task) as *const (), &TASK_WAKER_VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}

/// Statistics for `MultiCoreExecutor`.
#[derive(Debug, Clone, Copy, Default)]
pub struct ExecutorStats {
    pub total_spawned: usize,
    pub total_completed: usize,
    pub ready_tasks: usize,
}

/// Multi-core, thread-safe asynchronous task executor.
///
/// Designed to allow tasks to be spawned by any thread/core and polled
/// cooperatively by the BSP or concurrently by multiple AP worker cores.
#[derive(Clone)]
pub struct MultiCoreExecutor {
    ready_queue: Arc<SpinLock<VecDeque<Arc<TaskNode>>>>,
    running: Arc<AtomicBool>,
    total_spawned: Arc<AtomicUsize>,
    total_completed: Arc<AtomicUsize>,
}

impl MultiCoreExecutor {
    /// Creates a new `MultiCoreExecutor`.
    pub fn new() -> Self {
        Self {
            ready_queue: Arc::new(SpinLock::new(VecDeque::new())),
            running: Arc::new(AtomicBool::new(true)),
            total_spawned: Arc::new(AtomicUsize::new(0)),
            total_completed: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Spawns a new future onto the multi-core executor ready queue.
    pub fn spawn<F>(&self, future: F) -> TaskId
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.spawn_handle(future).id
    }

    /// Spawns a new future onto the multi-core executor ready queue and returns a `TaskHandle`.
    pub fn spawn_handle<F>(&self, future: F) -> TaskHandle
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let id = TaskId::new();
        let node = Arc::new(TaskNode {
            id,
            future: SpinLock::new(Some(Box::pin(future))),
            task_queue: Arc::clone(&self.ready_queue),
            is_queued: AtomicBool::new(true),
            is_completed: AtomicBool::new(false),
        });

        self.total_spawned.fetch_add(1, Ordering::SeqCst);
        let mut queue = self.ready_queue.lock();
        queue.push_back(Arc::clone(&node));
        TaskHandle { id, node }
    }

    /// Pops the next ready task node from the shared ready queue, if available.
    pub fn pop_ready_task(&self) -> Option<Arc<TaskNode>> {
        let mut queue = self.ready_queue.lock();
        let node = queue.pop_front()?;
        node.is_queued.store(false, Ordering::SeqCst);
        Some(node)
    }

    /// Polls a single ready task.
    ///
    /// Returns `true` if a task was dequeued and polled, or `false` if the queue was empty.
    pub fn poll_one(&self) -> bool {
        let node = match self.pop_ready_task() {
            Some(n) => n,
            None => return false,
        };

        if node.is_completed.load(Ordering::SeqCst) {
            return true;
        }

        let waker = task_waker(Arc::clone(&node));
        let mut context = Context::from_waker(&waker);

        let mut future_slot = node.future.lock();
        if let Some(fut) = future_slot.as_mut() {
            match fut.as_mut().poll(&mut context) {
                Poll::Ready(()) => {
                    node.is_completed.store(true, Ordering::SeqCst);
                    *future_slot = None;
                    self.total_completed.fetch_add(1, Ordering::SeqCst);
                }
                Poll::Pending => {
                    // Task yielded. If it was woken during poll, wake_by_ref already re-enqueued it.
                    // If not re-enqueued yet, it remains sleeping until an event wakes its waker.
                }
            }
        }

        true
    }

    /// Polls ready tasks currently in the queue during a single tick (e.g., in a frame loop).
    ///
    /// Returns the number of tasks polled.
    pub fn run_ready_tasks(&self) -> usize {
        let initial_count = {
            let q = self.ready_queue.lock();
            q.len()
        };

        let mut processed = 0;
        for _ in 0..initial_count {
            if self.poll_one() {
                processed += 1;
            } else {
                break;
            }
        }
        processed
    }

    /// Returns whether the executor is currently running.
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Stops the executor, signalling workers to terminate.
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Returns the number of tasks currently waiting in the ready queue.
    pub fn ready_count(&self) -> usize {
        self.ready_queue.lock().len()
    }

    /// Returns `true` if there are no tasks in the ready queue.
    pub fn is_empty(&self) -> bool {
        self.ready_count() == 0
    }

    /// Returns execution statistics.
    pub fn stats(&self) -> ExecutorStats {
        ExecutorStats {
            total_spawned: self.total_spawned.load(Ordering::SeqCst),
            total_completed: self.total_completed.load(Ordering::SeqCst),
            ready_tasks: self.ready_count(),
        }
    }

    /// Worker routine designed for execution on AP cores.
    ///
    /// Continually polls tasks while the executor is running, spinning when idle.
    pub fn worker_loop(&self) {
        while self.is_running() {
            if !self.poll_one() {
                core::hint::spin_loop();
            }
        }
    }

    /// Starts background AP workers to process ready tasks from the global executor.
    pub fn start_ap_workers(&self) -> Result<usize, &'static str> {
        crate::hardware::cpu::mp::start_global_ap_workers()
    }

    /// Stops all running background AP workers.
    pub fn stop_ap_workers(&self) {
        crate::hardware::cpu::mp::stop_global_ap_workers();
    }

    /// Returns the number of active background AP workers.
    pub fn active_ap_workers(&self) -> usize {
        crate::hardware::cpu::mp::active_global_ap_workers()
    }

    /// Returns runtime statistics for all AP workers.
    pub fn ap_stats(&self) -> alloc::vec::Vec<crate::hardware::cpu::mp::ApWorkerStat> {
        crate::hardware::cpu::mp::global_ap_worker_stats()
    }
}

impl Default for MultiCoreExecutor {
    fn default() -> Self {
        Self::new()
    }
}

/// Global shared multi-core executor instance for HPVMx.
pub static GLOBAL_EXECUTOR: SpinLock<Option<MultiCoreExecutor>> = SpinLock::new(None);

/// Initializes the global multi-core executor if not already initialized.
pub fn init_global_executor() {
    let mut guard = GLOBAL_EXECUTOR.lock();
    if guard.is_none() {
        *guard = Some(MultiCoreExecutor::new());
    }
}

/// Spawns a task onto the global multi-core executor.
pub fn spawn_global<F>(future: F) -> Option<TaskId>
where
    F: Future<Output = ()> + Send + 'static,
{
    let guard = GLOBAL_EXECUTOR.lock();
    guard.as_ref().map(|exec| exec.spawn(future))
}

/// Spawns a future onto the global multi-core executor, returning a `TaskHandle`.
pub fn spawn_global_handle<F>(future: F) -> Option<TaskHandle>
where
    F: Future<Output = ()> + Send + 'static,
{
    let guard = GLOBAL_EXECUTOR.lock();
    guard.as_ref().map(|exec| exec.spawn_handle(future))
}

/// Returns runtime statistics for the global executor.
pub fn global_executor_stats() -> Option<ExecutorStats> {
    let guard = GLOBAL_EXECUTOR.lock();
    guard.as_ref().map(|exec| exec.stats())
}

/// Polls a single ready task on the global multi-core executor.
///
/// Returns `true` if a task was dequeued and polled, or `false` if the ready queue was empty.
pub fn poll_global_one() -> bool {
    let guard = GLOBAL_EXECUTOR.lock();
    guard.as_ref().map(|exec| exec.poll_one()).unwrap_or(false)
}

/// Polls ready tasks on the global multi-core executor.
pub fn poll_global_ready() -> usize {
    let guard = GLOBAL_EXECUTOR.lock();
    guard.as_ref().map(|exec| exec.run_ready_tasks()).unwrap_or(0)
}

/// Starts background AP workers on all available enabled AP cores.
pub fn start_global_ap_workers() -> Result<usize, &'static str> {
    crate::hardware::cpu::mp::start_global_ap_workers()
}

/// Stops all active background AP workers.
pub fn stop_global_ap_workers() {
    crate::hardware::cpu::mp::stop_global_ap_workers();
}

/// Returns the number of currently active AP workers.
pub fn active_global_ap_workers() -> usize {
    crate::hardware::cpu::mp::active_global_ap_workers()
}

/// Returns the total number of tasks completed by AP workers.
pub fn total_ap_tasks_executed() -> usize {
    crate::hardware::cpu::mp::total_ap_tasks_executed()
}

/// Returns runtime statistics for all AP workers.
pub fn global_ap_worker_stats() -> alloc::vec::Vec<crate::hardware::cpu::mp::ApWorkerStat> {
    crate::hardware::cpu::mp::global_ap_worker_stats()
}

/// Single-threaded cooperative executor kept for backwards compatibility.
pub struct SimpleExecutor {
    inner: MultiCoreExecutor,
}

impl SimpleExecutor {
    pub fn new() -> Self {
        Self {
            inner: MultiCoreExecutor::new(),
        }
    }

    pub fn spawn(&mut self, future: impl Future<Output = ()> + Send + 'static) {
        self.inner.spawn(future);
    }

    /// Run one iteration of ready tasks during the dashboard UI frame
    pub fn run_ready_tasks(&mut self) {
        self.inner.run_ready_tasks();
    }
}

impl Default for SimpleExecutor {
    fn default() -> Self {
        Self::new()
    }
}

fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(p: *const ()) -> RawWaker { RawWaker::new(p, &VTABLE) }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(core::ptr::null(), &VTABLE)
}

pub fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

pub struct YieldFuture {
    yielded: bool,
}

impl Future for YieldFuture {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

pub fn yield_now() -> YieldFuture {
    YieldFuture { yielded: false }
}

/// Runs self-tests for the `MultiCoreExecutor` and `ArcWaker` multi-core async subsystem.
pub fn run_multipar_tests() -> bool {
    let executor = MultiCoreExecutor::new();

    static COUNTER_A: AtomicUsize = AtomicUsize::new(0);
    static COUNTER_B: AtomicUsize = AtomicUsize::new(0);
    COUNTER_A.store(0, Ordering::SeqCst);
    COUNTER_B.store(0, Ordering::SeqCst);

    // Spawn task A: increments 1 -> yields -> increments 10 -> yields -> increments 100
    executor.spawn(async {
        COUNTER_A.fetch_add(1, Ordering::SeqCst);
        yield_now().await;
        COUNTER_A.fetch_add(10, Ordering::SeqCst);
        yield_now().await;
        COUNTER_A.fetch_add(100, Ordering::SeqCst);
    });

    // Spawn task B: increments 500
    executor.spawn(async {
        COUNTER_B.fetch_add(500, Ordering::SeqCst);
    });

    if executor.ready_count() != 2 {
        return false;
    }

    // Step 1: Run ready tasks
    let polled = executor.run_ready_tasks();
    if polled != 2 {
        return false;
    }
    if COUNTER_A.load(Ordering::SeqCst) != 1 || COUNTER_B.load(Ordering::SeqCst) != 500 {
        return false;
    }

    // Task A should have re-enqueued itself via yield_now()'s wake_by_ref
    if executor.ready_count() != 1 {
        return false;
    }

    // Step 2: Run ready tasks
    let polled2 = executor.run_ready_tasks();
    if polled2 != 1 || COUNTER_A.load(Ordering::SeqCst) != 11 {
        return false;
    }

    // Step 3: Run ready tasks (task A finishes)
    let polled3 = executor.run_ready_tasks();
    if polled3 != 1 || COUNTER_A.load(Ordering::SeqCst) != 111 {
        return false;
    }

    // All tasks completed
    if executor.ready_count() != 0 {
        return false;
    }

    let stats = executor.stats();
    if stats.total_spawned != 2 || stats.total_completed != 2 {
        return false;
    }

    // Verify global executor single-task polling and TaskHandle (used by AP worker routines and env)
    init_global_executor();
    static GLOBAL_COUNTER: AtomicUsize = AtomicUsize::new(0);
    GLOBAL_COUNTER.store(0, Ordering::SeqCst);

    let handle = spawn_global_handle(async {
        GLOBAL_COUNTER.fetch_add(42, Ordering::SeqCst);
    });

    let handle = match handle {
        Some(h) => h,
        None => return false,
    };

    if handle.is_done() {
        return false;
    }

    if !poll_global_one() {
        return false;
    }
    if GLOBAL_COUNTER.load(Ordering::SeqCst) != 42 {
        return false;
    }
    if !handle.is_done() {
        return false;
    }
    // No more ready tasks in global executor
    if poll_global_one() {
        return false;
    }

    true
}