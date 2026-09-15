//! Request routing, Client verification, and UUID/Port/CID lifecycle bookkeeping.

use alloc::string::String;
use alloc::vec::Vec;
use crate::hosting::ap;
use crate::hosting::interface::{HostVmBackend, HostVmError};
use crate::hosting::lut::{CidLut, ClientDataLut, InstanceLut, PortLut};
use crate::hosting::types::{CommandAction, IPCMessage, InstancePayload, TidUuid};
use crate::vdebug_autoprefix;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestResult {
    pub tid: String,
    pub instance_uuid: String,
    pub vm_id: u32,
    pub output: Option<Vec<u8>>,
}

pub struct RequestProcessor<B> {
    backend: B,
    lut: InstanceLut,
    client_data_lut: ClientDataLut,
    cid_lut: CidLut,
    port_lut: PortLut,
}

impl<B: HostVmBackend> RequestProcessor<B> {
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            lut: InstanceLut::new(),
            client_data_lut: ClientDataLut::new(),
            cid_lut: CidLut::new(),
            port_lut: PortLut::new(),
        }
    }

    pub fn instance_count(&self) -> usize { self.lut.len() }
    pub fn lookup(&self, uuid: &str) -> Option<u32> { self.lut.resolve(uuid) }

    pub fn client_data_lut(&self) -> &ClientDataLut { &self.client_data_lut }
    pub fn client_data_lut_mut(&mut self) -> &mut ClientDataLut { &mut self.client_data_lut }

    pub fn cid_lut(&self) -> &CidLut { &self.cid_lut }
    pub fn cid_lut_mut(&mut self) -> &mut CidLut { &mut self.cid_lut }

    pub fn port_lut(&self) -> &PortLut { &self.port_lut }
    pub fn port_lut_mut(&mut self) -> &mut PortLut { &mut self.port_lut }

    pub fn process(&mut self, request: &IPCMessage) -> Result<RequestResult, HostVmError> {
        let tid_uuid = TidUuid::new(&request.request.tid, &request.request.inst_uuid);
        let combined_id = tid_uuid.formatted();
        vdebug_autoprefix!("RequestProcessor: processing {:?} for identifier '{}' (env: {:?})",
            request.action, combined_id, request.environment);

        // If client_token is provided in header, verify it against ClientDataLut if populated
        if let Some(token) = &request.request.client_token {
            if let Some(data) = self.client_data_lut.get_by_token(token) {
                if !data.tid.is_empty() && data.tid != request.request.tid {
                    vdebug_autoprefix!("RequestProcessor: Client token TID mismatch: {} vs {}", data.tid, request.request.tid);
                    return Err(HostVmError::InvalidConfiguration("client token does not match request tenant"));
                }
            }
        }

        let mut output_data: Option<Vec<u8>> = None;

        let vm_id = if request.action == CommandAction::Create {
            let created = ap::create_instance(&mut self.backend, request.environment, &request.request.inst_uuid, &request.payload)?;
            if let Err(error) = self.lut.bind(&combined_id, created) {
                let _ = self.backend.delete(created);
                return Err(HostVmError::InvalidConfiguration(error));
            }
            // If configuration specifies port mappings, register them in Port LUT
            if let InstancePayload::CfgPayload(cfg) = &request.payload {
                for &(host_port, guest_port) in &cfg.portmappings {
                    if let Err(err) = self.port_lut.map_port(created, host_port, guest_port) {
                        vdebug_autoprefix!("Port mapping warning: {}", err);
                    }
                }
            }
            created
        } else {
            let id = self.lut.resolve(&combined_id)
                .or_else(|| self.lut.resolve(&request.request.inst_uuid))
                .ok_or(HostVmError::InvalidConfiguration("unknown instance UUID"))?;

            let action_out = ap::execute_action(&mut self.backend, id, &request.action, &request.payload)?;
            if !action_out.is_empty() {
                output_data = Some(action_out);
            }

            if request.action == CommandAction::Delete {
                self.lut.unbind(&combined_id);
                self.lut.unbind(&request.request.inst_uuid);
                self.port_lut.unmap_vm_ports(id);
            }
            id
        };

        Ok(RequestResult {
            tid: request.request.tid.clone(),
            instance_uuid: request.request.inst_uuid.clone(),
            vm_id,
            output: output_data,
        })
    }

    pub fn into_backend(self) -> B { self.backend }
}
