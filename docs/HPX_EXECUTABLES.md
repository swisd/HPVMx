# HPX disk executables (ABI v1)

HPVMx loads trusted x86-64 position-independent machine-code images from the UEFI filesystem. Images must use the Windows x64 calling convention used by the x86-64 UEFI target. Applications and background workers are separate files and do not require rebuilding the OS. They execute in the kernel address space; v1 does not isolate or sandbox plugins. Only run code you trust.

## File layout

A `.hpx` file consists of a 40-byte little-endian header followed by `image_size` bytes of code/data. An optional `HPR1` relocation block follows the image. The loader validates each import and patches its 64-bit address at load time, allowing the same executable to run after kernel relocation. The built-in linker currently supports the Win64 assembly subset emitted by the C frontend and the `hpx_*` imports exposed by `src/micro_c_externs.rs`.

| Offset | Width | Field |
| ---: | ---: | --- |
| 0 | 4 | ASCII `HPX1` |
| 4 | 2 | ABI version (`1`) |
| 6 | 2 | Kind: `1` stepped app, `2` stepped background task |
| 8 | 4 | Header size (`40`) |
| 12 | 4 | Image size in bytes |
| 16 | 4 | Zero-initialized state size in bytes (maximum 16 MiB) |
| 20 | 4 | Required step callback offset in image |
| 24 | 4 | Optional draw callback offset (`0` means absent) |
| 28 | 4 | Optional input callback offset (`0` means absent) |
| 32 | 4 | App window width (ignored for task) |
| 36 | 4 | App window height (ignored for task) |

Images are limited to 64 MiB. The step callback has signature `u32 step(const PluginHostApi*, void *state)`: nonzero keeps it running; zero completes it. App draw is `void draw(const PluginHostApi*, void *state, usize x, usize y)`. App input is `void input(const PluginHostApi*, void *state, u32 key)`. The packed key uses the low 16 bits for a UEFI scan code or printable UTF-16 code unit; bit 16 is set for a special scan code. HPR1 stores a u32 relocation count followed by records of u32 patch offset, u16 symbol length, and UTF-8 symbol name.

The executable file format remains ABI v1. The host table reports its own version in `abi_version`: v1 provides drawing, basic file read/write, and aligned allocation/free; host API v2 appends directory/file management, global variables, CPU and PCI information/access, network access, audio, and sleep. Check `host->abi_version` before using v2 callbacks. `read_file` returns the required length when called with a null/zero output buffer, `-2` when the supplied buffer is too small, and `-1` on failure. The CPU/PCI result structs are fixed-width C layouts declared in `examples/hpx_v1.h`.

## Register and launch

Keep a long-lived `DiskExecutableManager` in the OS scheduler. Call `run_file(path, name, version, &mut global_data)` to load and dispatch an executable by its header. Apps are inserted into the dashboard's stepped app list. Call `step_background_tasks()` once per scheduler iteration to advance background executables and reap completed tasks. Loaded code pages remain resident for the manager's lifetime; unload is not implemented in ABI v1.


## Build a sample app

The counter sample uses CPU info, PC speaker (`B`), and network initialization/link status (`N`) in addition to drawing, file persistence (`S` saves and `L` loads), and host memory allocation. The saved value is stored at `\counter.txt`.

The repository includes a C ABI header and counter app under `examples/`. The on-device `cc` frontend is independent of Micro-C and currently supports scalar integer expressions, local address-of and dereference operations, direct calls, if/else, and braced while/for/do-while loops. `break` and `continue` are supported, with `continue` in a `for` loop executing its update expression and `continue` in a `do-while` loop evaluating its condition. Expressions include arithmetic, comparisons, bitwise/logical operators, shifts, ternary expressions, and compound assignments. It is not yet a complete ISO C implementation: preprocessing, pointer arithmetic, arrays, aggregates, strings, globals, floating point, variadic calls, and the full C ABI remain unsupported. `examples/hpx_minimal.c` demonstrates the supported syntax. The OS shell pipeline is:

```text
cc app.c app.o
hpx-link app.o library.asm app.bin
hpx-pack app app.bin app.hpx STEP STATE_SIZE DRAW INPUT WIDTH HEIGHT app.bin.hrel
exec app.hpx AppName
```

`cc` writes an HPVMx-native HXO v1 relocatable object (`.o`). `hpx-link` accepts any number of `.o` and Micro-C `.asm` inputs, prints callback offsets, and creates the `.hrel` sidecar. Pass those offsets to `hpx-pack`; use zero for absent callbacks. Background tasks use kind `background`, a step offset, and zero draw/input offsets. HXO v1 currently contains a relocatable assembly section, not a standard ELF/COFF object.

A prebuilt sample is also available at `examples/hpx_counter.hpx` (480×240 app window). Copy it to an HPVMx-accessible filesystem and launch it with `exec [path] Counter` (`run-hpx` remains an alias).
