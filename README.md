# HPVMx

A bare-metal (+BIOS&EFI) hardware provisioning and virtualization manager written in rust.


It *might* run DOOM.

## Current Surface

- Hypervisor dashboard with VM lifecycle controls and VM metadata save/restore.
- Storage explorer with properties, create file/folder, rename, copy, move, and delete confirmation.
- Network dashboard actions for SNP init, status, ping, LAN scan, and HTTP management listener.
- Package manager UI for registry refresh, dependency verification, uninstall, and update marking.
- Settings categories that publish runtime environment values such as `HPVMX_PROFILE`, `HPVMX_BOOT_TARGET`, and `HPVMX_NET_PROFILE`.
- Micro-C developer toolchain app (`MicroIDE`) with source editing, architecture target selection, and assembly output.

## Running

Build with Cargo, then boot the generated EFI payload in the project QEMU/UEFI environment.

```powershell
cargo build
.\qemu-start.ps1
```

Inside HPVMx, run:

```text
dashboard
```

## Micro-C

The Micro-C compiler can be used from the shell:

```text
micro-c compile /path/source.micro
```

Or launch `MicroIDE` from the Apps tab:

```text
F5       compile source
F6       cycle target architecture
F7       clear output
UP/DOWN  scroll source
```

See [doc/manual.md](doc/manual.md) for the in-system operator manual.

![image](/src/hpvmx_image.png)

![](/doc/img/term01.png)
![](/doc/img/dash01.png)
![](/doc/img/dash02.png)
![](/doc/img/dash03.png)
![](/doc/img/dash04.png)


[//]: # (Theoretical Final Flowchart &#40;what the finished/mostly complete product will be like&#41;)

[//]: # (![]&#40;img_7.png&#41;)
