//! Minimal host-controlled kernel for a container virtual machine.

use crate::vdebug_autoprefix;
use crate::vmm::container::ContainerId;
use crate::vmm::container_engine::ContainerEngine;
use crate::vmm::cvmbus::{CvmBus, CvmBusMessage};
use crate::vmm::interface::{PortIoError, PortIoInterface, PortOwner};
use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CvmKernelError {
    Container(&'static str),
    Port(PortIoError),
    Process(&'static str),
}

pub type CvmProcessId = u64;

const MAX_PROGRAM_BYTES: usize = 64 * 1024;
const MAX_CONTAINER_PROCESSES: usize = 64;
const MAX_CONTAINER_PROGRAM_BYTES: usize = 256 * 1024;
const MAX_PROCESS_MEMORY_BYTES: usize = 1024 * 1024;
const MAX_PROCESS_WRITE_BYTES: usize = 16 * 1024;
const PROCESS_QUANTUM: usize = 64;
const MAX_PROCESS_STEPS: u64 = 1_000_000;
const MAX_SYNCHRONOUS_QUANTA: usize = MAX_PROCESS_STEPS as usize / PROCESS_QUANTUM;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CvmProcessState {
    Runnable,
    Paused,
    Exited(u8),
    Failed,
}

struct CvmProcess {
    container_id: ContainerId,
    program: Vec<u8>,
    memory: Vec<u8>,
    instruction_pointer: usize,
    steps: u64,
    state: CvmProcessState,
}

pub struct CvmKernel {
    pub cvm_id: u32,
    pub bus: CvmBus,
    pub containers: ContainerEngine,
    pub ports: PortIoInterface,
    processes: BTreeMap<CvmProcessId, CvmProcess>,
    ready_queue: VecDeque<CvmProcessId>,
    next_process_id: CvmProcessId,
}

impl CvmKernel {
    pub fn new(cvm_id: u32) -> Self {
        vdebug_autoprefix!("CvmKernel: initialized for CVM {}", cvm_id);
        Self {
            cvm_id,
            bus: CvmBus::new(cvm_id),
            containers: ContainerEngine::new(),
            ports: PortIoInterface::new(PortOwner::Cvm(cvm_id)),
            processes: BTreeMap::new(),
            ready_queue: VecDeque::new(),
            next_process_id: 1,
        }
    }

    /// Load a bounded CVM bytecode program into its own zeroed address space.
    ///
    /// Opcodes: `0x01 STORE8 [address:u16][value:u8]`,
    /// `0x02 WRITE [address:u16][length:u16]`,
    /// `0x03 EXIT [status:u8]`, and `0x04 YIELD`.
    pub fn load_process(
        &mut self,
        container_id: ContainerId,
        program: &[u8],
        memory_bytes: usize,
    ) -> Result<CvmProcessId, CvmKernelError> {
        let container = self
            .containers
            .get(container_id)
            .ok_or(CvmKernelError::Container("container not found"))?;
        if container.state != crate::vmm::container::ContainerState::Running {
            return Err(CvmKernelError::Container(
                "container must be running to load a process",
            ));
        }
        if program.is_empty() || program.len() > MAX_PROGRAM_BYTES {
            return Err(CvmKernelError::Process(
                "process program is empty or exceeds 64 KiB",
            ));
        }
        let container_limit = (container.spec.resources.memory_mb as usize)
            .checked_mul(1024 * 1024)
            .ok_or(CvmKernelError::Process("container memory limit overflows"))?;
        let mut container_process_count = 0usize;
        let mut container_memory = 0usize;
        let mut container_program_bytes = 0usize;
        for process in self
            .processes
            .values()
            .filter(|process| process.container_id == container_id)
        {
            container_process_count = container_process_count
                .checked_add(1)
                .ok_or(CvmKernelError::Process("container process count overflows"))?;
            if matches!(
                process.state,
                CvmProcessState::Runnable | CvmProcessState::Paused
            ) {
                container_memory = container_memory
                    .checked_add(process.memory.len())
                    .ok_or(CvmKernelError::Process("container memory usage overflows"))?;
            }
            container_program_bytes = container_program_bytes
                .checked_add(process.program.len())
                .ok_or(CvmKernelError::Process("container program usage overflows"))?;
        }
        if container_process_count >= MAX_CONTAINER_PROCESSES {
            return Err(CvmKernelError::Process(
                "container process limit of 64 has been reached",
            ));
        }
        if program.len() > MAX_CONTAINER_PROGRAM_BYTES.saturating_sub(container_program_bytes) {
            return Err(CvmKernelError::Process(
                "container program memory limit of 256 KiB has been reached",
            ));
        }
        if memory_bytes == 0
            || memory_bytes > MAX_PROCESS_MEMORY_BYTES
            || memory_bytes > container_limit.saturating_sub(container_memory)
        {
            return Err(CvmKernelError::Process(
                "process memory exceeds container quota or 1 MiB runtime cap",
            ));
        }

        let mut owned_program = Vec::new();
        owned_program
            .try_reserve_exact(program.len())
            .map_err(|_| CvmKernelError::Process("could not allocate process program"))?;
        owned_program.extend_from_slice(program);
        let mut memory = Vec::new();
        memory
            .try_reserve_exact(memory_bytes)
            .map_err(|_| CvmKernelError::Process("could not allocate process memory"))?;
        memory.resize(memory_bytes, 0);

        let id = self.next_process_id;
        self.next_process_id = id
            .checked_add(1)
            .ok_or(CvmKernelError::Process("process ID space exhausted"))?;
        self.processes.insert(
            id,
            CvmProcess {
                container_id,
                program: owned_program,
                memory,
                instruction_pointer: 0,
                steps: 0,
                state: CvmProcessState::Runnable,
            },
        );
        self.ready_queue.push_back(id);
        Ok(id)
    }

    fn execute_builtin_process(
        &mut self,
        container_id: ContainerId,
        command: &str,
    ) -> Result<(), CvmKernelError> {
        let output = self
            .containers
            .builtin_output(container_id, command)
            .map_err(CvmKernelError::Container)?;
        let program_len = output
            .len()
            .checked_mul(4)
            .and_then(|length| length.checked_add(7))
            .ok_or(CvmKernelError::Process("built-in program size overflows"))?;
        let mut program = Vec::new();
        program
            .try_reserve_exact(program_len)
            .map_err(|_| CvmKernelError::Process("could not allocate built-in program"))?;
        for (address, byte) in output.iter().copied().enumerate() {
            let address = u16::try_from(address)
                .map_err(|_| CvmKernelError::Process("built-in output exceeds process memory"))?;
            program.push(0x01);
            program.extend_from_slice(&address.to_le_bytes());
            program.push(byte);
        }
        let output_length = u16::try_from(output.len())
            .map_err(|_| CvmKernelError::Process("built-in output exceeds process memory"))?;
        program.extend_from_slice(&[0x02, 0, 0]);
        program.extend_from_slice(&output_length.to_le_bytes());
        program.extend_from_slice(&[0x03, 0]);

        let process_id = self.load_process(
            container_id,
            &program,
            output.len().max(1).min(MAX_PROCESS_MEMORY_BYTES),
        )?;
        for _ in 0..MAX_SYNCHRONOUS_QUANTA {
            let Some((scheduled_id, state)) = self.run_next_process()? else {
                return Err(CvmKernelError::Process(
                    "scheduler became idle before command completion",
                ));
            };
            if scheduled_id == process_id {
                match state {
                    CvmProcessState::Exited(0) => return Ok(()),
                    CvmProcessState::Exited(_) | CvmProcessState::Failed => {
                        return Err(CvmKernelError::Process("built-in command process failed"));
                    }
                    CvmProcessState::Paused => {
                        return Err(CvmKernelError::Process(
                            "container paused before command completion",
                        ));
                    }
                    CvmProcessState::Runnable => {}
                }
            }
        }
        Err(CvmKernelError::Process(
            "built-in command exceeded the synchronous scheduling budget",
        ))
    }

    /// Run at most one scheduling quantum and return the process state.
    pub fn run_next_process(
        &mut self,
    ) -> Result<Option<(CvmProcessId, CvmProcessState)>, CvmKernelError> {
        let Some(id) = self.ready_queue.pop_front() else {
            return Ok(None);
        };
        let container_id = self
            .processes
            .get(&id)
            .ok_or(CvmKernelError::Process("ready queue references no process"))?
            .container_id;
        let Some(container) = self.containers.get(container_id) else {
            self.remove_container_processes(container_id);
            return Err(CvmKernelError::Container("process container was removed"));
        };
        if container.state != crate::vmm::container::ContainerState::Running {
            let process = self.processes.get_mut(&id).ok_or(CvmKernelError::Process(
                "process disappeared from scheduler",
            ))?;
            process.state = CvmProcessState::Paused;
            return Ok(Some((id, CvmProcessState::Paused)));
        }
        let process = self
            .processes
            .get_mut(&id)
            .ok_or(CvmKernelError::Process("ready queue references no process"))?;
        let mut output = Vec::new();

        for _ in 0..PROCESS_QUANTUM {
            if process.steps >= MAX_PROCESS_STEPS {
                process.state = CvmProcessState::Failed;
                return Err(CvmKernelError::Process(
                    "process instruction budget exhausted",
                ));
            }
            let Some(&opcode) = process.program.get(process.instruction_pointer) else {
                process.state = CvmProcessState::Failed;
                return Err(CvmKernelError::Process(
                    "process instruction pointer is outside its program",
                ));
            };
            process.steps += 1;
            match opcode {
                0x01 => {
                    let Some(operands) = process
                        .program
                        .get(process.instruction_pointer + 1..process.instruction_pointer + 4)
                    else {
                        process.state = CvmProcessState::Failed;
                        return Err(CvmKernelError::Process("truncated STORE8 instruction"));
                    };
                    let address = u16::from_le_bytes([operands[0], operands[1]]) as usize;
                    let Some(destination) = process.memory.get_mut(address) else {
                        process.state = CvmProcessState::Failed;
                        return Err(CvmKernelError::Process(
                            "STORE8 address is outside process memory",
                        ));
                    };
                    *destination = operands[2];
                    process.instruction_pointer += 4;
                }
                0x02 => {
                    let Some(operands) = process
                        .program
                        .get(process.instruction_pointer + 1..process.instruction_pointer + 5)
                    else {
                        process.state = CvmProcessState::Failed;
                        return Err(CvmKernelError::Process("truncated WRITE instruction"));
                    };
                    let address = u16::from_le_bytes([operands[0], operands[1]]) as usize;
                    let length = u16::from_le_bytes([operands[2], operands[3]]) as usize;
                    let Some(end) = address.checked_add(length) else {
                        process.state = CvmProcessState::Failed;
                        return Err(CvmKernelError::Process("WRITE range overflows"));
                    };
                    let Some(bytes) = process.memory.get(address..end) else {
                        process.state = CvmProcessState::Failed;
                        return Err(CvmKernelError::Process(
                            "WRITE range is outside process memory",
                        ));
                    };
                    let Some(output_size) = output.len().checked_add(bytes.len()) else {
                        process.state = CvmProcessState::Failed;
                        return Err(CvmKernelError::Process("process output size overflows"));
                    };
                    if output_size > MAX_PROCESS_WRITE_BYTES {
                        process.state = CvmProcessState::Failed;
                        return Err(CvmKernelError::Process(
                            "process output exceeds the 16 KiB quantum limit",
                        ));
                    }
                    if output.try_reserve(bytes.len()).is_err() {
                        process.state = CvmProcessState::Failed;
                        return Err(CvmKernelError::Process("could not allocate process output"));
                    }
                    output.extend_from_slice(bytes);
                    process.instruction_pointer += 5;
                }
                0x03 => {
                    let Some(&status) = process.program.get(process.instruction_pointer + 1) else {
                        process.state = CvmProcessState::Failed;
                        return Err(CvmKernelError::Process("truncated EXIT instruction"));
                    };
                    process.state = CvmProcessState::Exited(status);
                    break;
                }
                0x04 => {
                    process.instruction_pointer += 1;
                    break;
                }
                _ => {
                    process.state = CvmProcessState::Failed;
                    return Err(CvmKernelError::Process("unknown CVM bytecode opcode"));
                }
            }
        }

        let container_id = process.container_id;
        let state = process.state;
        if !output.is_empty() {
            let container = self
                .containers
                .get_mut(container_id)
                .ok_or(CvmKernelError::Container("process container was removed"))?;
            container.write_terminal(&output).map_err(|error| {
                process.state = CvmProcessState::Failed;
                CvmKernelError::Container(error)
            })?;
        }
        let state = process.state;
        if matches!(state, CvmProcessState::Exited(_) | CvmProcessState::Failed) {
            process.memory = Vec::new();
        }
        if state == CvmProcessState::Runnable {
            self.ready_queue.push_back(id);
        }
        Ok(Some((id, state)))
    }

    pub fn process_state(&self, id: CvmProcessId) -> Option<CvmProcessState> {
        self.processes.get(&id).map(|process| process.state)
    }

    pub fn reap_process(&mut self, id: CvmProcessId) -> Result<(), CvmKernelError> {
        let process = self
            .processes
            .get(&id)
            .ok_or(CvmKernelError::Process("process not found"))?;
        if matches!(
            process.state,
            CvmProcessState::Runnable | CvmProcessState::Paused
        ) {
            return Err(CvmKernelError::Process(
                "cannot reap a runnable or paused process",
            ));
        }
        self.processes.remove(&id);
        self.ready_queue.retain(|queued_id| *queued_id != id);
        Ok(())
    }

    fn pause_container_processes(&mut self, container_id: ContainerId) {
        for process in self.processes.values_mut() {
            if process.container_id == container_id && process.state == CvmProcessState::Runnable {
                process.state = CvmProcessState::Paused;
            }
        }
        self.ready_queue.retain(|id| {
            self.processes
                .get(id)
                .is_some_and(|process| process.container_id != container_id)
        });
    }

    fn resume_container_processes(&mut self, container_id: ContainerId) {
        for (id, process) in &mut self.processes {
            if process.container_id == container_id && process.state == CvmProcessState::Paused {
                process.state = CvmProcessState::Runnable;
                self.ready_queue.push_back(*id);
            }
        }
    }

    fn remove_container_processes(&mut self, container_id: ContainerId) {
        let removed = self
            .processes
            .iter()
            .filter_map(|(id, process)| (process.container_id == container_id).then_some(*id))
            .collect::<Vec<_>>();
        for id in &removed {
            self.processes.remove(id);
        }
        self.ready_queue
            .retain(|id| !removed.iter().any(|removed_id| removed_id == id));
    }
    /// Process one isolated container operation from CVMBUS.
    pub fn service_one(&mut self) -> Result<Option<ContainerId>, CvmKernelError> {
        let Some(message) = self.bus.receive() else {
            return Ok(None);
        };
        vdebug_autoprefix!(
            "CvmKernel: servicing CVMBUS message for CVM {}",
            self.cvm_id
        );
        match message {
            CvmBusMessage::CreateContainer(spec) => self
                .containers
                .create(spec)
                .map(Some)
                .map_err(CvmKernelError::Container),
            CvmBusMessage::StartContainer(id) => {
                self.containers
                    .start(id)
                    .map_err(CvmKernelError::Container)?;
                self.resume_container_processes(id);
                Ok(Some(id))
            }
            CvmBusMessage::StopContainer(id) => {
                self.containers
                    .stop(id)
                    .map_err(CvmKernelError::Container)?;
                self.pause_container_processes(id);
                Ok(Some(id))
            }
            CvmBusMessage::DeleteContainer(id) => {
                self.containers
                    .delete(id)
                    .map_err(CvmKernelError::Container)?;
                self.remove_container_processes(id);
                Ok(Some(id))
            }
            CvmBusMessage::ExecContainer {
                container_id,
                command,
            } => {
                self.execute_builtin_process(container_id, &command)?;
                Ok(Some(container_id))
            }
            CvmBusMessage::PortWrite {
                container_id,
                port,
                data,
            } => {
                let container = self
                    .containers
                    .get_mut(container_id)
                    .ok_or(CvmKernelError::Container("container not found"))?;
                container
                    .ports
                    .write(port, &data)
                    .map_err(CvmKernelError::Port)?;
                Ok(Some(container_id))
            }
        }
    }

    pub fn send_and_service(
        &mut self,
        message: CvmBusMessage,
    ) -> Result<Option<ContainerId>, CvmKernelError> {
        self.bus.send(message);
        self.service_one()
    }

    pub fn execute_container(
        &mut self,
        container_id: ContainerId,
        command: &str,
    ) -> Result<(), CvmKernelError> {
        match self.send_and_service(CvmBusMessage::ExecContainer {
            container_id,
            command: alloc::string::String::from(command),
        })? {
            Some(id) if id == container_id => Ok(()),
            Some(_) => Err(CvmKernelError::Container(
                "CVMBUS serviced an unexpected container",
            )),
            None => Err(CvmKernelError::Container("CVMBUS command was not serviced")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vmm::container::{ContainerResources, ContainerSpec};
    use alloc::string::String;

    fn running_container(kernel: &mut CvmKernel) -> ContainerId {
        let id = kernel
            .containers
            .create(ContainerSpec {
                name: String::from("runtime-test"),
                image: String::from("builtin:test"),
                resources: ContainerResources {
                    cpu_shares: 100,
                    memory_mb: 1,
                },
            })
            .unwrap();
        kernel.containers.start(id).unwrap();
        id
    }

    #[test]
    fn process_memory_and_terminal_syscall_are_isolated_and_bounded() {
        let mut kernel = CvmKernel::new(1);
        let container_id = running_container(&mut kernel);
        let program = [
            0x01, 0, 0, b'h', 0x01, 1, 0, b'i', 0x02, 0, 0, 2, 0, 0x03, 0,
        ];
        let process_id = kernel.load_process(container_id, &program, 64).unwrap();
        assert_eq!(
            kernel.run_next_process().unwrap(),
            Some((process_id, CvmProcessState::Exited(0)))
        );
        assert_eq!(kernel.containers.drain_terminal(container_id, 8), b"hi");
    }

    #[test]
    fn process_cannot_read_or_write_outside_its_private_memory() {
        let mut kernel = CvmKernel::new(1);
        let container_id = running_container(&mut kernel);
        let process_id = kernel
            .load_process(container_id, &[0x01, 64, 0, b'x'], 64)
            .unwrap();
        assert_eq!(
            kernel.run_next_process(),
            Err(CvmKernelError::Process(
                "STORE8 address is outside process memory"
            ))
        );
        assert_eq!(
            kernel.process_state(process_id),
            Some(CvmProcessState::Failed)
        );
        assert!(kernel.ready_queue.is_empty());
    }

    #[test]
    fn container_stop_pauses_and_restart_resumes_its_processes() {
        let mut kernel = CvmKernel::new(1);
        let container_id = running_container(&mut kernel);
        let process_id = kernel
            .load_process(container_id, &[0x04, 0x03, 0], 64)
            .unwrap();
        assert_eq!(
            kernel.run_next_process().unwrap(),
            Some((process_id, CvmProcessState::Runnable))
        );
        kernel
            .send_and_service(CvmBusMessage::StopContainer(container_id))
            .unwrap();
        assert_eq!(
            kernel.process_state(process_id),
            Some(CvmProcessState::Paused)
        );
        assert_eq!(kernel.ready_queue.len(), 0);
        kernel
            .send_and_service(CvmBusMessage::StartContainer(container_id))
            .unwrap();
        assert_eq!(
            kernel.run_next_process().unwrap(),
            Some((process_id, CvmProcessState::Exited(0)))
        );
    }

    #[test]
    fn process_memory_obeys_aggregate_container_quota() {
        let mut kernel = CvmKernel::new(1);
        let container_id = running_container(&mut kernel);
        kernel
            .load_process(container_id, &[0x04], 600 * 1024)
            .unwrap();
        assert_eq!(
            kernel.load_process(container_id, &[0x04], 600 * 1024),
            Err(CvmKernelError::Process(
                "process memory exceeds container quota or 1 MiB runtime cap"
            ))
        );
    }
}
