# HPVMx

A bare-metal (+BIOS & EFI) hardware provisioning and virtualization manager written in Rust. HPVMx provides a complete environment for managing virtual machines, exploring storage, and developing software on bare metal.

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
- QEMU (for emulation)
- OVMF/UEFI firmware (included as `code.fd` and `vars.fd`)

### Building and Running

1. Build the project using Cargo:
   ```powershell
   cargo build
   ```

2. Launch HPVMx in QEMU:
   ```powershell
   .\qemu-start.ps1
   ```

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
- [Project Architecture](hpvmx.md): Deep dive into the internal design of HPVMx.

## Gallery

![Hypervisor Dashboard](/doc/img/dash01.png)
*VM Management Interface*

![Terminal](/doc/img/term01.png)
*Interactive Shell*

[//]: # (Theoretical Final Flowchart)
[//]: # (![]&#40;img_7.png&#41;)
