# HPVMx

A bare-metal (+BIOS & EFI) hardware provisioning and virtualization manager written in Rust. HPVMx provides a complete environment for managing virtual machines, exploring storage, and developing software on bare metal.

![](https://img.shields.io/badge/latest_version-1.9.12-blue)\
![](https://img.shields.io/badge/supported_version*-1.9.8-green)
> *supported version is the oldest verion that does not have to be updated to the newest version

## Key Features

- **Hypervisor Dashboard**: Full VM lifecycle management with save/restore capabilities.
- **Storage Explorer**: Comprehensive file system management (create, rename, copy, move, delete).
- **Network Stack**: SNP-based networking with ping, LAN scanning, and an integrated HTTP management listener.
- **Package Manager**: Registry-based package management with dependency verification and updates.
- **Micro-C Toolchain**: A built-in C-to-Assembly compiler and IDE (`MicroIDE`) for bare-metal development.
- **Settings Registry**: Dynamic environment configuration through a structured settings UI.

## Getting Started

### Prerequisites

- Rust toolchain (nightly recommended)
- QEMU/VirtualBox (for emulation) or physical machine
- OVMF/UEFI firmware (included as `code.fd` and `vars.fd`)

### Building and Running


#### Virtual Hardware
0. Navigate to project directory
   ```powershell
   cd /path/to/HPVMx/
   ```

1. Create A VHD:
   1. Use Disk management to create a VHD of size `256MB`, labeled boot.vhd and placed in the project directory.
   2. Create a new volume on the entire size of the disk, formatted in `FAT32`, and label it with drive letter `X:`


2. Build the project using Cargo:
   ```powershell
   .\install.ps1
   ```

2. Launch HPVMx in QEMU:
   ```powershell
   .\qemu-start.ps1
   ```
   
2. Using another hypervisor
   - For virtualbox, add the vhd as the only drive in the storage section, and enable uefi.

#### Physical Hardware
0. Format a USB drive (>256MB) with FAT32, and label the volume X: in Disk Management if not already.

0. Navigate to project directory
   ```powershell
   cd /path/to/HPVMx/
   ```

2. Build the project using Cargo:
   ```powershell
   .\install-phy.ps1
   ```
   
3. On Target Device: 
   - Disable Secure Boot
   - Set USB to top of boot order
   - Insert USB
   - Start/Restart Computer

### First Steps

Once HPVMx boots into the shell:
- Type `help` to see available commands.
- Type `dashboard` to enter the graphical management console.
- Use `O, V, R, S, N, D, C, Z, P, A` keys to navigate dashboard tabs.

## Developer Tools

### Micro-C Compiler
Compile C source files directly from the shell:
```text
micro-c compile /path/to/source.micro
```

### MicroIDE
Launch `MicroIDE` from the **Apps** tab in the dashboard.
- **F5**: Compile source
- **F6**: Cycle target architecture (x86_64, win64, arm64)
- **F7**: Clear output
- **UP/DOWN**: Scroll source code

## Documentation

- [Operator Manual](doc/manual.md): Detailed usage instructions for the dashboard and shell.
- [Development Manual](doc/development.md): Guide for creating apps and extensions for HPVMx.
- [Project Architecture](hpvmx.md): Deep dive into the internal design of HPVMx.

## Planned Additions and Removals

### Upcoming Additions


- `EfiVirtualizedContext`
- `VirtualizedRuntime`
- vm iso loading
- vm `WindowedDisplay`
- vm terminal connection
- micro-c jit compiler
- micro-c live code
- `SteppedApplicationContext` improvements
- hot-reload xml based ui formatting for micro-c apps and future applications

### Deprecations


- `ApplicationContext`
- command `run-efi`
- command `load-into`
- command `start`
- command `run-app`


## Gallery

![Storage-Old](/doc/img/dash01.png)
*Storage Interface* **(OLD)[0.9.13]**

![Terminal](/doc/img/term01.png)
*Interactive Shell* **[1.3.12]**

![Terminal](/doc/img/img.png)
*Resource Monitor* **[1.9.8]**

![Terminal](/doc/img/img_1.png)
*Storage UI* **[1.9.8]**

![Terminal](/doc/img/img_2.png)
*Network UI* **[1.9.8]**

![Terminal](/doc/img/img_3.png)
*Package UI* **[1.9.8]**

[//]: # (Theoretical Final Flowchart)
[//]: # (![]&#40;img_7.png&#41;)
