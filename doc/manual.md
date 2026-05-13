# HPVMx Operator Manual

## Introduction

HPVMx is a bare-metal environment for virtualization and system management. It boots into a command-line shell by default, from which you can launch the graphical Dashboard or execute system commands.

## Shell Basics

The shell supports standard file system operations and system management commands.

- **Help**: Type `help` to see command categories. Use `help <category>` (e.g., `help fs`) for specific details.
- **Dashboard**: Run `dashboard` to enter the graphical user interface.

### Common Commands
- `ls`: List files in the current directory.
- `cd <dir>`: Change the current working directory.
- `cat <file>`: Display the contents of a file.
- `info`: Display basic system information.
- `shutdown [s|r]`: Shutdown (`s`) or Reboot (`r`).

## Dashboard Navigation

The Dashboard is the primary interface for managing the system. Use the following hotkeys to switch between tabs:

```text
O Overview    V VMs        R Resources   S Storage
N Network     D Devices    C Console     Z Settings
P Packages    A Apps
```

### Virtual Machines (V)

Manage VM definitions and their lifecycle.
- **SPACE**: Create a new VM.
- **UP/DOWN**: Select a VM from the list.
- **LEFT/RIGHT**: Select an action (Start, Stop, Reset, Delete, Save, Restore).
- **ENTER**: Execute the selected action.
- **Save/Restore**: VM metadata is saved to `/VMSTATE`.

### Storage (S)

A graphical file explorer.
- **UP/DOWN**: Navigate files and folders.
- **LEFT/RIGHT**: Select an action (Rename, Copy, Move, Delete).
- **END**: Confirm the selected action.
- **ESC**: Cancel a pending action.
- The properties panel displays metadata for the selected item.

### Network (N)

Control the UEFI Simple Network Protocol (SNP) stack.
- **Net Up**: Initialize the NIC.
- **Status**: View current NIC configuration.
- **Ping**: Test network connectivity.
- **HTTP On/Off**: Toggle the integrated HTTP management listener.

### Packages (P)

Manage system extensions and libraries.
- **Refresh**: Reload the package registry.
- **Verify**: Check for missing dependencies.
- **Update**: Mark packages for the next update cycle.

### Settings (Z)

Configure runtime environment variables.
- **UP/DOWN**: Select a setting.
- **ENTER**: Toggle or cycle through available values.
- Key variables include `HPVMX_PROFILE`, `HPVMX_BOOT_TARGET`, and `HPVMX_NET_PROFILE`.

## Developer Tools

### Micro-C IDE

Accessible via the **Apps** tab. Provides a lightweight environment for C development.
- **F5**: Compile the current source buffer.
- **F6**: Cycle through target architectures (`x86_64`, `win64`, `arm64`).
- **F7**: Clear the output/diagnostics pane.
- **UP/DOWN**: Scroll through the source code.

### Shell Compilation
You can also compile files directly from the shell:
```text
micro-c compile /path/to/source.micro
```
This generates a corresponding `.asm` file in the same directory.

## Troubleshooting

- **No Network**: Ensure `net up` has been run and a compatible NIC is detected.
- **VM Fails to Start**: Check available memory in the **Resources** tab.
- **File System Errors**: Use `help fs` in the shell to verify correct command syntax.
