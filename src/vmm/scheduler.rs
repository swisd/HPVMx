//! Round-robin scheduler for VMX-backed virtual CPUs.

use crate::hardware::cpu::vmx::VmxRunOutcome;
use alloc::collections::{BTreeMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduledVcpuState {
    Runnable,
    Halted,
    Failed,
}

pub struct VcpuScheduler {
    ready: VecDeque<(u32, u32)>,
    states: BTreeMap<(u32, u32), ScheduledVcpuState>,
}

impl VcpuScheduler {
    pub fn new() -> Self {
        Self {
            ready: VecDeque::new(),
            states: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, vm_id: u32, vcpu_id: u32) -> Result<(), &'static str> {
        let key = (vm_id, vcpu_id);
        if self.states.contains_key(&key) {
            return Err("vCPU is already registered with the scheduler");
        }
        self.states.insert(key, ScheduledVcpuState::Runnable);
        self.ready.push_back(key);
        Ok(())
    }

    pub fn wake(&mut self, vm_id: u32, vcpu_id: u32) -> Result<(), &'static str> {
        let key = (vm_id, vcpu_id);
        let state = self
            .states
            .get_mut(&key)
            .ok_or("vCPU is not registered with the scheduler")?;
        if *state == ScheduledVcpuState::Failed {
            return Err("failed vCPU cannot be woken");
        }
        if *state == ScheduledVcpuState::Runnable {
            return Err("vCPU is already runnable");
        }
        *state = ScheduledVcpuState::Runnable;
        self.ready.push_back(key);
        Ok(())
    }

    pub fn state(&self, vm_id: u32, vcpu_id: u32) -> Option<ScheduledVcpuState> {
        self.states.get(&(vm_id, vcpu_id)).copied()
    }

    /// Run one VM-exit-timer quantum. The closure must load and run the selected
    /// VMCS synchronously on its owning pinned logical processor.
    pub fn schedule_once(
        &mut self,
        mut run_quantum: impl FnMut(u32, u32) -> Result<VmxRunOutcome, &'static str>,
    ) -> Result<Option<(u32, u32, VmxRunOutcome)>, &'static str> {
        let Some(key) = self.ready.pop_front() else {
            return Ok(None);
        };
        match run_quantum(key.0, key.1) {
            Ok(outcome @ VmxRunOutcome::Yielded) => {
                self.ready.push_back(key);
                Ok(Some((key.0, key.1, outcome)))
            }
            Ok(outcome @ VmxRunOutcome::Halted) => {
                self.states.insert(key, ScheduledVcpuState::Halted);
                Ok(Some((key.0, key.1, outcome)))
            }
            Ok(outcome @ (VmxRunOutcome::Fatal | VmxRunOutcome::EntryFailed)) => {
                self.states.insert(key, ScheduledVcpuState::Failed);
                Ok(Some((key.0, key.1, outcome)))
            }
            Err(error) => {
                self.states.insert(key, ScheduledVcpuState::Failed);
                Err(error)
            }
        }
    }

    pub fn runnable_count(&self) -> usize {
        self.ready.len()
    }
}

impl Default for VcpuScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timer_yields_round_robin_and_halted_vcpus_require_wakeup() {
        let mut scheduler = VcpuScheduler::new();
        scheduler.register(1, 0).unwrap();
        scheduler.register(2, 0).unwrap();

        assert_eq!(
            scheduler.schedule_once(|_, _| Ok(VmxRunOutcome::Yielded)),
            Ok(Some((1, 0, VmxRunOutcome::Yielded)))
        );
        assert_eq!(
            scheduler.schedule_once(|_, _| Ok(VmxRunOutcome::Halted)),
            Ok(Some((2, 0, VmxRunOutcome::Halted)))
        );
        assert_eq!(scheduler.state(2, 0), Some(ScheduledVcpuState::Halted));
        assert_eq!(scheduler.runnable_count(), 1);
        scheduler.wake(2, 0).unwrap();
        assert_eq!(scheduler.runnable_count(), 2);
    }

    #[test]
    fn failed_vcpus_are_not_rescheduled() {
        let mut scheduler = VcpuScheduler::new();
        scheduler.register(5, 0).unwrap();
        assert_eq!(
            scheduler.schedule_once(|_, _| Ok(VmxRunOutcome::Fatal)),
            Ok(Some((5, 0, VmxRunOutcome::Fatal)))
        );
        assert_eq!(scheduler.runnable_count(), 0);
        assert_eq!(scheduler.wake(5, 0), Err("failed vCPU cannot be woken"));
    }
}
