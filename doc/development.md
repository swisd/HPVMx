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

- **Variables**: Declared with `let`.
  ```c
  let x = 10;
  let p: ptr = alloc_struct(MyStruct);
  ```
- **Functions**: Declared with `fn`. Use `fn main()` for the entry point.
  ```c
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

### Built-in Functions

- `peek(address)`: Read a 64-bit value from a memory address.
- `poke(address, value)`: Write a 64-bit value to a memory address.
- `alloc_struct(StructName)`: Allocate memory for a structure.

### Compilation

From the HPVMx shell:
```text
micro-c compile /path/to/source.micro
```
This generates a `.asm` file. You can also use **MicroIDE** from the Dashboard (**Apps** tab) for an interactive experience.

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

HPVMx uses a registry-based package manager located in `/packages`.

### Package Registry (`registry.json`)

Packages are defined by their type and metadata:
- **Library**: Shared code.
- **Executable**: Compiled binaries or scripts.
- **Driver**: Hardware interfaces.
- **Extension**: Dashboard enhancements.

Example entry:
```json
{
  "name": "test-pkg",
  "version": "1.0.0",
  "type": "Executable",
  "dependencies": ["core-lib"]
}
```

### Management Commands

- `pm list`: List all registered packages.
- `pm reload`: Refresh registry from disk.
- `pm verify [name]`: Check if dependencies are met.

---

## 4. EFI Applications

Since HPVMx runs on UEFI, you can execute standard `.efi` binaries.

### Execution

Place your compiled `.efi` file on the storage and run:
```text
run-efi /path/to/app.efi [args...]
```
HPVMx will pass control to the EFI application and return to the shell upon exit.
