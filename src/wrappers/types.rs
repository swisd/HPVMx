use alloc::alloc::{alloc, dealloc, Layout};
use core::cell::Cell;
use core::ops::Deref;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicUsize, Ordering};

pub trait RefCount {
    fn new(val: usize) -> Self;
    fn increment(&self);
    fn decrement(&self) -> bool;
}

pub struct CellCounter(Cell<usize>);
impl RefCount for CellCounter {
    fn new(val: usize) -> Self { Self(Cell::new(val)) }
    fn increment(&self) { self.0.set(self.0.get() + 1); }
    fn decrement(&self) -> bool {
        let next = self.0.get() - 1;
        self.0.set(next);
        next == 0
    }
}

pub struct AtomicCounter(AtomicUsize);
impl RefCount for AtomicCounter {
    fn new(val: usize) -> Self { Self(AtomicUsize::new(val)) }
    fn increment(&self) { self.0.fetch_add(1, Ordering::Relaxed); }
    fn decrement(&self) -> bool {
        self.0.fetch_sub(1, Ordering::AcqRel) == 1
    }
}


struct HybridBox<T, C: RefCount> {
    strong: C,
    value: T,
}

pub struct HybridPtr<T, C: RefCount> {
    ptr: NonNull<HybridBox<T, C>>,
}


pub type NonAtomicRc<T> = HybridPtr<T, CellCounter>;
pub type AtomicArc<T> = HybridPtr<T, AtomicCounter>;

impl<T, C: RefCount> HybridPtr<T, C> {
    pub fn new(value: T) -> Self {
        let layout = Layout::new::<HybridBox<T, C>>();
        unsafe {
            let raw_ptr = alloc(layout) as *mut HybridBox<T, C>;
            let ptr = NonNull::new(raw_ptr).expect("Allocation failed");

            ptr.as_ptr().write(HybridBox {
                strong: C::new(1),
                value,
            });

            Self { ptr }
        }
    }
}

impl<T, C: RefCount> Deref for HybridPtr<T, C> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &self.ptr.as_ref().value }
    }
}

impl<T, C: RefCount> Clone for HybridPtr<T, C> {
    fn clone(&self) -> Self {
        unsafe {
            // Increment the custom counter representation
            self.ptr.as_ref().strong.increment();
        }
        Self { ptr: self.ptr }
    }
}

impl<T, C: RefCount> Drop for HybridPtr<T, C> {
    fn drop(&mut self) {
        unsafe {
            let inner = self.ptr.as_ref();

            // Decrement and check if we are the last owner
            if inner.strong.decrement() {
                // Fully drop the inner value first
                core::ptr::drop_in_place(&mut (*self.ptr.as_ptr()).value);

                // Deallocate the backing heap block
                let layout = Layout::new::<HybridBox<T, C>>();
                dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}


unsafe impl<T: Send + Sync> Send for HybridPtr<T, AtomicCounter> {}
unsafe impl<T: Send + Sync> Sync for HybridPtr<T, AtomicCounter> {}
