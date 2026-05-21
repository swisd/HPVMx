# HPVMx Development Manual

This manual provides instructions for developing applications and extensions for HPVMx.

## Development Paths

There are three main ways to develop for HPVMx:
1. **Micro-C**: A lightweight, C-like scripting language for quick tools and logic.
2. **Native Rust Apps**: High-performance graphical applications integrated into the Dashboard.
3. **EFI Applications**: Standard UEFI binaries that can be executed from the shell.

---

## 1. Micro-C Toolchain

Micro-C is the built-in language for HPVMx. It is a simplified subset of C with some modern features.

### Language Basics

- **Variables**: Declared with `let`. Supports basic types and pointers.
  ```c
  let x = 10;
  let p: ptr = alloc_struct(MyStruct);
  ```
- **Functions**: Declared with `fn`. Use `export fn main()` for the entry point.
  ```c
  export fn main() {
      let z = add(10, 5);
      return z;
  }

  fn add(a, b) {
      return a + b;
  }
  ```
- **Control Flow**: `if`, `else`, `loop`, `break`, `continue`.
  ```c
  loop {
      if (i == 10) {
          break;
      }
      if (i == 5) {
          i = i + 1;
          continue;
      }
      i = i + 1;
  }
  ```
- **Structures**:
  ```c
  struct Point {
      x: i64;
      y: i64;
  }
  ```
- **Arrays**:
  ```c
  let arr = [1, 2, 3, 4, 5];
  let val = arr[2];
  arr[0] = 10;
  ```

### Memory and Pointers

Micro-C allows direct memory manipulation:

- `peek(address)`: Read a 64-bit value from a memory address (Expression).
- `poke(address, value)`: Write a 64-bit value to a memory address (Statement).
- `alloc_struct(StructName)`: Allocate memory for a structure. Returns a `ptr`.

### Modularity

Micro-C supports basic modularity:

- `#include <file.micro>`: Include another source file.
- `#export name.*`: Export symbols for use by other modules.

### Compilation

From the HPVMx shell:
```text
micro-c compile /path/to/source.micro
```
This generates a `.asm` file. You can also use **MicroIDE** from the Dashboard (**Apps** tab) for an interactive experience.

---
> You can find advanced techniqes at the [Micro-C Repository](https://github.com/swisd/Micro-C)
---

## 2. Native Rust Applications

Native applications are compiled into the HPVMx binary and can provide rich graphical interfaces.

### Implementing an App

To create a new app, implement the `AppInfo` and `Runnable` traits found in `src/env.rs`.

```rust
pub struct MyApp { ... }

impl AppInfo for MyApp {
    fn name(&self) -> &str { "MyApp" }
    fn version(&self) -> &str { "1.0.0" }
    fn icon(&self) -> [u32; 1024] { /* 32x32 icon data */ }
    fn dimensions(&self) -> (usize, usize) { (400, 300) }
}

impl Runnable for MyApp {
    fn draw(&self, graphics: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) {
        graphics.draw_text(x + 10, y + 10, "Hello HPVMx!", 0xFFFFFF);
    }

    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) {
        // App logic here
    }

    fn input(&mut self, key: Key) {
        // Handle input
    }
}
```

### PixelGraphics API

The `PixelGraphics` entity provided in the `draw` method offers various primitives:

#### Basic Drawing
- `draw_pixel(x, y, color)`: Draw a single pixel.
- `fill_rect(x, y, w, h, color)`: Fill a rectangular area.
- `draw_rect_outline(x, y, w, h, color)`: Draw a rectangle border.
- `draw_line(x1, y1, x2, y2, color)`: Draw a line.

#### Text and UI
- `draw_text(x, y, text, color)`: Render text.
- `draw_text_adv(x, y, text, color, scale)`: Render scaled text.
- `draw_button(x, y, w, h, text, is_focused)`: Draw a standard UI button.
- `draw_checkbox(x, y, checked, blocked, disabled, text)`: Draw a checkbox.
- `draw_progress_bar(x, y, w, h, value, max, color)`: Draw a progress bar.

#### Advanced
- `draw_icon(x, y, w, h, data)`: Render raw pixel data (e.g., icons).
- `draw_bmp(x, y, path)`: Load and draw a BMP file from storage.
- `apply_glitch()`: Apply a visual glitch effect to the buffer.
- `apply_scanlines()`: Apply a CRT-style scanline effect.

### App Registration

Register your app in `src/apps/mod.rs` to make it appear in the Dashboard.

Sample Registration:
```rust
("MyApp", || {
  let app = MyApp::new();
  let dims = crate::env::AppInfo::dimensions(&app);
  (Box::new(app), dims)
}, icons::TRAFFIC_LIGHT_32_ICON_DATA, "0.1.0"),
```


---

## 3. Package System

HPVMx uses a registry-based package manager located in `/packages`. THis package manager only
supports micro-c since it can be downloaded and compiled natively on HPVMx.

### Package Registry (`registry.json`)

Packages are defined by their type and metadata in a JSON registry.

#### Package Types
- **Library**: Shared code used by other packages.
- **Executable**: Compiled binaries, scripts, or apps.
- **Extension**: Dashboard enhancements or shell extensions.
- **Driver**: Low-level hardware interfaces.
- **ResourcePack**: Assets like icons, fonts, or localized text.
- **PShader**: Programmable visual effects or shaders.

Example entry:
```json
{
  "name": "network-util",
  "version": "1.2.0",
  "type": "Executable",
  "author": "HPVMx Team",
  "deps": ["core-lib", "net-lib"],
  "entry_point": "main",
  "description": "Advanced network diagnostic tools."
}
```

### Management Commands

- `pm list`: List all registered packages and their types.
- `pm reload`: Refresh the registry from `/packages/registry.json`.
- `pm verify [name]`: Perform a dependency check for the specified package.
- `pm version`: Show the current version of the Package Manager.

---

## 4. EFI Applications

Since HPVMx runs on UEFI, you can execute standard `.efi` binaries.

> [!WARNING]  
> This feature is deprecated and soon to be removed. `.efi` binaries will be loaded into an EfiVirtualizedContext and then ran, 
> allowing HPVMx to continue running in the backgorund and monitor the efi behavior.

> [!CAUTION]
> Running an efi using run-efi locks out the os 
> and can result in majot security vulnerabilities.

### Execution

Place your compiled `.efi` file on the storage and run:
```text
run-efi /path/to/app.efi [args...]
```
HPVMx will pass control to the EFI application and return to the shell upon exit.
