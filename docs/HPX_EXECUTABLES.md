# HPX disk executables (ABI v1)

HPVMx loads trusted x86-64 position-independent machine-code images from the UEFI filesystem. Applications and background workers are separate files and do not require rebuilding the OS. They execute in the kernel address space; v1 does not isolate or sandbox plugins. Only run code you trust.

## File layout

A `.hpx` file consists of a 40-byte little-endian header followed immediately by `image_size` bytes of code/data. No imports or relocations are processed. Link the image as position-independent code, keep internal references position independent, and use only the callback offsets and `PluginHostApi` for host services.

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

Images are limited to 64 MiB. The step callback has signature `u32 step(const PluginHostApi*, void *state)`: nonzero keeps it running; zero completes it. App draw is `void draw(const PluginHostApi*, void *state, usize x, usize y)`. App input is `void input(const PluginHostApi*, void *state, u32 key)`. The packed key uses the low 16 bits for a UEFI scan code or printable UTF-16 code unit; bit 16 is set for a special scan code.

The executable file format remains ABI v1. The host table reports its own version in `abi_version`: v1 provides drawing, basic file read/write, and aligned allocation/free; host API v2 appends directory/file management, global variables, CPU and PCI information/access, network access, audio, and sleep. Check `host->abi_version` before using v2 callbacks. `read_file` returns the required length when called with a null/zero output buffer, `-2` when the supplied buffer is too small, and `-1` on failure. The CPU/PCI result structs are fixed-width C layouts declared in `examples/hpx_v1.h`.

## Register and launch

Keep a long-lived `DiskExecutableManager` in the OS scheduler. Call `run_file(path, name, version, &mut global_data)` to load and dispatch an executable by its header. Apps are inserted into the dashboard's stepped app list. Call `step_background_tasks()` once per scheduler iteration to advance background executables and reap completed tasks. Loaded code pages remain resident for the manager's lifetime; unload is not implemented in ABI v1.


## Build a sample app

The counter sample uses CPU info, PC speaker (`B`), and network initialization/link status (`N`) in addition to drawing, file persistence (`S` saves and `L` loads), and host memory allocation. The saved value is stored at `\counter.txt`.

The repository includes a C ABI header and counter app under `examples/`. The counter demonstrates drawing, input, file persistence (`S` saves and `L` loads `\counter.txt`), and host allocation for file reads (`C` changes the label color). With Clang/LLVM installed, compile a flat PIC image without libc or unresolved imports:

```sh
clang -target x86_64-unknown-none -ffreestanding -fPIC -fno-stack-protector -fno-builtin -fno-unwind-tables -fno-asynchronous-unwind-tables -ffunction-sections -c examples/hpx_counter.c -o hpx_counter.o
ld.lld -T examples/hpx_link.ld -o hpx_counter.elf hpx_counter.o
llvm-nm -n --defined-only hpx_counter.elf
llvm-objcopy -O binary hpx_counter.elf hpx_counter.bin
python tools/hpx_pack.py hpx_counter.bin counter.hpx --kind app --state-size 16 --step 0xSTEP --draw 0xDRAW --input 0xINPUT --width 480 --height 240
```

Replace the callback offsets with the `hpx_step`, `hpx_draw`, and `hpx_input` addresses printed by `llvm-nm`. Copy `counter.hpx` to a filesystem HPVMx can access, then run `run-hpx [path] [name]` in the shell. Background executables use `--kind background`, state and step offsets, and no draw/input offsets.

A prebuilt sample is also available at `examples/hpx_counter.hpx` (480×240 app window). Copy it to an HPVMx-accessible filesystem and launch it with `run-hpx [path] Counter`.
