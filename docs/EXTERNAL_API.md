# HPVMx application API

`crate::externals` is the in-kernel Rust API for applications and OS components. It uses `core`/`alloc` types and the kernel's existing drivers and services.

```rust,ignore
let entries = crate::externals::fs::list("\\")?;
let cpu = crate::externals::hardware::cpu_info();
crate::externals::gui::draw_text(16, 16, "Hello", 0xFFFF_FFFF, 0xFF00_0000);
let pid = crate::externals::launch_application(
    runtime, "Example", "1.0", (480, 320), ExampleApp::new(),
);
```

## Service groups

- `fs`: read/write/append files, create/remove/rename/copy, directory listing, current directory, and disk counters.
- `hardware`: CPU capabilities and PCI enumeration/configuration access.
- `devices`: network initialization, link state, raw frame transmit/receive, PC speaker, and delay.
- `gui`: PixelGraphics resolution, clear, text, rectangles, and buttons.
- `registry`: load and save system and device settings.
- `system`: global variables and a millisecond timestamp.
- `memory`: raw allocator operations; these are `unsafe` and deallocation must use the original size and alignment.
- `processes`: count and terminate live stepped applications.
- Top-level helpers: construct/launch applications, attach cooperative tasks and futures, and load `.hpx` executables.

These APIs run with kernel privileges. PCI writes, raw network frames, filesystem writes, and memory operations take effect directly. Calls only exist for services currently implemented by HPVMx; unsupported facilities such as general-purpose process isolation are not implied by this API.
