//! Host-only VM action execution and Action Processor dispatching.

use alloc::vec::Vec;
use crate::hosting::interface::{HostVmBackend, HostVmError};
use crate::hosting::types::{CommandAction, EnvironmentType, InstancePayload};
use crate::vdebug_autoprefix;

pub fn create<B: HostVmBackend>(backend: &mut B, uuid: &str, payload: &InstancePayload) -> Result<u32, HostVmError> {
    create_instance(backend, EnvironmentType::VM, uuid, payload)
}

pub fn create_instance<B: HostVmBackend>(
    backend: &mut B,
    env: EnvironmentType,
    uuid: &str,
    payload: &InstancePayload,
) -> Result<u32, HostVmError> {
    vdebug_autoprefix!("Action Processor: dispatching CREATE for env={:?}, UUID={}", env, uuid);
    let InstancePayload::CfgPayload(config) = payload else {
        return Err(HostVmError::InvalidConfiguration("create requires configuration payload"));
    };
    backend.create_with_env(env, uuid, config)
}

pub fn execute_existing<B: HostVmBackend>(backend: &mut B, vm_id: u32, action: &CommandAction) -> Result<(), HostVmError> {
    execute_action(backend, vm_id, action, &InstancePayload::Empty).map(|_| ())
}

pub fn execute_action<B: HostVmBackend>(
    backend: &mut B,
    vm_id: u32,
    action: &CommandAction,
    payload: &InstancePayload,
) -> Result<Vec<u8>, HostVmError> {
    vdebug_autoprefix!("Action Processor: executing {:?} on VM ID {}", action, vm_id);
    match action {
        CommandAction::Start => {
            backend.start(vm_id)?;
            Ok(Vec::new())
        }
        CommandAction::Stop => {
            backend.stop(vm_id)?;
            Ok(Vec::new())
        }
        CommandAction::Restart => {
            backend.stop(vm_id)?;
            backend.start(vm_id)?;
            Ok(Vec::new())
        }
        CommandAction::Delete => {
            backend.delete(vm_id)?;
            Ok(Vec::new())
        }
        CommandAction::LoadISO { iso_path } => {
            backend.load_iso(vm_id, iso_path)?;
            Ok(Vec::new())
        }
        CommandAction::Terminal => {
            // Forward terminal keystrokes / input to VirtCOM IN_BUF and retrieve OUT_BUF
            if let InstancePayload::TerminalData(terminal) = payload {
                if !terminal.data.is_empty() {
                    let _ = backend.terminal_write(vm_id, &terminal.data);
                }
            }
            let response_bytes = backend.terminal_read(vm_id, 1024).unwrap_or_default();
            Ok(response_bytes)
        }
        CommandAction::Create => {
            Err(HostVmError::InvalidConfiguration("create is not an existing-instance action"))
        }
    }
}
