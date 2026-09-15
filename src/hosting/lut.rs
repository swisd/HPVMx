//! Stable external-instance UUID to local hypervisor-ID lookup table,
//! Client Data LUT, Container ID (CID) LUT, and Port LUT.

use alloc::collections::BTreeMap;
use alloc::string::String;
use crate::hosting::types::{ClientData, TidUuid};
use crate::vdebug_autoprefix;

#[derive(Default, Debug, Clone)]
pub struct InstanceLut {
    entries: BTreeMap<String, u32>,
}

impl InstanceLut {
    pub fn new() -> Self { Self::default() }

    pub fn bind(&mut self, uuid: &str, vm_id: u32) -> Result<(), &'static str> {
        let tid_uuid = TidUuid::parse(uuid)?;
        if !is_canonical_uuid(&tid_uuid.uuid) && !is_short_uuid(&tid_uuid.uuid) {
            return Err("instance UUID is not valid");
        }
        if self.entries.contains_key(uuid) {
            return Err("instance UUID is already bound");
        }
        vdebug_autoprefix!("InstanceLut: binding {} -> vm_id {}", uuid, vm_id);
        self.entries.insert(String::from(uuid), vm_id);
        // Also map just the UUID part for convenience
        if !self.entries.contains_key(&tid_uuid.uuid) {
            self.entries.insert(tid_uuid.uuid.clone(), vm_id);
        }
        Ok(())
    }

    pub fn resolve(&self, uuid: &str) -> Option<u32> {
        if let Some(id) = self.entries.get(uuid).copied() {
            return Some(id);
        }
        if let Ok(tid_uuid) = TidUuid::parse(uuid) {
            return self.entries.get(&tid_uuid.uuid).copied();
        }
        None
    }

    pub fn unbind(&mut self, uuid: &str) -> Option<u32> {
        vdebug_autoprefix!("InstanceLut: unbinding {}", uuid);
        let removed = self.entries.remove(uuid);
        if let Ok(tid_uuid) = TidUuid::parse(uuid) {
            self.entries.remove(&tid_uuid.uuid);
        }
        removed
    }

    pub fn len(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
}

/// Client Data LUT: Maps light token / client_token and verification_id to ClientData.
#[derive(Default, Debug, Clone)]
pub struct ClientDataLut {
    token_entries: BTreeMap<String, ClientData>,
    verification_entries: BTreeMap<String, String>, // verification_id -> client_token
}

impl ClientDataLut {
    pub fn new() -> Self { Self::default() }

    pub fn register(&mut self, data: ClientData) {
        vdebug_autoprefix!("ClientDataLut: registering client_token='{}' for tid='{}' uuid='{}'",
            data.client_token, data.tid, data.inst_uuid);
        self.verification_entries.insert(data.verification_id.clone(), data.client_token.clone());
        self.token_entries.insert(data.client_token.clone(), data);
    }

    pub fn get_by_token(&self, token: &str) -> Option<&ClientData> {
        self.token_entries.get(token)
    }

    pub fn get_by_verification_id(&self, ver_id: &str) -> Option<&ClientData> {
        let token = self.verification_entries.get(ver_id)?;
        self.token_entries.get(token)
    }

    pub fn remove_by_token(&mut self, token: &str) -> Option<ClientData> {
        if let Some(data) = self.token_entries.remove(token) {
            vdebug_autoprefix!("ClientDataLut: removing client_token='{}'", token);
            self.verification_entries.remove(&data.verification_id);
            Some(data)
        } else {
            None
        }
    }
}

/// CID LUT: Maps Container ID / Client ID <-> VM ID / Instance UUID.
#[derive(Default, Debug, Clone)]
pub struct CidLut {
    cid_to_vmid: BTreeMap<u64, u32>,
    cid_to_uuid: BTreeMap<u64, String>,
}

impl CidLut {
    pub fn new() -> Self { Self::default() }

    pub fn bind_container(&mut self, cid: u64, vm_id: u32, uuid: &str) {
        vdebug_autoprefix!("CidLut: binding CID {} -> VM {} (UUID: {})", cid, vm_id, uuid);
        self.cid_to_vmid.insert(cid, vm_id);
        self.cid_to_uuid.insert(cid, String::from(uuid));
    }

    pub fn resolve_vm_id(&self, cid: u64) -> Option<u32> {
        self.cid_to_vmid.get(&cid).copied()
    }

    pub fn resolve_uuid(&self, cid: u64) -> Option<&str> {
        self.cid_to_uuid.get(&cid).map(|s| s.as_str())
    }

    pub fn unbind_container(&mut self, cid: u64) {
        vdebug_autoprefix!("CidLut: unbinding CID {}", cid);
        self.cid_to_vmid.remove(&cid);
        self.cid_to_uuid.remove(&cid);
    }
}

/// Port LUT: Maps external VM Port (Reverse Proxy) <-> Port of OS (guest / container port).
#[derive(Default, Debug, Clone)]
pub struct PortLut {
    /// Maps external host_port -> (vm_id, guest_port)
    host_to_guest: BTreeMap<u16, (u32, u16)>,
    /// Maps (vm_id, guest_port) -> host_port
    guest_to_host: BTreeMap<(u32, u16), u16>,
}

impl PortLut {
    pub fn new() -> Self { Self::default() }

    pub fn map_port(&mut self, vm_id: u32, host_port: u16, guest_port: u16) -> Result<(), &'static str> {
        if self.host_to_guest.contains_key(&host_port) {
            return Err("host port is already allocated in Port LUT");
        }
        vdebug_autoprefix!("PortLut: mapping host port {} <-> VM {} port {}", host_port, vm_id, guest_port);
        self.host_to_guest.insert(host_port, (vm_id, guest_port));
        self.guest_to_host.insert((vm_id, guest_port), host_port);
        Ok(())
    }

    pub fn resolve_guest_port(&self, host_port: u16) -> Option<(u32, u16)> {
        self.host_to_guest.get(&host_port).copied()
    }

    pub fn resolve_host_port(&self, vm_id: u32, guest_port: u16) -> Option<u16> {
        self.guest_to_host.get(&(vm_id, guest_port)).copied()
    }

    pub fn unmap_vm_ports(&mut self, vm_id: u32) {
        vdebug_autoprefix!("PortLut: unmapping all ports for VM {}", vm_id);
        let ports_to_remove: alloc::vec::Vec<u16> = self.host_to_guest.iter()
            .filter(|(_, (v, _))| *v == vm_id)
            .map(|(&k, _)| k)
            .collect();
        for hp in ports_to_remove {
            if let Some((v, gp)) = self.host_to_guest.remove(&hp) {
                self.guest_to_host.remove(&(v, gp));
            }
        }
    }
}

pub fn is_canonical_uuid(value: &str) -> bool {
    value.len() == 36 && value.bytes().enumerate().all(|(index, byte)| match index {
        8 | 13 | 18 | 23 => byte == b'-',
        _ => byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte),
    })
}

pub fn is_short_uuid(value: &str) -> bool {
    // Diagram format: a1b2-c3d4-e5f6
    value.len() == 14 && value.bytes().enumerate().all(|(index, byte)| match index {
        4 | 9 => byte == b'-',
        _ => byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte),
    })
}
