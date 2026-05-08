# HPVMx Operator Manual

## Dashboard

HPVMx opens to a shell. Run `dashboard` to enter the graphical management console.

Use the top navigation keys:

```text
O Overview    V VMs        R Resources   S Storage
N Network     D Devices    C Console     Z Settings
P Packages    A Apps
```

## Virtual Machines

The VM tab manages VM definitions and lifecycle state.

```text
SPACE          create a VM
UP/DOWN        select a VM
LEFT/RIGHT     select an action
ENTER          run the selected action
```

Actions include start, stop, reset, zero, delete, save, and restore. Save and restore use `/VMSTATE` for VM metadata.

## Storage

The Storage tab is the file explorer.

```text
UP/DOWN        select an item
LEFT/RIGHT     select an action
END            run or confirm an action
ESC            cancel a pending action
```

The properties panel shows name, type, size, path, and selection index. Rename, copy, move, and delete require confirmation.

## Settings

Settings are organized by category. Some rows publish environment-style values for other HPVMx subsystems.

```text
LEFT/RIGHT     change category
UP/DOWN        select a row
ENTER          toggle or cycle the selected value
```

Examples:

```text
HPVMX_PROFILE=balanced
HPVMX_BOOT_TARGET=dashboard
HPVMX_NET_PROFILE=dhcp
HPVMX_PM_VERIFY=standard
```

## Packages

The Packages tab lists packages loaded by the package manager registry. It can refresh the registry, verify dependencies, remove a package from the active registry, and mark a package for update.

## Network

The Network tab exposes NIC and stack actions:

```text
Net Up      initialize SNP networking
Status      print NIC status
Ping        ping the current target
LAN Scan    scan the default LAN prefix
HTTP On     start HTTP management listener
HTTP Off    stop HTTP management listener
```

## Micro-C IDE

Open the Apps tab and launch `MicroIDE`.

```text
F5       compile current source
F6       cycle target architecture
F7       clear output
UP/DOWN  scroll source
```

Targets currently include `x86_64`, `win64`, and `arm64`. The right pane displays generated assembly or compiler diagnostics.

## Shell Commands

Useful shell commands:

```text
dashboard
help fs
help vm
help net
pm list
micro-c compile /path/file.micro
```
