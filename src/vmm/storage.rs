//! Virtual storage device management for VMs

use alloc::string::String;
use alloc::vec::Vec;

#[allow(dead_code)]
pub struct StorageManager {
    disks: Vec<VirtualDisk>,
}

#[allow(dead_code)]
pub struct VirtualDisk {
    pub id: u32,
    pub vm_id: u32,
    pub name: String,
    pub size_mb: u32,
    pub disk_type: DiskType,
    pub image_path: String,
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum DiskType {
    Qcow2,
    RawImage,
    VDI,
    VMDK,
}

#[allow(dead_code)]
impl StorageManager {
    pub fn new() -> Self {
        Self {
            disks: Vec::new(),
        }
    }

    pub fn attach_disk(&mut self, vm_id: u32, image_path: &str, size_mb: u32)
                       -> Result<u32, &str>
    {
        let disk = VirtualDisk {
            id: self.disks.len() as u32,
            vm_id,
            name: String::from("disk0"),
            size_mb,
            disk_type: DiskType::RawImage,
            image_path: String::from(image_path),
        };
        self.disks.push(disk);
        Ok(self.disks.len() as u32 - 1)
    }

    pub fn get_vm_disks(&self, vm_id: u32) -> Vec<&VirtualDisk> {
        self.disks.iter().filter(|d| d.vm_id == vm_id).collect()
    }
}