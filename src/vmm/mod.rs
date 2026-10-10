//! HPVMx Hypervisor - Virtual Machine Monitor subsystem

pub mod bootloader;
pub mod devices;
pub mod io;
pub(crate) mod loader;
pub mod memory;
pub mod storage;
pub mod vcpu;
pub mod vm;
pub mod vmm;

pub mod container;
pub mod container_engine;
pub mod cvm;
pub mod cvm_kernel;
pub mod cvmbus;
pub mod ept;
pub mod ghm;
pub mod hwbus;
pub mod interface;
pub mod mapper;
pub mod partitioner;
pub mod scheduler;
pub mod security;
pub mod vmbus;
pub mod vmcs;
pub mod vmexit;

pub use vmm::HypervisorManager;
