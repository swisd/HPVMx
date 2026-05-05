# Crate Documentation

**Version:** 1.9.2

**Format Version:** 57

# Module `HPVMx`

## Modules

## Module `ui`

**Attributes:**

- `Other("#[allow(dead_code, deprecated)]")`

User Interface and dashboard management.

This module contains the core UI logic, including the `DashboardUI`
which manages the main display, active applications, and system status.

```rust
pub(crate) mod ui { /* ... */ }
```

### Modules

## Module `graphics`

```rust
pub(in ::ui) mod graphics { /* ... */ }
```

### Types

#### Struct `AbsolutePointer`

**Attributes:**

- `Repr(AttributeRepr { kind: Transparent, align: None, packed: None, int: None })`

```rust
pub(in ::ui::graphics) struct AbsolutePointer(pub(in ::ui::graphics) uefi_raw::protocol::console::AbsolutePointerProtocol);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `uefi_raw::protocol::console::AbsolutePointerProtocol` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::ui::graphics) fn read_state(self: &mut Self) -> uefi::Result<Option<uefi_raw::protocol::console::AbsolutePointerState>> { /* ... */ }
  ```

- ```rust
  pub(in ::ui::graphics) fn mode(self: &Self) -> &uefi_raw::protocol::console::AbsolutePointerMode { /* ... */ }
  ```

- ```rust
  pub(in ::ui::graphics) fn reset(self: &mut Self, extended: bool) -> uefi::Result { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Identify**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **Protocol**
- **ProtocolPointer**
  - ```rust
    unsafe fn ptr_from_ffi(ptr: *const c_void) -> *const P { /* ... */ }
    ```

  - ```rust
    unsafe fn mut_ptr_from_ffi(ptr: *mut c_void) -> *mut P { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Cursor`

**Attributes:**

- `Other("#[allow(dead_code)]")`

```rust
pub struct Cursor {
    pub x: i32,
    pub y: i32,
    pub visible: bool,
    pub left_button: bool,
    pub right_button: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `x` | `i32` |  |
| `y` | `i32` |  |
| `visible` | `bool` |  |
| `left_button` | `bool` |  |
| `right_button` | `bool` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub unsafe fn update_from_mouse(self: &mut Self, screen_width: usize, screen_height: usize) { /* ... */ }
  ```

- ```rust
  pub(in ::ui::graphics) unsafe fn try_update_absolute(self: &mut Self, screen_width: usize, screen_height: usize) -> u32 { /* ... */ }
  ```

- ```rust
  pub(in ::ui::graphics) unsafe fn try_update_relative(self: &mut Self, screen_width: usize, screen_height: usize) -> u32 { /* ... */ }
  ```

- ```rust
  pub fn render(self: &Self, stdout: &mut uefi::proto::console::text::Output) { /* ... */ }
  ```

- ```rust
  pub fn debug_mouse() { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `WinNTColors`

```rust
pub struct WinNTColors;
```

##### Implementations

###### Methods

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Rect`

```rust
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `x` | `usize` |  |
| `y` | `usize` |  |
| `width` | `usize` |  |
| `height` | `usize` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self { /* ... */ }
  ```

- ```rust
  pub fn contains(self: &Self, x: usize, y: usize) -> bool { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Graphics`

**Attributes:**

- `Other("#[allow(dead_code)]")`

```rust
pub struct Graphics;
```

##### Implementations

###### Methods

- ```rust
  pub fn clear_screen(bg: Color) { /* ... */ }
  ```
  Clear screen with background color

- ```rust
  pub fn set_cursor(col: usize, row: usize) { /* ... */ }
  ```
  Set cursor position (column, row)

- ```rust
  pub fn draw_box(rect: &Rect, title: &str, active: bool) { /* ... */ }
  ```
  Draw a titled box (window frame)

- ```rust
  pub fn draw_button(rect: &Rect, label: &str, focused: bool) { /* ... */ }
  ```
  Draw a 3D button

- ```rust
  pub fn draw_textbox(rect: &Rect, text: &str, focused: bool) { /* ... */ }
  ```
  Draw a text input field

- ```rust
  pub fn draw_menu_bar(items: &[&str]) { /* ... */ }
  ```
  Draw a menu bar

- ```rust
  pub fn draw_taskbar(time: &str) { /* ... */ }
  ```
  Draw taskbar at bottom

- ```rust
  pub fn draw_list(rect: &Rect, items: &[&str], selected: usize) { /* ... */ }
  ```
  Draw a scrollable list

- ```rust
  pub fn print_at(col: usize, row: usize, text: &str, fg: Color, bg: Color) { /* ... */ }
  ```
  Print text at position

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `pixel_graphics`

```rust
pub mod pixel_graphics { /* ... */ }
```

### Modules

## Module `icons`

```rust
pub mod icons { /* ... */ }
```

### Types

#### Type Alias `ICON16`

```rust
pub type ICON16 = [u32; 256];
```

#### Type Alias `ICON32`

```rust
pub type ICON32 = [u32; 1024];
```

### Constants and Statics

#### Static `RAM_ICON_DATA`

```rust
pub static RAM_ICON_DATA: [u32; 256] = _;
```

#### Static `PCI_GREEN_ICON_DATA`

```rust
pub static PCI_GREEN_ICON_DATA: [u32; 256] = _;
```

#### Static `PCI_BLUE_ICON_DATA`

```rust
pub static PCI_BLUE_ICON_DATA: [u32; 256] = _;
```

#### Static `CPU_ICON_DATA`

```rust
pub static CPU_ICON_DATA: [u32; 256] = _;
```

#### Static `HOURGLASS_ICON_DATA`

```rust
pub static HOURGLASS_ICON_DATA: [u32; 256] = _;
```

#### Static `ETHERNET_ICON_DATA`

```rust
pub static ETHERNET_ICON_DATA: [u32; 256] = _;
```

#### Static `HDD_INTERNAL_ICON_DATA`

```rust
pub static HDD_INTERNAL_ICON_DATA: [u32; 256] = _;
```

#### Static `SETTINGS_ICON_DATA`

```rust
pub static SETTINGS_ICON_DATA: [u32; 256] = _;
```

#### Static `GTK_CUBE_32_ICON_DATA`

```rust
pub static GTK_CUBE_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `PACKAGE_ICON_DATA`

```rust
pub static PACKAGE_ICON_DATA: [u32; 256] = _;
```

#### Static `FOLDER_ICON_DATA`

```rust
pub static FOLDER_ICON_DATA: [u32; 256] = _;
```

#### Static `FILE_ICON_DATA`

```rust
pub static FILE_ICON_DATA: [u32; 256] = _;
```

#### Static `JSON_ICON_DATA`

```rust
pub static JSON_ICON_DATA: [u32; 256] = _;
```

#### Static `CODE_ICON_DATA`

```rust
pub static CODE_ICON_DATA: [u32; 256] = _;
```

#### Static `EXECUTABLE_ICON_DATA`

```rust
pub static EXECUTABLE_ICON_DATA: [u32; 256] = _;
```

#### Static `SERVER_ICON_DATA`

```rust
pub static SERVER_ICON_DATA: [u32; 256] = _;
```

#### Static `INTEGRATED_CIRCUIT_32_ICON_DATA`

```rust
pub static INTEGRATED_CIRCUIT_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `CUBE_WINDOW_RED_32_ICON_DATA`

```rust
pub static CUBE_WINDOW_RED_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `SCRIPT_BLUE_32_ICON_DATA`

```rust
pub static SCRIPT_BLUE_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `SCRIPT_YELLOW_32_ICON_DATA`

```rust
pub static SCRIPT_YELLOW_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `WINDOW_3D_32_ICON_DATA`

```rust
pub static WINDOW_3D_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `WINDOW_CMD_32_ICON_DATA`

```rust
pub static WINDOW_CMD_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `OBJECT_KEY_32_ICON_DATA`

```rust
pub static OBJECT_KEY_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `REGEDIT_CUBES_32_ICON_DATA`

```rust
pub static REGEDIT_CUBES_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `GEAR_WINDOW_YELLOW_32_ICON_DATA`

```rust
pub static GEAR_WINDOW_YELLOW_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `INTEGRATED_CIRCUIT_COMPOUND_32_ICON_DATA`

```rust
pub static INTEGRATED_CIRCUIT_COMPOUND_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `TRAFFIC_LIGHT_32_ICON_DATA`

```rust
pub static TRAFFIC_LIGHT_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `WINOBJ_SEMAPHORE_32_ICON_DATA`

```rust
pub static WINOBJ_SEMAPHORE_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `WINOBJ_TIMER_32_ICON_DATA`

```rust
pub static WINOBJ_TIMER_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `WINOBJ_REGISTRY_32_ICON_DATA`

```rust
pub static WINOBJ_REGISTRY_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `WINOBJ_DEVICE_32_ICON_DATA`

```rust
pub static WINOBJ_DEVICE_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `WINOBJ_EVENT_32_ICON_DATA`

```rust
pub static WINOBJ_EVENT_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `WINOBJ_PORT_32_ICON_DATA`

```rust
pub static WINOBJ_PORT_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `WINOBJ_SECTION_32_ICON_DATA`

```rust
pub static WINOBJ_SECTION_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `WINOBJ_JOB_32_ICON_DATA`

```rust
pub static WINOBJ_JOB_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `WINOBJ_MUTEX_32_ICON_DATA`

```rust
pub static WINOBJ_MUTEX_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `WINOBJ_FILTER_32_ICON_DATA`

```rust
pub static WINOBJ_FILTER_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `GEAR_YB_32_ICON_DATA`

```rust
pub static GEAR_YB_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `LIGHTBULB_ON_32_ICON_DATA`

```rust
pub static LIGHTBULB_ON_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `SPEAKER_ON_32_ICON_DATA`

```rust
pub static SPEAKER_ON_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `SPEAKER_OFF_32_ICON_DATA`

```rust
pub static SPEAKER_OFF_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `WARNING_ISSUE_SUB_32_ICON_DATA`

```rust
pub static WARNING_ISSUE_SUB_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `CUBE_TREE_32_ICON_DATA`

```rust
pub static CUBE_TREE_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `GEAR_WINDOW_SETTINGS_32_ICON_DATA`

```rust
pub static GEAR_WINDOW_SETTINGS_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `FLOPPY_SAVE_32_ICON_DATA`

```rust
pub static FLOPPY_SAVE_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `STEPS_STAIRS_32_ICON_DATA`

```rust
pub static STEPS_STAIRS_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `REGISTRY_HIVE_32_ICON_DATA`

```rust
pub static REGISTRY_HIVE_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `COMPUTE_UNIT_H_32_ICON_DATA`

```rust
pub static COMPUTE_UNIT_H_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `COMPUTE_UNIT_V_HEALTH_32_ICON_DATA`

```rust
pub static COMPUTE_UNIT_V_HEALTH_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `MANUAL_BOOK_32_ICON_DATA`

```rust
pub static MANUAL_BOOK_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `TAPE_WRITE_32_ICON_DATA`

```rust
pub static TAPE_WRITE_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `COMPUTE_UNIT_V_CLUSTER_32_ICON_DATA`

```rust
pub static COMPUTE_UNIT_V_CLUSTER_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `COMPUTE_UNIT_V_GLOBE_32_ICON_DATA`

```rust
pub static COMPUTE_UNIT_V_GLOBE_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `MANUAL_BOOK_CLOSED_32_ICON_DATA`

```rust
pub static MANUAL_BOOK_CLOSED_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `DATABASE_CLUSTER_32_ICON_DATA`

```rust
pub static DATABASE_CLUSTER_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `TAPE_READ_32_ICON_DATA`

```rust
pub static TAPE_READ_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `COMPUTE_UNIT_V_SECURE_32_ICON_DATA`

```rust
pub static COMPUTE_UNIT_V_SECURE_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `COMPUTE_UNIT_V_DATABASE_32_ICON_DATA`

```rust
pub static COMPUTE_UNIT_V_DATABASE_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `COM_PORT_32_ICON_DATA`

```rust
pub static COM_PORT_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `CONNECTION_PIPE_32_ICON_DATA`

```rust
pub static CONNECTION_PIPE_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `CD_DISK_32_ICON_DATA`

```rust
pub static CD_DISK_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `MANUAL_BOOK_TURN_32_ICON_DATA`

```rust
pub static MANUAL_BOOK_TURN_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `MODEM_CLUSTER_32_ICON_DATA`

```rust
pub static MODEM_CLUSTER_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `FLOPPY_DATA_2_32_ICON_DATA`

```rust
pub static FLOPPY_DATA_2_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `CHECKLIST_PAGE_32_ICON_DATA`

```rust
pub static CHECKLIST_PAGE_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `NOTEBOOK_JOURNAL_32_ICON_DATA`

```rust
pub static NOTEBOOK_JOURNAL_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `LOCK_32_ICON_DATA`

```rust
pub static LOCK_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `NETWORK_FOLDER_32_ICON_DATA`

```rust
pub static NETWORK_FOLDER_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `KEYS_KEYCHAIN_32_ICON_DATA`

```rust
pub static KEYS_KEYCHAIN_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `TERMINAL_LAPTOP_32_ICON_DATA`

```rust
pub static TERMINAL_LAPTOP_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `DATABASE_32_ICON_DATA`

```rust
pub static DATABASE_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `BLADE_NETWORK_32_ICON_DATA`

```rust
pub static BLADE_NETWORK_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `GRAPHICS_2D_32_ICON_DATA`

```rust
pub static GRAPHICS_2D_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `CLOCK_RED_32_ICON_DATA`

```rust
pub static CLOCK_RED_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `BLADE_NETWORK_32_LIGHT_ICON_DATA`

```rust
pub static BLADE_NETWORK_32_LIGHT_ICON_DATA: [u32; 1024] = _;
```

#### Static `ADD_PLUS_32_ICON_DATA`

```rust
pub static ADD_PLUS_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `SNAKE_32_ICON_DATA`

```rust
pub static SNAKE_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `SETUPAPI_32_ICON_DATA`

```rust
pub static SETUPAPI_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `FILE_DATABASE_32_ICON_DATA`

```rust
pub static FILE_DATABASE_32_ICON_DATA: [u32; 1024] = _;
```

#### Static `DOOM_32_ICON_DATA`

```rust
pub static DOOM_32_ICON_DATA: [u32; 1024] = _;
```

### Types

#### Struct `TreeViewNode`

```rust
pub struct TreeViewNode<''a> {
    pub label: &''a str,
    pub children: &''a [TreeViewNode<''a>],
    pub expanded: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `label` | `&''a str` |  |
| `children` | `&''a [TreeViewNode<''a>]` |  |
| `expanded` | `bool` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `PixelGraphics`

```rust
pub struct PixelGraphics {
    pub(in ::ui::pixel_graphics) framebuffer: *mut u32,
    pub(in ::ui::pixel_graphics) width: usize,
    pub(in ::ui::pixel_graphics) height: usize,
    pub(in ::ui::pixel_graphics) stride: usize,
    pub(in ::ui::pixel_graphics) backbuffer: Option<alloc::vec::Vec<u32>>,
    pub glitch_y: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `framebuffer` | `*mut u32` |  |
| `width` | `usize` |  |
| `height` | `usize` |  |
| `stride` | `usize` |  |
| `backbuffer` | `Option<alloc::vec::Vec<u32>>` |  |
| `glitch_y` | `usize` |  |

##### Implementations

###### Methods

- ```rust
  pub fn draw_line_graph<T: Into<u64> + Copy>(self: &mut Self, x: usize, y: usize, width: usize, height: usize, data: &[T], max_val: u64, color: u32, len: usize) { /* ... */ }
  ```

- ```rust
  pub fn new() -> Option<Self> { /* ... */ }
  ```

- ```rust
  pub fn with_backbuffer(self: Self) -> Self { /* ... */ }
  ```

- ```rust
  pub fn backbuffer_slice_mut(self: &mut Self) -> Option<&mut [u32]> { /* ... */ }
  ```

- ```rust
  pub fn flip(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn exit(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn draw_pixel(self: &mut Self, x: usize, y: usize, color: u32) { /* ... */ }
  ```

- ```rust
  pub fn fill_rect(self: &mut Self, x: usize, y: usize, width: usize, height: usize, color: u32) { /* ... */ }
  ```

- ```rust
  pub fn draw_line(self: &mut Self, x1: usize, y1: usize, x2: usize, y2: usize, color: u32) { /* ... */ }
  ```

- ```rust
  pub fn draw_line_adv(self: &mut Self, x1: usize, y1: usize, x2: usize, y2: usize, color: u32, thickness: usize, style: u32) { /* ... */ }
  ```

- ```rust
  pub fn draw_line_styled(self: &mut Self, x1: usize, y1: usize, x2: usize, y2: usize, color: u32, style: u32) { /* ... */ }
  ```

- ```rust
  pub fn draw_char(self: &mut Self, x: usize, y: usize, c: char, color: u32) { /* ... */ }
  ```

- ```rust
  pub fn draw_char_adv(self: &mut Self, x: usize, y: usize, c: char, color: u32, scale: usize) { /* ... */ }
  ```

- ```rust
  pub fn draw_symbol(self: &mut Self, x: usize, y: usize, symbol: u16, color: u32) { /* ... */ }
  ```

- ```rust
  pub fn draw_u64_le_sym(self: &mut Self, x: usize, y: usize, val: u64, color: u32) { /* ... */ }
  ```

- ```rust
  pub fn draw_char_bg(self: &mut Self, x: usize, y: usize, c: char, color: u32, bg: u32) { /* ... */ }
  ```

- ```rust
  pub fn draw_text(self: &mut Self, x: usize, y: usize, text: &str, color: u32) { /* ... */ }
  ```

- ```rust
  pub fn draw_text_adv(self: &mut Self, x: usize, y: usize, text: &str, color: u32, scale: usize) { /* ... */ }
  ```

- ```rust
  pub fn draw_text_bg(self: &mut Self, x: usize, y: usize, text: &str, color: u32, bg: u32) { /* ... */ }
  ```

- ```rust
  pub fn draw_checkbox(self: &mut Self, x: usize, y: usize, checked: bool, blocked: bool, disabled: bool, text: &str) { /* ... */ }
  ```

- ```rust
  pub fn draw_tristate_checkbox(self: &mut Self, x: usize, y: usize, text: &str, color: u32, some: bool, checked: bool) { /* ... */ }
  ```

- ```rust
  pub fn draw_radio_button(self: &mut Self, x: usize, y: usize, checked: bool) { /* ... */ }
  ```

- ```rust
  pub fn draw_rect_outline(self: &mut Self, x: usize, y: usize, width: usize, height: usize, color: u32) { /* ... */ }
  ```

- ```rust
  pub fn draw_rect_outline_adv(self: &mut Self, x: usize, y: usize, width: usize, height: usize, color: u32, thickness: usize, style: u32) { /* ... */ }
  ```

- ```rust
  pub fn polygon_outline(self: &mut Self, points: &[(usize, usize)], color: u32) { /* ... */ }
  ```

- ```rust
  pub fn polygon_fill(self: &mut Self, points: &[(usize, usize)], color: u32) { /* ... */ }
  ```

- ```rust
  pub fn draw_progress_bar(self: &mut Self, x: usize, y: usize, width: usize, height: usize, value: usize, max: usize, color: u32) { /* ... */ }
  ```

- ```rust
  pub fn draw_slider(self: &mut Self, x: usize, y: usize, width: usize, value: usize, max: usize, vertical: bool) { /* ... */ }
  ```

- ```rust
  pub fn draw_lcd_number(self: &mut Self, x: usize, y: usize, value: &str) { /* ... */ }
  ```

- ```rust
  pub fn clear(self: &mut Self, color: u32) { /* ... */ }
  ```

- ```rust
  pub fn draw_cursor(self: &mut Self, x: usize, y: usize) { /* ... */ }
  ```

- ```rust
  pub fn resolution(self: &Self) -> (usize, usize) { /* ... */ }
  ```

- ```rust
  pub fn draw_button(self: &mut Self, x: usize, y: usize, width: usize, height: usize, text: &str, is_focused: bool) { /* ... */ }
  ```

- ```rust
  pub fn u64_sym_le(self: &Self, value: u64) -> [u8; 16] { /* ... */ }
  ```

- ```rust
  pub fn draw_list_view(self: &mut Self, x: usize, y: usize, width: usize, height: usize, items: &[&str], selected_idx: Option<usize>) { /* ... */ }
  ```

- ```rust
  pub fn draw_table_view(self: &mut Self, x: usize, y: usize, width: usize, height: usize, headers: &[&str], rows: &[&[&str]]) { /* ... */ }
  ```

- ```rust
  pub fn draw_tree_view(self: &mut Self, x: usize, y: usize, width: usize, height: usize, root: &TreeViewNode<''_>) { /* ... */ }
  ```

- ```rust
  pub fn draw_tree_view_icon(self: &mut Self, x: usize, y: usize, width: usize, height: usize, root: &TreeViewNode<''_>, icon: &[u32]) { /* ... */ }
  ```

- ```rust
  pub(in ::ui::pixel_graphics) fn draw_tree_node_icon(self: &mut Self, x: usize, y_pos: &mut usize, node: &TreeViewNode<''_>, depth: usize, icon: &[u32]) { /* ... */ }
  ```

- ```rust
  pub(in ::ui::pixel_graphics) fn draw_tree_node(self: &mut Self, x: usize, y_pos: &mut usize, node: &TreeViewNode<''_>, depth: usize) { /* ... */ }
  ```

- ```rust
  pub fn draw_spinbox(self: &mut Self, x: usize, y: usize, width: usize, value: i32, label: &str) { /* ... */ }
  ```

- ```rust
  pub fn draw_double_spinbox(self: &mut Self, x: usize, y: usize, width: usize, value: f64, precision: usize) { /* ... */ }
  ```

- ```rust
  pub fn draw_dial(self: &mut Self, x: usize, y: usize, radius: usize, value: usize, max: usize) { /* ... */ }
  ```

- ```rust
  pub fn draw_octagon_outline(self: &mut Self, x: usize, y: usize, r: usize, color_light: u32, color_dark: u32) { /* ... */ }
  ```
  Helper to draw a beveled octagon for the dial's outer edge

- ```rust
  pub fn draw_icon(self: &mut Self, x: usize, y: usize, width: usize, height: usize, data: &[u32]) { /* ... */ }
  ```

- ```rust
  pub fn draw_buffer_at(self: &mut Self, x: usize, y: usize, width: usize, height: usize, buffer_ptr: *const u32) { /* ... */ }
  ```

- ```rust
  pub fn apply_scanlines(self: &mut Self) { /* ... */ }
  ```
  Scanline Shader

- ```rust
  pub fn apply_dither(self: &mut Self) { /* ... */ }
  ```
  Simulates 16-bit or 8-bit color banding (Dithering-like feel)

- ```rust
  pub fn apply_glitch(self: &mut Self) { /* ... */ }
  ```
  "Moving-Glitch" shader, does not work at this time

- ```rust
  pub fn apply_edge_aberration(self: &mut Self, max_strength: f32) { /* ... */ }
  ```
  Chromatic Aberration edge shader, requires a lot of cpu, resource intensive. Unused but not deprecated

- ```rust
  pub fn app_context_border(self: &mut Self, appname: &str) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Functions

#### Function `get_font_data`

Simple 8x16 font data

```rust
pub(in ::ui::pixel_graphics) fn get_font_data(c: char) -> [u8; 16] { /* ... */ }
```

#### Function `get_sym_data`

```rust
pub(in ::ui::pixel_graphics) fn get_sym_data(s: u16) -> [u8; 16] { /* ... */ }
```

### Constants and Statics

#### Static `GOP_CACHE`

```rust
pub(in ::ui::pixel_graphics) static mut GOP_CACHE: Option<(*mut u32, usize, usize, usize)> = None;
```

#### Constant `BASIC_FONT`

```rust
pub(in ::ui::pixel_graphics) const BASIC_FONT: [[u8; 16]; 256] = _;
```

#### Constant `SYMBOL_LIB`

```rust
pub(in ::ui::pixel_graphics) const SYMBOL_LIB: [[u8; 16]; 256] = _;
```

## Module `graphics3d`

```rust
pub mod graphics3d { /* ... */ }
```

### Types

#### Struct `Vector3`

```rust
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `x` | `f64` |  |
| `y` | `f64` |  |
| `z` | `f64` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(x: f64, y: f64, z: f64) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Matrix4`

```rust
pub struct Matrix4 {
    pub m: [[f64; 4]; 4],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `m` | `[[f64; 4]; 4]` |  |

##### Implementations

###### Methods

- ```rust
  pub fn identity() -> Self { /* ... */ }
  ```

- ```rust
  pub fn rotation_x(angle: f64) -> Self { /* ... */ }
  ```

- ```rust
  pub fn rotation_y(angle: f64) -> Self { /* ... */ }
  ```

- ```rust
  pub fn rotation_z(angle: f64) -> Self { /* ... */ }
  ```

- ```rust
  pub fn translation(x: f64, y: f64, z: f64) -> Self { /* ... */ }
  ```

- ```rust
  pub fn multiply_vec(self: &Self, v: &Vector3) -> Vector3 { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Graphics3D`

```rust
pub struct Graphics3D<''a> {
    pub pg: &''a mut crate::ui::pixel_graphics::PixelGraphics,
    pub width: f64,
    pub height: f64,
    pub fov: f64,
    pub z_near: f64,
    pub z_far: f64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `pg` | `&''a mut crate::ui::pixel_graphics::PixelGraphics` |  |
| `width` | `f64` |  |
| `height` | `f64` |  |
| `fov` | `f64` |  |
| `z_near` | `f64` |  |
| `z_far` | `f64` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(pg: &''a mut PixelGraphics) -> Self { /* ... */ }
  ```

- ```rust
  pub fn project(self: &Self, v: &Vector3) -> (usize, usize) { /* ... */ }
  ```

- ```rust
  pub fn draw_wireframe_poly(self: &mut Self, points: &[Vector3], color: u32) { /* ... */ }
  ```

- ```rust
  pub fn draw_filled_poly(self: &mut Self, points: &[Vector3], color: u32) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Types

#### Struct `FileEntry`

```rust
pub struct FileEntry {
    pub name: alloc::string::String,
    pub size: u64,
    pub is_dir: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `alloc::string::String` |  |
| `size` | `u64` |  |
| `is_dir` | `bool` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FileEntry { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `DeviceEntry`

```rust
pub struct DeviceEntry {
    pub name: alloc::string::String,
    pub path: alloc::string::String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `alloc::string::String` |  |
| `path` | `alloc::string::String` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DeviceEntry { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `DeviceCategory`

```rust
pub struct DeviceCategory {
    pub name: alloc::string::String,
    pub devices: alloc::vec::Vec<DeviceEntry>,
    pub expanded: bool,
    pub icon: alloc::string::String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `alloc::string::String` |  |
| `devices` | `alloc::vec::Vec<DeviceEntry>` |  |
| `expanded` | `bool` |  |
| `icon` | `alloc::string::String` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DeviceCategory { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `DashboardUI`

Main UI manager for the HPVMx system.

Handles the dashboard, windowing system for applications,
and user input routing.

```rust
pub struct DashboardUI {
    pub(in ::ui) selected_tab: DashboardTab,
    pub vms: alloc::vec::Vec<VmDisplayInfo>,
    pub resources: SystemResources,
    pub(in ::ui) scroll_offset: usize,
    pub(in ::ui) cursor: crate::graphics::Cursor,
    pub current_path: alloc::string::String,
    pub files: alloc::vec::Vec<FileEntry>,
    pub selected_file_idx: usize,
    pub categories: alloc::vec::Vec<DeviceCategory>,
    pub selected_device_idx: usize,
    pub(in ::ui) exit_requested: bool,
    pub new_vm_name: alloc::string::String,
    pub new_vm_memory_mb: u32,
    pub new_vm_vcpus: u32,
    pub create_vm_focus_idx: usize,
    pub vm_action_idx: usize,
    pub selected_vm_idx: usize,
    pub filesys_action_idx: usize,
    pub term_selected: bool,
    pub term_buf: alloc::string::String,
    pub editor: Option<TextEditor>,
    pub package_manager: manager::PackageManager,
    pub iter: u64,
    pub active_apps: alloc::vec::Vec<crate::env::SteppedApplicationContext>,
    pub focused_process_idx: Option<usize>,
    pub selected_app_idx: usize,
    pub app_window_position: (usize, usize),
    pub ctrl_mode: bool,
    pub alt_mode: bool,
    pub fn_mode: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `selected_tab` | `DashboardTab` |  |
| `vms` | `alloc::vec::Vec<VmDisplayInfo>` |  |
| `resources` | `SystemResources` |  |
| `scroll_offset` | `usize` |  |
| `cursor` | `crate::graphics::Cursor` |  |
| `current_path` | `alloc::string::String` |  |
| `files` | `alloc::vec::Vec<FileEntry>` |  |
| `selected_file_idx` | `usize` |  |
| `categories` | `alloc::vec::Vec<DeviceCategory>` |  |
| `selected_device_idx` | `usize` |  |
| `exit_requested` | `bool` |  |
| `new_vm_name` | `alloc::string::String` |  |
| `new_vm_memory_mb` | `u32` |  |
| `new_vm_vcpus` | `u32` |  |
| `create_vm_focus_idx` | `usize` |  |
| `vm_action_idx` | `usize` |  |
| `selected_vm_idx` | `usize` |  |
| `filesys_action_idx` | `usize` |  |
| `term_selected` | `bool` |  |
| `term_buf` | `alloc::string::String` |  |
| `editor` | `Option<TextEditor>` |  |
| `package_manager` | `manager::PackageManager` |  |
| `iter` | `u64` |  |
| `active_apps` | `alloc::vec::Vec<crate::env::SteppedApplicationContext>` |  |
| `focused_process_idx` | `Option<usize>` |  |
| `selected_app_idx` | `usize` |  |
| `app_window_position` | `(usize, usize)` |  |
| `ctrl_mode` | `bool` |  |
| `alt_mode` | `bool` |  |
| `fn_mode` | `bool` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(package_manager: PackageManager) -> Self { /* ... */ }
  ```

- ```rust
  pub fn add_vm(self: &mut Self, vm: VmDisplayInfo) { /* ... */ }
  ```

- ```rust
  pub fn set_resources(self: &mut Self, resources: SystemResources) { /* ... */ }
  ```

- ```rust
  pub unsafe fn draw(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub(in ::ui) fn count_running_vms(self: &Self) -> usize { /* ... */ }
  ```

- ```rust
  pub fn refresh_devices(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn refresh_storage(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn get_create_vm_data(self: &Self) -> (String, u32, u32) { /* ... */ }
  ```

- ```rust
  pub fn reset_create_vm_data(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn get_selected_vm_id(self: &Self) -> Option<u32> { /* ... */ }
  ```

- ```rust
  pub fn get_selected_action(self: &Self) -> usize { /* ... */ }
  ```

- ```rust
  pub fn is_create_vm_requested(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn set_tab(self: &mut Self, tab: DashboardTab) { /* ... */ }
  ```

- ```rust
  pub fn get_tab(self: &Self) -> DashboardTab { /* ... */ }
  ```

- ```rust
  pub fn handle_input(self: &mut Self, key: Key) { /* ... */ }
  ```

- ```rust
  pub fn exit_requested(self: &Self) -> bool { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `DashboardTab`

Available tabs in the dashboard.

```rust
pub enum DashboardTab {
    Overview,
    VirtualMachines,
    Resources,
    Storage,
    Network,
    Console,
    Devices,
    Test,
    CreateVM,
    Editor,
    Settings,
    Packages,
    Apps,
}
```

##### Variants

###### `Overview`

###### `VirtualMachines`

###### `Resources`

###### `Storage`

###### `Network`

###### `Console`

###### `Devices`

###### `Test`

###### `CreateVM`

###### `Editor`

###### `Settings`

###### `Packages`

###### `Apps`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DashboardTab { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `EditorMode`

```rust
pub enum EditorMode {
    Normal,
    Insert,
    Command,
}
```

##### Variants

###### `Normal`

###### `Insert`

###### `Command`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &EditorMode) -> bool { /* ... */ }
    ```

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `TextEditor`

```rust
pub struct TextEditor {
    pub file_path: alloc::string::String,
    pub buffer: alloc::vec::Vec<u8>,
    pub cursor_pos: (usize, usize),
    pub scroll_offset: usize,
    pub mode: EditorMode,
    pub is_hex: bool,
    pub command_buffer: alloc::string::String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `file_path` | `alloc::string::String` |  |
| `buffer` | `alloc::vec::Vec<u8>` |  |
| `cursor_pos` | `(usize, usize)` |  |
| `scroll_offset` | `usize` |  |
| `mode` | `EditorMode` |  |
| `is_hex` | `bool` |  |
| `command_buffer` | `alloc::string::String` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(path: String, data: Vec<u8>) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `VmDisplayInfo`

```rust
pub struct VmDisplayInfo {
    pub id: u32,
    pub name: alloc::string::String,
    pub state: alloc::string::String,
    pub cpu_usage: u32,
    pub memory_usage_mb: u32,
    pub disk_usage_mb: u32,
    pub uptime_seconds: u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `id` | `u32` |  |
| `name` | `alloc::string::String` |  |
| `state` | `alloc::string::String` |  |
| `cpu_usage` | `u32` |  |
| `memory_usage_mb` | `u32` |  |
| `disk_usage_mb` | `u32` |  |
| `uptime_seconds` | `u64` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `SystemResources`

```rust
pub struct SystemResources {
    pub total_memory_mb: u32,
    pub used_memory_mb: u32,
    pub cpu_count: u32,
    pub cpu_usage: u32,
    pub cpu_core_usage: alloc::vec::Vec<u32>,
    pub disk_read_kbps: u64,
    pub disk_write_kbps: u64,
    pub net_rx_kbps: u64,
    pub net_tx_kbps: u64,
    pub gpu_usage: u32,
    pub cpu_history: alloc::vec::Vec<u32>,
    pub mem_history: alloc::vec::Vec<u32>,
    pub disk_read_history: alloc::vec::Vec<u64>,
    pub disk_write_history: alloc::vec::Vec<u64>,
    pub net_rx_history: alloc::vec::Vec<u64>,
    pub net_tx_history: alloc::vec::Vec<u64>,
    pub gpu_history: alloc::vec::Vec<u32>,
    pub fps: usize,
    pub frame_ms: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `total_memory_mb` | `u32` |  |
| `used_memory_mb` | `u32` |  |
| `cpu_count` | `u32` |  |
| `cpu_usage` | `u32` |  |
| `cpu_core_usage` | `alloc::vec::Vec<u32>` |  |
| `disk_read_kbps` | `u64` |  |
| `disk_write_kbps` | `u64` |  |
| `net_rx_kbps` | `u64` |  |
| `net_tx_kbps` | `u64` |  |
| `gpu_usage` | `u32` |  |
| `cpu_history` | `alloc::vec::Vec<u32>` |  |
| `mem_history` | `alloc::vec::Vec<u32>` |  |
| `disk_read_history` | `alloc::vec::Vec<u64>` |  |
| `disk_write_history` | `alloc::vec::Vec<u64>` |  |
| `net_rx_history` | `alloc::vec::Vec<u64>` |  |
| `net_tx_history` | `alloc::vec::Vec<u64>` |  |
| `gpu_history` | `alloc::vec::Vec<u32>` |  |
| `fps` | `usize` |  |
| `frame_ms` | `usize` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `kernel`

```rust
pub(crate) mod kernel { /* ... */ }
```

### Types

#### Struct `KernelLoader`

```rust
pub struct KernelLoader;
```

##### Implementations

###### Methods

- ```rust
  pub fn load_kernel(path: &str) -> Result<Vec<u8>, &''static str> { /* ... */ }
  ```
  Load a kernel file from the filesystem

- ```rust
  pub fn load_kernel_dangerous(path: &str) -> Result<Vec<u8>, &''static str> { /* ... */ }
  ```

- ```rust
  pub unsafe fn execute_kernel(_kernel_data: &[u8], entry_point: u64) -> never { /* ... */ }
  ```
  Execute a loaded kernel (basic entry point jump)

- ```rust
  pub fn validate_kernel(data: &[u8]) -> Result<u64, &''static str> { /* ... */ }
  ```

- ```rust
  pub fn u16_to_cstr16_unsafe(data: &[u16]) -> Result<CString16, &''static str> { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `filesystem`

File system and device management.

This module provides an abstraction over the UEFI file system,
including path resolution, file operations, and device mapping.

```rust
pub(crate) mod filesystem { /* ... */ }
```

### Types

#### Struct `State`

Global file system state.

Holds the current working directory, device mappings, and drive information.

```rust
pub struct State {
    pub(in ::filesystem) cwd: alloc::string::String,
    pub device_map: alloc::vec::Vec<(alloc::string::String, alloc::string::String)>,
    pub root_handle: Option<uefi::Handle>,
    pub drive_handles: alloc::vec::Vec<(alloc::string::String, uefi::Handle)>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `cwd` | `alloc::string::String` |  |
| `device_map` | `alloc::vec::Vec<(alloc::string::String, alloc::string::String)>` |  |
| `root_handle` | `Option<uefi::Handle>` |  |
| `drive_handles` | `alloc::vec::Vec<(alloc::string::String, uefi::Handle)>` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> State { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Persistable**
  - ```rust
    fn magic() -> u32 { /* ... */ }
    ```

  - ```rust
    fn get_heap_bytes(self: &Self) -> Vec<u8> { /* ... */ }
    ```

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `FileSystem`

Entry point for file system operations.

```rust
pub struct FileSystem;
```

##### Implementations

###### Methods

- ```rust
  pub fn is_handle() -> bool { /* ... */ }
  ```

- ```rust
  pub fn get_state() -> &''static mut State { /* ... */ }
  ```
  Internal helper to access global state

- ```rust
  pub fn set_root_handle(handle: Option<Handle>) { /* ... */ }
  ```

- ```rust
  pub(in ::filesystem) fn resolve_path(path: &str) -> String { /* ... */ }
  ```
  Resolves path based on Aliases (dev0:), Root-relative (/), or CWD

- ```rust
  pub fn cd(path: &str) { /* ... */ }
  ```
  Change current directory

- ```rust
  pub fn scan_and_map_devices(map_file_path: &str) -> Result<(), &''static str> { /* ... */ }
  ```
  Scans all drives and writes "alias -> path" to a file

- ```rust
  pub fn mkdir(path: &str) -> Result<(), &''static str> { /* ... */ }
  ```

- ```rust
  pub fn touch(path: &str) -> Result<(), &''static str> { /* ... */ }
  ```

- ```rust
  pub fn copy(src: &str, dst: &str) -> Result<(), &''static str> { /* ... */ }
  ```

- ```rust
  pub fn move_file(src: &str, dst: &str) -> Result<(), &''static str> { /* ... */ }
  ```

- ```rust
  pub fn remove(path: &str) -> Result<(), &''static str> { /* ... */ }
  ```

- ```rust
  pub fn cat(path: &str) -> Result<(), &''static str> { /* ... */ }
  ```

- ```rust
  pub fn clone_dir(src: &str, dst: &str) -> Result<(), &''static str> { /* ... */ }
  ```

- ```rust
  pub fn write_to_file(path: &str, data: &str, mode: char) -> Result<(), &''static str> { /* ... */ }
  ```

- ```rust
  pub fn write_to_file_bytes(path: &str, data: &[u8], mode: char) -> Result<(), &''static str> { /* ... */ }
  ```

- ```rust
  pub(in ::filesystem) fn path_to_cstr16(path: &str) -> Result<&CStr16, &''static str> { /* ... */ }
  ```

- ```rust
  pub(in ::filesystem) fn get_root(drive_name: Option<&str>) -> Result<uefi::proto::media::file::Directory, &''static str> { /* ... */ }
  ```

- ```rust
  pub fn get_drives(drive_ref_file_path: &str) { /* ... */ }
  ```

- ```rust
  pub fn list_files() { /* ... */ }
  ```

- ```rust
  pub fn get_cwd() -> Result<String, ()> { /* ... */ }
  ```

- ```rust
  pub fn read_file(path: &str) -> Result<Vec<u8>, &''static str> { /* ... */ }
  ```
  Read a file and return its contents as a `Vec<u8>`

- ```rust
  pub fn read_file_to_string(path: &str) -> Result<String, &''static str> { /* ... */ }
  ```
  Read a file as a string

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Constants and Statics

#### Static `STATE`

```rust
pub(in ::filesystem) static mut STATE: Option<State> = None;
```

## Module `graphics`

```rust
pub(crate) mod graphics { /* ... */ }
```

### Types

#### Struct `AbsolutePointer`

**Attributes:**

- `Repr(AttributeRepr { kind: Transparent, align: None, packed: None, int: None })`

```rust
pub(in ::graphics) struct AbsolutePointer(pub(in ::graphics) uefi_raw::protocol::console::AbsolutePointerProtocol);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `uefi_raw::protocol::console::AbsolutePointerProtocol` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::graphics) fn read_state(self: &mut Self) -> uefi::Result<Option<uefi_raw::protocol::console::AbsolutePointerState>> { /* ... */ }
  ```

- ```rust
  pub(in ::graphics) fn mode(self: &Self) -> &uefi_raw::protocol::console::AbsolutePointerMode { /* ... */ }
  ```

- ```rust
  pub(in ::graphics) fn reset(self: &mut Self, extended: bool) -> uefi::Result { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Identify**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **Protocol**
- **ProtocolPointer**
  - ```rust
    unsafe fn ptr_from_ffi(ptr: *const c_void) -> *const P { /* ... */ }
    ```

  - ```rust
    unsafe fn mut_ptr_from_ffi(ptr: *mut c_void) -> *mut P { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Cursor`

**Attributes:**

- `Other("#[allow(dead_code)]")`

```rust
pub struct Cursor {
    pub x: i32,
    pub y: i32,
    pub visible: bool,
    pub left_button: bool,
    pub right_button: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `x` | `i32` |  |
| `y` | `i32` |  |
| `visible` | `bool` |  |
| `left_button` | `bool` |  |
| `right_button` | `bool` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub unsafe fn update_from_mouse(self: &mut Self, screen_width: usize, screen_height: usize) { /* ... */ }
  ```

- ```rust
  pub(in ::graphics) unsafe fn try_update_absolute(self: &mut Self, screen_width: usize, screen_height: usize) -> u32 { /* ... */ }
  ```

- ```rust
  pub(in ::graphics) unsafe fn try_update_relative(self: &mut Self, screen_width: usize, screen_height: usize) -> u32 { /* ... */ }
  ```

- ```rust
  pub fn render(self: &Self, stdout: &mut uefi::proto::console::text::Output) { /* ... */ }
  ```

- ```rust
  pub fn debug_mouse() { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `WinNTColors`

```rust
pub struct WinNTColors;
```

##### Implementations

###### Methods

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Rect`

```rust
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `x` | `usize` |  |
| `y` | `usize` |  |
| `width` | `usize` |  |
| `height` | `usize` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self { /* ... */ }
  ```

- ```rust
  pub fn contains(self: &Self, x: usize, y: usize) -> bool { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Graphics`

**Attributes:**

- `Other("#[allow(dead_code)]")`

```rust
pub struct Graphics;
```

##### Implementations

###### Methods

- ```rust
  pub fn clear_screen(bg: Color) { /* ... */ }
  ```
  Clear screen with background color

- ```rust
  pub fn set_cursor(col: usize, row: usize) { /* ... */ }
  ```
  Set cursor position (column, row)

- ```rust
  pub fn draw_box(rect: &Rect, title: &str, active: bool) { /* ... */ }
  ```
  Draw a titled box (window frame)

- ```rust
  pub fn draw_button(rect: &Rect, label: &str, focused: bool) { /* ... */ }
  ```
  Draw a 3D button

- ```rust
  pub fn draw_textbox(rect: &Rect, text: &str, focused: bool) { /* ... */ }
  ```
  Draw a text input field

- ```rust
  pub fn draw_menu_bar(items: &[&str]) { /* ... */ }
  ```
  Draw a menu bar

- ```rust
  pub fn draw_taskbar(time: &str) { /* ... */ }
  ```
  Draw taskbar at bottom

- ```rust
  pub fn draw_list(rect: &Rect, items: &[&str], selected: usize) { /* ... */ }
  ```
  Draw a scrollable list

- ```rust
  pub fn print_at(col: usize, row: usize, text: &str, fg: Color, bg: Color) { /* ... */ }
  ```
  Print text at position

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `interrupts`

```rust
pub(crate) mod interrupts { /* ... */ }
```

### Functions

#### Function `init_idt`

```rust
pub fn init_idt() { /* ... */ }
```

#### Function `breakpoint_handler`

```rust
pub(in ::interrupts) extern ""x86-interrupt"" fn breakpoint_handler(stack_frame: x86_64::structures::idt::InterruptStackFrame) { /* ... */ }
```

#### Function `double_fault_handler`

```rust
pub(in ::interrupts) extern ""x86-interrupt"" fn double_fault_handler(stack_frame: x86_64::structures::idt::InterruptStackFrame, _error_code: u64) -> never { /* ... */ }
```

#### Function `general_protection_fault_handler`

```rust
pub(in ::interrupts) extern ""x86-interrupt"" fn general_protection_fault_handler(stack_frame: x86_64::structures::idt::InterruptStackFrame, error_code: u64) { /* ... */ }
```

#### Function `page_fault_handler`

```rust
pub(in ::interrupts) extern ""x86-interrupt"" fn page_fault_handler(stack_frame: x86_64::structures::idt::InterruptStackFrame, error_code: x86_64::structures::idt::PageFaultErrorCode) { /* ... */ }
```

### Constants and Statics

#### Static `IDT`

```rust
pub(in ::interrupts) static mut IDT: x86_64::structures::idt::InterruptDescriptorTable = _;
```

## Module `gdt`

```rust
pub(crate) mod gdt { /* ... */ }
```

### Types

#### Struct `Selectors`

```rust
pub struct Selectors {
    pub code_selector: x86_64::structures::gdt::SegmentSelector,
    pub tss_selector: x86_64::structures::gdt::SegmentSelector,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `code_selector` | `x86_64::structures::gdt::SegmentSelector` |  |
| `tss_selector` | `x86_64::structures::gdt::SegmentSelector` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Functions

#### Function `init`

```rust
pub fn init() { /* ... */ }
```

### Constants and Statics

#### Constant `DOUBLE_FAULT_IST_INDEX`

```rust
pub const DOUBLE_FAULT_IST_INDEX: u16 = 1;
```

#### Static `TSS`

```rust
pub(in ::gdt) static mut TSS: x86_64::structures::tss::TaskStateSegment = _;
```

#### Static `GDT`

```rust
pub(in ::gdt) static mut GDT: x86_64::structures::gdt::GlobalDescriptorTable = _;
```

#### Static `SELECTORS`

```rust
pub(in ::gdt) static mut SELECTORS: Option<Selectors> = None;
```

## Module `imx`

```rust
pub(crate) mod imx { /* ... */ }
```

## Module `paging`

```rust
pub(crate) mod paging { /* ... */ }
```

### Types

#### Struct `PagingManager`

```rust
pub struct PagingManager;
```

##### Implementations

###### Methods

- ```rust
  pub unsafe fn get_active_mapper(physical_memory_offset: VirtAddr) -> OffsetPageTable<''static> { /* ... */ }
  ```
  Returns a mapper for the current active level 4 page table.

- ```rust
  pub fn map_address</* synthetic */ impl FrameAllocator<Size4KiB>: FrameAllocator<Size4KiB>>(virt: u64, phys: u64, flags: x86_64::structures::paging::PageTableFlags, mapper: &mut OffsetPageTable<''_>, frame_allocator: &mut impl FrameAllocator<Size4KiB>) -> Result<(), &''static str> { /* ... */ }
  ```
  Example: Map a specific virtual address to a physical address

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `tools`

```rust
pub(crate) mod tools { /* ... */ }
```

### Modules

## Module `dsk`

```rust
pub(crate) mod dsk { /* ... */ }
```

### Functions

#### Function `find_block_devs`

**Attributes:**

- `Other("#[allow(dead_code)]")`

```rust
pub(in ::tools::dsk) fn find_block_devs() -> Result<(), ()> { /* ... */ }
```

## Module `vmm`

HPVMx Hypervisor - Virtual Machine Monitor subsystem

```rust
pub(crate) mod vmm { /* ... */ }
```

### Modules

## Module `vmm`

HPVMx Virtual Machine Monitor - Core hypervisor coordinator

```rust
pub mod vmm { /* ... */ }
```

### Types

#### Struct `HypervisorManager`

Manages the lifecycle and execution of all virtual machines.

```rust
pub struct HypervisorManager {
    pub(crate) vms: alloc::collections::BTreeMap<u32, crate::vmm::vm::VirtualMachine>,
    pub is_initialized: bool,
    pub vm_count: u32,
    pub ghm: crate::vmm::ghm::GlobalHardwareManager,
    pub partitioner: crate::vmm::partitioner::HardwarePartitioner,
    pub security: crate::vmm::security::DeepLevelSecurity,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `vms` | `alloc::collections::BTreeMap<u32, crate::vmm::vm::VirtualMachine>` | Map of VM ID to Virtual Machine instance |
| `is_initialized` | `bool` | Hypervisor capabilities |
| `vm_count` | `u32` |  |
| `ghm` | `crate::vmm::ghm::GlobalHardwareManager` |  |
| `partitioner` | `crate::vmm::partitioner::HardwarePartitioner` |  |
| `security` | `crate::vmm::security::DeepLevelSecurity` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Create a new hypervisor manager instance

- ```rust
  pub fn initialize(self: &mut Self) -> Result<(), &''static str> { /* ... */ }
  ```
  Initialize the hypervisor

- ```rust
  pub fn create_vm(self: &mut Self, name: &str, memory_mb: u32, vcpu_count: u32) -> Result<u32, &''static str> { /* ... */ }
  ```
  Creates a new virtual machine.

- ```rust
  pub fn trigger_autolytic_response(self: &mut Self, vm_id: u32, error_code: u32) -> Result<(), &''static str> { /* ... */ }
  ```
  Trigger the Autolytic Protocol for a VM

- ```rust
  pub fn start_vm(self: &mut Self, vm_id: u32) -> Result<(), &''static str> { /* ... */ }
  ```
  Start a virtual machine

- ```rust
  pub fn stop_vm(self: &mut Self, vm_id: u32) -> Result<(), &''static str> { /* ... */ }
  ```
  Stop a virtual machine

- ```rust
  pub fn reset_vm(self: &mut Self, vm_id: u32) -> Result<(), &''static str> { /* ... */ }
  ```
  Reset a virtual machine

- ```rust
  pub fn zero_vm(self: &mut Self, vm_id: u32) -> Result<(), &''static str> { /* ... */ }
  ```
  Zero out a virtual machine's resources (Wipe memory and disk assignments)

- ```rust
  pub fn get_vm(self: &Self, vm_id: u32) -> Option<&VirtualMachine> { /* ... */ }
  ```
  Get VM by ID

- ```rust
  pub fn get_vm_mut(self: &mut Self, vm_id: u32) -> Option<&mut VirtualMachine> { /* ... */ }
  ```
  Get mutable VM by ID

- ```rust
  pub fn list_vms(self: &Self) -> Vec<(u32, String, VmState)> { /* ... */ }
  ```
  List all VMs

- ```rust
  pub fn delete_vm(self: &mut Self, vm_id: u32) -> Result<(), &''static str> { /* ... */ }
  ```
  Delete a VM

- ```rust
  pub fn get_stats(self: &Self) -> HypervisorStats { /* ... */ }
  ```
  Get hypervisor statistics

- ```rust
  pub fn get_stats_advanced(self: &Self) -> (HypervisorStats, String) { /* ... */ }
  ```

- ```rust
  pub fn boot_vm_with_media(self: &mut Self, vm_id: u32, media_path: &str) -> Result<(), &''static str> { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `HypervisorStats`

Global statistics for the hypervisor.

```rust
pub struct HypervisorStats {
    pub initialized: bool,
    pub total_vms: u32,
    pub running_vms: u32,
    pub total_memory_mb: u32,
    pub total_physical_memory_mb: u32,
    pub used_physical_memory_mb: u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `initialized` | `bool` | Whether the hypervisor is initialized. |
| `total_vms` | `u32` | Number of virtual machines currently defined. |
| `running_vms` | `u32` | Number of virtual machines currently running. |
| `total_memory_mb` | `u32` | Total memory allocated to all VMs in MB. |
| `total_physical_memory_mb` | `u32` | Total system memory in MB. |
| `used_physical_memory_mb` | `u32` | Memory used by system in MB. |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> HypervisorStats { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Constants and Statics

#### Static `VM_ID_COUNTER`

```rust
pub(in ::vmm::vmm) static VM_ID_COUNTER: core::sync::atomic::AtomicU32 = _;
```

## Module `vm`

Virtual Machine abstraction and lifecycle management

```rust
pub mod vm { /* ... */ }
```

### Types

#### Enum `VmState`

**Attributes:**

- `Other("#[allow(dead_code)]")`

Virtual Machine state
Possible execution states for a virtual machine.

```rust
pub enum VmState {
    Created,
    Running,
    Paused,
    Stopped,
    Failed,
    Decommissioned,
}
```

##### Variants

###### `Created`

Newly created, not yet started.

###### `Running`

Actively executing.

###### `Paused`

Temporarily suspended.

###### `Stopped`

Gracefully or forcefully shut down.

###### `Failed`

Encountered a critical error.

###### `Decommissioned`

Securely wiped and removed from the system.

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> VmState { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut core::fmt::Formatter<''_>) -> core::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &VmState) -> bool { /* ... */ }
    ```

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `VirtualMachine`

Represents an instance of a virtual machine.

```rust
pub struct VirtualMachine {
    pub id: u32,
    pub name: alloc::string::String,
    pub state: VmState,
    pub memory_mb: u32,
    pub vcpu_count: u32,
    pub vcpus: alloc::vec::Vec<crate::vmm::vcpu::VirtualCpu>,
    pub guest_memory_base: Option<usize>,
    pub vmbus: crate::vmm::vmbus::VmBus,
    pub mapper: crate::vmm::mapper::ResourceMapper,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `id` | `u32` | Unique identifier for the VM. |
| `name` | `alloc::string::String` | User-friendly name. |
| `state` | `VmState` | Current execution state. |
| `memory_mb` | `u32` | Memory size in megabytes. |
| `vcpu_count` | `u32` | Number of virtual CPUs. |
| `vcpus` | `alloc::vec::Vec<crate::vmm::vcpu::VirtualCpu>` | List of virtual CPUs associated with this VM. |
| `guest_memory_base` | `Option<usize>` | Base physical address of guest memory, if allocated. |
| `vmbus` | `crate::vmm::vmbus::VmBus` | Communication bus for guest-host interaction. |
| `mapper` | `crate::vmm::mapper::ResourceMapper` | Mapper for virtualized resources (Memory, Disk). |

##### Implementations

###### Methods

- ```rust
  pub fn new(id: u32, name: &str, memory_mb: u32, vcpu_count: u32) -> Result<Self, &''static str> { /* ... */ }
  ```
  Create a new virtual machine

- ```rust
  pub fn allocate_memory(self: &mut Self, base_addr: usize) -> Result<(), &''static str> { /* ... */ }
  ```
  Allocate guest memory for this VM

- ```rust
  pub fn get_memory_base(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Get guest memory base address

- ```rust
  pub fn get_memory_size(self: &Self) -> usize { /* ... */ }
  ```
  Get guest memory size in bytes

- ```rust
  pub fn add_vcpu(self: &mut Self) -> u32 { /* ... */ }
  ```
  Add a vCPU to this VM

- ```rust
  pub fn get_vcpu(self: &Self, vcpu_id: u32) -> Option<&VirtualCpu> { /* ... */ }
  ```
  Get a specific vCPU

- ```rust
  pub fn get_vcpu_mut(self: &mut Self, vcpu_id: u32) -> Option<&mut VirtualCpu> { /* ... */ }
  ```
  Get mutable vCPU

- ```rust
  pub fn resume(self: &mut Self) -> Result<(), &''static str> { /* ... */ }
  ```
  Resume execution

- ```rust
  pub fn pause(self: &mut Self) -> Result<(), &''static str> { /* ... */ }
  ```
  Pause execution

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `vcpu`

Virtual CPU management

```rust
pub mod vcpu { /* ... */ }
```

### Types

#### Struct `VCpuFlags`

vCPU execution flags

```rust
pub struct VCpuFlags(pub(in ::vmm::vcpu) <VCpuFlags as $crate::__private::PublicFlags>::Internal);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `<VCpuFlags as $crate::__private::PublicFlags>::Internal` |  |

##### Implementations

###### Methods

- ```rust
  pub const fn empty() -> Self { /* ... */ }
  ```
  Get a flags value with all bits unset.

- ```rust
  pub const fn all() -> Self { /* ... */ }
  ```
  Get a flags value with all known bits set.

- ```rust
  pub const fn bits(self: &Self) -> u32 { /* ... */ }
  ```
  Get the underlying bits value.

- ```rust
  pub const fn from_bits(bits: u32) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Convert from a bits value.

- ```rust
  pub const fn from_bits_truncate(bits: u32) -> Self { /* ... */ }
  ```
  Convert from a bits value, unsetting any unknown bits.

- ```rust
  pub const fn from_bits_retain(bits: u32) -> Self { /* ... */ }
  ```
  Convert from a bits value exactly.

- ```rust
  pub fn from_name(name: &str) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Get a flags value with the bits of a flag with the given name set.

- ```rust
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Whether all bits in this flags value are unset.

- ```rust
  pub const fn is_all(self: &Self) -> bool { /* ... */ }
  ```
  Whether all known bits in this flags value are set.

- ```rust
  pub const fn intersects(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether any set bits in a source flags value are also set in a target flags value.

- ```rust
  pub const fn contains(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether all set bits in a source flags value are also set in a target flags value.

- ```rust
  pub fn insert(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub fn remove(self: &mut Self, other: Self) { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags

- ```rust
  pub fn toggle(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub fn set(self: &mut Self, other: Self, value: bool) { /* ... */ }
  ```
  Call `insert` when `value` is `true` or `remove` when `value` is `false`.

- ```rust
  pub const fn intersection(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise and (`&`) of the bits in two flags values.

- ```rust
  pub const fn union(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub const fn difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags

- ```rust
  pub const fn symmetric_difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub const fn complement(self: Self) -> Self { /* ... */ }
  ```
  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- ```rust
  pub const fn iter(self: &Self) -> $crate::iter::Iter<VCpuFlags> { /* ... */ }
  ```
  Yield a set of contained flags values.

- ```rust
  pub const fn iter_names(self: &Self) -> $crate::iter::IterNames<VCpuFlags> { /* ... */ }
  ```
  Yield a set of contained named flags values.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: VCpuFlags) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> VCpuFlags { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **Flags**
  - ```rust
    fn bits(self: &Self) -> u32 { /* ... */ }
    ```

  - ```rust
    fn from_bits_retain(bits: u32) -> VCpuFlags { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **FromIterator**
  - ```rust
    fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> Self { /* ... */ }
    ```
    The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **PublicFlags**
- **RefUnwindSafe**
- **Send**
- **Sub**
  - ```rust
    fn sub(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

#### Struct `VirtualCpu`

**Attributes:**

- `Other("#[allow(dead_code)]")`

Virtual CPU state

```rust
pub struct VirtualCpu {
    pub id: u32,
    pub flags: VCpuFlags,
    pub rip: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub interrupt_pending: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `id` | `u32` |  |
| `flags` | `VCpuFlags` |  |
| `rip` | `u64` |  |
| `rsp` | `u64` |  |
| `rbp` | `u64` |  |
| `interrupt_pending` | `bool` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(id: u32) -> Self { /* ... */ }
  ```
  Create a new virtual CPU

- ```rust
  pub fn set_instruction_pointer(self: &mut Self, rip: u64) { /* ... */ }
  ```
  Set instruction pointer

- ```rust
  pub fn set_stack_pointer(self: &mut Self, rsp: u64) { /* ... */ }
  ```
  Set stack pointer

- ```rust
  pub fn halt(self: &mut Self) { /* ... */ }
  ```
  Halt the vCPU

- ```rust
  pub fn resume(self: &mut Self) { /* ... */ }
  ```
  Resume the vCPU

- ```rust
  pub fn is_halted(self: &Self) -> bool { /* ... */ }
  ```
  Is vCPU halted?

- ```rust
  pub fn inject_interrupt(self: &mut Self) { /* ... */ }
  ```
  Inject an interrupt

- ```rust
  pub fn get_registers(self: &Self) -> VCpuRegisters { /* ... */ }
  ```
  Get guest register state (simplified)

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> VirtualCpu { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `VCpuRegisters`

**Attributes:**

- `Other("#[allow(dead_code)]")`

Simplified vCPU register state

```rust
pub struct VCpuRegisters {
    pub rip: u64,
    pub rsp: u64,
    pub rbp: u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `rip` | `u64` |  |
| `rsp` | `u64` |  |
| `rbp` | `u64` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> VCpuRegisters { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `memory`

Guest memory management and isolation

```rust
pub mod memory { /* ... */ }
```

### Modules

## Module `ept`

**Attributes:**

- `Other("#[allow(dead_code)]")`
- `Other("#[attr = CfgTrace([NameValue { name: \"target_arch\", value: Some(\"x86_64\"), span: src\\vmm\\memory.rs:82:7: 82:29 (#0) }])]")`

Extended Page Tables (EPT) support for VT-x

```rust
pub mod ept { /* ... */ }
```

### Types

#### Struct `EptFlags`

EPT page table entry flags

```rust
pub struct EptFlags(pub(in ::vmm::memory::ept) <EptFlags as $crate::__private::PublicFlags>::Internal);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `<EptFlags as $crate::__private::PublicFlags>::Internal` |  |

##### Implementations

###### Methods

- ```rust
  pub const fn empty() -> Self { /* ... */ }
  ```
  Get a flags value with all bits unset.

- ```rust
  pub const fn all() -> Self { /* ... */ }
  ```
  Get a flags value with all known bits set.

- ```rust
  pub const fn bits(self: &Self) -> u64 { /* ... */ }
  ```
  Get the underlying bits value.

- ```rust
  pub const fn from_bits(bits: u64) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Convert from a bits value.

- ```rust
  pub const fn from_bits_truncate(bits: u64) -> Self { /* ... */ }
  ```
  Convert from a bits value, unsetting any unknown bits.

- ```rust
  pub const fn from_bits_retain(bits: u64) -> Self { /* ... */ }
  ```
  Convert from a bits value exactly.

- ```rust
  pub fn from_name(name: &str) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Get a flags value with the bits of a flag with the given name set.

- ```rust
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Whether all bits in this flags value are unset.

- ```rust
  pub const fn is_all(self: &Self) -> bool { /* ... */ }
  ```
  Whether all known bits in this flags value are set.

- ```rust
  pub const fn intersects(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether any set bits in a source flags value are also set in a target flags value.

- ```rust
  pub const fn contains(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether all set bits in a source flags value are also set in a target flags value.

- ```rust
  pub fn insert(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub fn remove(self: &mut Self, other: Self) { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags

- ```rust
  pub fn toggle(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub fn set(self: &mut Self, other: Self, value: bool) { /* ... */ }
  ```
  Call `insert` when `value` is `true` or `remove` when `value` is `false`.

- ```rust
  pub const fn intersection(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise and (`&`) of the bits in two flags values.

- ```rust
  pub const fn union(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub const fn difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags

- ```rust
  pub const fn symmetric_difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub const fn complement(self: Self) -> Self { /* ... */ }
  ```
  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- ```rust
  pub const fn iter(self: &Self) -> $crate::iter::Iter<EptFlags> { /* ... */ }
  ```
  Yield a set of contained flags values.

- ```rust
  pub const fn iter_names(self: &Self) -> $crate::iter::IterNames<EptFlags> { /* ... */ }
  ```
  Yield a set of contained named flags values.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: EptFlags) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **Flags**
  - ```rust
    fn bits(self: &Self) -> u64 { /* ... */ }
    ```

  - ```rust
    fn from_bits_retain(bits: u64) -> EptFlags { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **FromIterator**
  - ```rust
    fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> Self { /* ... */ }
    ```
    The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **PublicFlags**
- **RefUnwindSafe**
- **Send**
- **Sub**
  - ```rust
    fn sub(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

#### Struct `EptEntry`

**Attributes:**

- `Repr(AttributeRepr { kind: Transparent, align: None, packed: None, int: None })`

EPT page table entry

```rust
pub struct EptEntry(pub(in ::vmm::memory::ept) u64);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u64` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(physical_addr: u64, flags: EptFlags) -> Self { /* ... */ }
  ```

- ```rust
  pub fn flags(self: &Self) -> EptFlags { /* ... */ }
  ```

- ```rust
  pub fn physical_address(self: &Self) -> u64 { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Types

#### Struct `MemoryPage`

**Attributes:**

- `Other("#[allow(dead_code)]")`

Guest memory page entry

```rust
pub struct MemoryPage {
    pub gpa: u64,
    pub hpa: u64,
    pub size: usize,
    pub writable: bool,
    pub executable: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `gpa` | `u64` |  |
| `hpa` | `u64` |  |
| `size` | `usize` |  |
| `writable` | `bool` |  |
| `executable` | `bool` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MemoryPage { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `MemoryManager`

**Attributes:**

- `Other("#[allow(dead_code)]")`

Guest memory manager

```rust
pub struct MemoryManager {
    pub(in ::vmm::memory) pages: alloc::vec::Vec<MemoryPage>,
    pub(in ::vmm::memory) total_size: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `pages` | `alloc::vec::Vec<MemoryPage>` |  |
| `total_size` | `usize` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(total_size: usize) -> Self { /* ... */ }
  ```
  Create a new memory manager for guest memory

- ```rust
  pub fn map_page(self: &mut Self, gpa: u64, hpa: u64, size: usize, writable: bool, executable: bool) -> Result<(), &''static str> { /* ... */ }
  ```
  Map guest physical address to host physical address

- ```rust
  pub fn translate_gpa(self: &Self, gpa: u64) -> Option<u64> { /* ... */ }
  ```
  Lookup GPA to HPA translation

- ```rust
  pub fn page_count(self: &Self) -> usize { /* ... */ }
  ```
  Get page count

- ```rust
  pub fn total_size(self: &Self) -> usize { /* ... */ }
  ```
  Get total allocated memory

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `io`

I/O device virtualization and emulation

```rust
pub mod io { /* ... */ }
```

### Types

#### Struct `VirtualConsole`

**Attributes:**

- `Other("#[allow(dead_code)]")`

Virtual Serial Console device

```rust
pub struct VirtualConsole {
    pub name: alloc::string::String,
    pub data_buffer: [u8; 256],
    pub buffer_pos: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `alloc::string::String` |  |
| `data_buffer` | `[u8; 256]` |  |
| `buffer_pos` | `usize` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn write_byte(self: &mut Self, byte: u8) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **VirtioDevice**
  - ```rust
    fn device_name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn handle_io_read(self: &Self, _port: u16, _size: u32) -> u32 { /* ... */ }
    ```

  - ```rust
    fn handle_io_write(self: &mut Self, _port: u16, _size: u32, data: u32) -> Result<(), &''static str> { /* ... */ }
    ```

#### Struct `IoManager`

**Attributes:**

- `Other("#[allow(dead_code)]")`

Virtual device manager

```rust
pub struct IoManager {
    pub(in ::vmm::io) devices: alloc::collections::BTreeMap<alloc::string::String, usize>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `devices` | `alloc::collections::BTreeMap<alloc::string::String, usize>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn register_device(self: &mut Self, name: String) { /* ... */ }
  ```

- ```rust
  pub fn device_count(self: &Self) -> usize { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Traits

#### Trait `VirtioDevice`

**Attributes:**

- `Other("#[allow(dead_code)]")`

Virtual I/O device trait

```rust
pub trait VirtioDevice {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `device_name`
- `handle_io_read`
- `handle_io_write`

##### Implementations

This trait is implemented for the following types:

- `VirtualConsole`

## Module `loader`

```rust
pub(crate) mod loader { /* ... */ }
```

## Module `bootloader`

Bootloader support for various OS types

```rust
pub mod bootloader { /* ... */ }
```

### Types

#### Struct `BootLoader`

**Attributes:**

- `Other("#[allow(dead_code)]")`

```rust
pub struct BootLoader {
    pub boot_type: BootType,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `boot_type` | `BootType` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(boot_type: BootType) -> Self { /* ... */ }
  ```

- ```rust
  pub fn load_efi_firmware(self: &Self, _path: &str) -> Result<Vec<u8>, &str> { /* ... */ }
  ```

- ```rust
  pub fn load_kernel(self: &Self, path: &str) -> Result<Vec<u8>, &''static str> { /* ... */ }
  ```

- ```rust
  pub fn load_initrd(self: &Self, _path: &str) -> Result<Vec<u8>, &str> { /* ... */ }
  ```

- ```rust
  pub fn prepare_boot_environment(self: &Self, _kernel: &[u8], _initrd: Option<&[u8]>, cmdline: &str) -> Result<BootEnvironment, &str> { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `BootType`

**Attributes:**

- `Other("#[allow(dead_code)]")`

```rust
pub enum BootType {
    BIOS,
    UEFI,
    Multiboot,
    Multiboot2,
}
```

##### Variants

###### `BIOS`

###### `UEFI`

###### `Multiboot`

###### `Multiboot2`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> BootType { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `BootEnvironment`

**Attributes:**

- `Other("#[allow(dead_code)]")`

```rust
pub struct BootEnvironment {
    pub kernel_addr: u64,
    pub initrd_addr: u64,
    pub cmdline: alloc::string::String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `kernel_addr` | `u64` |  |
| `initrd_addr` | `u64` |  |
| `cmdline` | `alloc::string::String` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `storage`

Virtual storage device management for VMs

```rust
pub mod storage { /* ... */ }
```

### Types

#### Struct `StorageManager`

**Attributes:**

- `Other("#[allow(dead_code)]")`

```rust
pub struct StorageManager {
    pub(in ::vmm::storage) disks: alloc::vec::Vec<VirtualDisk>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `disks` | `alloc::vec::Vec<VirtualDisk>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn attach_disk(self: &mut Self, vm_id: u32, image_path: &str, size_mb: u32) -> Result<u32, &str> { /* ... */ }
  ```

- ```rust
  pub fn get_vm_disks(self: &Self, vm_id: u32) -> Vec<&VirtualDisk> { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `VirtualDisk`

**Attributes:**

- `Other("#[allow(dead_code)]")`

```rust
pub struct VirtualDisk {
    pub id: u32,
    pub vm_id: u32,
    pub name: alloc::string::String,
    pub size_mb: u32,
    pub disk_type: DiskType,
    pub image_path: alloc::string::String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `id` | `u32` |  |
| `vm_id` | `u32` |  |
| `name` | `alloc::string::String` |  |
| `size_mb` | `u32` |  |
| `disk_type` | `DiskType` |  |
| `image_path` | `alloc::string::String` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `DiskType`

**Attributes:**

- `Other("#[allow(dead_code)]")`

```rust
pub enum DiskType {
    Qcow2,
    RawImage,
    VDI,
    VMDK,
}
```

##### Variants

###### `Qcow2`

###### `RawImage`

###### `VDI`

###### `VMDK`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DiskType { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `devices`

```rust
pub mod devices { /* ... */ }
```

## Module `ghm`

Global Hardware Manager (GHM) - A "Push-Only" allocator that assigns physical resources to VM Units.
Zero-Request Model: The GHM accepts zero incoming requests from VMs, eliminating hypercall-based privilege escalation.

```rust
pub mod ghm { /* ... */ }
```

### Types

#### Enum `PhysicalResourceId`

```rust
pub enum PhysicalResourceId {
    CpuCore(u32),
    MemorySegment(u64, usize),
}
```

##### Variants

###### `CpuCore`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

###### `MemorySegment`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u64` |  |
| 1 | `usize` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PhysicalResourceId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `ResourceAssignment`

```rust
pub struct ResourceAssignment {
    pub vm_id: u32,
    pub resource_id: PhysicalResourceId,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `vm_id` | `u32` |  |
| `resource_id` | `PhysicalResourceId` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `GlobalHardwareManager`

```rust
pub struct GlobalHardwareManager {
    pub(in ::vmm::ghm) assignments: alloc::vec::Vec<ResourceAssignment>,
    pub(in ::vmm::ghm) available_cores: alloc::vec::Vec<u32>,
    pub(in ::vmm::ghm) available_memory: alloc::vec::Vec<(u64, usize)>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `assignments` | `alloc::vec::Vec<ResourceAssignment>` |  |
| `available_cores` | `alloc::vec::Vec<u32>` |  |
| `available_memory` | `alloc::vec::Vec<(u64, usize)>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(total_cores: u32, total_memory_mb: usize) -> Self { /* ... */ }
  ```

- ```rust
  pub fn push_assignment(self: &mut Self, vm_id: u32, resource: PhysicalResourceId) { /* ... */ }
  ```
  Push an assignment to a VM. This is the only way resources are allocated.

- ```rust
  pub fn allocate_core_to_vm(self: &mut Self, vm_id: u32) -> Result<u32, &''static str> { /* ... */ }
  ```

- ```rust
  pub fn allocate_memory_to_vm(self: &mut Self, vm_id: u32, size_bytes: usize) -> Result<u64, &''static str> { /* ... */ }
  ```

- ```rust
  pub fn revoke_assignments(self: &mut Self, vm_id: u32) { /* ... */ }
  ```
  Revoke all assignments for a specific VM (used during decommissioning)

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `partitioner`

Hardware Partitioner - Carves physical CPU cores and memory segments into isolated "Silicons."
Prevents cross-VM interference and significantly mitigates CPU side-channel leaks.

```rust
pub mod partitioner { /* ... */ }
```

### Types

#### Struct `SiliconUnit`

```rust
pub struct SiliconUnit {
    pub vm_id: u32,
    pub cores: alloc::vec::Vec<u32>,
    pub memory_base: u64,
    pub memory_size: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `vm_id` | `u32` |  |
| `cores` | `alloc::vec::Vec<u32>` |  |
| `memory_base` | `u64` |  |
| `memory_size` | `usize` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SiliconUnit { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `HardwarePartitioner`

```rust
pub struct HardwarePartitioner {
    pub(in ::vmm::partitioner) units: alloc::vec::Vec<SiliconUnit>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `units` | `alloc::vec::Vec<SiliconUnit>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn create_silicon_unit(self: &mut Self, vm_id: u32, cores: Vec<u32>, memory_base: u64, memory_size: usize) -> Result<(), &''static str> { /* ... */ }
  ```
  Carve out a new Silicon unit for a VM.

- ```rust
  pub fn remove_silicon_unit(self: &mut Self, vm_id: u32) { /* ... */ }
  ```
  Remove a Silicon unit.

- ```rust
  pub fn get_unit(self: &Self, vm_id: u32) -> Option<&SiliconUnit> { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `vmbus`

Virtual Machine Bus (VMBUS) - Serialized and inspected communication channel.
All inter-unit communication is serialized and inspected via the VMBUS,
ensuring no "side-door" access to system components.

```rust
pub mod vmbus { /* ... */ }
```

### Types

#### Enum `VmBusMessage`

```rust
pub enum VmBusMessage {
    IoRequest {
        address: u64,
        size: usize,
        write: bool,
        data: Option<alloc::vec::Vec<u8>>,
    },
    Interrupt {
        vector: u8,
    },
    StorageRequest {
        sector: u64,
        count: u32,
        write: bool,
        data: Option<alloc::vec::Vec<u8>>,
    },
}
```

##### Variants

###### `IoRequest`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `address` | `u64` |  |
| `size` | `usize` |  |
| `write` | `bool` |  |
| `data` | `Option<alloc::vec::Vec<u8>>` |  |

###### `Interrupt`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `vector` | `u8` |  |

###### `StorageRequest`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `sector` | `u64` |  |
| `count` | `u32` |  |
| `write` | `bool` |  |
| `data` | `Option<alloc::vec::Vec<u8>>` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> VmBusMessage { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `VmBus`

```rust
pub struct VmBus {
    pub vm_id: u32,
    pub(in ::vmm::vmbus) queue: alloc::collections::VecDeque<VmBusMessage>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `vm_id` | `u32` |  |
| `queue` | `alloc::collections::VecDeque<VmBusMessage>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(vm_id: u32) -> Self { /* ... */ }
  ```

- ```rust
  pub fn send_message(self: &mut Self, message: VmBusMessage) { /* ... */ }
  ```
  Send a message over the bus. This is the primary communication method.

- ```rust
  pub fn receive_message(self: &mut Self) -> Option<VmBusMessage> { /* ... */ }
  ```
  Receive a message from the bus.

- ```rust
  pub fn inspect_messages<F>(self: &Self, inspector: F)
where
    F: FnMut(&VmBusMessage) -> bool { /* ... */ }
  ```
  Inspect all messages currently in the queue (used by Deep Level Security).

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `security`

Deep Level Security & Autolytic "Fail-Stop" Protocol.
Central gateway for intercepting all VMBUS traffic and managing intrusion response.

```rust
pub mod security { /* ... */ }
```

### Types

#### Struct `DeepLevelSecurity`

```rust
pub struct DeepLevelSecurity {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn inspect_bus(self: &Self, bus: &mut VmBus) -> Result<(), &''static str> { /* ... */ }
  ```
  Intercept and inspect traffic on the VMBUS.

- ```rust
  pub(in ::vmm::security) fn is_authorized(self: &Self, message: &VmBusMessage) -> bool { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `AutolyticProtocol`

```rust
pub struct AutolyticProtocol;
```

##### Implementations

###### Methods

- ```rust
  pub fn detect_violation(vm_id: u32, error_code: u32) { /* ... */ }
  ```
  Phase 1: Detection

- ```rust
  pub(in ::vmm::security) fn trigger_fail_stop(vm_id: u32, _error_code: u32) { /* ... */ }
  ```
  Phase 2: System Hibernation

- ```rust
  pub fn forensic_scan(vhd_path: &str, error_code: u32) -> bool { /* ... */ }
  ```
  Phase 3: Forensic Deep Scan

- ```rust
  pub fn decommissioning_and_zeroing(vm_id: u32) { /* ... */ }
  ```
  Phase 4: Decommissioning & Zeroing

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `mapper`

VDISK and VMEM Mapper.
Translates virtual addresses/sectors to physical ones.
Abstraction layer that prevents a VM from seeing or touching the host's actual memory or disk structure.

```rust
pub mod mapper { /* ... */ }
```

### Types

#### Struct `MemoryMapping`

```rust
pub struct MemoryMapping {
    pub gpa: u64,
    pub hpa: u64,
    pub size: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `gpa` | `u64` |  |
| `hpa` | `u64` |  |
| `size` | `usize` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `DiskMapping`

```rust
pub struct DiskMapping {
    pub guest_sector: u64,
    pub host_vhd_offset: u64,
    pub size_sectors: u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `guest_sector` | `u64` |  |
| `host_vhd_offset` | `u64` |  |
| `size_sectors` | `u64` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `ResourceMapper`

```rust
pub struct ResourceMapper {
    pub vm_id: u32,
    pub(in ::vmm::mapper) memory_mappings: alloc::vec::Vec<MemoryMapping>,
    pub(in ::vmm::mapper) disk_mappings: alloc::vec::Vec<DiskMapping>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `vm_id` | `u32` |  |
| `memory_mappings` | `alloc::vec::Vec<MemoryMapping>` |  |
| `disk_mappings` | `alloc::vec::Vec<DiskMapping>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(vm_id: u32) -> Self { /* ... */ }
  ```

- ```rust
  pub fn add_memory_mapping(self: &mut Self, gpa: u64, hpa: u64, size: usize) { /* ... */ }
  ```

- ```rust
  pub fn add_disk_mapping(self: &mut Self, guest_sector: u64, host_vhd_offset: u64, size_sectors: u64) { /* ... */ }
  ```

- ```rust
  pub fn get_memory_mappings(self: &Self) -> &[MemoryMapping] { /* ... */ }
  ```

- ```rust
  pub fn translate_gpa(self: &Self, gpa: u64) -> Option<u64> { /* ... */ }
  ```
  Translate GPA to HPA.

- ```rust
  pub fn translate_sector(self: &Self, sector: u64) -> Option<u64> { /* ... */ }
  ```
  Translate guest sector to host VHD offset.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Re-exports

#### Re-export `HypervisorManager`

```rust
pub use vmm::HypervisorManager;
```

## Module `hardware`

Hardware abstraction layer with virtualization support

```rust
pub(crate) mod hardware { /* ... */ }
```

### Modules

## Module `cpu`

CPU hardware abstraction and virtualization support

```rust
pub mod cpu { /* ... */ }
```

### Modules

## Module `vmx`

**Attributes:**

- `Other("#[attr = CfgTrace([NameValue { name: \"target_arch\", value: Some(\"x86_64\"), span: src\\hardware\\cpu.rs:8:7: 8:29 (#0) }])]")`

Intel VT-x virtualization support for HPVMx

```rust
pub mod vmx { /* ... */ }
```

### Types

#### Struct `VtxCapabilities`

**Attributes:**

- `Other("#[allow(dead_code)]")`

VT-x capability flags

```rust
pub struct VtxCapabilities {
    pub available: bool,
    pub vmxon_supported: bool,
    pub ept_supported: bool,
    pub vpid_supported: bool,
    pub unrestricted_guest: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `available` | `bool` |  |
| `vmxon_supported` | `bool` |  |
| `ept_supported` | `bool` |  |
| `vpid_supported` | `bool` |  |
| `unrestricted_guest` | `bool` |  |

##### Implementations

###### Methods

- ```rust
  pub fn detect() -> Self { /* ... */ }
  ```
  Detect VT-x capabilities on this CPU

- ```rust
  pub fn supports_unrestricted_guest(self: &Self) -> bool { /* ... */ }
  ```
  Check if CPU supports unrestricted guest mode

- ```rust
  pub fn supports_ept(self: &Self) -> bool { /* ... */ }
  ```
  Check if we can use Extended Page Tables

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> VtxCapabilities { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `VmcsControl`

VMCS control bits for VM execution control

```rust
pub struct VmcsControl(pub(in ::hardware::cpu::vmx) <VmcsControl as $crate::__private::PublicFlags>::Internal);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `<VmcsControl as $crate::__private::PublicFlags>::Internal` |  |

##### Implementations

###### Methods

- ```rust
  pub const fn empty() -> Self { /* ... */ }
  ```
  Get a flags value with all bits unset.

- ```rust
  pub const fn all() -> Self { /* ... */ }
  ```
  Get a flags value with all known bits set.

- ```rust
  pub const fn bits(self: &Self) -> u32 { /* ... */ }
  ```
  Get the underlying bits value.

- ```rust
  pub const fn from_bits(bits: u32) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Convert from a bits value.

- ```rust
  pub const fn from_bits_truncate(bits: u32) -> Self { /* ... */ }
  ```
  Convert from a bits value, unsetting any unknown bits.

- ```rust
  pub const fn from_bits_retain(bits: u32) -> Self { /* ... */ }
  ```
  Convert from a bits value exactly.

- ```rust
  pub fn from_name(name: &str) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Get a flags value with the bits of a flag with the given name set.

- ```rust
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Whether all bits in this flags value are unset.

- ```rust
  pub const fn is_all(self: &Self) -> bool { /* ... */ }
  ```
  Whether all known bits in this flags value are set.

- ```rust
  pub const fn intersects(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether any set bits in a source flags value are also set in a target flags value.

- ```rust
  pub const fn contains(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether all set bits in a source flags value are also set in a target flags value.

- ```rust
  pub fn insert(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub fn remove(self: &mut Self, other: Self) { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags

- ```rust
  pub fn toggle(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub fn set(self: &mut Self, other: Self, value: bool) { /* ... */ }
  ```
  Call `insert` when `value` is `true` or `remove` when `value` is `false`.

- ```rust
  pub const fn intersection(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise and (`&`) of the bits in two flags values.

- ```rust
  pub const fn union(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub const fn difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags

- ```rust
  pub const fn symmetric_difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub const fn complement(self: Self) -> Self { /* ... */ }
  ```
  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- ```rust
  pub const fn iter(self: &Self) -> $crate::iter::Iter<VmcsControl> { /* ... */ }
  ```
  Yield a set of contained flags values.

- ```rust
  pub const fn iter_names(self: &Self) -> $crate::iter::IterNames<VmcsControl> { /* ... */ }
  ```
  Yield a set of contained named flags values.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: VmcsControl) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **Flags**
  - ```rust
    fn bits(self: &Self) -> u32 { /* ... */ }
    ```

  - ```rust
    fn from_bits_retain(bits: u32) -> VmcsControl { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **FromIterator**
  - ```rust
    fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> Self { /* ... */ }
    ```
    The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **PublicFlags**
- **RefUnwindSafe**
- **Send**
- **Sub**
  - ```rust
    fn sub(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

#### Struct `VmcsEntryControl`

VMCS entry control bits

```rust
pub struct VmcsEntryControl(pub(in ::hardware::cpu::vmx) <VmcsEntryControl as $crate::__private::PublicFlags>::Internal);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `<VmcsEntryControl as $crate::__private::PublicFlags>::Internal` |  |

##### Implementations

###### Methods

- ```rust
  pub const fn empty() -> Self { /* ... */ }
  ```
  Get a flags value with all bits unset.

- ```rust
  pub const fn all() -> Self { /* ... */ }
  ```
  Get a flags value with all known bits set.

- ```rust
  pub const fn bits(self: &Self) -> u32 { /* ... */ }
  ```
  Get the underlying bits value.

- ```rust
  pub const fn from_bits(bits: u32) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Convert from a bits value.

- ```rust
  pub const fn from_bits_truncate(bits: u32) -> Self { /* ... */ }
  ```
  Convert from a bits value, unsetting any unknown bits.

- ```rust
  pub const fn from_bits_retain(bits: u32) -> Self { /* ... */ }
  ```
  Convert from a bits value exactly.

- ```rust
  pub fn from_name(name: &str) -> $crate::__private::core::option::Option<Self> { /* ... */ }
  ```
  Get a flags value with the bits of a flag with the given name set.

- ```rust
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Whether all bits in this flags value are unset.

- ```rust
  pub const fn is_all(self: &Self) -> bool { /* ... */ }
  ```
  Whether all known bits in this flags value are set.

- ```rust
  pub const fn intersects(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether any set bits in a source flags value are also set in a target flags value.

- ```rust
  pub const fn contains(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Whether all set bits in a source flags value are also set in a target flags value.

- ```rust
  pub fn insert(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub fn remove(self: &mut Self, other: Self) { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags

- ```rust
  pub fn toggle(self: &mut Self, other: Self) { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub fn set(self: &mut Self, other: Self, value: bool) { /* ... */ }
  ```
  Call `insert` when `value` is `true` or `remove` when `value` is `false`.

- ```rust
  pub const fn intersection(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise and (`&`) of the bits in two flags values.

- ```rust
  pub const fn union(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise or (`|`) of the bits in two flags values.

- ```rust
  pub const fn difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The intersection of a source flags value with the complement of a target flags

- ```rust
  pub const fn symmetric_difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  The bitwise exclusive-or (`^`) of the bits in two flags values.

- ```rust
  pub const fn complement(self: Self) -> Self { /* ... */ }
  ```
  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- ```rust
  pub const fn iter(self: &Self) -> $crate::iter::Iter<VmcsEntryControl> { /* ... */ }
  ```
  Yield a set of contained flags values.

- ```rust
  pub const fn iter_names(self: &Self) -> $crate::iter::IterNames<VmcsEntryControl> { /* ... */ }
  ```
  Yield a set of contained named flags values.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise and (`&`) of the bits in two flags values.

- **BitOr**
  - ```rust
    fn bitor(self: Self, other: VmcsEntryControl) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in two flags values.

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The bitwise exclusive-or (`^`) of the bits in two flags values.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Extend**
  - ```rust
    fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T) { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **Flags**
  - ```rust
    fn bits(self: &Self) -> u32 { /* ... */ }
    ```

  - ```rust
    fn from_bits_retain(bits: u32) -> VmcsEntryControl { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **FromIterator**
  - ```rust
    fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self { /* ... */ }
    ```
    The bitwise or (`|`) of the bits in each flags value.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> Self { /* ... */ }
    ```
    The bitwise negation (`!`) of the bits in a flags value, truncating the result.

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

- **Pointee**
- **PublicFlags**
- **RefUnwindSafe**
- **Send**
- **Sub**
  - ```rust
    fn sub(self: Self, other: Self) -> Self { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    The intersection of a source flags value with the complement of a target flags value (`&!`).

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<''_>) -> $crate::__private::core::fmt::Result { /* ... */ }
    ```

#### Struct `VmxonRegion`

**Attributes:**

- `Other("#[allow(dead_code)]")`
- `Repr(AttributeRepr { kind: C, align: Some(4096), packed: None, int: None })`

VMXON region structure (4KB aligned)

```rust
pub struct VmxonRegion {
    pub revision_id: u32,
    pub data: [u8; 4092],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `revision_id` | `u32` |  |
| `data` | `[u8; 4092]` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `VmcsRegion`

**Attributes:**

- `Other("#[allow(dead_code)]")`
- `Repr(AttributeRepr { kind: C, align: Some(4096), packed: None, int: None })`

VMCS (Virtual Machine Control Structure) region

```rust
pub struct VmcsRegion {
    pub revision_id: u32,
    pub data: [u8; 4092],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `revision_id` | `u32` |  |
| `data` | `[u8; 4092]` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(revision_id: u32) -> Self { /* ... */ }
  ```
  Create a new VMCS region with proper revision ID

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Types

#### Struct `CpuInfo`

**Attributes:**

- `Other("#[allow(dead_code)]")`

Information about the host CPU's capabilities.

```rust
pub struct CpuInfo {
    pub brand: alloc::string::String,
    pub cores: u32,
    pub threads: u32,
    pub supports_64bit: bool,
    pub supports_vmx: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `brand` | `alloc::string::String` | The CPU brand string. |
| `cores` | `u32` | The number of physical cores. |
| `threads` | `u32` | The number of logical threads. |
| `supports_64bit` | `bool` | Whether the CPU supports 64-bit operations. |
| `supports_vmx` | `bool` | Whether the CPU supports Intel VMX (Virtual Machine Extensions). |

##### Implementations

###### Methods

- ```rust
  pub fn detect() -> Self { /* ... */ }
  ```
  Detect CPU capabilities

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CpuInfo { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Macros

#### Macro `hpvm_log`

**Attributes:**

- `Other("#[allow(dead_code)]")`

```rust
pub(crate) macro_rules! hpvm_log {
    /* macro_rules! hpvm_log {
    ($color:expr, $prefix:expr, $($arg:tt)*) => { ... };
} */
}
```

#### Macro `hpvm_info`

```rust
pub(crate) macro_rules! hpvm_info {
    /* macro_rules! hpvm_info {
    ($tag:expr, $($arg:tt)*) => { ... };
} */
}
```

### Re-exports

#### Re-export `vmx`

**Attributes:**

- `Other("#[attr = CfgTrace([NameValue { name: \"target_arch\", value: Some(\"x86_64\"), span: src\\hardware\\mod.rs:6:7: 6:29 (#0) }])]")`

```rust
pub use cpu::vmx;
```

## Module `logiclang_int`

LogicLang Interpreter - A logic programming language for HPVMx

This is a Rust port of the Python LogicLang interpreter
It supports custom command execution for the HPVMx OS

```rust
pub(crate) mod logiclang_int { /* ... */ }
```

### Modules

## Module `lexer`

Lexer for LogicLang - tokenizes input

```rust
pub mod lexer { /* ... */ }
```

### Types

#### Enum `Token`

```rust
pub enum Token {
    Atom(alloc::string::String),
    Variable(alloc::string::String),
    Integer(i64),
    String(alloc::string::String),
    ColonDash,
    Dot,
    Comma,
    Pipe,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanEq,
    GreaterThanEq,
    Is,
    Plus,
    Minus,
    Star,
    Slash,
    Mod,
    Cut,
    Underscore,
    Eof,
}
```

##### Variants

###### `Atom`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `Variable`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `Integer`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i64` |  |

###### `String`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `ColonDash`

###### `Dot`

###### `Comma`

###### `Pipe`

###### `LeftParen`

###### `RightParen`

###### `LeftBracket`

###### `RightBracket`

###### `LeftBrace`

###### `RightBrace`

###### `Equals`

###### `NotEquals`

###### `LessThan`

###### `GreaterThan`

###### `LessThanEq`

###### `GreaterThanEq`

###### `Is`

###### `Plus`

###### `Minus`

###### `Star`

###### `Slash`

###### `Mod`

###### `Cut`

###### `Underscore`

###### `Eof`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Token { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Token) -> bool { /* ... */ }
    ```

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Lexer`

```rust
pub struct Lexer {
    pub(in ::logiclang_int::lexer) input: alloc::vec::Vec<char>,
    pub(in ::logiclang_int::lexer) position: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `input` | `alloc::vec::Vec<char>` |  |
| `position` | `usize` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(input: &str) -> Self { /* ... */ }
  ```

- ```rust
  pub fn tokenize(self: &mut Self) -> Result<Vec<Token>, String> { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::lexer) fn next_token(self: &mut Self) -> Result<Token, String> { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::lexer) fn read_atom_or_keyword(self: &mut Self) -> Result<Token, String> { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::lexer) fn read_variable(self: &mut Self) -> Result<Token, String> { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::lexer) fn read_string(self: &mut Self) -> Result<Token, String> { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::lexer) fn read_quoted_atom(self: &mut Self) -> Result<Token, String> { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::lexer) fn read_identifier(self: &mut Self) -> String { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::lexer) fn read_number(self: &mut Self) -> Result<Token, String> { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::lexer) fn skip_whitespace_and_comments(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::lexer) fn current_char(self: &Self) -> char { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::lexer) fn peek_char(self: &Self) -> Option<char> { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `parser`

Parser for LogicLang - converts tokens to AST

```rust
pub mod parser { /* ... */ }
```

### Types

#### Enum `Term`

```rust
pub enum Term {
    Atom(alloc::string::String),
    Variable(alloc::string::String),
    Integer(i64),
    String(alloc::string::String),
    Compound(alloc::string::String, alloc::vec::Vec<Term>),
    List(alloc::vec::Vec<Term>),
    Nil,
}
```

##### Variants

###### `Atom`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `Variable`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `Integer`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i64` |  |

###### `String`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `Compound`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |
| 1 | `alloc::vec::Vec<Term>` |  |

###### `List`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::vec::Vec<Term>` |  |

###### `Nil`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Term { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Clause`

**Attributes:**

- `Other("#[allow(dead_code)]")`

```rust
pub struct Clause {
    pub head: Term,
    pub body: alloc::vec::Vec<Goal>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `head` | `Term` |  |
| `body` | `alloc::vec::Vec<Goal>` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Clause { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `Goal`

**Attributes:**

- `Other("#[allow(dead_code)]")`

```rust
pub enum Goal {
    Predicate(Term),
    Comparison(CompOp, Term, Term),
    Cut,
    Call(alloc::string::String),
}
```

##### Variants

###### `Predicate`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Term` |  |

###### `Comparison`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `CompOp` |  |
| 1 | `Term` |  |
| 2 | `Term` |  |

###### `Cut`

###### `Call`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Goal { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `CompOp`

```rust
pub enum CompOp {
    Unify,
    NotUnify,
    LessThan,
    GreaterThan,
    LessThanEq,
    GreaterThanEq,
    Is,
}
```

##### Variants

###### `Unify`

###### `NotUnify`

###### `LessThan`

###### `GreaterThan`

###### `LessThanEq`

###### `GreaterThanEq`

###### `Is`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CompOp { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Parser`

```rust
pub struct Parser {
    pub(in ::logiclang_int::parser) tokens: alloc::vec::Vec<super::lexer::Token>,
    pub(in ::logiclang_int::parser) position: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `tokens` | `alloc::vec::Vec<super::lexer::Token>` |  |
| `position` | `usize` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(tokens: Vec<Token>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn parse(self: &mut Self) -> Result<Vec<Clause>, LogicError> { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::parser) fn parse_clause(self: &mut Self) -> Result<Clause, LogicError> { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::parser) fn parse_goals(self: &mut Self) -> Result<Vec<Goal>, LogicError> { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::parser) fn parse_goal(self: &mut Self) -> Result<Goal, LogicError> { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::parser) fn parse_term(self: &mut Self) -> Result<Term, LogicError> { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::parser) fn parse_args(self: &mut Self) -> Result<Vec<Term>, LogicError> { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::parser) fn parse_list(self: &mut Self) -> Result<Term, LogicError> { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::parser) fn match_token(self: &mut Self, token: &Token) -> bool { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::parser) fn expect_token(self: &mut Self, token: Token) -> Result<(), LogicError> { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::parser) fn current_token(self: &Self) -> &Token { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::parser) fn current_token_matches(self: &Self, token: &Token) -> bool { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::parser) fn is_at_end(self: &Self) -> bool { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `interpreter`

```rust
pub mod interpreter { /* ... */ }
```

### Types

#### Struct `LogicInterpreter`

**Attributes:**

- `Other("#[allow(dead_code)]")`

```rust
pub struct LogicInterpreter {
    pub(in ::logiclang_int::interpreter) clauses: alloc::vec::Vec<alloc::vec::Vec<alloc::string::String>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `clauses` | `alloc::vec::Vec<alloc::vec::Vec<alloc::string::String>>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(clauses: Vec<Vec<String>>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn interpret(text: String) -> Result<Vec<Clause>, LogicError> { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::interpreter) fn interpret_query(self: &Self, _tokens: &[String]) -> HashMap<String, String> { /* ... */ }
  ```

- ```rust
  pub(in ::logiclang_int::interpreter) fn unify_clause(self: &Self, clause: &[String], query: &[String]) -> bool { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `builtins`

Built-in predicates and command execution for LogicLang

```rust
pub mod builtins { /* ... */ }
```

### Types

#### Struct `HPVMxCommandExecutor`

**Attributes:**

- `Other("#[allow(dead_code)]")`

```rust
pub struct HPVMxCommandExecutor;
```

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CommandExecutor**
  - ```rust
    fn execute(self: &Self, cmd: &str, args: &[Term]) -> Result<String, String> { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Traits

#### Trait `CommandExecutor`

**Attributes:**

- `Other("#[allow(unused)]")`

```rust
pub trait CommandExecutor {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `execute`

##### Implementations

This trait is implemented for the following types:

- `HPVMxCommandExecutor`

### Functions

#### Function `term_to_string`

**Attributes:**

- `Other("#[allow(dead_code)]")`

```rust
pub(in ::logiclang_int::builtins) fn term_to_string(term: &super::parser::Term) -> alloc::string::String { /* ... */ }
```

## Module `error`

Error types for LogicLang

```rust
pub mod error { /* ... */ }
```

### Types

#### Enum `LogicError`

**Attributes:**

- `Other("#[allow(dead_code)]")`

```rust
pub enum LogicError {
    SyntaxError(alloc::string::String),
    RuntimeError(alloc::string::String),
    UndefinedVariable(alloc::string::String),
    TypeError(alloc::string::String),
    UnificationFailed(alloc::string::String),
    InvalidRule(alloc::string::String),
    StackOverflow,
    NoSolution,
    ExecutionError(alloc::string::String),
    Error(alloc::string::String),
}
```

##### Variants

###### `SyntaxError`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `RuntimeError`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `UndefinedVariable`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `TypeError`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `UnificationFailed`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `InvalidRule`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `StackOverflow`

###### `NoSolution`

###### `ExecutionError`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `Error`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

##### Implementations

###### Methods

- ```rust
  pub fn message(self: &Self) -> String { /* ... */ }
  ```

- ```rust
  pub fn string_to_logicerror(error: String) -> LogicError { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> LogicError { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Re-exports

#### Re-export `LogicError`

```rust
pub use error::LogicError;
```

#### Re-export `Lexer`

```rust
pub use lexer::Lexer;
```

#### Re-export `Parser`

```rust
pub use parser::Parser;
```

## Module `devices`

Device management and drivers.

This module provides interfaces for various hardware devices,
such as network cards and timers.

```rust
pub(crate) mod devices { /* ... */ }
```

### Modules

## Module `net`

**Attributes:**

- `Other("#[allow(dead_code)]")`

Minimal networking scaffolding for HPVMx
Provides placeholder implementations for ping, lanscan, and an HTTP management server
so shell commands are available without requiring a fully wired NIC.

```rust
pub mod net { /* ... */ }
```

### Types

#### Struct `DhcpPacket`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: Some(1), int: None })`

```rust
pub(in ::devices::net) struct DhcpPacket {
    pub(in ::devices::net) op: u8,
    pub(in ::devices::net) htype: u8,
    pub(in ::devices::net) hlen: u8,
    pub(in ::devices::net) hops: u8,
    pub(in ::devices::net) xid: u32,
    pub(in ::devices::net) secs: u16,
    pub(in ::devices::net) flags: u16,
    pub(in ::devices::net) ciaddr: [u8; 4],
    pub(in ::devices::net) yiaddr: [u8; 4],
    pub(in ::devices::net) siaddr: [u8; 4],
    pub(in ::devices::net) giaddr: [u8; 4],
    pub(in ::devices::net) chaddr: [u8; 16],
    pub(in ::devices::net) sname: [u8; 64],
    pub(in ::devices::net) file: [u8; 128],
    pub(in ::devices::net) magic: u32,
    pub(in ::devices::net) options: [u8; 312],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `op` | `u8` |  |
| `htype` | `u8` |  |
| `hlen` | `u8` |  |
| `hops` | `u8` |  |
| `xid` | `u32` |  |
| `secs` | `u16` |  |
| `flags` | `u16` |  |
| `ciaddr` | `[u8; 4]` |  |
| `yiaddr` | `[u8; 4]` |  |
| `siaddr` | `[u8; 4]` |  |
| `giaddr` | `[u8; 4]` |  |
| `chaddr` | `[u8; 16]` |  |
| `sname` | `[u8; 64]` |  |
| `file` | `[u8; 128]` |  |
| `magic` | `u32` |  |
| `options` | `[u8; 312]` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `DhcpReply`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: Some(1), int: None })`

```rust
pub(in ::devices::net) struct DhcpReply {
    pub(in ::devices::net) _unused: [u8; 16],
    pub(in ::devices::net) yiaddr: [u8; 4],
    pub(in ::devices::net) _unused2: [u8; 216],
    pub(in ::devices::net) magic: u32,
    pub(in ::devices::net) options: [u8; 312],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_unused` | `[u8; 16]` |  |
| `yiaddr` | `[u8; 4]` |  |
| `_unused2` | `[u8; 216]` |  |
| `magic` | `u32` |  |
| `options` | `[u8; 312]` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Functions

#### Function `parse_dhcp_offer`

```rust
pub(in ::devices::net) fn parse_dhcp_offer(data: &[u8]) -> Option<([u8; 4], [u8; 4], [u8; 4])> { /* ... */ }
```

#### Function `discover_config`

```rust
pub fn discover_config() -> Option<([u8; 4], [u8; 4], [u8; 4])> { /* ... */ }
```

#### Function `send_dhcp_request`

```rust
pub fn send_dhcp_request(offered_ip: [u8; 4], server_ip: [u8; 4]) { /* ... */ }
```

#### Function `poll_for_dhcp_response`

```rust
pub(in ::devices::net) fn poll_for_dhcp_response() -> Option<([u8; 4], [u8; 4], [u8; 4])> { /* ... */ }
```

#### Function `parse_ip`

Helper to parse "1.2.3.4" into [u8; 4]

```rust
pub(in ::devices::net) fn parse_ip(ip: &str) -> Option<[u8; 4]> { /* ... */ }
```

#### Function `ensure_hw`

Ensure hardware is initialized (SNP). Best-effort.

```rust
pub(in ::devices::net) fn ensure_hw() { /* ... */ }
```

#### Function `ensure_net`

```rust
pub(in ::devices::net) fn ensure_net() { /* ... */ }
```

#### Function `status`

Print simple NIC status to the console (MAC/MTU/link).

```rust
pub fn status() { /* ... */ }
```

#### Function `ping`

```rust
pub fn ping(ip_str: &str, _count: usize, _timeout_ms: u64) -> Result<u32, &''static str> { /* ... */ }
```

#### Function `lanscan`

Scan a /24 network by trying TCP port 80 (HTTP) like the provided batch example.
Example prefix: "192.168.1."

```rust
pub fn lanscan(prefix: &str) { /* ... */ }
```

#### Function `httpd_start`

Start a very small management HTTP server on a separate thread (placeholder).
In UEFI context, we simulate a background loop.

```rust
pub fn httpd_start(port: u16) { /* ... */ }
```

#### Function `httpd_stop`

```rust
pub fn httpd_stop() { /* ... */ }
```

### Constants and Statics

#### Static `HTTPD_RUNNING`

```rust
pub(in ::devices::net) static HTTPD_RUNNING: core::sync::atomic::AtomicBool = _;
```

#### Constant `UDP_PORT_DHCP_CLIENT`

```rust
pub(in ::devices::net) const UDP_PORT_DHCP_CLIENT: u16 = 68;
```

#### Constant `UDP_PORT_DHCP_SERVER`

```rust
pub(in ::devices::net) const UDP_PORT_DHCP_SERVER: u16 = 67;
```

## Module `net_hw`

**Attributes:**

- `Other("#[allow(dead_code)]")`

UEFI Simple Network Protocol (SNP) bring-up for HPVMx
Minimal hardware binding so higher-level networking knows a NIC exists.

```rust
pub mod net_hw { /* ... */ }
```

### Types

#### Struct `NetHwInfo`

```rust
pub struct NetHwInfo {
    pub mac: [u8; 32],
    pub mac_len: usize,
    pub mtu: u32,
    pub media_present: bool,
    pub state: u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `mac` | `[u8; 32]` |  |
| `mac_len` | `usize` |  |
| `mtu` | `u32` |  |
| `media_present` | `bool` |  |
| `state` | `u32` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> NetHwInfo { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Functions

#### Function `is_initialized`

**Attributes:**

- `Other("#[attr = Inline(Hint)]")`

```rust
pub fn is_initialized() -> bool { /* ... */ }
```

#### Function `get_info`

```rust
pub fn get_info() -> Option<NetHwInfo> { /* ... */ }
```

#### Function `nic_handle`

**Attributes:**

- `Other("#[attr = Inline(Hint)]")`

```rust
pub fn nic_handle() -> Option<uefi::Handle> { /* ... */ }
```

#### Function `snp_open`

Convenience helper to open the SNP protocol on the selected NIC.

```rust
pub fn snp_open() -> Option<uefi::boot::ScopedProtocol<uefi::proto::network::snp::SimpleNetwork>> { /* ... */ }
```

#### Function `link_up`

Current link status (best-effort).

```rust
pub fn link_up() -> bool { /* ... */ }
```

#### Function `tx`

Transmit a raw Ethernet frame via SNP (best-effort).

```rust
pub fn tx(frame: &[u8]) -> Result<(), &''static str> { /* ... */ }
```

#### Function `rx`

Receive a raw Ethernet frame into the provided buffer. Returns length if a packet was received.

```rust
pub fn rx(buf: &mut [u8]) -> Result<usize, &''static str> { /* ... */ }
```

#### Function `init`

Try to locate a NIC via UEFI SNP and initialize it (Start + Initialize).
Returns Ok if at least one device was started and initialized.

```rust
pub fn init() -> Result<(), &''static str> { /* ... */ }
```

#### Function `format_mac`

```rust
pub(in ::devices::net_hw) fn format_mac(mac: &[u8; 32], len: usize) -> alloc::string::String { /* ... */ }
```

#### Function `get_mac`

```rust
pub(crate) fn get_mac() -> [u8; 32] { /* ... */ }
```

### Constants and Statics

#### Static `NET_INITIALIZED`

```rust
pub(in ::devices::net_hw) static NET_INITIALIZED: core::sync::atomic::AtomicBool = _;
```

#### Static `NET_INFO`

```rust
pub(in ::devices::net_hw) static mut NET_INFO: Option<NetHwInfo> = None;
```

#### Static `NIC_HANDLE`

```rust
pub(in ::devices::net_hw) static mut NIC_HANDLE: Option<uefi::Handle> = None;
```

## Module `net_stack`

**Attributes:**

- `Other("#[allow(dead_code, static_mut_refs)]")`

UEFI Network Module (0.36.1) - Basic ICMP/IP over SNP

```rust
pub mod net_stack { /* ... */ }
```

### Types

#### Struct `EthHeader`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: Some(1), int: None })`

```rust
pub struct EthHeader {
    pub(in ::devices::net_stack) dst: [u8; 6],
    pub(in ::devices::net_stack) src: [u8; 6],
    pub(crate) ethertype: u16,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `dst` | `[u8; 6]` |  |
| `src` | `[u8; 6]` |  |
| `ethertype` | `u16` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `ArpPacket`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: Some(1), int: None })`

```rust
pub(in ::devices::net_stack) struct ArpPacket {
    pub(in ::devices::net_stack) hw_type: u16,
    pub(in ::devices::net_stack) proto_type: u16,
    pub(in ::devices::net_stack) hw_size: u8,
    pub(in ::devices::net_stack) proto_size: u8,
    pub(in ::devices::net_stack) opcode: u16,
    pub(in ::devices::net_stack) sender_mac: [u8; 6],
    pub(in ::devices::net_stack) sender_ip: [u8; 4],
    pub(in ::devices::net_stack) target_mac: [u8; 6],
    pub(in ::devices::net_stack) target_ip: [u8; 4],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `hw_type` | `u16` |  |
| `proto_type` | `u16` |  |
| `hw_size` | `u8` |  |
| `proto_size` | `u8` |  |
| `opcode` | `u16` |  |
| `sender_mac` | `[u8; 6]` |  |
| `sender_ip` | `[u8; 4]` |  |
| `target_mac` | `[u8; 6]` |  |
| `target_ip` | `[u8; 4]` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Ipv4Header`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: Some(1), int: None })`

```rust
pub(in ::devices::net_stack) struct Ipv4Header {
    pub(in ::devices::net_stack) ver_ihl: u8,
    pub(in ::devices::net_stack) tos: u8,
    pub(in ::devices::net_stack) len: u16,
    pub(in ::devices::net_stack) id: u16,
    pub(in ::devices::net_stack) off: u16,
    pub(in ::devices::net_stack) ttl: u8,
    pub(in ::devices::net_stack) proto: u8,
    pub(in ::devices::net_stack) checksum: u16,
    pub(in ::devices::net_stack) src: [u8; 4],
    pub(in ::devices::net_stack) dst: [u8; 4],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `ver_ihl` | `u8` |  |
| `tos` | `u8` |  |
| `len` | `u16` |  |
| `id` | `u16` |  |
| `off` | `u16` |  |
| `ttl` | `u8` |  |
| `proto` | `u8` |  |
| `checksum` | `u16` |  |
| `src` | `[u8; 4]` |  |
| `dst` | `[u8; 4]` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `IcmpHeader`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: Some(1), int: None })`

```rust
pub(in ::devices::net_stack) struct IcmpHeader {
    pub(in ::devices::net_stack) msg_type: u8,
    pub(in ::devices::net_stack) code: u8,
    pub(in ::devices::net_stack) checksum: u16,
    pub(in ::devices::net_stack) ident: u16,
    pub(in ::devices::net_stack) seq: u16,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `msg_type` | `u8` |  |
| `code` | `u8` |  |
| `checksum` | `u16` |  |
| `ident` | `u16` |  |
| `seq` | `u16` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `TcpHeader`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: Some(1), int: None })`

```rust
pub(in ::devices::net_stack) struct TcpHeader {
    pub(in ::devices::net_stack) src_port: u16,
    pub(in ::devices::net_stack) dst_port: u16,
    pub(in ::devices::net_stack) seq: u32,
    pub(in ::devices::net_stack) ack: u32,
    pub(in ::devices::net_stack) off_flags: u16,
    pub(in ::devices::net_stack) window: u16,
    pub(in ::devices::net_stack) checksum: u16,
    pub(in ::devices::net_stack) urgent: u16,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `src_port` | `u16` |  |
| `dst_port` | `u16` |  |
| `seq` | `u32` |  |
| `ack` | `u32` |  |
| `off_flags` | `u16` |  |
| `window` | `u16` |  |
| `checksum` | `u16` |  |
| `urgent` | `u16` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `UdpHeader`

**Attributes:**

- `Repr(AttributeRepr { kind: C, align: None, packed: Some(1), int: None })`

```rust
pub(in ::devices::net_stack) struct UdpHeader {
    pub(in ::devices::net_stack) src_port: u16,
    pub(in ::devices::net_stack) dst_port: u16,
    pub(in ::devices::net_stack) len: u16,
    pub(in ::devices::net_stack) checksum: u16,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `src_port` | `u16` |  |
| `dst_port` | `u16` |  |
| `len` | `u16` |  |
| `checksum` | `u16` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `NetStats`

```rust
pub struct NetStats {
    pub rx_pkts: u64,
    pub rx_bytes: u64,
    pub tx_pkts: u64,
    pub tx_bytes: u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `rx_pkts` | `u64` |  |
| `rx_bytes` | `u64` |  |
| `tx_pkts` | `u64` |  |
| `tx_bytes` | `u64` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> NetStats { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> NetStats { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `ArpEntry`

```rust
pub(in ::devices::net_stack) struct ArpEntry {
    pub(in ::devices::net_stack) ip: [u8; 4],
    pub(in ::devices::net_stack) mac: [u8; 6],
    pub(in ::devices::net_stack) valid: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `ip` | `[u8; 4]` |  |
| `mac` | `[u8; 6]` |  |
| `valid` | `bool` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArpEntry { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ArpEntry { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `NetState`

```rust
pub struct NetState {
    pub ip_addr: [u8; 4],
    pub gateway: [u8; 4],
    pub subnet_mask: [u8; 4],
    pub mac_addr: [u8; 6],
    pub stats: NetStats,
    pub(in ::devices::net_stack) arp_cache: [ArpEntry; 16],
    pub ping_success: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `ip_addr` | `[u8; 4]` |  |
| `gateway` | `[u8; 4]` |  |
| `subnet_mask` | `[u8; 4]` |  |
| `mac_addr` | `[u8; 6]` |  |
| `stats` | `NetStats` |  |
| `arp_cache` | `[ArpEntry; 16]` |  |
| `ping_success` | `bool` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> NetState { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> NetState { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Functions

#### Function `backend_name`

Report which networking backend is active.
- "SNP (raw)" when a NIC via UEFI SNP is initialized.
- "loopback-stub" otherwise.

```rust
pub fn backend_name() -> &''static str { /* ... */ }
```

#### Function `calculate_checksum`

```rust
pub(in ::devices::net_stack) fn calculate_checksum(data: &[u8]) -> u16 { /* ... */ }
```

#### Function `is_local`

Checks if a destination IP is within our local subnet.

```rust
pub(in ::devices::net_stack) fn is_local(dest_ip: [u8; 4]) -> bool { /* ... */ }
```

#### Function `init`

```rust
pub fn init(ip: [u8; 4], gw: [u8; 4], mask: [u8; 4]) { /* ... */ }
```

#### Function `resolve_mac`

Resolves a MAC address. If target is remote, resolves the Gateway's MAC instead.

```rust
pub(in ::devices::net_stack) fn resolve_mac(target_ip: [u8; 4], timeout_loops: u32) -> Option<[u8; 6]> { /* ... */ }
```

#### Function `ping_external`

```rust
pub fn ping_external(target_ip: [u8; 4], timeout: u32, print: bool) -> bool { /* ... */ }
```

#### Function `update_arp_cache`

```rust
pub(in ::devices::net_stack) fn update_arp_cache(ip: [u8; 4], mac: [u8; 6]) { /* ... */ }
```

#### Function `find_mac`

```rust
pub(in ::devices::net_stack) fn find_mac(ip: [u8; 4]) -> Option<[u8; 6]> { /* ... */ }
```

#### Function `is_initialized`

**Attributes:**

- `Other("#[attr = Inline(Hint)]")`

```rust
pub fn is_initialized() -> bool { /* ... */ }
```

#### Function `poll_tick`

**Attributes:**

- `Other("#[allow(static_mut_refs)]")`

```rust
pub fn poll_tick() { /* ... */ }
```

#### Function `process_packet`

A very primitive packet dispatcher

```rust
pub(in ::devices::net_stack) fn process_packet(frame: &[u8]) { /* ... */ }
```

#### Function `ping_loopback`

Very small loopback ping: if dst is 127.0.0.1, report success with tiny RTT.

```rust
pub fn ping_loopback(dst: &str) -> Result<u32, &''static str> { /* ... */ }
```

#### Function `handle_arp`

```rust
pub(in ::devices::net_stack) fn handle_arp(packet: &[u8]) { /* ... */ }
```

#### Function `send_arp_packet`

```rust
pub(in ::devices::net_stack) fn send_arp_packet(opcode: u16, target_ip: [u8; 4], target_mac: [u8; 6]) { /* ... */ }
```

#### Function `handle_ipv4`

```rust
pub(in ::devices::net_stack) fn handle_ipv4(packet: &[u8], src_mac: [u8; 6]) { /* ... */ }
```

#### Function `send_raw_udp`

Sends a UDP packet even if the stack isn't fully initialized (useful for DHCP).

```rust
pub fn send_raw_udp(src_ip: [u8; 4], dst_ip: [u8; 4], dst_mac: [u8; 6], src_port: u16, dst_port: u16, data: &[u8]) -> Result<(), &''static str> { /* ... */ }
```

#### Function `send_udp`

Send a UDP packet to a specific IP/Port

```rust
pub fn send_udp(dst_ip: [u8; 4], dst_mac: [u8; 6], src_port: u16, dst_port: u16, data: &[u8]) { /* ... */ }
```

#### Function `handle_tcp`

```rust
pub(in ::devices::net_stack) fn handle_tcp(src_ip: [u8; 4], src_mac: [u8; 6], packet: &[u8]) { /* ... */ }
```

#### Function `send_tcp_packet`

```rust
pub(in ::devices::net_stack) fn send_tcp_packet(dst_ip: [u8; 4], dst_mac: [u8; 6], src_port: u16, dst_port: u16, seq: u32, ack: u32, flags: u8, data: &[u8]) { /* ... */ }
```

#### Function `calculate_tcp_checksum`

```rust
pub(in ::devices::net_stack) fn calculate_tcp_checksum(src: [u8; 4], dst: [u8; 4], tcp_segment: &[u8]) -> u16 { /* ... */ }
```

#### Function `send_icmp_reply`

```rust
pub(in ::devices::net_stack) fn send_icmp_reply(dst_ip: [u8; 4], dst_mac: [u8; 6], ident: u16, seq: u16, payload: &[u8]) { /* ... */ }
```

#### Function `handle_udp`

```rust
pub(in ::devices::net_stack) fn handle_udp(src_ip: [u8; 4], port: u16, data: &[u8]) { /* ... */ }
```

#### Function `httpd_start`

```rust
pub fn httpd_start(_port: u16) { /* ... */ }
```

#### Function `httpd_stop`

```rust
pub fn httpd_stop() { /* ... */ }
```

#### Function `stats`

**Attributes:**

- `Other("#[allow(static_mut_refs)]")`

Return a snapshot of current network stats (RX/TX counters).

```rust
pub fn stats() -> NetStats { /* ... */ }
```

#### Function `get_state`

```rust
pub fn get_state() -> NetState { /* ... */ }
```

#### Function `snp_tx`

**Attributes:**

- `Other("#[allow(static_mut_refs)]")`

Transmit a raw frame via SNP if available. Increments TX counters on success.

```rust
pub fn snp_tx(frame: &[u8]) -> Result<(), &''static str> { /* ... */ }
```

#### Function `send_icmp_echo`

Sends a real ICMP Echo Request (Ping) to an external IP.
Generic ICMP sender: msg_type 8 for Request, 0 for Reply

```rust
pub(in ::devices::net_stack) fn send_icmp_echo(msg_type: u8, dst_ip: [u8; 4], dst_mac: [u8; 6], ident: u16, seq: u16, payload: &[u8]) { /* ... */ }
```

#### Function `ping`

Public API: High-level Ping

```rust
pub fn ping(target_ip: [u8; 4], timeout_loops: u32) -> bool { /* ... */ }
```

### Constants and Statics

#### Constant `ETHERTYPE_IPV4`

```rust
pub const ETHERTYPE_IPV4: u16 = 0x0800;
```

#### Constant `ETHERTYPE_ARP`

```rust
pub const ETHERTYPE_ARP: u16 = 0x0806;
```

#### Constant `ARP_REQUEST`

```rust
pub(in ::devices::net_stack) const ARP_REQUEST: u16 = 1;
```

#### Constant `ARP_REPLY`

```rust
pub(in ::devices::net_stack) const ARP_REPLY: u16 = 2;
```

#### Constant `IP_PROTO_ICMP`

```rust
pub(in ::devices::net_stack) const IP_PROTO_ICMP: u8 = 1;
```

#### Constant `IP_PROTO_UDP`

```rust
pub(in ::devices::net_stack) const IP_PROTO_UDP: u8 = 17;
```

#### Constant `IP_PROTO_TCP`

```rust
pub(in ::devices::net_stack) const IP_PROTO_TCP: u8 = 6;
```

#### Constant `TCP_FLAG_FIN`

```rust
pub(in ::devices::net_stack) const TCP_FLAG_FIN: u8 = 0x01;
```

#### Constant `TCP_FLAG_SYN`

```rust
pub(in ::devices::net_stack) const TCP_FLAG_SYN: u8 = 0x02;
```

#### Constant `TCP_FLAG_RST`

```rust
pub(in ::devices::net_stack) const TCP_FLAG_RST: u8 = 0x04;
```

#### Constant `TCP_FLAG_ACK`

```rust
pub(in ::devices::net_stack) const TCP_FLAG_ACK: u8 = 0x10;
```

#### Constant `MAX_ARP_ENTRIES`

```rust
pub(in ::devices::net_stack) const MAX_ARP_ENTRIES: usize = 16;
```

#### Static `HTTPD`

```rust
pub(in ::devices::net_stack) static HTTPD: core::sync::atomic::AtomicBool = _;
```

#### Static `NET_STATE`

```rust
pub(in ::devices::net_stack) static mut NET_STATE: core::mem::MaybeUninit<Option<NetState>> = _;
```

#### Static `STACK_INIT`

```rust
pub static STACK_INIT: core::sync::atomic::AtomicBool = _;
```

## Module `timer`

```rust
pub mod timer { /* ... */ }
```

### Types

#### Type Alias `Time`

```rust
pub type Time = [u16; 8];
```

#### Struct `Timer`

```rust
pub struct Timer {
    pub(in ::devices::timer) time: [u16; 8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `time` | `[u16; 8]` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Timer { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `hpvmlog`

```rust
pub(crate) mod hpvmlog { /* ... */ }
```

### Types

#### Struct `LogEntry`

```rust
pub struct LogEntry {
    pub level: uefi::proto::console::text::Color,
    pub tag: alloc::string::String,
    pub message: alloc::string::String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `level` | `uefi::proto::console::text::Color` |  |
| `tag` | `alloc::string::String` |  |
| `message` | `alloc::string::String` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> LogEntry { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Functions

#### Function `init_log_buffer`

```rust
pub fn init_log_buffer() { /* ... */ }
```

#### Function `push_log`

```rust
pub fn push_log(level: uefi::proto::console::text::Color, tag: &str, msg: &str) { /* ... */ }
```

#### Function `get_logs`

```rust
pub fn get_logs() -> alloc::vec::Vec<(uefi::proto::console::text::Color, alloc::string::String, alloc::string::String)> { /* ... */ }
```

#### Function `get_log_buffer`

**Attributes:**

- `Other("#[allow(static_mut_refs)]")`

```rust
pub unsafe fn get_log_buffer() -> &''static Option<alloc::vec::Vec<LogEntry>> { /* ... */ }
```

### Constants and Statics

#### Constant `MAX_LOGS`

```rust
pub(in ::hpvmlog) const MAX_LOGS: usize = 4096;
```

#### Static `LOG_BUFFER`

```rust
pub static mut LOG_BUFFER: Option<alloc::vec::Vec<LogEntry>> = None;
```

#### Static `LOG_COUNT`

```rust
pub(in ::hpvmlog) static LOG_COUNT: core::sync::atomic::AtomicUsize = _;
```

#### Static `LOGGING_SILENCED`

```rust
pub static mut LOGGING_SILENCED: bool = false;
```

#### Static `BUSY_TSC`

```rust
pub static mut BUSY_TSC: u64 = 0;
```

## Module `consts`

**Attributes:**

- `Other("#[allow(unsafe_code, dead_code, non_camel_case_types, non_snake_case, unused)]")`

```rust
pub(crate) mod consts { /* ... */ }
```

### Modules

## Module `pci`

```rust
pub mod pci { /* ... */ }
```

### Constants and Statics

#### Constant `PCI_CLASS_NOT_DEFINED`

```rust
pub const PCI_CLASS_NOT_DEFINED: u32 = 0x000000;
```

#### Constant `PCI_CLASS_NOT_DEFINED_VGA`

```rust
pub const PCI_CLASS_NOT_DEFINED_VGA: u32 = 0x000100;
```

#### Constant `PCI_CLASS_STORAGE_SCSI`

```rust
pub const PCI_CLASS_STORAGE_SCSI: u32 = 0x010000;
```

#### Constant `PCI_CLASS_STORAGE_IDE`

```rust
pub const PCI_CLASS_STORAGE_IDE: u32 = 0x010100;
```

#### Constant `PCI_CLASS_STORAGE_FLOPPY`

```rust
pub const PCI_CLASS_STORAGE_FLOPPY: u32 = 0x010200;
```

#### Constant `PCI_CLASS_STORAGE_IPI`

```rust
pub const PCI_CLASS_STORAGE_IPI: u32 = 0x010300;
```

#### Constant `PCI_CLASS_STORAGE_RAID`

```rust
pub const PCI_CLASS_STORAGE_RAID: u32 = 0x010400;
```

#### Constant `PCI_CLASS_STORAGE_SATA`

```rust
pub const PCI_CLASS_STORAGE_SATA: u32 = 0x010600;
```

#### Constant `PCI_CLASS_STORAGE_SATA_AHCI`

```rust
pub const PCI_CLASS_STORAGE_SATA_AHCI: u32 = 0x010601;
```

#### Constant `PCI_CLASS_STORAGE_SAS`

```rust
pub const PCI_CLASS_STORAGE_SAS: u32 = 0x010700;
```

#### Constant `PCI_CLASS_STORAGE_EXPRESS`

```rust
pub const PCI_CLASS_STORAGE_EXPRESS: u32 = 0x010802;
```

#### Constant `PCI_CLASS_STORAGE_OTHER`

```rust
pub const PCI_CLASS_STORAGE_OTHER: u32 = 0x018000;
```

#### Constant `PCI_CLASS_STORAGE_RVA`

```rust
pub const PCI_CLASS_STORAGE_RVA: u32 = 0x010500;
```

#### Constant `PCI_CLASS_NETWORK_ETHERNET`

```rust
pub const PCI_CLASS_NETWORK_ETHERNET: u32 = 0x020000;
```

#### Constant `PCI_CLASS_NETWORK_TOKEN_RING`

```rust
pub const PCI_CLASS_NETWORK_TOKEN_RING: u32 = 0x020100;
```

#### Constant `PCI_CLASS_NETWORK_FDDI`

```rust
pub const PCI_CLASS_NETWORK_FDDI: u32 = 0x020200;
```

#### Constant `PCI_CLASS_NETWORK_ATM`

```rust
pub const PCI_CLASS_NETWORK_ATM: u32 = 0x020300;
```

#### Constant `PCI_CLASS_NETWORK_OTHER`

```rust
pub const PCI_CLASS_NETWORK_OTHER: u32 = 0x028000;
```

#### Constant `PCI_CLASS_DISPLAY_VGA`

```rust
pub const PCI_CLASS_DISPLAY_VGA: u32 = 0x030000;
```

#### Constant `PCI_CLASS_DISPLAY_XGA`

```rust
pub const PCI_CLASS_DISPLAY_XGA: u32 = 0x030100;
```

#### Constant `PCI_CLASS_DISPLAY_3D`

```rust
pub const PCI_CLASS_DISPLAY_3D: u32 = 0x030200;
```

#### Constant `PCI_CLASS_DISPLAY_OTHER`

```rust
pub const PCI_CLASS_DISPLAY_OTHER: u32 = 0x038000;
```

#### Constant `PCI_CLASS_MULTIMEDIA_VIDEO`

```rust
pub const PCI_CLASS_MULTIMEDIA_VIDEO: u32 = 0x040000;
```

#### Constant `PCI_CLASS_MULTIMEDIA_AUDIO`

```rust
pub const PCI_CLASS_MULTIMEDIA_AUDIO: u32 = 0x040100;
```

#### Constant `PCI_CLASS_MULTIMEDIA_PHONE`

```rust
pub const PCI_CLASS_MULTIMEDIA_PHONE: u32 = 0x040200;
```

#### Constant `PCI_CLASS_MUTIMEDIA_HD_AUDIO`

```rust
pub const PCI_CLASS_MUTIMEDIA_HD_AUDIO: u32 = 0x040300;
```

#### Constant `PCI_CLASS_MULTIMEDIA_OTHER`

```rust
pub const PCI_CLASS_MULTIMEDIA_OTHER: u32 = 0x048000;
```

#### Constant `PCI_CLASS_MEMORY_RAM`

```rust
pub const PCI_CLASS_MEMORY_RAM: u32 = 0x050000;
```

#### Constant `PCI_CLASS_MEMORY_FLASH`

```rust
pub const PCI_CLASS_MEMORY_FLASH: u32 = 0x050100;
```

#### Constant `PCI_CLASS_MEMORY_CXL`

```rust
pub const PCI_CLASS_MEMORY_CXL: u32 = 0x050200;
```

#### Constant `PCI_CLASS_MEMORY_SRAM`

```rust
pub const PCI_CLASS_MEMORY_SRAM: u32 = 0x050300;
```

#### Constant `PCI_CLASS_MEMORY_DRAM`

```rust
pub const PCI_CLASS_MEMORY_DRAM: u32 = 0x050400;
```

#### Constant `PCI_CLASS_MEMORY_OTHER`

```rust
pub const PCI_CLASS_MEMORY_OTHER: u32 = 0x058000;
```

#### Constant `PCI_CLASS_BRIDGE_HOST`

```rust
pub const PCI_CLASS_BRIDGE_HOST: u32 = 0x060000;
```

#### Constant `PCI_CLASS_BRIDGE_ISA`

```rust
pub const PCI_CLASS_BRIDGE_ISA: u32 = 0x060100;
```

#### Constant `PCI_CLASS_BRIDGE_EISA`

```rust
pub const PCI_CLASS_BRIDGE_EISA: u32 = 0x060200;
```

#### Constant `PCI_CLASS_BRIDGE_MC`

```rust
pub const PCI_CLASS_BRIDGE_MC: u32 = 0x060300;
```

#### Constant `PCI_CLASS_BRIDGE_PCI_NORMAL`

```rust
pub const PCI_CLASS_BRIDGE_PCI_NORMAL: u32 = 0x060400;
```

#### Constant `PCI_CLASS_BRIDGE_PCI_SUBTRACTIVE`

```rust
pub const PCI_CLASS_BRIDGE_PCI_SUBTRACTIVE: u32 = 0x060401;
```

#### Constant `PCI_CLASS_BRIDGE_PCMIA`

```rust
pub const PCI_CLASS_BRIDGE_PCMIA: u32 = 0x060500;
```

#### Constant `PCI_CLASS_BRIDGE_NUBUS`

```rust
pub const PCI_CLASS_BRIDGE_NUBUS: u32 = 0x060600;
```

#### Constant `PCI_CLASS_BRIDGE_CARDBUS`

```rust
pub const PCI_CLASS_BRIDGE_CARDBUS: u32 = 0x060700;
```

#### Constant `PCI_CLASS_BRIDGE_RACEWAY`

```rust
pub const PCI_CLASS_BRIDGE_RACEWAY: u32 = 0x060800;
```

#### Constant `PCI_CLASS_BRIDGE_OTHER`

```rust
pub const PCI_CLASS_BRIDGE_OTHER: u32 = 0x068000;
```

#### Constant `PCI_CLASS_COMMUNICATION_SERIAL`

```rust
pub const PCI_CLASS_COMMUNICATION_SERIAL: u32 = 0x070000;
```

#### Constant `PCI_CLASS_COMMUNICATION_PARALLEL`

```rust
pub const PCI_CLASS_COMMUNICATION_PARALLEL: u32 = 0x070100;
```

#### Constant `PCI_CLASS_COMMUNICATION_MULTISERIAL`

```rust
pub const PCI_CLASS_COMMUNICATION_MULTISERIAL: u32 = 0x070200;
```

#### Constant `PCI_CLASS_COMMUNICATION_MODEM`

```rust
pub const PCI_CLASS_COMMUNICATION_MODEM: u32 = 0x070300;
```

#### Constant `PCI_CLASS_COMMUNICATION_OTHER`

```rust
pub const PCI_CLASS_COMMUNICATION_OTHER: u32 = 0x078000;
```

#### Constant `PCI_CLASS_SYSTEM_PIC`

```rust
pub const PCI_CLASS_SYSTEM_PIC: u32 = 0x080000;
```

#### Constant `PCI_CLASS_SYSTEM_PIC_IOAPIC`

```rust
pub const PCI_CLASS_SYSTEM_PIC_IOAPIC: u32 = 0x080010;
```

#### Constant `PCI_CLASS_SYSTEM_PIC_IOXAPIC`

```rust
pub const PCI_CLASS_SYSTEM_PIC_IOXAPIC: u32 = 0x080020;
```

#### Constant `PCI_CLASS_SYSTEM_DMA`

```rust
pub const PCI_CLASS_SYSTEM_DMA: u32 = 0x080100;
```

#### Constant `PCI_CLASS_SYSTEM_TIMER`

```rust
pub const PCI_CLASS_SYSTEM_TIMER: u32 = 0x080200;
```

#### Constant `PCI_CLASS_SYSTEM_RTC`

```rust
pub const PCI_CLASS_SYSTEM_RTC: u32 = 0x080300;
```

#### Constant `PCI_CLASS_SYSTEM_PCI_HOTPLUG`

```rust
pub const PCI_CLASS_SYSTEM_PCI_HOTPLUG: u32 = 0x080400;
```

#### Constant `PCI_CLASS_SYSTEM_SDHCI`

```rust
pub const PCI_CLASS_SYSTEM_SDHCI: u32 = 0x080500;
```

#### Constant `PCI_CLASS_SYSTEM_RCEC`

```rust
pub const PCI_CLASS_SYSTEM_RCEC: u32 = 0x080700;
```

#### Constant `PCI_CLASS_SYSTEM_OTHER`

```rust
pub const PCI_CLASS_SYSTEM_OTHER: u32 = 0x088000;
```

#### Constant `PCI_CLASS_INPUT_KEYBOARD`

```rust
pub const PCI_CLASS_INPUT_KEYBOARD: u32 = 0x090000;
```

#### Constant `PCI_CLASS_INPUT_PEN`

```rust
pub const PCI_CLASS_INPUT_PEN: u32 = 0x090100;
```

#### Constant `PCI_CLASS_INPUT_MOUSE`

```rust
pub const PCI_CLASS_INPUT_MOUSE: u32 = 0x090200;
```

#### Constant `PCI_CLASS_INPUT_SCANNER`

```rust
pub const PCI_CLASS_INPUT_SCANNER: u32 = 0x090300;
```

#### Constant `PCI_CLASS_INPUT_GAMEPORT`

```rust
pub const PCI_CLASS_INPUT_GAMEPORT: u32 = 0x090400;
```

#### Constant `PCI_CLASS_INPUT_OTHER`

```rust
pub const PCI_CLASS_INPUT_OTHER: u32 = 0x098000;
```

#### Constant `PCI_CLASS_DOCKING_GENERIC`

```rust
pub const PCI_CLASS_DOCKING_GENERIC: u32 = 0x0a0000;
```

#### Constant `PCI_CLASS_DOCKING_OTHER`

```rust
pub const PCI_CLASS_DOCKING_OTHER: u32 = 0x0a8000;
```

#### Constant `PCI_CLASS_PROCESSOR_386`

```rust
pub const PCI_CLASS_PROCESSOR_386: u32 = 0x0b0000;
```

#### Constant `PCI_CLASS_PROCESSOR_486`

```rust
pub const PCI_CLASS_PROCESSOR_486: u32 = 0x0b0100;
```

#### Constant `PCI_CLASS_PROCESSOR_PENTIUM`

```rust
pub const PCI_CLASS_PROCESSOR_PENTIUM: u32 = 0x0b0200;
```

#### Constant `PCI_CLASS_PROCESSOR_ALPHA`

```rust
pub const PCI_CLASS_PROCESSOR_ALPHA: u32 = 0x0b1000;
```

#### Constant `PCI_CLASS_PROCESSOR_POWERPC`

```rust
pub const PCI_CLASS_PROCESSOR_POWERPC: u32 = 0x0b2000;
```

#### Constant `PCI_CLASS_PROCESSOR_MIPS`

```rust
pub const PCI_CLASS_PROCESSOR_MIPS: u32 = 0x0b4000;
```

#### Constant `PCI_CLASS_PROCESSOR_CO`

```rust
pub const PCI_CLASS_PROCESSOR_CO: u32 = 0x0b5000;
```

#### Constant `PCI_CLASS_SERIAL_FIREWIRE`

```rust
pub const PCI_CLASS_SERIAL_FIREWIRE: u32 = 0x0c0000;
```

#### Constant `PCI_CLASS_SERIAL_FIREWIRE_OHCI`

```rust
pub const PCI_CLASS_SERIAL_FIREWIRE_OHCI: u32 = 0x0c0010;
```

#### Constant `PCI_CLASS_SERIAL_ACCESS`

```rust
pub const PCI_CLASS_SERIAL_ACCESS: u32 = 0x0c0100;
```

#### Constant `PCI_CLASS_SERIAL_SSA`

```rust
pub const PCI_CLASS_SERIAL_SSA: u32 = 0x0c0200;
```

#### Constant `PCI_CLASS_SERIAL_USB_UHCI`

```rust
pub const PCI_CLASS_SERIAL_USB_UHCI: u32 = 0x0c0300;
```

#### Constant `PCI_CLASS_SERIAL_USB_OHCI`

```rust
pub const PCI_CLASS_SERIAL_USB_OHCI: u32 = 0x0c0310;
```

#### Constant `PCI_CLASS_SERIAL_USB_EHCI`

```rust
pub const PCI_CLASS_SERIAL_USB_EHCI: u32 = 0x0c0320;
```

#### Constant `PCI_CLASS_SERIAL_USB_XHCI`

```rust
pub const PCI_CLASS_SERIAL_USB_XHCI: u32 = 0x0c0330;
```

#### Constant `PCI_CLASS_SERIAL_USB_CDNS`

```rust
pub const PCI_CLASS_SERIAL_USB_CDNS: u32 = 0x0c0380;
```

#### Constant `PCI_CLASS_SERIAL_USB_DEVICE`

```rust
pub const PCI_CLASS_SERIAL_USB_DEVICE: u32 = 0x0c03fe;
```

#### Constant `PCI_CLASS_SERIAL_FIBER`

```rust
pub const PCI_CLASS_SERIAL_FIBER: u32 = 0x0c0400;
```

#### Constant `PCI_CLASS_SERIAL_SMBUS`

```rust
pub const PCI_CLASS_SERIAL_SMBUS: u32 = 0x0c0500;
```

#### Constant `PCI_CLASS_SERIAL_IPMI_SMIC`

```rust
pub const PCI_CLASS_SERIAL_IPMI_SMIC: u32 = 0x0c0700;
```

#### Constant `PCI_CLASS_SERIAL_IPMI_KCS`

```rust
pub const PCI_CLASS_SERIAL_IPMI_KCS: u32 = 0x0c0701;
```

#### Constant `PCI_CLASS_SERIAL_IPMI_BT`

```rust
pub const PCI_CLASS_SERIAL_IPMI_BT: u32 = 0x0c0702;
```

#### Constant `PCI_CLASS_WIRELESS_RF_CONTROLLER`

```rust
pub const PCI_CLASS_WIRELESS_RF_CONTROLLER: u32 = 0x0d1000;
```

#### Constant `PCI_CLASS_WIRELESS_WHCI`

```rust
pub const PCI_CLASS_WIRELESS_WHCI: u32 = 0x0d1010;
```

#### Constant `PCI_CLASS_INTELLIGENT_I2O`

```rust
pub const PCI_CLASS_INTELLIGENT_I2O: u32 = 0x0e0000;
```

#### Constant `PCI_CLASS_SATELLITE_TV`

```rust
pub const PCI_CLASS_SATELLITE_TV: u32 = 0x0f0000;
```

#### Constant `PCI_CLASS_SATELLITE_AUDIO`

```rust
pub const PCI_CLASS_SATELLITE_AUDIO: u32 = 0x0f0100;
```

#### Constant `PCI_CLASS_SATELLITE_VOICE`

```rust
pub const PCI_CLASS_SATELLITE_VOICE: u32 = 0x0f0300;
```

#### Constant `PCI_CLASS_SATELLITE_DATA`

```rust
pub const PCI_CLASS_SATELLITE_DATA: u32 = 0x0f0400;
```

#### Constant `PCI_CLASS_CRYPT_NETWORK`

```rust
pub const PCI_CLASS_CRYPT_NETWORK: u32 = 0x100000;
```

#### Constant `PCI_CLASS_CRYPT_ENTERTAINMENT`

```rust
pub const PCI_CLASS_CRYPT_ENTERTAINMENT: u32 = 0x100100;
```

#### Constant `PCI_CLASS_CRYPT_OTHER`

```rust
pub const PCI_CLASS_CRYPT_OTHER: u32 = 0x108000;
```

#### Constant `PCI_CLASS_SP_DPIO`

```rust
pub const PCI_CLASS_SP_DPIO: u32 = 0x110000;
```

#### Constant `PCI_CLASS_SP_OTHER`

```rust
pub const PCI_CLASS_SP_OTHER: u32 = 0x118000;
```

#### Constant `PCI_CLASS_ACCELERATOR_PROCESSING`

```rust
pub const PCI_CLASS_ACCELERATOR_PROCESSING: u32 = 0x120000;
```

#### Constant `PCI_CLASS_OTHERS`

```rust
pub const PCI_CLASS_OTHERS: u32 = 0xff0000;
```

#### Constant `PCI_VENDOR_ID_PCI_SIG`

```rust
pub const PCI_VENDOR_ID_PCI_SIG: u16 = 0x0001;
```

#### Constant `PCI_VENDOR_ID_LOONGSON`

```rust
pub const PCI_VENDOR_ID_LOONGSON: u16 = 0x0014;
```

#### Constant `PCI_VENDOR_ID_SOLIDIGM`

```rust
pub const PCI_VENDOR_ID_SOLIDIGM: u16 = 0x025e;
```

#### Constant `PCI_VENDOR_ID_TTTECH`

```rust
pub const PCI_VENDOR_ID_TTTECH: u16 = 0x0357;
```

#### Constant `PCI_VENDOR_ID_DYNALINK`

```rust
pub const PCI_VENDOR_ID_DYNALINK: u16 = 0x0675;
```

#### Constant `PCI_VENDOR_ID_UBIQUITI`

```rust
pub const PCI_VENDOR_ID_UBIQUITI: u16 = 0x0777;
```

#### Constant `PCI_VENDOR_ID_BERKOM`

```rust
pub const PCI_VENDOR_ID_BERKOM: u16 = 0x0871;
```

#### Constant `PCI_VENDOR_ID_ITTIM`

```rust
pub const PCI_VENDOR_ID_ITTIM: u16 = 0x0b48;
```

#### Constant `PCI_VENDOR_ID_COMPAQ`

```rust
pub const PCI_VENDOR_ID_COMPAQ: u16 = 0x0e11;
```

#### Constant `PCI_VENDOR_ID_LSI_LOGIC`

```rust
pub const PCI_VENDOR_ID_LSI_LOGIC: u16 = 0x1000;
```

#### Constant `PCI_VENDOR_ID_ATI`

```rust
pub const PCI_VENDOR_ID_ATI: u16 = 0x1002;
```

#### Constant `PCI_VENDOR_ID_VLSI`

```rust
pub const PCI_VENDOR_ID_VLSI: u16 = 0x1004;
```

#### Constant `PCI_VENDOR_ID_ADL`

```rust
pub const PCI_VENDOR_ID_ADL: u16 = 0x1005;
```

#### Constant `PCI_VENDOR_ID_NS`

```rust
pub const PCI_VENDOR_ID_NS: u16 = 0x100b;
```

#### Constant `PCI_VENDOR_ID_TSENG`

```rust
pub const PCI_VENDOR_ID_TSENG: u16 = 0x100c;
```

#### Constant `PCI_VENDOR_ID_WEITEK`

```rust
pub const PCI_VENDOR_ID_WEITEK: u16 = 0x100e;
```

#### Constant `PCI_VENDOR_ID_DEC`

```rust
pub const PCI_VENDOR_ID_DEC: u16 = 0x1011;
```

#### Constant `PCI_VENDOR_ID_CIRRUS`

```rust
pub const PCI_VENDOR_ID_CIRRUS: u16 = 0x1013;
```

#### Constant `PCI_VENDOR_ID_IBM`

```rust
pub const PCI_VENDOR_ID_IBM: u16 = 0x1014;
```

#### Constant `PCI_VENDOR_ID_UNISYS`

```rust
pub const PCI_VENDOR_ID_UNISYS: u16 = 0x1018;
```

#### Constant `PCI_VENDOR_ID_COMPEX2`

```rust
pub const PCI_VENDOR_ID_COMPEX2: u16 = 0x101a;
```

#### Constant `PCI_VENDOR_ID_WD`

```rust
pub const PCI_VENDOR_ID_WD: u16 = 0x101c;
```

#### Constant `PCI_VENDOR_ID_AMI`

```rust
pub const PCI_VENDOR_ID_AMI: u16 = 0x101e;
```

#### Constant `PCI_VENDOR_ID_AMD`

```rust
pub const PCI_VENDOR_ID_AMD: u16 = 0x1022;
```

#### Constant `PCI_VENDOR_ID_TRIDENT`

```rust
pub const PCI_VENDOR_ID_TRIDENT: u16 = 0x1023;
```

#### Constant `PCI_VENDOR_ID_AI`

```rust
pub const PCI_VENDOR_ID_AI: u16 = 0x1025;
```

#### Constant `PCI_VENDOR_ID_DELL`

```rust
pub const PCI_VENDOR_ID_DELL: u16 = 0x1028;
```

#### Constant `PCI_VENDOR_ID_MATROX`

```rust
pub const PCI_VENDOR_ID_MATROX: u16 = 0x102B;
```

#### Constant `PCI_VENDOR_ID_MOBILITY_ELECTRONICS`

```rust
pub const PCI_VENDOR_ID_MOBILITY_ELECTRONICS: u16 = 0x14f2;
```

#### Constant `PCI_VENDOR_ID_CT`

```rust
pub const PCI_VENDOR_ID_CT: u16 = 0x102c;
```

#### Constant `PCI_VENDOR_ID_MIRO`

```rust
pub const PCI_VENDOR_ID_MIRO: u16 = 0x1031;
```

#### Constant `PCI_VENDOR_ID_NEC`

```rust
pub const PCI_VENDOR_ID_NEC: u16 = 0x1033;
```

#### Constant `PCI_VENDOR_ID_FD`

```rust
pub const PCI_VENDOR_ID_FD: u16 = 0x1036;
```

#### Constant `PCI_VENDOR_ID_SI`

```rust
pub const PCI_VENDOR_ID_SI: u16 = 0x1039;
```

#### Constant `PCI_VENDOR_ID_HP`

```rust
pub const PCI_VENDOR_ID_HP: u16 = 0x103c;
```

#### Constant `PCI_VENDOR_ID_HP_3PAR`

```rust
pub const PCI_VENDOR_ID_HP_3PAR: u16 = 0x1590;
```

#### Constant `PCI_VENDOR_ID_PCTECH`

```rust
pub const PCI_VENDOR_ID_PCTECH: u16 = 0x1042;
```

#### Constant `PCI_VENDOR_ID_ASUSTEK`

```rust
pub const PCI_VENDOR_ID_ASUSTEK: u16 = 0x1043;
```

#### Constant `PCI_VENDOR_ID_DPT`

```rust
pub const PCI_VENDOR_ID_DPT: u16 = 0x1044;
```

#### Constant `PCI_VENDOR_ID_OPTI`

```rust
pub const PCI_VENDOR_ID_OPTI: u16 = 0x1045;
```

#### Constant `PCI_VENDOR_ID_ELSA`

```rust
pub const PCI_VENDOR_ID_ELSA: u16 = 0x1048;
```

#### Constant `PCI_VENDOR_ID_STMICRO`

```rust
pub const PCI_VENDOR_ID_STMICRO: u16 = 0x104A;
```

#### Constant `PCI_VENDOR_ID_BUSLOGIC`

```rust
pub const PCI_VENDOR_ID_BUSLOGIC: u16 = 0x104B;
```

#### Constant `PCI_VENDOR_ID_TI`

```rust
pub const PCI_VENDOR_ID_TI: u16 = 0x104c;
```

#### Constant `PCI_VENDOR_ID_SONY`

```rust
pub const PCI_VENDOR_ID_SONY: u16 = 0x104d;
```

#### Constant `PCI_VENDOR_ID_WINBOND2`

```rust
pub const PCI_VENDOR_ID_WINBOND2: u16 = 0x1050;
```

#### Constant `PCI_VENDOR_ID_ANIGMA`

```rust
pub const PCI_VENDOR_ID_ANIGMA: u16 = 0x1051;
```

#### Constant `PCI_VENDOR_ID_EFAR`

```rust
pub const PCI_VENDOR_ID_EFAR: u16 = 0x1055;
```

#### Constant `PCI_VENDOR_ID_MOTOROLA`

```rust
pub const PCI_VENDOR_ID_MOTOROLA: u16 = 0x1057;
```

#### Constant `PCI_VENDOR_ID_PROMISE`

```rust
pub const PCI_VENDOR_ID_PROMISE: u16 = 0x105a;
```

#### Constant `PCI_VENDOR_ID_FOXCONN`

```rust
pub const PCI_VENDOR_ID_FOXCONN: u16 = 0x105b;
```

#### Constant `PCI_VENDOR_ID_UMC`

```rust
pub const PCI_VENDOR_ID_UMC: u16 = 0x1060;
```

#### Constant `PCI_VENDOR_ID_PICOPOWER`

```rust
pub const PCI_VENDOR_ID_PICOPOWER: u16 = 0x1066;
```

#### Constant `PCI_VENDOR_ID_MYLEX`

```rust
pub const PCI_VENDOR_ID_MYLEX: u16 = 0x1069;
```

#### Constant `PCI_VENDOR_ID_APPLE`

```rust
pub const PCI_VENDOR_ID_APPLE: u16 = 0x106b;
```

#### Constant `PCI_VENDOR_ID_YAMAHA`

```rust
pub const PCI_VENDOR_ID_YAMAHA: u16 = 0x1073;
```

#### Constant `PCI_VENDOR_ID_QLOGIC`

```rust
pub const PCI_VENDOR_ID_QLOGIC: u16 = 0x1077;
```

#### Constant `PCI_VENDOR_ID_CYRIX`

```rust
pub const PCI_VENDOR_ID_CYRIX: u16 = 0x1078;
```

#### Constant `PCI_VENDOR_ID_CONTAQ`

```rust
pub const PCI_VENDOR_ID_CONTAQ: u16 = 0x1080;
```

#### Constant `PCI_VENDOR_ID_OLICOM`

```rust
pub const PCI_VENDOR_ID_OLICOM: u16 = 0x108d;
```

#### Constant `PCI_VENDOR_ID_SUN`

```rust
pub const PCI_VENDOR_ID_SUN: u16 = 0x108e;
```

#### Constant `PCI_VENDOR_ID_NI`

```rust
pub const PCI_VENDOR_ID_NI: u16 = 0x1093;
```

#### Constant `PCI_VENDOR_ID_CMD`

```rust
pub const PCI_VENDOR_ID_CMD: u16 = 0x1095;
```

#### Constant `PCI_VENDOR_ID_BROOKTREE`

```rust
pub const PCI_VENDOR_ID_BROOKTREE: u16 = 0x109e;
```

#### Constant `PCI_VENDOR_ID_SGI`

```rust
pub const PCI_VENDOR_ID_SGI: u16 = 0x10a9;
```

#### Constant `PCI_VENDOR_ID_WINBOND`

```rust
pub const PCI_VENDOR_ID_WINBOND: u16 = 0x10ad;
```

#### Constant `PCI_VENDOR_ID_PLX`

```rust
pub const PCI_VENDOR_ID_PLX: u16 = 0x10b5;
```

#### Constant `PCI_VENDOR_ID_MADGE`

```rust
pub const PCI_VENDOR_ID_MADGE: u16 = 0x10b6;
```

#### Constant `PCI_VENDOR_ID_3COM`

```rust
pub const PCI_VENDOR_ID_3COM: u16 = 0x10b7;
```

#### Constant `PCI_VENDOR_ID_AL`

```rust
pub const PCI_VENDOR_ID_AL: u16 = 0x10b9;
```

#### Constant `PCI_VENDOR_ID_NEOMAGIC`

```rust
pub const PCI_VENDOR_ID_NEOMAGIC: u16 = 0x10c8;
```

#### Constant `PCI_VENDOR_ID_TCONRAD`

```rust
pub const PCI_VENDOR_ID_TCONRAD: u16 = 0x10da;
```

#### Constant `PCI_VENDOR_ID_ROHM`

```rust
pub const PCI_VENDOR_ID_ROHM: u16 = 0x10db;
```

#### Constant `PCI_VENDOR_ID_NVIDIA`

```rust
pub const PCI_VENDOR_ID_NVIDIA: u16 = 0x10de;
```

#### Constant `PCI_VENDOR_ID_IMS`

```rust
pub const PCI_VENDOR_ID_IMS: u16 = 0x10e0;
```

#### Constant `PCI_VENDOR_ID_AMCC`

```rust
pub const PCI_VENDOR_ID_AMCC: u16 = 0x10e8;
```

#### Constant `PCI_VENDOR_ID_AMPERE`

```rust
pub const PCI_VENDOR_ID_AMPERE: u16 = 0x1def;
```

#### Constant `PCI_VENDOR_ID_INTERG`

```rust
pub const PCI_VENDOR_ID_INTERG: u16 = 0x10ea;
```

#### Constant `PCI_VENDOR_ID_REALTEK`

```rust
pub const PCI_VENDOR_ID_REALTEK: u16 = 0x10ec;
```

#### Constant `PCI_VENDOR_ID_XILINX`

```rust
pub const PCI_VENDOR_ID_XILINX: u16 = 0x10ee;
```

#### Constant `PCI_VENDOR_ID_INIT`

```rust
pub const PCI_VENDOR_ID_INIT: u16 = 0x1101;
```

#### Constant `PCI_VENDOR_ID_CREATIVE`

```rust
pub const PCI_VENDOR_ID_CREATIVE: u16 = 0x1102;
```

#### Constant `PCI_VENDOR_ID_TTI`

```rust
pub const PCI_VENDOR_ID_TTI: u16 = 0x1103;
```

#### Constant `PCI_VENDOR_ID_SIGMA`

```rust
pub const PCI_VENDOR_ID_SIGMA: u16 = 0x1105;
```

#### Constant `PCI_VENDOR_ID_VIA`

```rust
pub const PCI_VENDOR_ID_VIA: u16 = 0x1106;
```

#### Constant `PCI_VENDOR_ID_SIEMENS`

```rust
pub const PCI_VENDOR_ID_SIEMENS: u16 = 0x110A;
```

#### Constant `PCI_VENDOR_ID_VORTEX`

```rust
pub const PCI_VENDOR_ID_VORTEX: u16 = 0x1119;
```

#### Constant `PCI_VENDOR_ID_EF`

```rust
pub const PCI_VENDOR_ID_EF: u16 = 0x111a;
```

#### Constant `PCI_VENDOR_ID_IDT`

```rust
pub const PCI_VENDOR_ID_IDT: u16 = 0x111d;
```

#### Constant `PCI_VENDOR_ID_FORE`

```rust
pub const PCI_VENDOR_ID_FORE: u16 = 0x1127;
```

#### Constant `PCI_VENDOR_ID_PHILIPS`

```rust
pub const PCI_VENDOR_ID_PHILIPS: u16 = 0x1131;
```

#### Constant `PCI_VENDOR_ID_EICON`

```rust
pub const PCI_VENDOR_ID_EICON: u16 = 0x1133;
```

#### Constant `PCI_VENDOR_ID_CISCO`

```rust
pub const PCI_VENDOR_ID_CISCO: u16 = 0x1137;
```

#### Constant `PCI_VENDOR_ID_ZIATECH`

```rust
pub const PCI_VENDOR_ID_ZIATECH: u16 = 0x1138;
```

#### Constant `PCI_VENDOR_ID_SYSKONNECT`

```rust
pub const PCI_VENDOR_ID_SYSKONNECT: u16 = 0x1148;
```

#### Constant `PCI_VENDOR_ID_DIGI`

```rust
pub const PCI_VENDOR_ID_DIGI: u16 = 0x114f;
```

#### Constant `PCI_VENDOR_ID_XIRCOM`

```rust
pub const PCI_VENDOR_ID_XIRCOM: u16 = 0x115d;
```

#### Constant `PCI_VENDOR_ID_SERVERWORKS`

```rust
pub const PCI_VENDOR_ID_SERVERWORKS: u16 = 0x1166;
```

#### Constant `PCI_VENDOR_ID_ALTERA`

```rust
pub const PCI_VENDOR_ID_ALTERA: u16 = 0x1172;
```

#### Constant `PCI_VENDOR_ID_SBE`

```rust
pub const PCI_VENDOR_ID_SBE: u16 = 0x1176;
```

#### Constant `PCI_VENDOR_ID_TOSHIBA`

```rust
pub const PCI_VENDOR_ID_TOSHIBA: u16 = 0x1179;
```

#### Constant `PCI_VENDOR_ID_TOSHIBA_2`

```rust
pub const PCI_VENDOR_ID_TOSHIBA_2: u16 = 0x102f;
```

#### Constant `PCI_VENDOR_ID_ATTO`

```rust
pub const PCI_VENDOR_ID_ATTO: u16 = 0x117c;
```

#### Constant `PCI_VENDOR_ID_RICOH`

```rust
pub const PCI_VENDOR_ID_RICOH: u16 = 0x1180;
```

#### Constant `PCI_VENDOR_ID_DLINK`

```rust
pub const PCI_VENDOR_ID_DLINK: u16 = 0x1186;
```

#### Constant `PCI_VENDOR_ID_ARTOP`

```rust
pub const PCI_VENDOR_ID_ARTOP: u16 = 0x1191;
```

#### Constant `PCI_VENDOR_ID_ZEITNET`

```rust
pub const PCI_VENDOR_ID_ZEITNET: u16 = 0x1193;
```

#### Constant `PCI_VENDOR_ID_FUJITSU_ME`

```rust
pub const PCI_VENDOR_ID_FUJITSU_ME: u16 = 0x119e;
```

#### Constant `PCI_VENDOR_ID_MARVELL`

```rust
pub const PCI_VENDOR_ID_MARVELL: u16 = 0x11ab;
```

#### Constant `PCI_VENDOR_ID_MARVELL_EXT`

```rust
pub const PCI_VENDOR_ID_MARVELL_EXT: u16 = 0x1b4b;
```

#### Constant `PCI_VENDOR_ID_V3`

```rust
pub const PCI_VENDOR_ID_V3: u16 = 0x11b0;
```

#### Constant `PCI_VENDOR_ID_ATT`

```rust
pub const PCI_VENDOR_ID_ATT: u16 = 0x11c1;
```

#### Constant `PCI_VENDOR_ID_SPECIALIX`

```rust
pub const PCI_VENDOR_ID_SPECIALIX: u16 = 0x11cb;
```

#### Constant `PCI_VENDOR_ID_ANALOG_DEVICES`

```rust
pub const PCI_VENDOR_ID_ANALOG_DEVICES: u16 = 0x11d4;
```

#### Constant `PCI_VENDOR_ID_ZORAN`

```rust
pub const PCI_VENDOR_ID_ZORAN: u16 = 0x11de;
```

#### Constant `PCI_VENDOR_ID_COMPEX`

```rust
pub const PCI_VENDOR_ID_COMPEX: u16 = 0x11f6;
```

#### Constant `PCI_VENDOR_ID_MICROSEMI`

```rust
pub const PCI_VENDOR_ID_MICROSEMI: u16 = 0x11f8;
```

#### Constant `PCI_VENDOR_ID_RP`

```rust
pub const PCI_VENDOR_ID_RP: u16 = 0x11fe;
```

#### Constant `PCI_VENDOR_ID_CYCLADES`

```rust
pub const PCI_VENDOR_ID_CYCLADES: u16 = 0x120e;
```

#### Constant `PCI_VENDOR_ID_O2`

```rust
pub const PCI_VENDOR_ID_O2: u16 = 0x1217;
```

#### Constant `PCI_VENDOR_ID_3DFX`

```rust
pub const PCI_VENDOR_ID_3DFX: u16 = 0x121a;
```

#### Constant `PCI_VENDOR_ID_AVM`

```rust
pub const PCI_VENDOR_ID_AVM: u16 = 0x1244;
```

#### Constant `PCI_VENDOR_ID_STALLION`

```rust
pub const PCI_VENDOR_ID_STALLION: u16 = 0x124d;
```

#### Constant `PCI_VENDOR_ID_AT`

```rust
pub const PCI_VENDOR_ID_AT: u16 = 0x1259;
```

#### Constant `PCI_VENDOR_ID_ASIX`

```rust
pub const PCI_VENDOR_ID_ASIX: u16 = 0x125b;
```

#### Constant `PCI_VENDOR_ID_ESS`

```rust
pub const PCI_VENDOR_ID_ESS: u16 = 0x125d;
```

#### Constant `PCI_VENDOR_ID_SATSAGEM`

```rust
pub const PCI_VENDOR_ID_SATSAGEM: u16 = 0x1267;
```

#### Constant `PCI_VENDOR_ID_ENSONIQ`

```rust
pub const PCI_VENDOR_ID_ENSONIQ: u16 = 0x1274;
```

#### Constant `PCI_VENDOR_ID_TRANSMETA`

```rust
pub const PCI_VENDOR_ID_TRANSMETA: u16 = 0x1279;
```

#### Constant `PCI_VENDOR_ID_ROCKWELL`

```rust
pub const PCI_VENDOR_ID_ROCKWELL: u16 = 0x127A;
```

#### Constant `PCI_VENDOR_ID_ITE`

```rust
pub const PCI_VENDOR_ID_ITE: u16 = 0x1283;
```

#### Constant `PCI_VENDOR_ID_ALTEON`

```rust
pub const PCI_VENDOR_ID_ALTEON: u16 = 0x12ae;
```

#### Constant `PCI_VENDOR_ID_NVIDIA_SGS`

```rust
pub const PCI_VENDOR_ID_NVIDIA_SGS: u16 = 0x12d2;
```

#### Constant `PCI_VENDOR_ID_PERICOM`

```rust
pub const PCI_VENDOR_ID_PERICOM: u16 = 0x12D8;
```

#### Constant `PCI_VENDOR_ID_AUREAL`

```rust
pub const PCI_VENDOR_ID_AUREAL: u16 = 0x12eb;
```

#### Constant `PCI_VENDOR_ID_ELECTRONICDESIGNGMBH`

```rust
pub const PCI_VENDOR_ID_ELECTRONICDESIGNGMBH: u16 = 0x12f8;
```

#### Constant `PCI_VENDOR_ID_ESDGMBH`

```rust
pub const PCI_VENDOR_ID_ESDGMBH: u16 = 0x12fe;
```

#### Constant `PCI_VENDOR_ID_CB`

```rust
pub const PCI_VENDOR_ID_CB: u16 = 0x1307;
```

#### Constant `PCI_VENDOR_ID_SIIG`

```rust
pub const PCI_VENDOR_ID_SIIG: u16 = 0x131f;
```

#### Constant `PCI_VENDOR_ID_RADISYS`

```rust
pub const PCI_VENDOR_ID_RADISYS: u16 = 0x1331;
```

#### Constant `PCI_VENDOR_ID_MICRO_MEMORY`

```rust
pub const PCI_VENDOR_ID_MICRO_MEMORY: u16 = 0x1332;
```

#### Constant `PCI_VENDOR_ID_DOMEX`

```rust
pub const PCI_VENDOR_ID_DOMEX: u16 = 0x134a;
```

#### Constant `PCI_VENDOR_ID_INTASHIELD`

```rust
pub const PCI_VENDOR_ID_INTASHIELD: u16 = 0x135a;
```

#### Constant `PCI_VENDOR_ID_QUATECH`

```rust
pub const PCI_VENDOR_ID_QUATECH: u16 = 0x135C;
```

#### Constant `PCI_VENDOR_ID_SEALEVEL`

```rust
pub const PCI_VENDOR_ID_SEALEVEL: u16 = 0x135e;
```

#### Constant `PCI_VENDOR_ID_HYPERCOPE`

```rust
pub const PCI_VENDOR_ID_HYPERCOPE: u16 = 0x1365;
```

#### Constant `PCI_VENDOR_ID_DIGIGRAM`

```rust
pub const PCI_VENDOR_ID_DIGIGRAM: u16 = 0x1369;
```

#### Constant `PCI_VENDOR_ID_KAWASAKI`

```rust
pub const PCI_VENDOR_ID_KAWASAKI: u16 = 0x136b;
```

#### Constant `PCI_VENDOR_ID_CNET`

```rust
pub const PCI_VENDOR_ID_CNET: u16 = 0x1371;
```

#### Constant `PCI_VENDOR_ID_LMC`

```rust
pub const PCI_VENDOR_ID_LMC: u16 = 0x1376;
```

#### Constant `PCI_VENDOR_ID_NETGEAR`

```rust
pub const PCI_VENDOR_ID_NETGEAR: u16 = 0x1385;
```

#### Constant `PCI_VENDOR_ID_APPLICOM`

```rust
pub const PCI_VENDOR_ID_APPLICOM: u16 = 0x1389;
```

#### Constant `PCI_VENDOR_ID_MOXA`

```rust
pub const PCI_VENDOR_ID_MOXA: u16 = 0x1393;
```

#### Constant `PCI_VENDOR_ID_CCD`

```rust
pub const PCI_VENDOR_ID_CCD: u16 = 0x1397;
```

#### Constant `PCI_VENDOR_ID_EXAR`

```rust
pub const PCI_VENDOR_ID_EXAR: u16 = 0x13a8;
```

#### Constant `PCI_VENDOR_ID_MICROGATE`

```rust
pub const PCI_VENDOR_ID_MICROGATE: u16 = 0x13c0;
```

#### Constant `PCI_VENDOR_ID_3WARE`

```rust
pub const PCI_VENDOR_ID_3WARE: u16 = 0x13C1;
```

#### Constant `PCI_VENDOR_ID_IOMEGA`

```rust
pub const PCI_VENDOR_ID_IOMEGA: u16 = 0x13ca;
```

#### Constant `PCI_VENDOR_ID_ABOCOM`

```rust
pub const PCI_VENDOR_ID_ABOCOM: u16 = 0x13D1;
```

#### Constant `PCI_VENDOR_ID_SUNDANCE`

```rust
pub const PCI_VENDOR_ID_SUNDANCE: u16 = 0x13f0;
```

#### Constant `PCI_VENDOR_ID_CMEDIA`

```rust
pub const PCI_VENDOR_ID_CMEDIA: u16 = 0x13f6;
```

#### Constant `PCI_VENDOR_ID_ADVANTECH`

```rust
pub const PCI_VENDOR_ID_ADVANTECH: u16 = 0x13fe;
```

#### Constant `PCI_VENDOR_ID_MEILHAUS`

```rust
pub const PCI_VENDOR_ID_MEILHAUS: u16 = 0x1402;
```

#### Constant `PCI_VENDOR_ID_LAVA`

```rust
pub const PCI_VENDOR_ID_LAVA: u16 = 0x1407;
```

#### Constant `PCI_VENDOR_ID_TIMEDIA`

```rust
pub const PCI_VENDOR_ID_TIMEDIA: u16 = 0x1409;
```

#### Constant `PCI_VENDOR_ID_ICE`

```rust
pub const PCI_VENDOR_ID_ICE: u16 = 0x1412;
```

#### Constant `PCI_VENDOR_ID_MICROSOFT`

```rust
pub const PCI_VENDOR_ID_MICROSOFT: u16 = 0x1414;
```

#### Constant `PCI_VENDOR_ID_OXSEMI`

```rust
pub const PCI_VENDOR_ID_OXSEMI: u16 = 0x1415;
```

#### Constant `PCI_VENDOR_ID_CHELSIO`

```rust
pub const PCI_VENDOR_ID_CHELSIO: u16 = 0x1425;
```

#### Constant `PCI_VENDOR_ID_EDIMAX`

```rust
pub const PCI_VENDOR_ID_EDIMAX: u16 = 0x1432;
```

#### Constant `PCI_VENDOR_ID_ADLINK`

```rust
pub const PCI_VENDOR_ID_ADLINK: u16 = 0x144a;
```

#### Constant `PCI_VENDOR_ID_SAMSUNG`

```rust
pub const PCI_VENDOR_ID_SAMSUNG: u16 = 0x144d;
```

#### Constant `PCI_VENDOR_ID_GIGABYTE`

```rust
pub const PCI_VENDOR_ID_GIGABYTE: u16 = 0x1458;
```

#### Constant `PCI_VENDOR_ID_AMBIT`

```rust
pub const PCI_VENDOR_ID_AMBIT: u16 = 0x1468;
```

#### Constant `PCI_VENDOR_ID_MYRICOM`

```rust
pub const PCI_VENDOR_ID_MYRICOM: u16 = 0x14c1;
```

#### Constant `PCI_VENDOR_ID_MEDIATEK`

```rust
pub const PCI_VENDOR_ID_MEDIATEK: u16 = 0x14c3;
```

#### Constant `PCI_VENDOR_ID_TITAN`

```rust
pub const PCI_VENDOR_ID_TITAN: u16 = 0x14D2;
```

#### Constant `PCI_VENDOR_ID_PANACOM`

```rust
pub const PCI_VENDOR_ID_PANACOM: u16 = 0x14d4;
```

#### Constant `PCI_VENDOR_ID_SIPACKETS`

```rust
pub const PCI_VENDOR_ID_SIPACKETS: u16 = 0x14d9;
```

#### Constant `PCI_VENDOR_ID_AFAVLAB`

```rust
pub const PCI_VENDOR_ID_AFAVLAB: u16 = 0x14db;
```

#### Constant `PCI_VENDOR_ID_AMPLICON`

```rust
pub const PCI_VENDOR_ID_AMPLICON: u16 = 0x14dc;
```

#### Constant `PCI_VENDOR_ID_BCM_GVC`

```rust
pub const PCI_VENDOR_ID_BCM_GVC: u16 = 0x14a4;
```

#### Constant `PCI_VENDOR_ID_BROADCOM`

```rust
pub const PCI_VENDOR_ID_BROADCOM: u16 = 0x14e4;
```

#### Constant `PCI_VENDOR_ID_TOPIC`

```rust
pub const PCI_VENDOR_ID_TOPIC: u16 = 0x151f;
```

#### Constant `PCI_VENDOR_ID_MAINPINE`

```rust
pub const PCI_VENDOR_ID_MAINPINE: u16 = 0x1522;
```

#### Constant `PCI_VENDOR_ID_ENE`

```rust
pub const PCI_VENDOR_ID_ENE: u16 = 0x1524;
```

#### Constant `PCI_VENDOR_ID_SYBA`

```rust
pub const PCI_VENDOR_ID_SYBA: u16 = 0x1592;
```

#### Constant `PCI_VENDOR_ID_MORETON`

```rust
pub const PCI_VENDOR_ID_MORETON: u16 = 0x15aa;
```

#### Constant `PCI_VENDOR_ID_VMWARE`

```rust
pub const PCI_VENDOR_ID_VMWARE: u16 = 0x15ad;
```

#### Constant `PCI_VENDOR_ID_ZOLTRIX`

```rust
pub const PCI_VENDOR_ID_ZOLTRIX: u16 = 0x15b0;
```

#### Constant `PCI_VENDOR_ID_MELLANOX`

```rust
pub const PCI_VENDOR_ID_MELLANOX: u16 = 0x15b3;
```

#### Constant `PCI_VENDOR_ID_DFI`

```rust
pub const PCI_VENDOR_ID_DFI: u16 = 0x15bd;
```

#### Constant `PCI_VENDOR_ID_QUICKNET`

```rust
pub const PCI_VENDOR_ID_QUICKNET: u16 = 0x15e2;
```

#### Constant `PCI_VENDOR_ID_ADDIDATA`

```rust
pub const PCI_VENDOR_ID_ADDIDATA: u16 = 0x15B8;
```

#### Constant `PCI_VENDOR_ID_PDC`

```rust
pub const PCI_VENDOR_ID_PDC: u16 = 0x15e9;
```

#### Constant `PCI_VENDOR_ID_FARSITE`

```rust
pub const PCI_VENDOR_ID_FARSITE: u16 = 0x1619;
```

#### Constant `PCI_VENDOR_ID_ARIMA`

```rust
pub const PCI_VENDOR_ID_ARIMA: u16 = 0x161f;
```

#### Constant `PCI_VENDOR_ID_BROCADE`

```rust
pub const PCI_VENDOR_ID_BROCADE: u16 = 0x1657;
```

#### Constant `PCI_VENDOR_ID_SIBYTE`

```rust
pub const PCI_VENDOR_ID_SIBYTE: u16 = 0x166d;
```

#### Constant `PCI_VENDOR_ID_ATHEROS`

```rust
pub const PCI_VENDOR_ID_ATHEROS: u16 = 0x168c;
```

#### Constant `PCI_VENDOR_ID_NETCELL`

```rust
pub const PCI_VENDOR_ID_NETCELL: u16 = 0x169c;
```

#### Constant `PCI_VENDOR_ID_CENATEK`

```rust
pub const PCI_VENDOR_ID_CENATEK: u16 = 0x16CA;
```

#### Constant `PCI_VENDOR_ID_SYNOPSYS`

```rust
pub const PCI_VENDOR_ID_SYNOPSYS: u16 = 0x16c3;
```

#### Constant `PCI_VENDOR_ID_USR`

```rust
pub const PCI_VENDOR_ID_USR: u16 = 0x16ec;
```

#### Constant `PCI_VENDOR_ID_VITESSE`

```rust
pub const PCI_VENDOR_ID_VITESSE: u16 = 0x1725;
```

#### Constant `PCI_VENDOR_ID_LINKSYS`

```rust
pub const PCI_VENDOR_ID_LINKSYS: u16 = 0x1737;
```

#### Constant `PCI_VENDOR_ID_ALTIMA`

```rust
pub const PCI_VENDOR_ID_ALTIMA: u16 = 0x173b;
```

#### Constant `PCI_VENDOR_ID_CAVIUM`

```rust
pub const PCI_VENDOR_ID_CAVIUM: u16 = 0x177d;
```

#### Constant `PCI_VENDOR_ID_TECHWELL`

```rust
pub const PCI_VENDOR_ID_TECHWELL: u16 = 0x1797;
```

#### Constant `PCI_VENDOR_ID_BELKIN`

```rust
pub const PCI_VENDOR_ID_BELKIN: u16 = 0x1799;
```

#### Constant `PCI_VENDOR_ID_RDC`

```rust
pub const PCI_VENDOR_ID_RDC: u16 = 0x17f3;
```

#### Constant `PCI_VENDOR_ID_GLI`

```rust
pub const PCI_VENDOR_ID_GLI: u16 = 0x17a0;
```

#### Constant `PCI_VENDOR_ID_LENOVO`

```rust
pub const PCI_VENDOR_ID_LENOVO: u16 = 0x17aa;
```

#### Constant `PCI_VENDOR_ID_QCOM`

```rust
pub const PCI_VENDOR_ID_QCOM: u16 = 0x17cb;
```

#### Constant `PCI_VENDOR_ID_CDNS`

```rust
pub const PCI_VENDOR_ID_CDNS: u16 = 0x17cd;
```

#### Constant `PCI_VENDOR_ID_ARECA`

```rust
pub const PCI_VENDOR_ID_ARECA: u16 = 0x17d3;
```

#### Constant `PCI_VENDOR_ID_S2IO`

```rust
pub const PCI_VENDOR_ID_S2IO: u16 = 0x17d5;
```

#### Constant `PCI_VENDOR_ID_SITECOM`

```rust
pub const PCI_VENDOR_ID_SITECOM: u16 = 0x182d;
```

#### Constant `PCI_VENDOR_ID_TOPSPIN`

```rust
pub const PCI_VENDOR_ID_TOPSPIN: u16 = 0x1867;
```

#### Constant `PCI_VENDOR_ID_COMMTECH`

```rust
pub const PCI_VENDOR_ID_COMMTECH: u16 = 0x18f7;
```

#### Constant `PCI_VENDOR_ID_SILAN`

```rust
pub const PCI_VENDOR_ID_SILAN: u16 = 0x1904;
```

#### Constant `PCI_VENDOR_ID_RENESAS`

```rust
pub const PCI_VENDOR_ID_RENESAS: u16 = 0x1912;
```

#### Constant `PCI_VENDOR_ID_SOLARFLARE`

```rust
pub const PCI_VENDOR_ID_SOLARFLARE: u16 = 0x1924;
```

#### Constant `PCI_VENDOR_ID_TDI`

```rust
pub const PCI_VENDOR_ID_TDI: u16 = 0x192E;
```

#### Constant `PCI_VENDOR_ID_NXP`

```rust
pub const PCI_VENDOR_ID_NXP: u16 = 0x1957;
```

#### Constant `PCI_VENDOR_ID_PASEMI`

```rust
pub const PCI_VENDOR_ID_PASEMI: u16 = 0x1959;
```

#### Constant `PCI_VENDOR_ID_ATTANSIC`

```rust
pub const PCI_VENDOR_ID_ATTANSIC: u16 = 0x1969;
```

#### Constant `PCI_VENDOR_ID_JMICRON`

```rust
pub const PCI_VENDOR_ID_JMICRON: u16 = 0x197B;
```

#### Constant `PCI_VENDOR_ID_KORENIX`

```rust
pub const PCI_VENDOR_ID_KORENIX: u16 = 0x1982;
```

#### Constant `PCI_VENDOR_ID_HUAWEI`

```rust
pub const PCI_VENDOR_ID_HUAWEI: u16 = 0x19e5;
```

#### Constant `PCI_VENDOR_ID_NETRONOME`

```rust
pub const PCI_VENDOR_ID_NETRONOME: u16 = 0x19ee;
```

#### Constant `PCI_VENDOR_ID_QMI`

```rust
pub const PCI_VENDOR_ID_QMI: u16 = 0x1a32;
```

#### Constant `PCI_VENDOR_ID_AZWAVE`

```rust
pub const PCI_VENDOR_ID_AZWAVE: u16 = 0x1a3b;
```

#### Constant `PCI_VENDOR_ID_REDHAT_QUMRANET`

```rust
pub const PCI_VENDOR_ID_REDHAT_QUMRANET: u16 = 0x1af4;
```

#### Constant `PCI_VENDOR_ID_ASMEDIA`

```rust
pub const PCI_VENDOR_ID_ASMEDIA: u16 = 0x1b21;
```

#### Constant `PCI_VENDOR_ID_REDHAT`

```rust
pub const PCI_VENDOR_ID_REDHAT: u16 = 0x1b36;
```

#### Constant `PCI_VENDOR_ID_WCHIC`

```rust
pub const PCI_VENDOR_ID_WCHIC: u16 = 0x1c00;
```

#### Constant `PCI_VENDOR_ID_SILICOM_DENMARK`

```rust
pub const PCI_VENDOR_ID_SILICOM_DENMARK: u16 = 0x1c2c;
```

#### Constant `PCI_VENDOR_ID_AMAZON_ANNAPURNA_LABS`

```rust
pub const PCI_VENDOR_ID_AMAZON_ANNAPURNA_LABS: u16 = 0x1c36;
```

#### Constant `PCI_VENDOR_ID_CIRCUITCO`

```rust
pub const PCI_VENDOR_ID_CIRCUITCO: u16 = 0x1cc8;
```

#### Constant `PCI_VENDOR_ID_AMAZON`

```rust
pub const PCI_VENDOR_ID_AMAZON: u16 = 0x1d0f;
```

#### Constant `PCI_VENDOR_ID_ZHAOXIN`

```rust
pub const PCI_VENDOR_ID_ZHAOXIN: u16 = 0x1d17;
```

#### Constant `PCI_VENDOR_ID_ROCKCHIP`

```rust
pub const PCI_VENDOR_ID_ROCKCHIP: u16 = 0x1d87;
```

#### Constant `PCI_VENDOR_ID_HYGON`

```rust
pub const PCI_VENDOR_ID_HYGON: u16 = 0x1d94;
```

#### Constant `PCI_VENDOR_ID_META`

```rust
pub const PCI_VENDOR_ID_META: u16 = 0x1d9b;
```

#### Constant `PCI_VENDOR_ID_FUNGIBLE`

```rust
pub const PCI_VENDOR_ID_FUNGIBLE: u16 = 0x1dad;
```

#### Constant `PCI_VENDOR_ID_HXT`

```rust
pub const PCI_VENDOR_ID_HXT: u16 = 0x1dbf;
```

#### Constant `PCI_VENDOR_ID_TEKRAM`

```rust
pub const PCI_VENDOR_ID_TEKRAM: u16 = 0x1de1;
```

#### Constant `PCI_VENDOR_ID_RPI`

```rust
pub const PCI_VENDOR_ID_RPI: u16 = 0x1de4;
```

#### Constant `PCI_VENDOR_ID_ALIBABA`

```rust
pub const PCI_VENDOR_ID_ALIBABA: u16 = 0x1ded;
```

#### Constant `PCI_VENDOR_ID_CXL`

```rust
pub const PCI_VENDOR_ID_CXL: u16 = 0x1e98;
```

#### Constant `PCI_VENDOR_ID_TEHUTI`

```rust
pub const PCI_VENDOR_ID_TEHUTI: u16 = 0x1fc9;
```

#### Constant `PCI_VENDOR_ID_SUNIX`

```rust
pub const PCI_VENDOR_ID_SUNIX: u16 = 0x1fd4;
```

#### Constant `PCI_VENDOR_ID_HINT`

```rust
pub const PCI_VENDOR_ID_HINT: u16 = 0x3388;
```

#### Constant `PCI_VENDOR_ID_3DLABS`

```rust
pub const PCI_VENDOR_ID_3DLABS: u16 = 0x3d3d;
```

#### Constant `PCI_VENDOR_ID_NETXEN`

```rust
pub const PCI_VENDOR_ID_NETXEN: u16 = 0x4040;
```

#### Constant `PCI_VENDOR_ID_AKS`

```rust
pub const PCI_VENDOR_ID_AKS: u16 = 0x416c;
```

#### Constant `PCI_VENDOR_ID_WCHCN`

```rust
pub const PCI_VENDOR_ID_WCHCN: u16 = 0x4348;
```

#### Constant `PCI_VENDOR_ID_ACCESSIO`

```rust
pub const PCI_VENDOR_ID_ACCESSIO: u16 = 0x494f;
```

#### Constant `PCI_VENDOR_ID_S3`

```rust
pub const PCI_VENDOR_ID_S3: u16 = 0x5333;
```

#### Constant `PCI_VENDOR_ID_DUNORD`

```rust
pub const PCI_VENDOR_ID_DUNORD: u16 = 0x5544;
```

#### Constant `PCI_VENDOR_ID_DCI`

```rust
pub const PCI_VENDOR_ID_DCI: u16 = 0x6666;
```

#### Constant `PCI_VENDOR_ID_GLENFLY`

```rust
pub const PCI_VENDOR_ID_GLENFLY: u16 = 0x6766;
```

#### Constant `PCI_VENDOR_ID_INTEL`

```rust
pub const PCI_VENDOR_ID_INTEL: u16 = 0x8086;
```

#### Constant `PCI_VENDOR_ID_WANGXUN`

```rust
pub const PCI_VENDOR_ID_WANGXUN: u16 = 0x8088;
```

#### Constant `PCI_VENDOR_ID_SCALEMP`

```rust
pub const PCI_VENDOR_ID_SCALEMP: u16 = 0x8686;
```

#### Constant `PCI_VENDOR_ID_COMPUTONE`

```rust
pub const PCI_VENDOR_ID_COMPUTONE: u16 = 0x8e0e;
```

#### Constant `PCI_VENDOR_ID_KTI`

```rust
pub const PCI_VENDOR_ID_KTI: u16 = 0x8e2e;
```

#### Constant `PCI_VENDOR_ID_ADAPTEC`

```rust
pub const PCI_VENDOR_ID_ADAPTEC: u16 = 0x9004;
```

#### Constant `PCI_VENDOR_ID_ADAPTEC2`

```rust
pub const PCI_VENDOR_ID_ADAPTEC2: u16 = 0x9005;
```

#### Constant `PCI_VENDOR_ID_HOLTEK`

```rust
pub const PCI_VENDOR_ID_HOLTEK: u16 = 0x9412;
```

#### Constant `PCI_VENDOR_ID_NETMOS`

```rust
pub const PCI_VENDOR_ID_NETMOS: u16 = 0x9710;
```

#### Constant `PCI_VENDOR_ID_3COM_2`

```rust
pub const PCI_VENDOR_ID_3COM_2: u16 = 0xa727;
```

#### Constant `PCI_VENDOR_ID_SOLIDRUN`

```rust
pub const PCI_VENDOR_ID_SOLIDRUN: u16 = 0xd063;
```

#### Constant `PCI_VENDOR_ID_DIGIUM`

```rust
pub const PCI_VENDOR_ID_DIGIUM: u16 = 0xd161;
```

#### Constant `PCI_VENDOR_ID_TIGERJET`

```rust
pub const PCI_VENDOR_ID_TIGERJET: u16 = 0xe159;
```

#### Constant `PCI_VENDOR_ID_XILINX_RME`

```rust
pub const PCI_VENDOR_ID_XILINX_RME: u16 = 0xea60;
```

#### Constant `PCI_VENDOR_ID_XEN`

```rust
pub const PCI_VENDOR_ID_XEN: u16 = 0x5853;
```

#### Constant `PCI_VENDOR_ID_OCZ`

```rust
pub const PCI_VENDOR_ID_OCZ: u16 = 0x1b85;
```

#### Constant `PCI_VENDOR_ID_NCUBE`

```rust
pub const PCI_VENDOR_ID_NCUBE: u16 = 0x10ff;
```

## Module `fs`

```rust
pub mod fs { /* ... */ }
```

### Modules

## Module `flags`

```rust
pub mod flags { /* ... */ }
```

### Constants and Statics

#### Constant `O_APPEND`

File is opened in append mode.

```rust
pub const O_APPEND: u32 = 0x0000;
```

#### Constant `O_ASYNC`

Signal-driven I/O is enabled.

```rust
pub const O_ASYNC: u32 = 0x0000;
```

#### Constant `O_CLOEXEC`

Close-on-exec flag is set.

```rust
pub const O_CLOEXEC: u32 = 0x0000;
```

#### Constant `O_CREATE`

File was created if it didn't already exist.

```rust
pub const O_CREATE: u32 = 0x0000;
```

#### Constant `O_DIRECT`

Direct I/O is enabled for this file.

```rust
pub const O_DIRECT: u32 = 0x0000;
```

#### Constant `O_DIRECTORY`

File must be a directory.

```rust
pub const O_DIRECTORY: u32 = 0x0000;
```

#### Constant `O_DSYNC`

Like [`O_SYNC`] except metadata is not synced.

```rust
pub const O_DSYNC: u32 = 0x0000;
```

#### Constant `O_EXCL`

Ensure that this file is created with the `open(2)` call.

```rust
pub const O_EXCL: u32 = 0x0000;
```

#### Constant `O_LARGEFILE`

Large file size enabled (`off64_t` over `off_t`).

```rust
pub const O_LARGEFILE: u32 = 0x0000;
```

#### Constant `O_NOATIME`

Do not update the file last access time.

```rust
pub const O_NOATIME: u32 = 0x0000;
```

#### Constant `O_NOCTTY`

File should not be used as process's controlling terminal.

```rust
pub const O_NOCTTY: u32 = 0x0000;
```

#### Constant `O_NOFOLLOW`

If basename of path is a symbolic link, fail open.

```rust
pub const O_NOFOLLOW: u32 = 0x0000;
```

#### Constant `O_NONBLOCK`

File is using nonblocking I/O.

```rust
pub const O_NONBLOCK: u32 = 0x0000;
```

#### Constant `O_NDELAY`

File is using nonblocking I/O.

```rust
pub const O_NDELAY: u32 = 0x0000;
```

#### Constant `O_PATH`

Used to obtain a path file descriptor.

```rust
pub const O_PATH: u32 = 0x0000;
```

#### Constant `O_SYNC`

Write operations on this file will flush data and metadata.

```rust
pub const O_SYNC: u32 = 0x0000;
```

#### Constant `O_TMPFILE`

This file is an unnamed temporary regular file.

```rust
pub const O_TMPFILE: u32 = 0x0000;
```

#### Constant `O_TRUNC`

File should be truncated to length 0.

```rust
pub const O_TRUNC: u32 = 0x0000;
```

#### Constant `O_ACCMODE`

Bitmask for access mode flags.

# Examples

```
use kernel::fs::file;
# fn do_something() {}
# let flags = 0;
if (flags & file::flags::O_ACCMODE) == file::flags::O_RDONLY {
    do_something();
}
```

```rust
pub const O_ACCMODE: u32 = 0x0000;
```

#### Constant `O_RDONLY`

File is read only.

```rust
pub const O_RDONLY: u32 = 0x0000;
```

#### Constant `O_WRONLY`

File is write only.

```rust
pub const O_WRONLY: u32 = 0x0000;
```

#### Constant `O_RDWR`

File can be both read and written.

```rust
pub const O_RDWR: u32 = 0x0000;
```

### Types

#### Type Alias `Offset`

```rust
pub type Offset = u32;
```

## Module `iommu`

```rust
pub mod iommu { /* ... */ }
```

### Modules

## Module `prot`

```rust
pub mod prot { /* ... */ }
```

### Constants and Statics

#### Constant `READ`

Read access.

```rust
pub const READ: u32 = 0x0000;
```

#### Constant `WRITE`

Write access.

```rust
pub const WRITE: u32 = 0x0000;
```

#### Constant `CACHE`

Request cache coherency.

```rust
pub const CACHE: u32 = 0x0000;
```

#### Constant `NOEXEC`

Request no-execute permission.

```rust
pub const NOEXEC: u32 = 0x0000;
```

#### Constant `MMIO`

MMIO peripheral mapping.

```rust
pub const MMIO: u32 = 0x0000;
```

#### Constant `PRIVILEGED`

Privileged mapping.

```rust
pub const PRIVILEGED: u32 = 0x0000;
```

## Module `irq`

```rust
pub mod irq { /* ... */ }
```

### Types

#### Struct `Flags`

```rust
pub struct Flags(pub(in ::consts::irq) u64);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u64` |  |

##### Implementations

###### Methods

- ```rust
  pub(crate) fn into_inner(self: Self) -> u64 { /* ... */ }
  ```

- ```rust
  pub(in ::consts::irq) const fn new(value: u32) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `mm_virt`

```rust
pub mod mm_virt { /* ... */ }
```

### Modules

## Module `flags`

```rust
pub mod flags { /* ... */ }
```

### Types

#### Type Alias `vm_flags_t`

```rust
pub type vm_flags_t = u32;
```

### Constants and Statics

#### Constant `NONE`

No flags are set.

```rust
pub const NONE: u32 = 0x00000000;
```

#### Constant `READ`

Mapping allows reads.

```rust
pub const READ: u32 = 0x00000000;
```

#### Constant `WRITE`

Mapping allows writes.

```rust
pub const WRITE: u32 = 0x00000000;
```

#### Constant `EXEC`

Mapping allows execution.

```rust
pub const EXEC: u32 = 0x00000000;
```

#### Constant `SHARED`

Mapping is shared.

```rust
pub const SHARED: u32 = 0x00000000;
```

#### Constant `MAYREAD`

Mapping may be updated to allow reads.

```rust
pub const MAYREAD: u32 = 0x00000000;
```

#### Constant `MAYWRITE`

Mapping may be updated to allow writes.

```rust
pub const MAYWRITE: u32 = 0x00000000;
```

#### Constant `MAYEXEC`

Mapping may be updated to allow execution.

```rust
pub const MAYEXEC: u32 = 0x00000000;
```

#### Constant `MAYSHARE`

Mapping may be updated to be shared.

```rust
pub const MAYSHARE: u32 = 0x00000000;
```

#### Constant `PFNMAP`

Page-ranges managed without `struct page`, just pure PFN.

```rust
pub const PFNMAP: u32 = 0x00000000;
```

#### Constant `IO`

Memory mapped I/O or similar.

```rust
pub const IO: u32 = 0x00000000;
```

#### Constant `DONTCOPY`

Do not copy this vma on fork.

```rust
pub const DONTCOPY: u32 = 0x00000000;
```

#### Constant `DONTEXPAND`

Cannot expand with mremap().

```rust
pub const DONTEXPAND: u32 = 0x00000000;
```

#### Constant `LOCKONFAULT`

Lock the pages covered when they are faulted in.

```rust
pub const LOCKONFAULT: u32 = 0x00000000;
```

#### Constant `ACCOUNT`

Is a VM accounted object.

```rust
pub const ACCOUNT: u32 = 0x00000000;
```

#### Constant `NORESERVE`

Should the VM suppress accounting.

```rust
pub const NORESERVE: u32 = 0x00000000;
```

#### Constant `HUGETLB`

Huge TLB Page VM.

```rust
pub const HUGETLB: u32 = 0x00000000;
```

#### Constant `SYNC`

Synchronous page faults. (DAX-specific)

```rust
pub const SYNC: u32 = 0x00000000;
```

#### Constant `ARCH_1`

Architecture-specific flag.

```rust
pub const ARCH_1: u32 = 0x00000000;
```

#### Constant `WIPEONFORK`

Wipe VMA contents in child on fork.

```rust
pub const WIPEONFORK: u32 = 0x00000000;
```

#### Constant `DONTDUMP`

Do not include in the core dump.

```rust
pub const DONTDUMP: u32 = 0x00000000;
```

#### Constant `SOFTDIRTY`

Not soft dirty clean area.

```rust
pub const SOFTDIRTY: u32 = 0x00000000;
```

#### Constant `MIXEDMAP`

Can contain `struct page` and pure PFN pages.

```rust
pub const MIXEDMAP: u32 = 0x00000000;
```

#### Constant `HUGEPAGE`

MADV_HUGEPAGE marked this vma.

```rust
pub const HUGEPAGE: u32 = 0x00000000;
```

#### Constant `NOHUGEPAGE`

MADV_NOHUGEPAGE marked this vma.

```rust
pub const NOHUGEPAGE: u32 = 0x00000000;
```

#### Constant `MERGEABLE`

KSM may merge identical pages.

```rust
pub const MERGEABLE: u32 = 0x00000000;
```

## Module `net`

```rust
pub mod net { /* ... */ }
```

### Modules

## Module `phy`

```rust
pub mod phy { /* ... */ }
```

### Modules

## Module `flags`

```rust
pub mod flags { /* ... */ }
```

### Constants and Statics

#### Constant `IS_INTERNAL`

PHY is internal.

```rust
pub const IS_INTERNAL: u32 = 0x0;
```

#### Constant `RST_AFTER_CLK_EN`

PHY needs to be reset after the refclk is enabled.

```rust
pub const RST_AFTER_CLK_EN: u32 = 0x0;
```

#### Constant `POLL_CABLE_TEST`

Polling is used to detect PHY status changes.

```rust
pub const POLL_CABLE_TEST: u32 = 0x0;
```

#### Constant `ALWAYS_CALL_SUSPEND`

Don't suspend.

```rust
pub const ALWAYS_CALL_SUSPEND: u32 = 0x0;
```

### Traits

#### Trait `Driver`

```rust
pub trait Driver {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Constants

- `NAME`: The friendly name of this PHY type.

##### Provided Methods

### Constants and Statics

#### Constant `PI`

```rust
pub const PI: f64 = 3.141592653589793;
```

#### Constant `E`

```rust
pub const E: f64 = 2.71828;
```

#### Constant `TAU`

```rust
pub const TAU: f64 = 6.28318530717958647692528676655900576839433879875021164194988918461563281257241799;
```

#### Constant `NAN`

```rust
pub const NAN: f64 = _;
```

#### Constant `INF`

```rust
pub const INF: f64 = _;
```

#### Constant `NULL`

```rust
pub const NULL: u8 = 0x00;
```

#### Constant `ELLIPSIS`

```rust
pub const ELLIPSIS: &str = "...";
```

#### Constant `EMPTY`

```rust
pub const EMPTY: &str = "";
```

#### Constant `SPACE`

```rust
pub const SPACE: &str = " ";
```

#### Constant `TAB`

```rust
pub const TAB: &str = "\t";
```

#### Constant `NEWLINE`

```rust
pub const NEWLINE: &str = "\n";
```

#### Constant `CR`

```rust
pub const CR: &str = "\r";
```

#### Constant `LF`

```rust
pub const LF: &str = "\n";
```

#### Constant `CRLF`

```rust
pub const CRLF: &str = "\r\n";
```

#### Constant `NULL_CHAR`

```rust
pub const NULL_CHAR: &str = "\0";
```

#### Constant `NULL_BYTE`

```rust
pub const NULL_BYTE: &str = "\0";
```

#### Constant `NULL_WORD`

```rust
pub const NULL_WORD: &str = "\0\0";
```

#### Constant `NULL_DWORD`

```rust
pub const NULL_DWORD: &str = "\0\0\0\0";
```

#### Constant `NULL_QWORD`

```rust
pub const NULL_QWORD: &str = "\0\0\0\0\0\0\0\0";
```

#### Constant `NULL_FLOAT`

```rust
pub const NULL_FLOAT: &str = "\0\0\0\0";
```

#### Constant `NULL_DOUBLE`

```rust
pub const NULL_DOUBLE: &str = "\0\0\0\0\0\0\0\0";
```

#### Constant `NULL_BOOL`

```rust
pub const NULL_BOOL: &str = "\0";
```

## Module `types`

**Attributes:**

- `Other("#[allow(unsafe_code, dead_code, non_camel_case_types, non_snake_case, unused)]")`

Core types and traits for the HPVMx system.

This module defines fundamental structures like points, rectangles,
vectors, and matrices, as well as hardware-related traits like `Peripheral`.

```rust
pub(crate) mod types { /* ... */ }
```

### Modules

## Module `net`

```rust
pub mod net { /* ... */ }
```

### Modules

## Module `phy`

```rust
pub mod phy { /* ... */ }
```

### Modules

## Module `reg`

```rust
pub mod reg { /* ... */ }
```

### Types

#### Struct `C22`

A single MDIO clause 22 register address (5 bits).

```rust
pub struct C22(pub(in ::types::net::phy::reg) u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub const fn vendor_specific<const N: u8>() -> Self { /* ... */ }
  ```
  Creates a new instance of `C22` with a vendor specific register.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> C22 { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Mmd`

A single MDIO clause 45 register device and address.

```rust
pub struct Mmd(pub(in ::types::net::phy::reg) u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Mmd { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `C45`

A single MDIO clause 45 register device and address.

Clause 45 uses a 5-bit device address to access a specific MMD within
a port, then a 16-bit register address to access a location within
that device. `C45` represents this by storing a [`Mmd`] and
a register number.

```rust
pub struct C45 {
    pub(in ::types::net::phy::reg) devad: Mmd,
    pub(in ::types::net::phy::reg) regnum: u16,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `devad` | `Mmd` |  |
| `regnum` | `u16` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(devad: Mmd, regnum: u16) -> Self { /* ... */ }
  ```
  Creates a new instance of `C45`.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Traits

#### Trait `Register`

```rust
pub trait Register {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `read`: Reads a PHY register.
- `write`: Writes a PHY register.
- `read_status`: Checks the link status and updates current link state.

### Types

#### Struct `MDIODeviceID`

```rust
pub struct MDIODeviceID {
    pub(in ::types::net::phy) phy_id: u32,
    pub(in ::types::net::phy) phy_id_mask: u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `phy_id` | `u32` |  |
| `phy_id_mask` | `u32` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(phy_id: u32, phy_id_mask: u32) -> MDIODeviceID { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `PhyDeviceId`

```rust
pub struct PhyDeviceId(pub(in ::types::net::phy) MDIODeviceID);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `MDIODeviceID` |  |

##### Implementations

###### Methods

- ```rust
  pub const fn new_with_exact_mask(id: u32) -> Self { /* ... */ }
  ```
  Creates a new instance with the exact match mask.

- ```rust
  pub const fn new_with_model_mask(id: u32) -> Self { /* ... */ }
  ```
  Creates a new instance with the model match mask.

- ```rust
  pub const fn new_with_vendor_mask(id: u32) -> Self { /* ... */ }
  ```
  Creates a new instance with the vendor match mask.

- ```rust
  pub const fn new_with_custom_mask(id: u32, mask: u32) -> Self { /* ... */ }
  ```
  Creates a new instance with a custom match mask.

- ```rust
  pub const fn new_with_driver<T: Driver>() -> Self { /* ... */ }
  ```
  Creates a new instance from [`Driver`].

- ```rust
  pub const fn id(self: &Self) -> u32 { /* ... */ }
  ```
  Get the MDIO device's PHY ID.

- ```rust
  pub const fn mask_as_int(self: &Self) -> u32 { /* ... */ }
  ```
  Get the MDIO device's match mask.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `DeviceState`

```rust
pub enum DeviceState {
    Down,
    Ready,
    Halted,
    Error,
    Up,
    Running,
    NoLink,
    CableTest,
}
```

##### Variants

###### `Down`

PHY device and driver are not ready for anything.

###### `Ready`

PHY is ready to send and receive packets.

###### `Halted`

PHY is up, but no polling or interrupts are done.

###### `Error`

PHY is up, but is in an error state.

###### `Up`

PHY and attached device are ready to do work.

###### `Running`

PHY is currently running.

###### `NoLink`

PHY is up, but not currently plugged in.

###### `CableTest`

PHY is performing a cable test.

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DeviceState) -> bool { /* ... */ }
    ```

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `DuplexMode`

A mode of Ethernet communication.

PHY drivers get duplex information from hardware and update the current state.

```rust
pub enum DuplexMode {
    Full,
    Half,
    Unknown,
}
```

##### Variants

###### `Full`

PHY is in full-duplex mode.

###### `Half`

PHY is in half-duplex mode.

###### `Unknown`

PHY is in unknown duplex mode.

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `DeviceMask`

```rust
pub(in ::types::net::phy) enum DeviceMask {
    Exact,
    Model,
    Vendor,
    Custom(u32),
}
```

##### Variants

###### `Exact`

###### `Model`

###### `Vendor`

###### `Custom`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u32` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types::net::phy) const fn as_int(self: &Self) -> u32 { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `esm`

```rust
pub struct esm {
    pub(in ::types::net::phy) get: u8,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `get` | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types::net::phy) fn get(self: &Self) -> u8 { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Device`

```rust
pub struct Device {
    pub(in ::types::net::phy) i: u32,
    pub(in ::types::net::phy) phyindex: u32,
    pub(in ::types::net::phy) phy_device_id: u32,
    pub(in ::types::net::phy) e: esm,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `i` | `u32` |  |
| `phyindex` | `u32` |  |
| `phy_device_id` | `u32` |  |
| `e` | `esm` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Traits

#### Trait `Driver`

```rust
pub trait Driver {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Constants

- `NAME`: The friendly name of this PHY type.

##### Provided Methods

- ```rust
  fn soft_reset(_dev: &mut Device) -> Result<(), ()> { /* ... */ }
  ```
  Issues a PHY software reset.

- ```rust
  fn probe(_dev: &mut Device) -> Result<(), ()> { /* ... */ }
  ```
  Sets up device-specific structures during discovery.

- ```rust
  fn suspend(_dev: &mut Device) -> Result<(), ()> { /* ... */ }
  ```

- ```rust
  fn resume(_dev: &mut Device) -> Result<(), ()> { /* ... */ }
  ```

### Types

#### Type Alias `BYTE`

```rust
pub type BYTE = u8;
```

#### Type Alias `WORD`

```rust
pub type WORD = u16;
```

#### Type Alias `DWORD`

```rust
pub type DWORD = u32;
```

#### Type Alias `QWORD`

```rust
pub type QWORD = u64;
```

#### Type Alias `DOUBLE`

```rust
pub type DOUBLE = f64;
```

#### Type Alias `FLOAT`

```rust
pub type FLOAT = f32;
```

#### Type Alias `BOOL`

```rust
pub type BOOL = bool;
```

#### Type Alias `CHAR`

```rust
pub type CHAR = char;
```

#### Type Alias `SHORT`

```rust
pub type SHORT = i16;
```

#### Type Alias `INT`

```rust
pub type INT = i32;
```

#### Type Alias `LONG`

```rust
pub type LONG = i64;
```

#### Type Alias `UCHAR`

```rust
pub type UCHAR = u8;
```

#### Type Alias `USHORT`

```rust
pub type USHORT = u16;
```

#### Type Alias `UINT`

```rust
pub type UINT = u32;
```

#### Type Alias `ULONG`

```rust
pub type ULONG = u64;
```

#### Type Alias `SCHAR`

```rust
pub type SCHAR = i8;
```

#### Type Alias `ProgramCall`

```rust
pub type ProgramCall = u8;
```

#### Type Alias `ADDR8`

```rust
pub type ADDR8 = u8;
```

#### Type Alias `ADDR16`

```rust
pub type ADDR16 = u16;
```

#### Type Alias `ADDR32`

```rust
pub type ADDR32 = u32;
```

#### Type Alias `ADDR64`

```rust
pub type ADDR64 = u64;
```

#### Type Alias `POINTER`

```rust
pub type POINTER = *mut u8;
```

#### Type Alias `SIZE`

```rust
pub type SIZE = usize;
```

#### Type Alias `INDEX`

```rust
pub type INDEX = isize;
```

#### Type Alias `STR`

```rust
pub type STR = alloc::string::String;
```

#### Type Alias `VECTOR`

```rust
pub type VECTOR = alloc::vec::Vec<u8>;
```

#### Type Alias `VECTOR2`

```rust
pub type VECTOR2 = alloc::vec::Vec<u16>;
```

#### Type Alias `VECTOR4`

```rust
pub type VECTOR4 = alloc::vec::Vec<u32>;
```

#### Type Alias `VECTOR8`

```rust
pub type VECTOR8 = alloc::vec::Vec<u64>;
```

#### Type Alias `VECTOR16`

```rust
pub type VECTOR16 = alloc::vec::Vec<u128>;
```

#### Type Alias `VECTOR32`

```rust
pub type VECTOR32 = alloc::vec::Vec<u32>;
```

#### Type Alias `VECTOR64`

```rust
pub type VECTOR64 = alloc::vec::Vec<u64>;
```

#### Type Alias `VECTOR128`

```rust
pub type VECTOR128 = alloc::vec::Vec<u128>;
```

#### Type Alias `ARRAY`

```rust
pub type ARRAY = [u8; 16];
```

#### Type Alias `ARRAY2`

```rust
pub type ARRAY2 = [u16; 8];
```

#### Type Alias `ARRAY4`

```rust
pub type ARRAY4 = [u32; 4];
```

#### Type Alias `ARRAY8`

```rust
pub type ARRAY8 = [u64; 2];
```

#### Type Alias `ARRAY16`

```rust
pub type ARRAY16 = [u128; 1];
```

#### Type Alias `ARRAY32`

```rust
pub type ARRAY32 = [u32; 2];
```

#### Type Alias `ARRAY64`

```rust
pub type ARRAY64 = [u64; 4];
```

#### Type Alias `ARRAY128`

```rust
pub type ARRAY128 = [u128; 2];
```

#### Type Alias `ARRAY256`

```rust
pub type ARRAY256 = [u128; 4];
```

#### Type Alias `ARRAY512`

```rust
pub type ARRAY512 = [u128; 8];
```

#### Type Alias `TREEMAP`

```rust
pub type TREEMAP<K, V> = alloc::collections::BTreeMap<K, V>;
```

#### Type Alias `TREESET`

```rust
pub type TREESET<T> = alloc::collections::BTreeSet<T>;
```

#### Type Alias `TUPLE`

```rust
pub type TUPLE = (u8, u16, u32, u64, u128);
```

#### Type Alias `TUPLE2`

```rust
pub type TUPLE2 = (u16, u32, u64, u128);
```

#### Type Alias `TUPLE4`

```rust
pub type TUPLE4 = (u32, u64, u128);
```

#### Type Alias `TUPLE8`

```rust
pub type TUPLE8 = (u64, u128);
```

#### Type Alias `TUPLE16`

```rust
pub type TUPLE16 = (u128);
```

#### Type Alias `TUPLE32`

```rust
pub type TUPLE32 = (u32, u64, u128);
```

#### Type Alias `TUPLE64`

```rust
pub type TUPLE64 = (u64, u128);
```

#### Type Alias `TUPLE128`

```rust
pub type TUPLE128 = (u128, u128);
```

#### Type Alias `TUPLE256`

```rust
pub type TUPLE256 = (u128, u128, u128);
```

#### Type Alias `TUPLE512`

```rust
pub type TUPLE512 = (u128, u128, u128, u128);
```

#### Type Alias `HWADDR`

```rust
pub type HWADDR = [u8; 6];
```

#### Type Alias `MEMADDR`

```rust
pub type MEMADDR = [u8; 16];
```

#### Type Alias `PCIEADDR`

```rust
pub type PCIEADDR = [u8; 16];
```

#### Type Alias `VADDR`

```rust
pub type VADDR = [u8; 16];
```

#### Type Alias `PADDR`

```rust
pub type PADDR = [u8; 16];
```

#### Type Alias `ADDR128`

```rust
pub type ADDR128 = u128;
```

#### Type Alias `ADDR256`

```rust
pub type ADDR256 = u128;
```

#### Type Alias `ADDR512`

```rust
pub type ADDR512 = u128;
```

#### Type Alias `ADDR1024`

```rust
pub type ADDR1024 = u128;
```

#### Type Alias `ADDR2048`

```rust
pub type ADDR2048 = u128;
```

#### Type Alias `ADDR4096`

```rust
pub type ADDR4096 = u128;
```

#### Type Alias `ADDR8192`

```rust
pub type ADDR8192 = u128;
```

#### Type Alias `ENUM`

```rust
pub type ENUM = u32;
```

#### Type Alias `BIT`

```rust
pub type BIT = u8;
```

#### Type Alias `BIT2`

```rust
pub type BIT2 = u8;
```

#### Type Alias `NIBBLE`

```rust
pub type NIBBLE = u8;
```

#### Type Alias `BYTEARRAY`

```rust
pub type BYTEARRAY = [u8; 16];
```

#### Type Alias `WORDARRAY`

```rust
pub type WORDARRAY = [u16; 8];
```

#### Type Alias `DWORDARRAY`

```rust
pub type DWORDARRAY = [u32; 4];
```

#### Type Alias `QWORDARRAY`

```rust
pub type QWORDARRAY = [u64; 2];
```

#### Type Alias `FLOATARRAY`

```rust
pub type FLOATARRAY = [f32; 4];
```

#### Type Alias `DOUBLEARRAY`

```rust
pub type DOUBLEARRAY = [f64; 2];
```

#### Type Alias `BOOLARRAY`

```rust
pub type BOOLARRAY = [bool; 8];
```

#### Type Alias `CHARARRAY`

```rust
pub type CHARARRAY = [char; 16];
```

#### Type Alias `SHORTARRAY`

```rust
pub type SHORTARRAY = [i16; 8];
```

#### Type Alias `INTARRAY`

```rust
pub type INTARRAY = [i32; 4];
```

#### Type Alias `LONGARRAY`

```rust
pub type LONGARRAY = [i64; 2];
```

#### Type Alias `UCHARARRAY`

```rust
pub type UCHARARRAY = [u8; 16];
```

#### Type Alias `USHORTARRAY`

```rust
pub type USHORTARRAY = [u16; 8];
```

#### Type Alias `UINTARRAY`

```rust
pub type UINTARRAY = [u32; 4];
```

#### Type Alias `ULONGARRAY`

```rust
pub type ULONGARRAY = [u64; 2];
```

#### Type Alias `SCHARARRAY`

```rust
pub type SCHARARRAY = [i8; 16];
```

#### Type Alias `BYTEARRAY2`

```rust
pub type BYTEARRAY2 = [u8; 32];
```

#### Type Alias `WORDARRAY2`

```rust
pub type WORDARRAY2 = [u16; 16];
```

#### Type Alias `DWORDARRAY2`

```rust
pub type DWORDARRAY2 = [u32; 8];
```

#### Type Alias `QWORDARRAY2`

```rust
pub type QWORDARRAY2 = [u64; 4];
```

#### Type Alias `FLOATARRAY2`

```rust
pub type FLOATARRAY2 = [f32; 8];
```

#### Type Alias `DOUBLEARRAY2`

```rust
pub type DOUBLEARRAY2 = [f64; 4];
```

#### Type Alias `BOOLARRAY2`

```rust
pub type BOOLARRAY2 = [bool; 16];
```

#### Type Alias `CHARARRAY2`

```rust
pub type CHARARRAY2 = [char; 32];
```

#### Type Alias `SHORTARRAY2`

```rust
pub type SHORTARRAY2 = [i16; 16];
```

#### Type Alias `INTARRAY2`

```rust
pub type INTARRAY2 = [i32; 8];
```

#### Type Alias `LONGARRAY2`

```rust
pub type LONGARRAY2 = [i64; 4];
```

#### Type Alias `INT2`

```rust
pub type INT2 = [i32; 2];
```

#### Type Alias `INT4`

```rust
pub type INT4 = [i32; 4];
```

#### Type Alias `INT8`

```rust
pub type INT8 = [i32; 8];
```

#### Type Alias `INT16`

```rust
pub type INT16 = [i32; 16];
```

#### Type Alias `INT32`

```rust
pub type INT32 = [i32; 32];
```

#### Type Alias `INT64`

```rust
pub type INT64 = [i32; 64];
```

#### Type Alias `INT128`

```rust
pub type INT128 = [i32; 128];
```

#### Type Alias `UINT2`

```rust
pub type UINT2 = [u32; 2];
```

#### Type Alias `UINT4`

```rust
pub type UINT4 = [u32; 4];
```

#### Type Alias `UINT8`

```rust
pub type UINT8 = [u32; 8];
```

#### Type Alias `UINT16`

```rust
pub type UINT16 = [u32; 16];
```

#### Type Alias `UINT32`

```rust
pub type UINT32 = [u32; 32];
```

#### Type Alias `UINT64`

```rust
pub type UINT64 = [u32; 64];
```

#### Type Alias `UINT128`

```rust
pub type UINT128 = [u32; 128];
```

#### Type Alias `FLOAT2`

```rust
pub type FLOAT2 = [f32; 2];
```

#### Type Alias `FLOAT4`

```rust
pub type FLOAT4 = [f32; 4];
```

#### Type Alias `FLOAT8`

```rust
pub type FLOAT8 = [f32; 8];
```

#### Type Alias `FLOAT16`

```rust
pub type FLOAT16 = [f32; 16];
```

#### Type Alias `FLOAT32`

```rust
pub type FLOAT32 = [f32; 32];
```

#### Type Alias `FLOAT64`

```rust
pub type FLOAT64 = [f32; 64];
```

#### Type Alias `FLOAT128`

```rust
pub type FLOAT128 = [f32; 128];
```

#### Type Alias `DOUBLE2`

```rust
pub type DOUBLE2 = [f64; 2];
```

#### Type Alias `DOUBLE4`

```rust
pub type DOUBLE4 = [f64; 4];
```

#### Type Alias `DOUBLE8`

```rust
pub type DOUBLE8 = [f64; 8];
```

#### Type Alias `DOUBLE16`

```rust
pub type DOUBLE16 = [f64; 16];
```

#### Type Alias `DOUBLE32`

```rust
pub type DOUBLE32 = [f64; 32];
```

#### Type Alias `DOUBLE64`

```rust
pub type DOUBLE64 = [f64; 64];
```

#### Type Alias `DOUBLE128`

```rust
pub type DOUBLE128 = [f64; 128];
```

#### Type Alias `BOOL2`

```rust
pub type BOOL2 = [bool; 2];
```

#### Type Alias `BOOL4`

```rust
pub type BOOL4 = [bool; 4];
```

#### Type Alias `BOOL8`

```rust
pub type BOOL8 = [bool; 8];
```

#### Type Alias `BOOL16`

```rust
pub type BOOL16 = [bool; 16];
```

#### Type Alias `BOOL32`

```rust
pub type BOOL32 = [bool; 32];
```

#### Type Alias `BOOL64`

```rust
pub type BOOL64 = [bool; 64];
```

#### Type Alias `BOOL128`

```rust
pub type BOOL128 = [bool; 128];
```

#### Type Alias `CHAR2`

```rust
pub type CHAR2 = [char; 2];
```

#### Type Alias `CHAR4`

```rust
pub type CHAR4 = [char; 4];
```

#### Type Alias `CHAR8`

```rust
pub type CHAR8 = [char; 8];
```

#### Type Alias `CHAR16`

```rust
pub type CHAR16 = [char; 16];
```

#### Type Alias `CHAR32`

```rust
pub type CHAR32 = [char; 32];
```

#### Type Alias `SHORT2`

```rust
pub type SHORT2 = [i16; 2];
```

#### Type Alias `SHORT4`

```rust
pub type SHORT4 = [i16; 4];
```

#### Type Alias `SHORT8`

```rust
pub type SHORT8 = [i16; 8];
```

#### Type Alias `SHORT16`

```rust
pub type SHORT16 = [i16; 16];
```

#### Type Alias `INT2ARRAY2`

```rust
pub type INT2ARRAY2 = [i32; 4];
```

#### Type Alias `INT4ARRAY2`

```rust
pub type INT4ARRAY2 = [i32; 8];
```

#### Type Alias `INT8ARRAY2`

```rust
pub type INT8ARRAY2 = [i32; 16];
```

#### Type Alias `INT16ARRAY2`

```rust
pub type INT16ARRAY2 = [i32; 32];
```

#### Type Alias `INT32ARRAY2`

```rust
pub type INT32ARRAY2 = [i32; 64];
```

#### Type Alias `INT64ARRAY2`

```rust
pub type INT64ARRAY2 = [i32; 128];
```

#### Type Alias `INT128ARRAY2`

```rust
pub type INT128ARRAY2 = [i32; 256];
```

#### Type Alias `UINT2ARRAY2`

```rust
pub type UINT2ARRAY2 = [u32; 4];
```

#### Type Alias `UINT4ARRAY2`

```rust
pub type UINT4ARRAY2 = [u32; 8];
```

#### Type Alias `UINT8ARRAY2`

```rust
pub type UINT8ARRAY2 = [u32; 16];
```

#### Type Alias `UINT16ARRAY2`

```rust
pub type UINT16ARRAY2 = [u32; 32];
```

#### Type Alias `UINT32ARRAY2`

```rust
pub type UINT32ARRAY2 = [u32; 64];
```

#### Type Alias `UINT64ARRAY2`

```rust
pub type UINT64ARRAY2 = [u32; 128];
```

#### Type Alias `UINT128ARRAY2`

```rust
pub type UINT128ARRAY2 = [u32; 256];
```

#### Type Alias `FLOAT2ARRAY2`

```rust
pub type FLOAT2ARRAY2 = [f32; 4];
```

#### Type Alias `FLOAT4ARRAY2`

```rust
pub type FLOAT4ARRAY2 = [f32; 8];
```

#### Type Alias `FLOAT8ARRAY2`

```rust
pub type FLOAT8ARRAY2 = [f32; 16];
```

#### Type Alias `FLOAT16ARRAY2`

```rust
pub type FLOAT16ARRAY2 = [f32; 32];
```

#### Type Alias `FLOAT32ARRAY2`

```rust
pub type FLOAT32ARRAY2 = [f32; 64];
```

#### Type Alias `FLOAT64ARRAY2`

```rust
pub type FLOAT64ARRAY2 = [f32; 128];
```

#### Type Alias `FLOAT128ARRAY2`

```rust
pub type FLOAT128ARRAY2 = [f32; 256];
```

#### Type Alias `DOUBLE2ARRAY2`

```rust
pub type DOUBLE2ARRAY2 = [f64; 4];
```

#### Type Alias `DOUBLE4ARRAY2`

```rust
pub type DOUBLE4ARRAY2 = [f64; 8];
```

#### Type Alias `DOUBLE8ARRAY2`

```rust
pub type DOUBLE8ARRAY2 = [f64; 16];
```

#### Type Alias `DOUBLE16ARRAY2`

```rust
pub type DOUBLE16ARRAY2 = [f64; 32];
```

#### Type Alias `DOUBLE32ARRAY2`

```rust
pub type DOUBLE32ARRAY2 = [f64; 64];
```

#### Type Alias `DOUBLE64ARRAY2`

```rust
pub type DOUBLE64ARRAY2 = [f64; 128];
```

#### Type Alias `DOUBLE128ARRAY2`

```rust
pub type DOUBLE128ARRAY2 = [f64; 256];
```

#### Type Alias `BOOL2ARRAY2`

```rust
pub type BOOL2ARRAY2 = [bool; 4];
```

#### Type Alias `BOOL4ARRAY2`

```rust
pub type BOOL4ARRAY2 = [bool; 8];
```

#### Type Alias `BOOL8ARRAY2`

```rust
pub type BOOL8ARRAY2 = [bool; 16];
```

#### Type Alias `BOOL16ARRAY2`

```rust
pub type BOOL16ARRAY2 = [bool; 32];
```

#### Type Alias `BOOL32ARRAY2`

```rust
pub type BOOL32ARRAY2 = [bool; 64];
```

#### Type Alias `BOOL64ARRAY2`

```rust
pub type BOOL64ARRAY2 = [bool; 128];
```

#### Type Alias `BOOL128ARRAY2`

```rust
pub type BOOL128ARRAY2 = [bool; 256];
```

#### Type Alias `CHAR2ARRAY2`

```rust
pub type CHAR2ARRAY2 = [char; 4];
```

#### Type Alias `CHAR4ARRAY2`

```rust
pub type CHAR4ARRAY2 = [char; 8];
```

#### Type Alias `CHAR8ARRAY2`

```rust
pub type CHAR8ARRAY2 = [char; 16];
```

#### Type Alias `CHAR16ARRAY2`

```rust
pub type CHAR16ARRAY2 = [char; 32];
```

#### Type Alias `SHORT2ARRAY2`

```rust
pub type SHORT2ARRAY2 = [i16; 4];
```

#### Type Alias `SHORT4ARRAY2`

```rust
pub type SHORT4ARRAY2 = [i16; 8];
```

#### Type Alias `SHORT8ARRAY2`

```rust
pub type SHORT8ARRAY2 = [i16; 16];
```

#### Type Alias `SHORT16ARRAY2`

```rust
pub type SHORT16ARRAY2 = [i16; 32];
```

#### Type Alias `IPV4`

```rust
pub type IPV4 = u32;
```

#### Type Alias `IPV6`

```rust
pub type IPV6 = u64;
```

#### Type Alias `PORT`

```rust
pub type PORT = u16;
```

#### Type Alias `IPADDR`

```rust
pub type IPADDR = alloc::string::String;
```

#### Type Alias `MACADDR`

```rust
pub type MACADDR = alloc::string::String;
```

#### Struct `RIG`

```rust
pub struct RIG {
    pub(in ::types) mem: [u64; 17293822569102704640],
    pub(in ::types) debugflags: i32,
    pub(in ::types) slots: [Option<alloc::boxed::Box<dyn Peripheral>>; 16],
    pub(in ::types) nreads: u16,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `mem` | `[u64; 17293822569102704640]` |  |
| `debugflags` | `i32` |  |
| `slots` | `[Option<alloc::boxed::Box<dyn Peripheral>>; 16]` |  |
| `nreads` | `u16` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Type Alias `Interrrupt`

```rust
pub type Interrrupt = u16;
```

#### Type Alias `Buffer`

```rust
pub type Buffer = u16;
```

#### Type Alias `Poly`

```rust
pub type Poly = u16;
```

#### Type Alias `DMI`

```rust
pub type DMI = u8;
```

#### Type Alias `PtrDiffT`

```rust
pub type PtrDiffT = i32;
```

#### Type Alias `Enum`

```rust
pub type Enum = u32;
```

#### Type Alias `Boolean`

```rust
pub type Boolean = u8;
```

#### Type Alias `Pointer`

```rust
pub type Pointer = *mut u8;
```

#### Type Alias `Size`

```rust
pub type Size = usize;
```

#### Type Alias `Index`

```rust
pub type Index = isize;
```

#### Type Alias `Char`

```rust
pub type Char = char;
```

#### Type Alias `Bool`

```rust
pub type Bool = bool;
```

#### Type Alias `Float`

```rust
pub type Float = f64;
```

#### Type Alias `Int`

```rust
pub type Int = i64;
```

#### Type Alias `UInt`

```rust
pub type UInt = u64;
```

#### Type Alias `Str`

```rust
pub type Str = alloc::string::String;
```

#### Type Alias `Vect`

```rust
pub type Vect = alloc::vec::Vec<u8>;
```

#### Struct `Point`

A 2D point with double-precision coordinates.

```rust
pub struct Point {
    pub(in ::types) x: f64,
    pub(in ::types) y: f64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `x` | `f64` |  |
| `y` | `f64` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn origin() -> Point { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn new(x: f64, y: f64) -> Point { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn distance(self: &Self, other: &Point) -> f64 { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn dot(self: &Self, other: &Point) -> f64 { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn cross(self: &Self, other: &Point) -> f64 { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Rect`

A rectangle defined by its position and size.

```rust
pub struct Rect {
    pub(in ::types) p1: Point,
    pub(in ::types) p2: Point,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `p1` | `Point` |  |
| `p2` | `Point` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn area(self: &Self) -> f64 { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn perimeter(self: &Self) -> f64 { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn translate(self: &mut Self, x: f64, y: f64) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Pair`

```rust
pub struct Pair(pub(in ::types) alloc::boxed::Box<i32>, pub(in ::types) alloc::boxed::Box<i32>);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::boxed::Box<i32>` |  |
| 1 | `alloc::boxed::Box<i32>` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn new(first: i32, second: i32) -> Pair { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn sum(self: &Self) -> i32 { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn product(self: &Self) -> i32 { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn destroy(self: Self) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Vec1`

```rust
pub struct Vec1 {
    pub(in ::types) x: f64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `x` | `f64` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn new(x: f64) -> Vec1 { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn distance(self: &Self, other: &Vec1) -> f64 { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn transform(self: &Self, mat: &Mat4) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Vec2`

```rust
pub struct Vec2 {
    pub(in ::types) x: f64,
    pub(in ::types) y: f64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `x` | `f64` |  |
| `y` | `f64` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn new(x: f64, y: f64) -> Vec2 { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn distance(self: &Self, other: &Vec2) -> f64 { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn transform(self: &Self, mat: &Mat4) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Vec3`

```rust
pub struct Vec3 {
    pub(in ::types) x: f64,
    pub(in ::types) y: f64,
    pub(in ::types) z: f64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `x` | `f64` |  |
| `y` | `f64` |  |
| `z` | `f64` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn new(x: f64, y: f64, z: f64) -> Vec3 { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn distance(self: &Self, other: &Vec3) -> f64 { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn transform(self: &Self, mat: &Mat4) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Vec4`

```rust
pub struct Vec4 {
    pub(in ::types) x: f64,
    pub(in ::types) y: f64,
    pub(in ::types) z: f64,
    pub(in ::types) w: f64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `x` | `f64` |  |
| `y` | `f64` |  |
| `z` | `f64` |  |
| `w` | `f64` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn new(x: f64, y: f64, z: f64, w: f64) -> Vec4 { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn distance(self: &Self, other: &Vec4) -> f64 { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn transform(self: &Self, mat: &Mat4) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Uniform`

```rust
pub struct Uniform {
    pub(in ::types) pos: Vec2,
    pub(in ::types) scale: Vec2,
    pub(in ::types) rot: f64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `pos` | `Vec2` |  |
| `scale` | `Vec2` |  |
| `rot` | `f64` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn new(pos: Vec2, scale: Vec2, rot: f64) -> Uniform { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Linear`

```rust
pub struct Linear {
    pub(in ::types) pos: Vec2,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `pos` | `Vec2` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn new(pos: Vec2) -> Linear { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Vector`

```rust
pub struct Vector {
    pub(in ::types) pos: Vec2,
    pub(in ::types) scale: Vec2,
    pub(in ::types) rot: f64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `pos` | `Vec2` |  |
| `scale` | `Vec2` |  |
| `rot` | `f64` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn new(pos: Vec2, scale: Vec2, rot: f64) -> Vector { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Matrix`

```rust
pub struct Matrix {
    pub(in ::types) pos: Vec2,
    pub(in ::types) scale: Vec2,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `pos` | `Vec2` |  |
| `scale` | `Vec2` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn new(pos: Vec2, scale: Vec2) -> Matrix { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Scalar`

```rust
pub struct Scalar {
    pub(in ::types) pos: Vec2,
    pub(in ::types) scale: Vec2,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `pos` | `Vec2` |  |
| `scale` | `Vec2` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn new(pos: Vec2, scale: Vec2) -> Scalar { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Mat2`

```rust
pub struct Mat2 {
    pub(in ::types) m: [[f64; 2]; 2],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `m` | `[[f64; 2]; 2]` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn identity() -> Mat2 { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn new(m: [[f64; 2]; 2]) -> Mat2 { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn shift(self: &Self, x: f64, y: f64) -> Mat2 { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Mat3`

```rust
pub struct Mat3 {
    pub(in ::types) m: [[f64; 3]; 3],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `m` | `[[f64; 3]; 3]` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn identity() -> Mat3 { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn new(m: [[f64; 3]; 3]) -> Mat3 { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn shift(self: &Self, x: f64, y: f64, z: f64) -> Mat3 { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Mat4`

```rust
pub struct Mat4 {
    pub(in ::types) m: [[f64; 4]; 4],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `m` | `[[f64; 4]; 4]` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn identity() -> Mat4 { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn new(m: [[f64; 4]; 4]) -> Mat4 { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn shift(self: &Self, x: f64, y: f64, z: f64, w: f64) -> Mat4 { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Frame`

```rust
pub struct Frame {
    pub(in ::types) frame: alloc::string::String,
    pub(in ::types) pos: Vec2,
    pub(in ::types) width: f64,
    pub(in ::types) height: f64,
    pub(in ::types) time: f64,
    pub(in ::types) data: alloc::vec::Vec<u8>,
    pub(in ::types) mat: Mat4,
    pub(in ::types) rect: Rect,
    pub(in ::types) pair: Pair,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `frame` | `alloc::string::String` |  |
| `pos` | `Vec2` |  |
| `width` | `f64` |  |
| `height` | `f64` |  |
| `time` | `f64` |  |
| `data` | `alloc::vec::Vec<u8>` |  |
| `mat` | `Mat4` |  |
| `rect` | `Rect` |  |
| `pair` | `Pair` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn new(frame: String, pos: Vec2, width: f64, height: f64, time: f64, data: Vec<u8>, mat: Mat4, rect: Rect, pair: Pair) -> Frame { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn append(self: &mut Self, frame: Frame) { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn clear(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn destroy(self: Self, frame: Frame) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Error`

```rust
pub struct Error {
    pub(in ::types) code: i32,
    pub(in ::types) message: alloc::string::String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `code` | `i32` |  |
| `message` | `alloc::string::String` |  |

##### Implementations

###### Methods

- ```rust
  pub(crate) fn new(code: i32, message: String) -> Error { /* ... */ }
  ```

- ```rust
  pub(crate) fn print(self: &Self) { /* ... */ }
  ```

- ```rust
  pub fn error(self: &Self) { /* ... */ }
  ```

- ```rust
  pub fn cerror(self: &Self) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Warning`

```rust
pub struct Warning {
    pub(in ::types) code: i32,
    pub(in ::types) message: alloc::string::String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `code` | `i32` |  |
| `message` | `alloc::string::String` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn new(code: i32, message: String) -> Warning { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn print(self: &Self) { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn warning(self: &Self) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Object`

```rust
pub struct Object {
    pub(in ::types) name: alloc::string::String,
    pub(in ::types) data: alloc::string::String,
    pub(in ::types) loc: u16,
    pub(in ::types) size: usize,
    pub(in ::types) type_: u32,
    pub(in ::types) parent: Option<alloc::boxed::Box<Object>>,
    pub(in ::types) children: alloc::vec::Vec<alloc::boxed::Box<Object>>,
    pub(in ::types) next: Option<alloc::boxed::Box<Object>>,
    pub(in ::types) prev: Option<alloc::boxed::Box<Object>>,
    pub(in ::types) first: Option<alloc::boxed::Box<Object>>,
    pub(in ::types) last: Option<alloc::boxed::Box<Object>>,
    pub(in ::types) flags: u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `alloc::string::String` |  |
| `data` | `alloc::string::String` |  |
| `loc` | `u16` |  |
| `size` | `usize` |  |
| `type_` | `u32` |  |
| `parent` | `Option<alloc::boxed::Box<Object>>` |  |
| `children` | `alloc::vec::Vec<alloc::boxed::Box<Object>>` |  |
| `next` | `Option<alloc::boxed::Box<Object>>` |  |
| `prev` | `Option<alloc::boxed::Box<Object>>` |  |
| `first` | `Option<alloc::boxed::Box<Object>>` |  |
| `last` | `Option<alloc::boxed::Box<Object>>` |  |
| `flags` | `u32` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn new(name: String, data: String, loc: u16, size: usize, type_: u32, parent: Option<Box<Object>>, children: Vec<Box<Object>>, next: Option<Box<Object>>, prev: Option<Box<Object>>, first: Option<Box<Object>>, last: Option<Box<Object>>, flags: u32) -> Object { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn repr(self: &Self) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Param`

```rust
pub struct Param {
    pub(in ::types) name: alloc::string::String,
    pub(in ::types) data: Object,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `alloc::string::String` |  |
| `data` | `Object` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn new(name: String, data: Object) -> Param { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Scene`

```rust
pub struct Scene {
    pub(in ::types) name: alloc::string::String,
    pub(in ::types) params: alloc::vec::Vec<Param>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `alloc::string::String` |  |
| `params` | `alloc::vec::Vec<Param>` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn new(name: String, params: Vec<Param>) -> Scene { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Polygon`

```rust
pub struct Polygon {
    pub(in ::types) points: alloc::vec::Vec<(Vertex, Vertex, Vertex)>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `points` | `alloc::vec::Vec<(Vertex, Vertex, Vertex)>` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn new(points: Vec<(Vertex, Vertex, Vertex)>) -> Polygon { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn num_points(self: &Self) -> usize { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn add_point(self: &mut Self, point: (Vertex, Vertex, Vertex)) { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn remove_point(self: &mut Self, index: usize) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Vertex`

```rust
pub struct Vertex {
    pub(in ::types) pos: Vec2,
    pub(in ::types) tex: Vec2,
    pub(in ::types) col: Vec3,
    pub(in ::types) norm: Vec3,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `pos` | `Vec2` |  |
| `tex` | `Vec2` |  |
| `col` | `Vec3` |  |
| `norm` | `Vec3` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn new(pos: Vec2, tex: Vec2, col: Vec3, norm: Vec3) -> Vertex { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn transform(self: &Self, mat: &Mat4) { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn transform_mut(self: &mut Self, mat: &Mat4) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `VecOp`

```rust
pub struct VecOp;
```

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn new() -> VecOp { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn create(self: &Self, typ: String) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Type Alias `Exception`

```rust
pub type Exception = Error;
```

#### Type Alias `ArithmeticError`

```rust
pub type ArithmeticError = Error;
```

#### Type Alias `AssertionError`

```rust
pub type AssertionError = Error;
```

#### Type Alias `AttributeError`

```rust
pub type AttributeError = Error;
```

#### Type Alias `WindowsError`

```rust
pub type WindowsError = Error;
```

#### Type Alias `OSError`

```rust
pub type OSError = Error;
```

#### Type Alias `IOError`

```rust
pub type IOError = Error;
```

#### Type Alias `EnvironmentError`

```rust
pub type EnvironmentError = Error;
```

#### Type Alias `BlockingIOError`

```rust
pub type BlockingIOError = Error;
```

#### Type Alias `ConnectionError`

```rust
pub type ConnectionError = Error;
```

#### Type Alias `BrokenPipeError`

```rust
pub type BrokenPipeError = Error;
```

#### Type Alias `BufferError`

```rust
pub type BufferError = Error;
```

#### Type Alias `ChildProcessError`

```rust
pub type ChildProcessError = Error;
```

#### Type Alias `ConnectionAbortedError`

```rust
pub type ConnectionAbortedError = Error;
```

#### Type Alias `ConnectionRefusedError`

```rust
pub type ConnectionRefusedError = Error;
```

#### Type Alias `ConnectionResetError`

```rust
pub type ConnectionResetError = Error;
```

#### Type Alias `EOFError`

```rust
pub type EOFError = Error;
```

#### Type Alias `FileExistsError`

```rust
pub type FileExistsError = Error;
```

#### Type Alias `FileNotFoundError`

```rust
pub type FileNotFoundError = Error;
```

#### Type Alias `FloatingPointError`

```rust
pub type FloatingPointError = Error;
```

#### Type Alias `SyntaxError`

```rust
pub type SyntaxError = Error;
```

#### Type Alias `LookupError`

```rust
pub type LookupError = Error;
```

#### Type Alias `IndexError`

```rust
pub type IndexError = Error;
```

#### Type Alias `InterruptedError`

```rust
pub type InterruptedError = Error;
```

#### Type Alias `IsADirectoryError`

```rust
pub type IsADirectoryError = Error;
```

#### Type Alias `KeyError`

```rust
pub type KeyError = Error;
```

#### Type Alias `MemoryError`

```rust
pub type MemoryError = Error;
```

#### Type Alias `NameError`

```rust
pub type NameError = Error;
```

#### Type Alias `NotADirectoryError`

```rust
pub type NotADirectoryError = Error;
```

#### Type Alias `RuntimeError`

```rust
pub type RuntimeError = Error;
```

#### Type Alias `NotImplementedError`

```rust
pub type NotImplementedError = Error;
```

#### Type Alias `OverflowError`

```rust
pub type OverflowError = Error;
```

#### Type Alias `PermissionError`

```rust
pub type PermissionError = Error;
```

#### Type Alias `ProcessLookupError`

```rust
pub type ProcessLookupError = Error;
```

#### Type Alias `RecursionError`

```rust
pub type RecursionError = Error;
```

#### Type Alias `ReferenceError`

```rust
pub type ReferenceError = Error;
```

#### Type Alias `SystemError`

```rust
pub type SystemError = Error;
```

#### Type Alias `TabError`

```rust
pub type TabError = Error;
```

#### Type Alias `TimeoutError`

```rust
pub type TimeoutError = Error;
```

#### Type Alias `TypeError`

```rust
pub type TypeError = Error;
```

#### Type Alias `UnboundLocalError`

```rust
pub type UnboundLocalError = Error;
```

#### Type Alias `ValueError`

```rust
pub type ValueError = Error;
```

#### Type Alias `UnicodeError`

```rust
pub type UnicodeError = Error;
```

#### Type Alias `UnicodeDecodeError`

```rust
pub type UnicodeDecodeError = Error;
```

#### Type Alias `UnicodeEncodeError`

```rust
pub type UnicodeEncodeError = Error;
```

#### Type Alias `UnicodeTranslateError`

```rust
pub type UnicodeTranslateError = Error;
```

#### Type Alias `ZeroDivisionError`

```rust
pub type ZeroDivisionError = Error;
```

#### Type Alias `KeyboardInterrupt`

```rust
pub type KeyboardInterrupt = u16;
```

#### Type Alias `Warning_`

```rust
pub type Warning_ = Warning;
```

#### Type Alias `BytesWarning`

```rust
pub type BytesWarning = Warning;
```

#### Type Alias `DeprecationWarning`

```rust
pub type DeprecationWarning = Warning;
```

#### Type Alias `EncodingWarning`

```rust
pub type EncodingWarning = Warning;
```

#### Type Alias `FutureWarning`

```rust
pub type FutureWarning = Warning;
```

#### Type Alias `ResourceWarning`

```rust
pub type ResourceWarning = Warning;
```

#### Type Alias `RuntimeWarning`

```rust
pub type RuntimeWarning = Warning;
```

#### Type Alias `SyntaxWarning`

```rust
pub type SyntaxWarning = Warning;
```

#### Type Alias `UnicodeWarning`

```rust
pub type UnicodeWarning = Warning;
```

#### Type Alias `UserWarning`

```rust
pub type UserWarning = Warning;
```

#### Struct `Serializer`

```rust
pub struct Serializer {
    pub(in ::types) key: u16,
    pub(in ::types) data: [u8; 8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `key` | `u16` |  |
| `data` | `[u8; 8]` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn new(key: u16, data: [u8; 8]) -> Serializer { /* ... */ }
  ```

- ```rust
  pub(in ::types) fn serialize(self: &Self) -> Vec<u8> { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Visitor`

```rust
pub struct Visitor<''s> {
    pub(in ::types) serializer: &''s mut dyn Serialized,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `serializer` | `&''s mut dyn Serialized` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::types) fn new(serializer: &''s mut dyn Serialized) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Type Alias `PciClass`

```rust
pub type PciClass = u32;
```

#### Type Alias `PciDevice`

```rust
pub type PciDevice = u32;
```

#### Type Alias `PciVendor`

```rust
pub type PciVendor = u16;
```

#### Type Alias `PhyDeviceId`

```rust
pub type PhyDeviceId = u32;
```

#### Type Alias `PhyClass`

```rust
pub type PhyClass = u32;
```

#### Type Alias `BoxedVec`

```rust
pub type BoxedVec<T> = alloc::boxed::Box<alloc::vec::Vec<T>>;
```

#### Type Alias `BoxedResult`

```rust
pub type BoxedResult<T, E> = alloc::boxed::Box<Result<T, E>>;
```

#### Type Alias `BoxedOption`

```rust
pub type BoxedOption<T> = alloc::boxed::Box<Option<T>>;
```

#### Type Alias `BoxedBox`

```rust
pub type BoxedBox<T> = alloc::boxed::Box<alloc::boxed::Box<T>>;
```

#### Type Alias `VecBox`

```rust
pub type VecBox<T> = alloc::vec::Vec<alloc::boxed::Box<T>>;
```

#### Type Alias `AtomicBox`

```rust
pub type AtomicBox<T> = core::sync::atomic::Atomic<alloc::boxed::Box<T>>;
```

#### Type Alias `DualVec`

```rust
pub type DualVec<T> = alloc::vec::Vec<alloc::vec::Vec<T>>;
```

#### Type Alias `TripleVec`

```rust
pub type TripleVec<T> = alloc::vec::Vec<alloc::vec::Vec<alloc::vec::Vec<T>>>;
```

#### Type Alias `QuadVec`

```rust
pub type QuadVec<T> = alloc::vec::Vec<alloc::vec::Vec<alloc::vec::Vec<alloc::vec::Vec<T>>>>;
```

#### Type Alias `DualBox`

```rust
pub type DualBox<T> = alloc::boxed::Box<alloc::boxed::Box<T>>;
```

#### Type Alias `TripleBox`

```rust
pub type TripleBox<T> = alloc::boxed::Box<alloc::boxed::Box<alloc::boxed::Box<T>>>;
```

#### Type Alias `QuadBox`

```rust
pub type QuadBox<T> = alloc::boxed::Box<alloc::boxed::Box<alloc::boxed::Box<alloc::boxed::Box<T>>>>;
```

#### Type Alias `DualOption`

```rust
pub type DualOption<T> = Option<Option<T>>;
```

#### Type Alias `TripleOption`

```rust
pub type TripleOption<T> = Option<Option<Option<T>>>;
```

#### Type Alias `QuadOption`

```rust
pub type QuadOption<T> = Option<Option<Option<Option<T>>>>;
```

#### Type Alias `Synchronous`

```rust
pub type Synchronous<T> = alloc::vec::Vec<alloc::boxed::Box<alloc::vec::Vec<T>>>;
```

#### Type Alias `VersionString`

```rust
pub type VersionString = alloc::string::String;
```

#### Struct `Compressed`

```rust
pub(in ::types) struct Compressed<T>(pub T);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Traits

#### Trait `Peripheral`

Trait for hardware peripherals that can perform I/O operations.

```rust
pub trait Peripheral {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `doIO`
- `doHighIO`

#### Trait `Serialized`

```rust
pub trait Serialized {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `serialize`

### Functions

#### Function `defs`

```rust
pub fn defs() -> (i32, i32, i32, alloc::vec::Vec<u8>, usize, usize, &''static [u8], char, bool, i32, alloc::string::String, usize, &''static [u8], usize, alloc::string::String, usize, &''static [u8], usize, alloc::string::String, usize, usize, &''static [u8]) { /* ... */ }
```

### Constants and Statics

#### Static `TID`

```rust
pub static TID: u32 = 0;
```

#### Static `TID_MAX`

```rust
pub static TID_MAX: u32 = 0xFFFFFFFF;
```

#### Static `TID_MIN`

```rust
pub static TID_MIN: u32 = 0;
```

#### Static `ABSTIME`

```rust
pub static ABSTIME: u64 = 0;
```

#### Static `RELTIME`

```rust
pub static RELTIME: u64 = 0;
```

#### Static `TIME`

```rust
pub static TIME: u64 = 0;
```

#### Static `TINTERVAL`

```rust
pub static TINTERVAL: u64 = 0;
```

#### Static `IPV4_`

```rust
pub static IPV4_: u32 = 0;
```

#### Static `IPV6_`

```rust
pub static IPV6_: u64 = 0;
```

#### Static `IPV4_MAX`

```rust
pub static IPV4_MAX: u32 = 0xFFFFFFFF;
```

#### Static `IPV6_MAX`

```rust
pub static IPV6_MAX: u64 = 0xFFFFFFFFFFFFFFFF;
```

#### Static `IPV4_MIN`

```rust
pub static IPV4_MIN: u32 = 0;
```

#### Static `IPV6_MIN`

```rust
pub static IPV6_MIN: u64 = 0;
```

#### Static `PORT_`

```rust
pub static PORT_: u16 = 0;
```

#### Static `PORT_MAX`

```rust
pub static PORT_MAX: u16 = 0xFFFF;
```

#### Static `HOST`

```rust
pub static HOST: alloc::string::String = _;
```

#### Static `IPADDR`

```rust
pub static IPADDR: alloc::string::String = _;
```

#### Static `MACADDR`

```rust
pub static MACADDR: alloc::string::String = _;
```

#### Static `LOCALHOST`

```rust
pub static LOCALHOST: alloc::string::String = _;
```

#### Static `LOCALHOST_IPV4`

```rust
pub static LOCALHOST_IPV4: u32 = 0x7F000001;
```

#### Static `LOCALHOST_IPV6`

```rust
pub static LOCALHOST_IPV6: u64 = 1;
```

#### Static `LOCALHOST_PORT`

```rust
pub static LOCALHOST_PORT: u16 = 80;
```

#### Static `COLOR_BUFFER_BIT`

```rust
pub static COLOR_BUFFER_BIT: u32 = 0x00004000;
```

#### Static `DEPTH_BUFFER_BIT`

```rust
pub static DEPTH_BUFFER_BIT: u32 = 0x00000100;
```

#### Static `STENCIL_BUFFER_BIT`

```rust
pub static STENCIL_BUFFER_BIT: u32 = 0x00000400;
```

#### Static `COLOR_BUFFER_BIT_MASK`

```rust
pub static COLOR_BUFFER_BIT_MASK: u32 = 0x00004000;
```

#### Static `DEPTH_BUFFER_BIT_MASK`

```rust
pub static DEPTH_BUFFER_BIT_MASK: u32 = 0x00000100;
```

#### Static `STENCIL_BUFFER_BIT_MASK`

```rust
pub static STENCIL_BUFFER_BIT_MASK: u32 = 0x00000400;
```

#### Static `COLOR_CLEAR_VALUE`

```rust
pub static COLOR_CLEAR_VALUE: u32 = 0x00000000;
```

#### Static `DEPTH_CLEAR_VALUE`

```rust
pub static DEPTH_CLEAR_VALUE: f64 = 1.0;
```

#### Static `STENCIL_CLEAR_VALUE`

```rust
pub static STENCIL_CLEAR_VALUE: u32 = 0;
```

#### Static `COLOR_WRITE_MASK`

```rust
pub static COLOR_WRITE_MASK: u32 = 0x0000000F;
```

#### Static `POINTS`

```rust
pub static POINTS: u32 = 0x0000;
```

#### Static `LINES`

```rust
pub static LINES: u32 = 0x0001;
```

#### Static `LINE_LOOP`

```rust
pub static LINE_LOOP: u32 = 0x0002;
```

#### Static `LINE_STRIP`

```rust
pub static LINE_STRIP: u32 = 0x0003;
```

#### Static `TRIANGLES`

```rust
pub static TRIANGLES: u32 = 0x0004;
```

#### Static `TRIANGLE_STRIP`

```rust
pub static TRIANGLE_STRIP: u32 = 0x0005;
```

#### Static `TRIANGLE_FAN`

```rust
pub static TRIANGLE_FAN: u32 = 0x0006;
```

#### Static `NEVER`

```rust
pub static NEVER: u32 = 0x0200;
```

#### Static `LESS`

```rust
pub static LESS: u32 = 0x0201;
```

#### Static `EQUAL`

```rust
pub static EQUAL: u32 = 0x0202;
```

#### Static `LEQUAL`

```rust
pub static LEQUAL: u32 = 0x0203;
```

#### Static `GREATER`

```rust
pub static GREATER: u32 = 0x0204;
```

#### Static `NOTEQUAL`

```rust
pub static NOTEQUAL: u32 = 0x0205;
```

#### Static `GEQUAL`

```rust
pub static GEQUAL: u32 = 0x0206;
```

#### Static `ALWAYS`

```rust
pub static ALWAYS: u32 = 0x0207;
```

#### Static `SRC_ALPHA`

```rust
pub static SRC_ALPHA: u32 = 0x0302;
```

#### Static `QUADS`

```rust
pub static QUADS: u32 = 0x0007;
```

#### Static `QUAD_STRIP`

```rust
pub static QUAD_STRIP: u32 = 0x0008;
```

#### Static `POLYGON`

```rust
pub static POLYGON: u32 = 0x0009;
```

#### Static `FRONT`

```rust
pub static FRONT: u32 = 0x0404;
```

#### Static `BACK`

```rust
pub static BACK: u32 = 0x0405;
```

#### Static `LEFT`

```rust
pub static LEFT: u32 = 0x0406;
```

#### Static `RIGHT`

```rust
pub static RIGHT: u32 = 0x0407;
```

#### Static `CCW`

```rust
pub static CCW: u32 = 0x0901;
```

#### Static `CW`

```rust
pub static CW: u32 = 0x0900;
```

#### Static `LINE_WIDTH`

```rust
pub static LINE_WIDTH: u32 = 0x0B21;
```

#### Static `CULL_FACE_MODE`

```rust
pub static CULL_FACE_MODE: u32 = 0x0B45;
```

#### Static `CULL_FACE_MODE_FRONT`

```rust
pub static CULL_FACE_MODE_FRONT: u32 = 0x0B46;
```

#### Static `CULL_FACE_MODE_BACK`

```rust
pub static CULL_FACE_MODE_BACK: u32 = 0x0B47;
```

#### Static `CULL_FACE_MODE_FRONT_AND_BACK`

```rust
pub static CULL_FACE_MODE_FRONT_AND_BACK: u32 = 0x0B48;
```

#### Static `FRONT_AND_BACK`

```rust
pub static FRONT_AND_BACK: u32 = 0x0408;
```

#### Static `CULL_FACE`

```rust
pub static CULL_FACE: u32 = 0x0B44;
```

#### Static `BLEND`

```rust
pub static BLEND: u32 = 0x0BE2;
```

#### Static `DITHER`

```rust
pub static DITHER: u32 = 0x0BD0;
```

#### Static `STENCIL_TEST`

```rust
pub static STENCIL_TEST: u32 = 0x0B90;
```

#### Static `DEPTH_TEST`

```rust
pub static DEPTH_TEST: u32 = 0x0B71;
```

#### Static `ZERO`

```rust
pub static ZERO: f64 = 0.0;
```

#### Static `ONE`

```rust
pub static ONE: f64 = 1.0;
```

#### Static `SRC_COLOR`

```rust
pub static SRC_COLOR: u32 = 0x0300;
```

#### Static `BMP`

```rust
pub static BMP: u32 = 0x1A00;
```

#### Static `BMP_RGB`

```rust
pub static BMP_RGB: u32 = 0x1A00;
```

#### Static `BMP_RGBA`

```rust
pub static BMP_RGBA: u32 = 0x1A01;
```

#### Static `BMP_INDEXED`

```rust
pub static BMP_INDEXED: u32 = 0x1A02;
```

#### Static `BMP_RGB_ALPHA`

```rust
pub static BMP_RGB_ALPHA: u32 = 0x1A03;
```

#### Static `BMP_RGBA_ALPHA`

```rust
pub static BMP_RGBA_ALPHA: u32 = 0x1A04;
```

#### Static `BMP_LUMINANCE`

```rust
pub static BMP_LUMINANCE: u32 = 0x1A06;
```

#### Static `BMP_LUMINANCE_ALPHA`

```rust
pub static BMP_LUMINANCE_ALPHA: u32 = 0x1A07;
```

#### Static `BMP_ALPHA`

```rust
pub static BMP_ALPHA: u32 = 0x1A08;
```

#### Constant `INT_I8_0`

```rust
pub const INT_I8_0: i8 = 0;
```

#### Constant `INT_I16_0`

```rust
pub const INT_I16_0: i16 = 0;
```

#### Constant `INT_I32_0`

```rust
pub const INT_I32_0: i32 = 0;
```

#### Constant `INT_I64_0`

```rust
pub const INT_I64_0: i64 = 0;
```

#### Constant `INT_I128_0`

```rust
pub const INT_I128_0: i128 = 0;
```

#### Constant `INT_U8_0`

```rust
pub const INT_U8_0: u8 = 0;
```

#### Constant `INT_U16_0`

```rust
pub const INT_U16_0: u16 = 0;
```

#### Constant `INT_U32_0`

```rust
pub const INT_U32_0: u32 = 0;
```

#### Constant `INT_U64_0`

```rust
pub const INT_U64_0: u64 = 0;
```

#### Constant `INT_U128_0`

```rust
pub const INT_U128_0: u128 = 0;
```

#### Constant `FLOAT_F32_0`

```rust
pub const FLOAT_F32_0: f32 = 0.0;
```

#### Constant `FLOAT_F64_0`

```rust
pub const FLOAT_F64_0: f64 = 0.0;
```

#### Constant `ISIZE_ISIZE_0`

```rust
pub const ISIZE_ISIZE_0: isize = 0;
```

#### Constant `ISIZE_USIZE_0`

```rust
pub const ISIZE_USIZE_0: usize = 0;
```

#### Constant `STR_0`

```rust
pub const STR_0: &str = "";
```

#### Constant `BOOL_0`

```rust
pub const BOOL_0: bool = false;
```

#### Constant `BOOL_1`

```rust
pub const BOOL_1: bool = true;
```

#### Constant `CHAR_0`

```rust
pub const CHAR_0: char = '\0';
```

#### Constant `HEX_0`

```rust
pub const HEX_0: u8 = 0x00;
```

#### Constant `HEX_255`

```rust
pub const HEX_255: u8 = 0xFF;
```

#### Constant `U8_BYTE_42`

```rust
pub const U8_BYTE_42: u8 = b'*';
```

#### Constant `UTF8_0`

```rust
pub const UTF8_0: char = '\u{0}';
```

#### Constant `UTF8_10000`

```rust
pub const UTF8_10000: char = '\u{10000}';
```

#### Constant `UTF8_10FFFF`

```rust
pub const UTF8_10FFFF: char = '\u{10FFFF}';
```

#### Constant `BYTE_MAX`

```rust
pub const BYTE_MAX: u8 = 0xFF;
```

#### Constant `WORD_MAX`

```rust
pub const WORD_MAX: u16 = 0xFFFF;
```

#### Constant `DWORD_MAX`

```rust
pub const DWORD_MAX: u32 = 0xFFFFFFFF;
```

#### Constant `QWORD_MAX`

```rust
pub const QWORD_MAX: u64 = 0xFFFFFFFFFFFFFFFF;
```

#### Constant `DOUBLE_MAX`

```rust
pub const DOUBLE_MAX: f64 = DOUBLE::MAX;
```

#### Constant `FLOAT_MAX`

```rust
pub const FLOAT_MAX: f32 = FLOAT::MAX;
```

#### Constant `BOOL_MAX`

```rust
pub const BOOL_MAX: bool = true;
```

#### Constant `CHAR_MAX`

```rust
pub const CHAR_MAX: char = CHAR::MAX;
```

#### Constant `SHORT_MAX`

```rust
pub const SHORT_MAX: i16 = SHORT::MAX;
```

#### Constant `INT_MAX`

```rust
pub const INT_MAX: i32 = INT::MAX;
```

#### Constant `LONG_MAX`

```rust
pub const LONG_MAX: i64 = LONG::MAX;
```

#### Constant `UCHAR_MAX`

```rust
pub const UCHAR_MAX: u8 = UCHAR::MAX;
```

#### Constant `USHORT_MAX`

```rust
pub const USHORT_MAX: u16 = USHORT::MAX;
```

#### Constant `UINT_MAX`

```rust
pub const UINT_MAX: u32 = UINT::MAX;
```

#### Constant `ULONG_MAX`

```rust
pub const ULONG_MAX: u64 = ULONG::MAX;
```

#### Constant `SCHAR_MAX`

```rust
pub const SCHAR_MAX: i8 = SCHAR::MAX;
```

#### Constant `PROGRAM_CALL_MAX`

```rust
pub const PROGRAM_CALL_MAX: u8 = ProgramCall::MAX;
```

#### Constant `ADDR8_MAX`

```rust
pub const ADDR8_MAX: u8 = ADDR8::MAX;
```

#### Constant `ADDR16_MAX`

```rust
pub const ADDR16_MAX: u16 = ADDR16::MAX;
```

#### Constant `ADDR32_MAX`

```rust
pub const ADDR32_MAX: u32 = ADDR32::MAX;
```

#### Constant `ADDR64_MAX`

```rust
pub const ADDR64_MAX: u64 = ADDR64::MAX;
```

#### Constant `NIBBLE_MAX`

```rust
pub const NIBBLE_MAX: u8 = NIBBLE::MAX;
```

#### Constant `BYTE_MIN`

```rust
pub const BYTE_MIN: u8 = 0x00;
```

#### Constant `WORD_MIN`

```rust
pub const WORD_MIN: u16 = 0x0000;
```

#### Constant `DWORD_MIN`

```rust
pub const DWORD_MIN: u32 = 0x00000000;
```

#### Constant `QWORD_MIN`

```rust
pub const QWORD_MIN: u64 = 0x0000000000000000;
```

#### Constant `DOUBLE_MIN`

```rust
pub const DOUBLE_MIN: f64 = DOUBLE::MIN;
```

#### Constant `FLOAT_MIN`

```rust
pub const FLOAT_MIN: f32 = FLOAT::MIN;
```

#### Constant `BOOL_MIN`

```rust
pub const BOOL_MIN: bool = false;
```

#### Constant `CHAR_MIN`

```rust
pub const CHAR_MIN: char = CHAR::MIN;
```

#### Constant `SHORT_MIN`

```rust
pub const SHORT_MIN: i16 = SHORT::MIN;
```

#### Constant `INT_MIN`

```rust
pub const INT_MIN: i32 = INT::MIN;
```

#### Constant `LONG_MIN`

```rust
pub const LONG_MIN: i64 = LONG::MIN;
```

#### Constant `UCHAR_MIN`

```rust
pub const UCHAR_MIN: u8 = UCHAR::MIN;
```

#### Constant `USHORT_MIN`

```rust
pub const USHORT_MIN: u16 = USHORT::MIN;
```

#### Constant `UINT_MIN`

```rust
pub const UINT_MIN: u32 = UINT::MIN;
```

#### Constant `ULONG_MIN`

```rust
pub const ULONG_MIN: u64 = ULONG::MIN;
```

#### Constant `SCHAR_MIN`

```rust
pub const SCHAR_MIN: i8 = SCHAR::MIN;
```

#### Constant `PROGRAM_CALL_MIN`

```rust
pub const PROGRAM_CALL_MIN: u8 = ProgramCall::MIN;
```

#### Constant `ADDR8_MIN`

```rust
pub const ADDR8_MIN: u8 = ADDR8::MIN;
```

#### Constant `ADDR16_MIN`

```rust
pub const ADDR16_MIN: u16 = ADDR16::MIN;
```

#### Constant `ADDR32_MIN`

```rust
pub const ADDR32_MIN: u32 = ADDR32::MIN;
```

#### Constant `ADDR64_MIN`

```rust
pub const ADDR64_MIN: u64 = ADDR64::MIN;
```

#### Constant `NIBBLE_MIN`

```rust
pub const NIBBLE_MIN: u8 = NIBBLE::MIN;
```

## Module `rng`

```rust
pub(crate) mod rng { /* ... */ }
```

### Types

#### Struct `XorShiftRng`

```rust
pub struct XorShiftRng {
    pub(in ::rng) state: u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `state` | `u64` |  |

##### Implementations

###### Methods

- ```rust
  pub const fn new(seed: u64) -> Self { /* ... */ }
  ```

- ```rust
  pub(in ::rng) fn next_u64(self: &mut Self) -> u64 { /* ... */ }
  ```

- ```rust
  pub fn rand(self: &mut Self, len: u8) -> u64 { /* ... */ }
  ```

- ```rust
  pub fn rand_range(self: &mut Self, min: u64, max: u64) -> u64 { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `state`

```rust
pub(crate) mod state { /* ... */ }
```

### Types

#### Struct `KernelState`

```rust
pub struct KernelState {
    pub(in ::state) state: u64,
    pub(in ::state) entropy: u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `state` | `u64` |  |
| `entropy` | `u64` |  |

##### Implementations

###### Methods

- ```rust
  pub unsafe fn new(_state: u64, _entropy: u64) -> KernelState { /* ... */ }
  ```

- ```rust
  pub fn save(self: &Self) { /* ... */ }
  ```

- ```rust
  pub fn restore(self: &mut Self) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Persistable**
  - ```rust
    fn magic() -> u32 { /* ... */ }
    ```

  - ```rust
    fn get_heap_bytes(self: &Self) -> Vec<u8> { /* ... */ }
    ```

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `PersistenceManager`

```rust
pub struct PersistenceManager;
```

##### Implementations

###### Methods

- ```rust
  pub fn save<T: Persistable>(path: &str, obj: &T, mode: char) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Traits

#### Trait `Persistable`

```rust
pub trait Persistable {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `magic`
- `get_heap_bytes`: Every struct must implement how to save its specific heap data

##### Provided Methods

- ```rust
  fn version() -> u32 { /* ... */ }
  ```

- ```rust
  fn get_stack_bytes(self: &Self) -> &[u8] { /* ... */ }
  ```
  Returns the raw bytes of the struct itself (the stack/address part)

##### Implementations

This trait is implemented for the following types:

- `&mut State`
- `&''static Option<alloc::vec::Vec<LogEntry>>`
- `KernelState`

### Functions

#### Function `SAVE`

```rust
pub unsafe fn SAVE() { /* ... */ }
```

## Module `loader`

```rust
pub(crate) mod loader { /* ... */ }
```

### Functions

#### Function `load_and_jump_os`

**Attributes:**

- `Other("#[allow(unsafe_code, unsafe_op_in_unsafe_fn)]")`

```rust
pub unsafe fn load_and_jump_os(path: &str) -> never { /* ... */ }
```

## Module `terminal`

```rust
pub(crate) mod terminal { /* ... */ }
```

### Functions

#### Function `cmd`

```rust
pub fn cmd(command: alloc::vec::Vec<&str>, parts: &alloc::vec::Vec<&str>, body: alloc::vec::Vec<&str>, package_manager: &mut manager::PackageManager) { /* ... */ }
```

#### Function `enter`

```rust
pub(in ::terminal) fn enter(itm: &str) { /* ... */ }
```

#### Function `start_kernel`

```rust
pub(in ::terminal) fn start_kernel(path: &str) { /* ... */ }
```

#### Function `shutdown`

```rust
pub(in ::terminal) fn shutdown(mode: char) { /* ... */ }
```

#### Function `handle_vm_command`

**Attributes:**

- `Other("#[allow(static_mut_refs)]")`

```rust
pub(in ::terminal) fn handle_vm_command(command: &[&str]) { /* ... */ }
```

#### Function `handle_vmm_command`

**Attributes:**

- `Other("#[allow(static_mut_refs)]")`

```rust
pub(in ::terminal) fn handle_vmm_command(command: &[&str]) { /* ... */ }
```

#### Function `boot_vm_with_media`

**Attributes:**

- `Other("#[allow(static_mut_refs)]")`

Boot a VM with an ISO, EFI file, or disk image

```rust
pub(in ::terminal) fn boot_vm_with_media(vm_id: u32, media_path: &str) { /* ... */ }
```

#### Function `run_efi_application`

**Attributes:**

- `Other("#[allow(static_mut_refs)]")`

Run a standalone EFI application

```rust
pub(in ::terminal) fn run_efi_application(efi_path: &str, args: &[&str]) { /* ... */ }
```

#### Function `show_dashboard_ui`

**Attributes:**

- `Other("#[allow(static_mut_refs)]")`

Display the dashboard UI

```rust
pub(in ::terminal) unsafe fn show_dashboard_ui(package_manager: &manager::PackageManager) { /* ... */ }
```

#### Function `attach_vm_console`

**Attributes:**

- `Other("#[allow(static_mut_refs, dead_code)]")`

Attach to a VM's console for interaction

```rust
pub(in ::terminal) fn attach_vm_console(vm_id: u32) { /* ... */ }
```

#### Function `load_boot_media`

Load boot media (ISO, EFI, IMG files) from filesystem

```rust
pub(in ::terminal) fn load_boot_media(path: &str) -> Result<alloc::vec::Vec<u8>, &''static str> { /* ... */ }
```

#### Function `read_boot_file`

**Attributes:**

- `Other("#[allow(dead_code)]")`

Helper function to read a file from the filesystem

```rust
pub(in ::terminal) fn read_boot_file(path: &str) -> Result<alloc::vec::Vec<u8>, &''static str> { /* ... */ }
```

## Module `pm`

Package Management system.

This module handles package installation, dependency management,
and command execution for system packages.

```rust
pub(crate) mod pm { /* ... */ }
```

### Modules

## Module `manager`

```rust
pub(in ::pm) mod manager { /* ... */ }
```

### Types

#### Struct `UUID`

```rust
pub struct UUID(pub(in ::pm::manager) [u8; 16]);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `[u8; 16]` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> UUID { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `PackageManager`

```rust
pub struct PackageManager {
    pub version: alloc::string::String,
    pub package_path: alloc::string::String,
    pub registry: alloc::collections::BTreeMap<alloc::string::String, Package>,
    pub buffer: alloc::vec::Vec<u8>,
    pub state: StateManager,
    pub config: Config,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `version` | `alloc::string::String` |  |
| `package_path` | `alloc::string::String` |  |
| `registry` | `alloc::collections::BTreeMap<alloc::string::String, Package>` |  |
| `buffer` | `alloc::vec::Vec<u8>` |  |
| `state` | `StateManager` |  |
| `config` | `Config` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn get_version(self: &Self) -> String { /* ... */ }
  ```

- ```rust
  pub fn clone(self: &Self) -> Self { /* ... */ }
  ```

- ```rust
  pub fn install(self: &mut Self, pkg: Package) { /* ... */ }
  ```

- ```rust
  pub fn verify_dependencies(self: &Self, pkg_name: &str) { /* ... */ }
  ```

- ```rust
  pub fn load_registry(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub(in ::pm::manager) fn load_from_json(self: &mut Self, json_data: &str) -> Result<(), &''static str> { /* ... */ }
  ```

- ```rust
  pub fn load_from_json_no_alloc(self: &mut Self, json: &str) { /* ... */ }
  ```

- ```rust
  pub fn list_packages(self: &Self) { /* ... */ }
  ```

- ```rust
  pub fn get_packages(self: &Self) -> BTreeMap<PackageType, Vec<Package>> { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PackageManager { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `StateManager`

```rust
pub struct StateManager {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> StateManager { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Config`

```rust
pub struct Config {
    pub general: GeneralConfig,
    pub paths: PathConfig,
    pub verification: VerificationConfig,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `general` | `GeneralConfig` |  |
| `paths` | `PathConfig` |  |
| `verification` | `VerificationConfig` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Config { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `GeneralConfig`

```rust
pub struct GeneralConfig {
    pub color: u32,
    pub parallel_downloads: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `color` | `u32` |  |
| `parallel_downloads` | `usize` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> GeneralConfig { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `PathConfig`

```rust
pub struct PathConfig {
    pub store_path: Option<alloc::string::String>,
    pub state_path: Option<alloc::string::String>,
    pub build_path: Option<alloc::string::String>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `store_path` | `Option<alloc::string::String>` |  |
| `state_path` | `Option<alloc::string::String>` |  |
| `build_path` | `Option<alloc::string::String>` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PathConfig { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `VerificationConfig`

```rust
pub struct VerificationConfig {
    pub enabled: bool,
    pub level: alloc::string::String,
    pub discrepancy_handling: DiscrepancyHandling,
    pub user_file_policy: UserFilePolicy,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `enabled` | `bool` |  |
| `level` | `alloc::string::String` |  |
| `discrepancy_handling` | `DiscrepancyHandling` |  |
| `user_file_policy` | `UserFilePolicy` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> VerificationConfig { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `DiscrepancyHandling`

```rust
pub enum DiscrepancyHandling {
    FailFast,
    ReportOnly,
    AutoHeal,
    AutoHealOrFail,
}
```

##### Variants

###### `FailFast`

Fail the operation when discrepancies are found

###### `ReportOnly`

Report discrepancies but continue operation

###### `AutoHeal`

Automatically heal discrepancies when possible

###### `AutoHealOrFail`

Auto-heal but fail if healing is not possible

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DiscrepancyHandling { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `UserFilePolicy`

```rust
pub enum UserFilePolicy {
    Preserve,
    Remove,
    Backup,
}
```

##### Variants

###### `Preserve`

Preserve user-created files

###### `Remove`

Remove user-created files

###### `Backup`

Backup user-created files before removal

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> UserFilePolicy { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `PackageType`

```rust
pub enum PackageType {
    Library,
    Executable,
    Extension,
    ResourcePack,
    Driver,
    PShader,
}
```

##### Variants

###### `Library`

###### `Executable`

###### `Extension`

###### `ResourcePack`

###### `Driver`

###### `PShader`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PackageType { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Comparable**
  - ```rust
    fn compare(self: &Self, key: &K) -> Ordering { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Eq**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &PackageType) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PackageType) -> bool { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &PackageType) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `MiniPackageType`

```rust
pub enum MiniPackageType {
    Library,
    Executable,
    Driver,
    PShader,
}
```

##### Variants

###### `Library`

###### `Executable`

###### `Driver`

###### `PShader`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MiniPackageType { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Package`

```rust
pub struct Package {
    pub name: alloc::string::String,
    pub version: alloc::string::String,
    pub description: alloc::string::String,
    pub author: alloc::string::String,
    pub deps: alloc::vec::Vec<alloc::string::String>,
    pub package_type: PackageType,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `alloc::string::String` |  |
| `version` | `alloc::string::String` |  |
| `description` | `alloc::string::String` |  |
| `author` | `alloc::string::String` |  |
| `deps` | `alloc::vec::Vec<alloc::string::String>` |  |
| `package_type` | `PackageType` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub(in ::pm::manager) fn assign_value(self: &mut Self, key: &str, value: &str) { /* ... */ }
  ```

- ```rust
  pub fn add_dependency(self: &mut Self, dep_name: &str) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Package { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `MiniPackage`

```rust
pub struct MiniPackage {
    pub name: alloc::string::String,
    pub version: alloc::string::String,
    pub description: alloc::string::String,
    pub mini_package_type: MiniPackageType,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `alloc::string::String` |  |
| `version` | `alloc::string::String` |  |
| `description` | `alloc::string::String` |  |
| `mini_package_type` | `MiniPackageType` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `BuildReport`

```rust
pub struct BuildReport {
    pub package: alloc::string::String,
    pub version: alloc::string::String,
    pub output_path: alloc::string::String,
    pub duration_ms: u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `package` | `alloc::string::String` | Package that was built |
| `version` | `alloc::string::String` | Version that was built |
| `output_path` | `alloc::string::String` | Output file path |
| `duration_ms` | `u64` | Build duration |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> BuildReport { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `InstallReport`

```rust
pub struct InstallReport {
    pub installed: alloc::vec::Vec<PackageChange>,
    pub updated: alloc::vec::Vec<PackageChange>,
    pub removed: alloc::vec::Vec<PackageChange>,
    pub state_id: UUID,
    pub duration_ms: u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `installed` | `alloc::vec::Vec<PackageChange>` | Packages that were installed |
| `updated` | `alloc::vec::Vec<PackageChange>` | Packages that were updated |
| `removed` | `alloc::vec::Vec<PackageChange>` | Packages that were removed |
| `state_id` | `UUID` | New state ID |
| `duration_ms` | `u64` | Total execution time |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> InstallReport { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `PackageChange`

```rust
pub struct PackageChange {
    pub name: alloc::string::String,
    pub from_version: Option<alloc::string::String>,
    pub to_version: Option<alloc::string::String>,
    pub size: Option<u64>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `alloc::string::String` | Package name |
| `from_version` | `Option<alloc::string::String>` | Previous version |
| `to_version` | `Option<alloc::string::String>` | New version |
| `size` | `Option<u64>` | Size in bytes |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PackageChange { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Type Alias `PackageRegistry`

```rust
pub(in ::pm::manager) type PackageRegistry = alloc::collections::BTreeMap<alloc::string::String, Package>;
```

#### Type Alias `SortablePackageRegistry`

```rust
pub(in ::pm::manager) type SortablePackageRegistry = alloc::collections::BTreeMap<alloc::string::String, alloc::collections::BTreeMap<alloc::string::String, Package>>;
```

### Constants and Statics

#### Static `VERSION`

```rust
pub(in ::pm::manager) static VERSION: &str = "0.3.2";
```

## Module `commands`

```rust
pub(in ::pm) mod commands { /* ... */ }
```

### Functions

#### Function `command`

```rust
pub fn command(parts: &alloc::vec::Vec<&str>, package_manager: &mut manager::PackageManager) { /* ... */ }
```

## Module `micro_c`

MicroC Compiler.

A lightweight C compiler targeting multiple architectures.
This module includes lexing, parsing, code generation,
and architecture-specific backends.

```rust
pub(crate) mod micro_c { /* ... */ }
```

### Modules

## Module `lexer`

```rust
pub mod lexer { /* ... */ }
```

### Types

#### Enum `Token`

```rust
pub enum Token {
    Let,
    Fn,
    Return,
    If,
    Elif,
    Else,
    Loop,
    Break,
    Continue,
    Export,
    Struct,
    Ident(alloc::string::String),
    Number(i64),
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Semicolon,
    Colon,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    EqEq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Assign,
    EOF,
    Arrow,
}
```

##### Variants

###### `Let`

###### `Fn`

###### `Return`

###### `If`

###### `Elif`

###### `Else`

###### `Loop`

###### `Break`

###### `Continue`

###### `Export`

###### `Struct`

###### `Ident`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `Number`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i64` |  |

###### `LParen`

###### `RParen`

###### `LBrace`

###### `RBrace`

###### `LBracket`

###### `RBracket`

###### `Comma`

###### `Semicolon`

###### `Colon`

###### `Dot`

###### `Plus`

###### `Minus`

###### `Star`

###### `Slash`

###### `EqEq`

###### `NotEq`

###### `Lt`

###### `Gt`

###### `LtEq`

###### `GtEq`

###### `Assign`

###### `EOF`

###### `Arrow`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Token { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Token) -> bool { /* ... */ }
    ```

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Lexer`

```rust
pub struct Lexer {
    pub(in ::micro_c::lexer) input: alloc::vec::Vec<char>,
    pub(in ::micro_c::lexer) pos: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `input` | `alloc::vec::Vec<char>` |  |
| `pos` | `usize` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(input: &str) -> Self { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::lexer) fn peek(self: &Self) -> Option<char> { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::lexer) fn next(self: &mut Self) -> Option<char> { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::lexer) fn skip_ws(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::lexer) fn ident(self: &mut Self) -> Token { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::lexer) fn number(self: &mut Self) -> Token { /* ... */ }
  ```

- ```rust
  pub fn next_token(self: &mut Self) -> Token { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `parser`

```rust
pub mod parser { /* ... */ }
```

### Types

#### Struct `Parser`

```rust
pub struct Parser {
    pub(in ::micro_c::parser) lexer: crate::lexer::Lexer,
    pub(in ::micro_c::parser) current: crate::lexer::Token,
    pub(in ::micro_c::parser) next: crate::lexer::Token,
    pub(in ::micro_c::parser) position: u64,
    pub(in ::micro_c::parser) condition: u8,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `lexer` | `crate::lexer::Lexer` |  |
| `current` | `crate::lexer::Token` |  |
| `next` | `crate::lexer::Token` |  |
| `position` | `u64` |  |
| `condition` | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(lexer: Lexer) -> Self { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::parser) fn advance(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::parser) fn expect(self: &mut Self, t: Token) { /* ... */ }
  ```

- ```rust
  pub fn parse_program(self: &mut Self) -> Vec<Stmt> { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::parser) fn parse_stmt(self: &mut Self) -> Stmt { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::parser) fn parse_let(self: &mut Self) -> Stmt { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::parser) fn parse_assign(self: &mut Self) -> Stmt { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::parser) fn parse_struct(self: &mut Self) -> Stmt { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::parser) fn parse_fn(self: &mut Self) -> Stmt { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::parser) fn parse_block(self: &mut Self) -> Vec<Stmt> { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::parser) fn parse_if(self: &mut Self) -> Stmt { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::parser) fn parse_loop(self: &mut Self) -> Stmt { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::parser) fn parse_return(self: &mut Self) -> Stmt { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::parser) fn parse_expr(self: &mut Self) -> Expr { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::parser) fn parse_equality(self: &mut Self) -> Expr { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::parser) fn parse_comparison(self: &mut Self) -> Expr { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::parser) fn parse_term(self: &mut Self) -> Expr { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::parser) fn parse_factor(self: &mut Self) -> Expr { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::parser) fn parse_primary(self: &mut Self) -> Expr { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `ast`

```rust
pub mod ast { /* ... */ }
```

### Types

#### Enum `Type`

```rust
pub enum Type {
    I64,
    Bool,
    Ptr(alloc::boxed::Box<Type>),
    Struct(alloc::string::String),
}
```

##### Variants

###### `I64`

###### `Bool`

###### `Ptr`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::boxed::Box<Type>` |  |

###### `Struct`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Type { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `Expr`

```rust
pub enum Expr {
    Number(i64),
    Variable(alloc::string::String),
    Binary(alloc::boxed::Box<Expr>, Op, alloc::boxed::Box<Expr>),
    Call(alloc::string::String, alloc::vec::Vec<Expr>),
    Peek(alloc::boxed::Box<Expr>),
    Index(alloc::boxed::Box<Expr>, alloc::boxed::Box<Expr>),
    Field(alloc::boxed::Box<Expr>, alloc::string::String),
}
```

##### Variants

###### `Number`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i64` |  |

###### `Variable`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `Binary`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::boxed::Box<Expr>` |  |
| 1 | `Op` |  |
| 2 | `alloc::boxed::Box<Expr>` |  |

###### `Call`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |
| 1 | `alloc::vec::Vec<Expr>` |  |

###### `Peek`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::boxed::Box<Expr>` |  |

###### `Index`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::boxed::Box<Expr>` |  |
| 1 | `alloc::boxed::Box<Expr>` |  |

###### `Field`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::boxed::Box<Expr>` |  |
| 1 | `alloc::string::String` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Expr { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `Stmt`

```rust
pub enum Stmt {
    Let {
        name: alloc::string::String,
        ty: Option<Type>,
        value: Expr,
    },
    Assign(alloc::string::String, Expr),
    AssignIndex {
        base: Expr,
        index: Expr,
        value: Expr,
    },
    AssignField {
        base: Expr,
        field: alloc::string::String,
        value: Expr,
    },
    Struct {
        name: alloc::string::String,
        fields: alloc::vec::Vec<(alloc::string::String, Type)>,
    },
    Return(Expr),
    Expr(Expr),
    Poke(Expr, Expr),
    If {
        cond: Expr,
        then_branch: alloc::vec::Vec<Stmt>,
        elif: alloc::vec::Vec<(Expr, alloc::vec::Vec<Stmt>)>,
        else_branch: Option<alloc::vec::Vec<Stmt>>,
    },
    Loop(alloc::vec::Vec<Stmt>),
    Break,
    Continue,
    Function {
        name: alloc::string::String,
        params: alloc::vec::Vec<alloc::string::String>,
        body: alloc::vec::Vec<Stmt>,
        export: bool,
    },
    None,
}
```

##### Variants

###### `Let`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `alloc::string::String` |  |
| `ty` | `Option<Type>` |  |
| `value` | `Expr` |  |

###### `Assign`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |
| 1 | `Expr` |  |

###### `AssignIndex`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `base` | `Expr` |  |
| `index` | `Expr` |  |
| `value` | `Expr` |  |

###### `AssignField`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `base` | `Expr` |  |
| `field` | `alloc::string::String` |  |
| `value` | `Expr` |  |

###### `Struct`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `alloc::string::String` |  |
| `fields` | `alloc::vec::Vec<(alloc::string::String, Type)>` |  |

###### `Return`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Expr` |  |

###### `Expr`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Expr` |  |

###### `Poke`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Expr` |  |
| 1 | `Expr` |  |

###### `If`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `cond` | `Expr` |  |
| `then_branch` | `alloc::vec::Vec<Stmt>` |  |
| `elif` | `alloc::vec::Vec<(Expr, alloc::vec::Vec<Stmt>)>` |  |
| `else_branch` | `Option<alloc::vec::Vec<Stmt>>` |  |

###### `Loop`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::vec::Vec<Stmt>` |  |

###### `Break`

###### `Continue`

###### `Function`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `alloc::string::String` |  |
| `params` | `alloc::vec::Vec<alloc::string::String>` |  |
| `body` | `alloc::vec::Vec<Stmt>` |  |
| `export` | `bool` |  |

###### `None`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Stmt { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Enum `Op`

```rust
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
    Lt,
    Gt,
    LtEq,
    GtEq,
}
```

##### Variants

###### `Add`

###### `Sub`

###### `Mul`

###### `Div`

###### `Eq`

###### `Neq`

###### `Lt`

###### `Gt`

###### `LtEq`

###### `GtEq`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Op { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `interpreter`

```rust
pub mod interpreter { /* ... */ }
```

### Types

#### Enum `Control`

```rust
pub(in ::micro_c::interpreter) enum Control {
    None,
    Return(i64),
    Break,
    Continue,
}
```

##### Variants

###### `None`

###### `Return`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i64` |  |

###### `Break`

###### `Continue`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Interpreter`

```rust
pub struct Interpreter {
    pub(in ::micro_c::interpreter) scopes: alloc::vec::Vec<hashbrown::HashMap<alloc::string::String, i64>>,
    pub(in ::micro_c::interpreter) types: hashbrown::HashMap<alloc::string::String, Type>,
    pub(in ::micro_c::interpreter) memory: hashbrown::HashMap<i64, i64>,
    pub(in ::micro_c::interpreter) functions: hashbrown::HashMap<alloc::string::String, Stmt>,
    pub(in ::micro_c::interpreter) structs: hashbrown::HashMap<alloc::string::String, alloc::vec::Vec<(alloc::string::String, Type)>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `scopes` | `alloc::vec::Vec<hashbrown::HashMap<alloc::string::String, i64>>` |  |
| `types` | `hashbrown::HashMap<alloc::string::String, Type>` |  |
| `memory` | `hashbrown::HashMap<i64, i64>` |  |
| `functions` | `hashbrown::HashMap<alloc::string::String, Stmt>` |  |
| `structs` | `hashbrown::HashMap<alloc::string::String, alloc::vec::Vec<(alloc::string::String, Type)>>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::interpreter) fn set(self: &mut Self, name: String, val: i64, ty: Type) { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::interpreter) fn get(self: &Self, name: &str) -> i64 { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::interpreter) fn field_offset(self: &Self, struct_name: &str, field: &str) -> i64 { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::interpreter) fn get_struct_type(self: &Self, expr: &Expr) -> String { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::interpreter) fn eval(self: &mut Self, expr: Expr) -> i64 { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::interpreter) fn call(self: &mut Self, name: &str, args: Vec<i64>) -> i64 { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::interpreter) fn exec_block(self: &mut Self, stmts: &[Stmt]) -> Control { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::interpreter) fn assign(self: &mut Self, name: String, val: i64) { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::interpreter) fn exec(self: &mut Self, stmt: Stmt) -> Control { /* ... */ }
  ```

- ```rust
  pub fn run(self: &mut Self, stmts: &[Stmt]) -> i64 { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `ir`

```rust
pub mod ir { /* ... */ }
```

### Types

#### Enum `IRInst`

```rust
pub enum IRInst {
    LoadConst(alloc::string::String, i64),
    LoadVar(alloc::string::String, alloc::string::String),
    StoreVar(alloc::string::String, alloc::string::String),
    Add(alloc::string::String, alloc::string::String, alloc::string::String),
    Sub(alloc::string::String, alloc::string::String, alloc::string::String),
    Mul(alloc::string::String, alloc::string::String, alloc::string::String),
    Div(alloc::string::String, alloc::string::String, alloc::string::String),
    Eq(alloc::string::String, alloc::string::String, alloc::string::String),
    Neq(alloc::string::String, alloc::string::String, alloc::string::String),
    Lt(alloc::string::String, alloc::string::String, alloc::string::String),
    Gt(alloc::string::String, alloc::string::String, alloc::string::String),
    LtEq(alloc::string::String, alloc::string::String, alloc::string::String),
    GtEq(alloc::string::String, alloc::string::String, alloc::string::String),
    Label(alloc::string::String),
    Jump(alloc::string::String),
    JumpIfZero(alloc::string::String, alloc::string::String),
    Call(alloc::string::String, alloc::string::String, alloc::vec::Vec<alloc::string::String>),
    Return(alloc::string::String),
}
```

##### Variants

###### `LoadConst`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |
| 1 | `i64` |  |

###### `LoadVar`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |
| 1 | `alloc::string::String` |  |

###### `StoreVar`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |
| 1 | `alloc::string::String` |  |

###### `Add`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |
| 1 | `alloc::string::String` |  |
| 2 | `alloc::string::String` |  |

###### `Sub`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |
| 1 | `alloc::string::String` |  |
| 2 | `alloc::string::String` |  |

###### `Mul`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |
| 1 | `alloc::string::String` |  |
| 2 | `alloc::string::String` |  |

###### `Div`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |
| 1 | `alloc::string::String` |  |
| 2 | `alloc::string::String` |  |

###### `Eq`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |
| 1 | `alloc::string::String` |  |
| 2 | `alloc::string::String` |  |

###### `Neq`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |
| 1 | `alloc::string::String` |  |
| 2 | `alloc::string::String` |  |

###### `Lt`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |
| 1 | `alloc::string::String` |  |
| 2 | `alloc::string::String` |  |

###### `Gt`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |
| 1 | `alloc::string::String` |  |
| 2 | `alloc::string::String` |  |

###### `LtEq`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |
| 1 | `alloc::string::String` |  |
| 2 | `alloc::string::String` |  |

###### `GtEq`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |
| 1 | `alloc::string::String` |  |
| 2 | `alloc::string::String` |  |

###### `Label`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `Jump`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `JumpIfZero`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |
| 1 | `alloc::string::String` |  |

###### `Call`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |
| 1 | `alloc::string::String` |  |
| 2 | `alloc::vec::Vec<alloc::string::String>` |  |

###### `Return`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> IRInst { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `codegen_ir`

```rust
pub mod codegen_ir { /* ... */ }
```

### Types

#### Struct `IRGenerator`

```rust
pub struct IRGenerator {
    pub(in ::micro_c::codegen_ir) temp_count: usize,
    pub(in ::micro_c::codegen_ir) label_count: usize,
    pub code: alloc::vec::Vec<IRInst>,
    pub function_params: hashbrown::HashMap<alloc::string::String, alloc::vec::Vec<alloc::string::String>>,
    pub(in ::micro_c::codegen_ir) position: u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `temp_count` | `usize` |  |
| `label_count` | `usize` |  |
| `code` | `alloc::vec::Vec<IRInst>` |  |
| `function_params` | `hashbrown::HashMap<alloc::string::String, alloc::vec::Vec<alloc::string::String>>` |  |
| `position` | `u64` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::codegen_ir) fn temp(self: &mut Self) -> String { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::codegen_ir) fn label(self: &mut Self) -> String { /* ... */ }
  ```

- ```rust
  pub fn gen_program(self: &mut Self, stmts: &[Stmt]) { /* ... */ }
  ```

- ```rust
  pub fn gen_stmt(self: &mut Self, stmt: Stmt) { /* ... */ }
  ```

- ```rust
  pub fn gen_expr(self: &mut Self, expr: Expr) -> String { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `backend`

```rust
pub mod backend { /* ... */ }
```

## Module `regalloc`

```rust
pub mod regalloc { /* ... */ }
```

### Types

#### Struct `RegisterAllocator`

```rust
pub struct RegisterAllocator {
    pub(in ::micro_c::regalloc) regs: alloc::vec::Vec<alloc::string::String>,
    pub(in ::micro_c::regalloc) map: hashbrown::HashMap<alloc::string::String, alloc::string::String>,
    pub(in ::micro_c::regalloc) next: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `regs` | `alloc::vec::Vec<alloc::string::String>` |  |
| `map` | `hashbrown::HashMap<alloc::string::String, alloc::string::String>` |  |
| `next` | `usize` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(regs: Vec<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn alloc(self: &mut Self, temp: &str) -> String { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `emitter`

```rust
pub mod emitter { /* ... */ }
```

## Module `compiler`

```rust
pub mod compiler { /* ... */ }
```

### Functions

#### Function `compile`

```rust
pub fn compile(source: &str, arch: &str) -> alloc::string::String { /* ... */ }
```

## Module `arch`

```rust
pub mod arch { /* ... */ }
```

### Modules

## Module `win64`

```rust
pub(crate) mod win64 { /* ... */ }
```

### Types

#### Struct `WIN64Backend`

```rust
pub struct WIN64Backend {
    pub(in ::micro_c::arch::win64) regs: crate::regalloc::RegisterAllocator,
    pub(in ::micro_c::arch::win64) function_params: hashbrown::HashMap<alloc::string::String, alloc::vec::Vec<alloc::string::String>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `regs` | `crate::regalloc::RegisterAllocator` |  |
| `function_params` | `hashbrown::HashMap<alloc::string::String, alloc::vec::Vec<alloc::string::String>>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(function_params: HashMap<String, Vec<String>>) -> Self { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::arch::win64) fn is_temp(name: &str) -> bool { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Architecture**
  - ```rust
    fn emit_program(self: &mut Self, ir: &[IRInst]) -> String { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `arm64`

```rust
pub(crate) mod arm64 { /* ... */ }
```

### Types

#### Struct `ARM64Backend`

```rust
pub struct ARM64Backend {
    pub(in ::micro_c::arch::arm64) regs: crate::regalloc::RegisterAllocator,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `regs` | `crate::regalloc::RegisterAllocator` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Architecture**
  - ```rust
    fn emit_program(self: &mut Self, ir: &[IRInst]) -> String { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `x86_64_raw`

```rust
pub(crate) mod x86_64_raw { /* ... */ }
```

### Types

#### Struct `X86_64RawBackend`

```rust
pub struct X86_64RawBackend {
    pub(in ::micro_c::arch::x86_64_raw) regs: crate::regalloc::RegisterAllocator,
    pub(in ::micro_c::arch::x86_64_raw) function_params: hashbrown::HashMap<alloc::string::String, alloc::vec::Vec<alloc::string::String>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `regs` | `crate::regalloc::RegisterAllocator` |  |
| `function_params` | `hashbrown::HashMap<alloc::string::String, alloc::vec::Vec<alloc::string::String>>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(function_params: HashMap<String, Vec<String>>) -> Self { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::arch::x86_64_raw) fn split_functions(ir: &[IRInst]) -> Vec<(String, Vec<IRInst>)> { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::arch::x86_64_raw) fn build_frame(self: &Self, name: &str, body: &[IRInst]) -> StackFrame { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::arch::x86_64_raw) fn emit_function(self: &mut Self, out: &mut String, name: &str, body: &[IRInst]) { /* ... */ }
  ```

- ```rust
  pub(in ::micro_c::arch::x86_64_raw) fn emit_inst(self: &mut Self, out: &mut String, inst: &IRInst, frame: &mut StackFrame) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Architecture**
  - ```rust
    fn emit_program(self: &mut Self, ir: &[IRInst]) -> String { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Traits

#### Trait `Architecture`

```rust
pub trait Architecture {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `emit_program`

##### Implementations

This trait is implemented for the following types:

- `WIN64Backend`
- `ARM64Backend`
- `X86_64RawBackend`

## Module `stackframe`

```rust
pub mod stackframe { /* ... */ }
```

### Types

#### Struct `StackFrame`

```rust
pub struct StackFrame {
    pub(crate) offsets: hashbrown::HashMap<alloc::string::String, i32>,
    pub(in ::micro_c::stackframe) next_offset: i32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `offsets` | `hashbrown::HashMap<alloc::string::String, i32>` |  |
| `next_offset` | `i32` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn alloc(self: &mut Self, name: &str) -> i32 { /* ... */ }
  ```

- ```rust
  pub fn get(self: &Self, name: &str) -> i32 { /* ... */ }
  ```

- ```rust
  pub fn frame_size(self: &Self) -> i32 { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `error`

```rust
pub mod error { /* ... */ }
```

### Functions

#### Function `error`

```rust
pub fn error(message: &str) { /* ... */ }
```

### Functions

#### Function `compile_from_file_to_asm`

Compiles a MicroC source file to assembly.

```rust
pub fn compile_from_file_to_asm(srcpath: alloc::string::String) -> alloc::string::String { /* ... */ }
```

## Module `cpucheck`

```rust
pub(crate) mod cpucheck { /* ... */ }
```

### Functions

#### Function `check`

```rust
pub(in ::cpucheck) fn check() { /* ... */ }
```

## Module `env`

Environment and application life-cycle management.

This module provides the infrastructure for running applications,
managing their environments, and handling background tasks.

```rust
pub(crate) mod env { /* ... */ }
```

### Types

#### Type Alias `EnvironmentVariable`

```rust
pub type EnvironmentVariable = (alloc::string::String, alloc::string::String);
```

#### Struct `Environment`

Local environment, app-specific
Local environment for an application.

Contains path variables and other settings that are specific to a single
application instance.

```rust
pub struct Environment {
    pub cd: (alloc::string::String, alloc::string::String),
    pub xd: (alloc::string::String, alloc::string::String),
    pub tmp: (alloc::string::String, alloc::string::String),
    pub user: (alloc::string::String, alloc::string::String),
    pub devname: (alloc::string::String, alloc::string::String),
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `cd` | `(alloc::string::String, alloc::string::String)` |  |
| `xd` | `(alloc::string::String, alloc::string::String)` |  |
| `tmp` | `(alloc::string::String, alloc::string::String)` |  |
| `user` | `(alloc::string::String, alloc::string::String)` |  |
| `devname` | `(alloc::string::String, alloc::string::String)` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::env) fn new() -> Environment { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Environment { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `GlobalEnvironment`

Global environment, not app-specific
Global system environment.

Contains system-wide variables like the number of processors and OS version.

```rust
pub struct GlobalEnvironment {
    pub cd: (alloc::string::String, alloc::string::String),
    pub xd: (alloc::string::String, alloc::string::String),
    pub tmp: (alloc::string::String, alloc::string::String),
    pub user: (alloc::string::String, alloc::string::String),
    pub devname: (alloc::string::String, alloc::string::String),
    pub processor_count: (alloc::string::String, alloc::string::String),
    pub os_version: (alloc::string::String, alloc::string::String),
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `cd` | `(alloc::string::String, alloc::string::String)` |  |
| `xd` | `(alloc::string::String, alloc::string::String)` |  |
| `tmp` | `(alloc::string::String, alloc::string::String)` |  |
| `user` | `(alloc::string::String, alloc::string::String)` |  |
| `devname` | `(alloc::string::String, alloc::string::String)` |  |
| `processor_count` | `(alloc::string::String, alloc::string::String)` |  |
| `os_version` | `(alloc::string::String, alloc::string::String)` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `Application`

Represents a runnable application.

This structure holds the application's metadata and its core logic
represented by the `Runnable` trait.

```rust
pub struct Application {
    pub name: alloc::string::String,
    pub version: alloc::string::String,
    pub jit_entry: Option<(alloc::string::String, u64)>,
    pub local_env: Option<Environment>,
    pub inner: alloc::boxed::Box<dyn Runnable>,
    pub dimensions: (usize, usize),
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `alloc::string::String` | The name of the application. |
| `version` | `alloc::string::String` | The version string. |
| `jit_entry` | `Option<(alloc::string::String, u64)>` | Entry point for JIT-compiled code, if applicable. |
| `local_env` | `Option<Environment>` | The local environment variables for this application. |
| `inner` | `alloc::boxed::Box<dyn Runnable>` | The actual application logic. |
| `dimensions` | `(usize, usize)` | The preferred window dimensions (width, height). |

##### Implementations

###### Methods

- ```rust
  pub fn new(inner: Box<dyn Runnable>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn dimensions(self: &Self) -> [usize; 2] { /* ... */ }
  ```

- ```rust
  pub fn draw(self: &Self, graphics_entity: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) { /* ... */ }
  ```

- ```rust
  pub fn logic(self: &mut Self, vars: &mut Vec<String>) { /* ... */ }
  ```

- ```rust
  pub fn input(self: &mut Self, key: Key) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `ApplicationContext`

**⚠️ Deprecated since 1.5.4**: use SteppedApplicationContext instead

Local context for an app
Context for running an application in a blocking loop.

```rust
pub struct ApplicationContext {
    pub parent: Option<Application>,
    pub application: Application,
    pub background_tasks: Option<alloc::vec::Vec<alloc::boxed::Box<dyn BackgroundTask>>>,
    pub global: bool,
    pub metadata: alloc::collections::BTreeMap<alloc::string::String, alloc::string::String>,
    pub environment: Environment,
    pub exit_requested: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `parent` | `Option<Application>` |  |
| `application` | `Application` |  |
| `background_tasks` | `Option<alloc::vec::Vec<alloc::boxed::Box<dyn BackgroundTask>>>` |  |
| `global` | `bool` |  |
| `metadata` | `alloc::collections::BTreeMap<alloc::string::String, alloc::string::String>` |  |
| `environment` | `Environment` |  |
| `exit_requested` | `bool` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(app: Application, background_tasks: Option<Vec<Box<dyn BackgroundTask>>>) -> ApplicationContext { /* ... */ }
  ```

- ```rust
  pub unsafe fn run(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn handle_input(self: &mut Self, key: Key) { /* ... */ }
  ```

- ```rust
  pub fn from_name(name: &str) -> Option<ApplicationContext> { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `SteppedApplicationContext`

Execution context for an application that can be stepped manually.

This is used by the windowing system to update multiple applications
concurrently in the same main loop.

```rust
pub struct SteppedApplicationContext {
    pub parent: Option<Application>,
    pub application: Application,
    pub background_tasks: Option<alloc::vec::Vec<alloc::boxed::Box<dyn BackgroundTask>>>,
    pub global: bool,
    pub metadata: alloc::collections::BTreeMap<alloc::string::String, alloc::string::String>,
    pub environment: Environment,
    pub exit_requested: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `parent` | `Option<Application>` |  |
| `application` | `Application` |  |
| `background_tasks` | `Option<alloc::vec::Vec<alloc::boxed::Box<dyn BackgroundTask>>>` |  |
| `global` | `bool` |  |
| `metadata` | `alloc::collections::BTreeMap<alloc::string::String, alloc::string::String>` |  |
| `environment` | `Environment` |  |
| `exit_requested` | `bool` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(app: Application, background_tasks: Option<Vec<Box<dyn BackgroundTask>>>) -> SteppedApplicationContext { /* ... */ }
  ```

- ```rust
  pub fn step(self: &mut Self, key: Option<Key>) -> bool { /* ... */ }
  ```
  Performs one 'tick' of the application.

- ```rust
  pub fn handle_input(self: &mut Self, key: Key) { /* ... */ }
  ```

- ```rust
  pub fn from_name(name: &str) -> Option<SteppedApplicationContext> { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Type Alias `Unknown`

Alias of Option\<T\>

```rust
pub type Unknown<T> = Option<T>;
```

### Traits

#### Trait `Runnable`

Core trait for application logic and rendering.

Any application that wants to be managed by the system must implement this trait.

```rust
pub trait Runnable {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `draw`: Renders the application to the provided `PixelGraphics` context.
- `logic`: Updates the application's internal state.
- `input`: Handles a single keyboard input event.

##### Implementations

This trait is implemented for the following types:

- `SimpleApp`
- `CH64App`
- `ClockApp`
- `CubeApp`
- `NetworkManagerApp`
- `InstructionManualApp`
- `AppInstallerApp`
- `SnakeApp`
- `DoomApp`
- `SysTestApp`
- `MinecraftApp`

#### Trait `BackgroundTask`

Represents a task that runs in the background.

```rust
pub trait BackgroundTask {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `tick`: Performs a single tick of work.

#### Trait `AppInfo`

Metadata and capability information about an application.

```rust
pub trait AppInfo {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `name`: Returns the display name of the application.
- `version`: Returns the version string.
- `icon`: Returns the application's 32x32 icon data (1024 pixels).
- `dimensions`: Returns the preferred window dimensions (width, height).

##### Provided Methods

- ```rust
  fn author(self: &Self) -> &str { /* ... */ }
  ```
  Returns the author's name. Defaults to "Unknown".

##### Implementations

This trait is implemented for the following types:

- `SimpleApp`
- `CH64App`
- `ClockApp`
- `CubeApp`
- `NetworkManagerApp`
- `InstructionManualApp`
- `AppInstallerApp`
- `SnakeApp`
- `DoomApp`
- `SysTestApp`
- `MinecraftApp`

## Module `apps`

Registry and management for parallel applications.

This module contains the `APP_REGISTRY`, which is the central list of
all available applications in the system.

Each application in the registry provides a name, a constructor,
an icon, and a version string.

```rust
pub(crate) mod apps { /* ... */ }
```

### Modules

## Module `simple_app`

```rust
pub(crate) mod simple_app { /* ... */ }
```

### Types

#### Struct `SimpleApp`

```rust
pub struct SimpleApp {
    pub color: [u32; 3],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `color` | `[u32; 3]` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AppInfo**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn version(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn icon(self: &Self) -> [u32; 1024] { /* ... */ }
    ```

  - ```rust
    fn dimensions(self: &Self) -> (usize, usize) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Runnable**
  - ```rust
    fn draw(self: &Self, graphics_entity: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) { /* ... */ }
    ```

  - ```rust
    fn logic(self: &mut Self, _vars: &mut Vec<String>) { /* ... */ }
    ```

  - ```rust
    fn input(self: &mut Self, key: Key) { /* ... */ }
    ```

- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `keystepper`

```rust
pub mod keystepper { /* ... */ }
```

### Types

#### Struct `CH64App`

```rust
pub struct CH64App {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AppInfo**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn version(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn icon(self: &Self) -> [u32; 1024] { /* ... */ }
    ```

  - ```rust
    fn dimensions(self: &Self) -> (usize, usize) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Runnable**
  - ```rust
    fn draw(self: &Self, graphics_entity: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) { /* ... */ }
    ```

  - ```rust
    fn logic(self: &mut Self, _vars: &mut Vec<String>) { /* ... */ }
    ```

  - ```rust
    fn input(self: &mut Self, key: Key) { /* ... */ }
    ```

- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `clock`

```rust
pub(in ::apps) mod clock { /* ... */ }
```

### Types

#### Struct `ClockApp`

```rust
pub struct ClockApp {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AppInfo**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn version(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn icon(self: &Self) -> [u32; 1024] { /* ... */ }
    ```

  - ```rust
    fn dimensions(self: &Self) -> (usize, usize) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Runnable**
  - ```rust
    fn draw(self: &Self, graphics: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) { /* ... */ }
    ```

  - ```rust
    fn logic(self: &mut Self, _vars: &mut Vec<String>) { /* ... */ }
    ```

  - ```rust
    fn input(self: &mut Self, _key: uefi::proto::console::text::Key) { /* ... */ }
    ```

- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `cube`

```rust
pub(in ::apps) mod cube { /* ... */ }
```

### Types

#### Struct `CubeApp`

```rust
pub struct CubeApp {
    pub(in ::apps::cube) angle_x: f64,
    pub(in ::apps::cube) angle_y: f64,
    pub(in ::apps::cube) speed_x: f64,
    pub(in ::apps::cube) speed_y: f64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `angle_x` | `f64` |  |
| `angle_y` | `f64` |  |
| `speed_x` | `f64` |  |
| `speed_y` | `f64` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AppInfo**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn version(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn icon(self: &Self) -> [u32; 1024] { /* ... */ }
    ```

  - ```rust
    fn dimensions(self: &Self) -> (usize, usize) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Runnable**
  - ```rust
    fn logic(self: &mut Self, _vars: &mut Vec<String>) { /* ... */ }
    ```

  - ```rust
    fn input(self: &mut Self, key: Key) { /* ... */ }
    ```

  - ```rust
    fn draw(self: &Self, graphics: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) { /* ... */ }
    ```

- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `netman`

```rust
pub(in ::apps) mod netman { /* ... */ }
```

### Types

#### Struct `NetworkManagerApp`

```rust
pub struct NetworkManagerApp {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AppInfo**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn version(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn icon(self: &Self) -> [u32; 1024] { /* ... */ }
    ```

  - ```rust
    fn dimensions(self: &Self) -> (usize, usize) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Runnable**
  - ```rust
    fn draw(self: &Self, graphics_entity: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) { /* ... */ }
    ```

  - ```rust
    fn logic(self: &mut Self, vars: &mut Vec<String>) { /* ... */ }
    ```

  - ```rust
    fn input(self: &mut Self, key: Key) { /* ... */ }
    ```

- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `manual`

```rust
pub(in ::apps) mod manual { /* ... */ }
```

### Types

#### Struct `InstructionManualApp`

```rust
pub struct InstructionManualApp {
    pub(in ::apps::manual) pages: alloc::vec::Vec<alloc::vec::Vec<alloc::string::String>>,
    pub(in ::apps::manual) current_page: usize,
    pub(in ::apps::manual) scroll_y: usize,
    pub(in ::apps::manual) width: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `pages` | `alloc::vec::Vec<alloc::vec::Vec<alloc::string::String>>` |  |
| `current_page` | `usize` |  |
| `scroll_y` | `usize` |  |
| `width` | `usize` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(content: &str, window_width: usize) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AppInfo**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn version(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn icon(self: &Self) -> [u32; 1024] { /* ... */ }
    ```

  - ```rust
    fn dimensions(self: &Self) -> (usize, usize) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Runnable**
  - ```rust
    fn draw(self: &Self, graphics: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) { /* ... */ }
    ```

  - ```rust
    fn logic(self: &mut Self, _vars: &mut Vec<String>) { /* ... */ }
    ```

  - ```rust
    fn input(self: &mut Self, key: Key) { /* ... */ }
    ```

- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `appinstaller`

```rust
pub(in ::apps) mod appinstaller { /* ... */ }
```

### Types

#### Struct `AppInstallerApp`

```rust
pub struct AppInstallerApp {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AppInfo**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn version(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn icon(self: &Self) -> [u32; 1024] { /* ... */ }
    ```

  - ```rust
    fn dimensions(self: &Self) -> (usize, usize) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Runnable**
  - ```rust
    fn draw(self: &Self, graphics_entity: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) { /* ... */ }
    ```

  - ```rust
    fn logic(self: &mut Self, vars: &mut Vec<String>) { /* ... */ }
    ```

  - ```rust
    fn input(self: &mut Self, key: Key) { /* ... */ }
    ```

- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `snake`

```rust
pub(in ::apps) mod snake { /* ... */ }
```

### Types

#### Enum `Direction`

```rust
pub(in ::apps::snake) enum Direction {
    Up,
    Down,
    Left,
    Right,
}
```

##### Variants

###### `Up`

###### `Down`

###### `Left`

###### `Right`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Direction { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Direction) -> bool { /* ... */ }
    ```

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `SnakeApp`

```rust
pub struct SnakeApp {
    pub(in ::apps::snake) snake: alloc::vec::Vec<(isize, isize)>,
    pub(in ::apps::snake) food: (isize, isize),
    pub(in ::apps::snake) dir: Direction,
    pub(in ::apps::snake) next_dir: Direction,
    pub(in ::apps::snake) game_over: bool,
    pub(in ::apps::snake) move_timer: u8,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `snake` | `alloc::vec::Vec<(isize, isize)>` |  |
| `food` | `(isize, isize)` |  |
| `dir` | `Direction` |  |
| `next_dir` | `Direction` |  |
| `game_over` | `bool` |  |
| `move_timer` | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AppInfo**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn version(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn icon(self: &Self) -> [u32; 1024] { /* ... */ }
    ```

  - ```rust
    fn dimensions(self: &Self) -> (usize, usize) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Runnable**
  - ```rust
    fn logic(self: &mut Self, _vars: &mut Vec<String>) { /* ... */ }
    ```

  - ```rust
    fn draw(self: &Self, graphics: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) { /* ... */ }
    ```

  - ```rust
    fn input(self: &mut Self, key: Key) { /* ... */ }
    ```

- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `doom`

```rust
pub(in ::apps) mod doom { /* ... */ }
```

### Types

#### Struct `DoomApp`

```rust
pub struct DoomApp {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AppInfo**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn version(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn icon(self: &Self) -> [u32; 1024] { /* ... */ }
    ```

  - ```rust
    fn dimensions(self: &Self) -> (usize, usize) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Runnable**
  - ```rust
    fn logic(self: &mut Self, _vars: &mut Vec<String>) { /* ... */ }
    ```

  - ```rust
    fn draw(self: &Self, graphics: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) { /* ... */ }
    ```

  - ```rust
    fn input(self: &mut Self, key: Key) { /* ... */ }
    ```

- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `doom_glue`

```rust
pub mod doom_glue { /* ... */ }
```

## Module `resource_tester`

```rust
pub(in ::apps) mod resource_tester { /* ... */ }
```

### Types

#### Struct `SysTestApp`

```rust
pub struct SysTestApp {
    pub(in ::apps::resource_tester) test_phase: u8,
    pub(in ::apps::resource_tester) cpu_int_score: u64,
    pub(in ::apps::resource_tester) fpu_score: u64,
    pub(in ::apps::resource_tester) mem_verified_mb: u64,
    pub(in ::apps::resource_tester) errors: u64,
    pub(in ::apps::resource_tester) heat_map: [f32; 100],
    pub(in ::apps::resource_tester) rng_state: u64,
    pub(in ::apps::resource_tester) test_buffer: alloc::vec::Vec<u8>,
    pub(in ::apps::resource_tester) mem_raw_buf: alloc::vec::Vec<u64>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `test_phase` | `u8` |  |
| `cpu_int_score` | `u64` |  |
| `fpu_score` | `u64` |  |
| `mem_verified_mb` | `u64` |  |
| `errors` | `u64` |  |
| `heat_map` | `[f32; 100]` |  |
| `rng_state` | `u64` |  |
| `test_buffer` | `alloc::vec::Vec<u8>` |  |
| `mem_raw_buf` | `alloc::vec::Vec<u64>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub(in ::apps::resource_tester) fn next_rng(self: &mut Self) -> u64 { /* ... */ }
  ```

- ```rust
  pub(in ::apps::resource_tester) fn update_heat(self: &mut Self, idx: usize, intensity: f32) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AppInfo**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn version(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn icon(self: &Self) -> [u32; 1024] { /* ... */ }
    ```

  - ```rust
    fn dimensions(self: &Self) -> (usize, usize) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Runnable**
  - ```rust
    fn logic(self: &mut Self, _vars: &mut Vec<String>) { /* ... */ }
    ```

  - ```rust
    fn input(self: &mut Self, key: Key) { /* ... */ }
    ```

  - ```rust
    fn draw(self: &Self, graphics: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) { /* ... */ }
    ```

- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `mc_app`

```rust
pub(in ::apps) mod mc_app { /* ... */ }
```

### Types

#### Struct `Block`

```rust
pub(in ::apps::mc_app) struct Block {
    pub(in ::apps::mc_app) x: f64,
    pub(in ::apps::mc_app) y: f64,
    pub(in ::apps::mc_app) z: f64,
    pub(in ::apps::mc_app) color: u32,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `x` | `f64` |  |
| `y` | `f64` |  |
| `z` | `f64` |  |
| `color` | `u32` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
#### Struct `MinecraftApp`

```rust
pub struct MinecraftApp {
    pub(in ::apps::mc_app) cam_x: f64,
    pub(in ::apps::mc_app) cam_y: f64,
    pub(in ::apps::mc_app) cam_z: f64,
    pub(in ::apps::mc_app) yaw: f64,
    pub(in ::apps::mc_app) pitch: f64,
    pub(in ::apps::mc_app) world: alloc::vec::Vec<Block>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `cam_x` | `f64` |  |
| `cam_y` | `f64` |  |
| `cam_z` | `f64` |  |
| `yaw` | `f64` |  |
| `pitch` | `f64` |  |
| `world` | `alloc::vec::Vec<Block>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub(in ::apps::mc_app) fn draw_block(self: &Self, graphics: &mut PixelGraphics, block: &Block, ox: usize, oy: usize, sy: f64, cy: f64, sp: f64, cp: f64) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AppInfo**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn version(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn icon(self: &Self) -> [u32; 1024] { /* ... */ }
    ```

  - ```rust
    fn dimensions(self: &Self) -> (usize, usize) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Runnable**
  - ```rust
    fn input(self: &mut Self, key: Key) { /* ... */ }
    ```

  - ```rust
    fn draw(self: &Self, graphics: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) { /* ... */ }
    ```

  - ```rust
    fn logic(self: &mut Self, vars: &mut Vec<String>) { /* ... */ }
    ```

- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
### Types

#### Type Alias `AppConstructor`

A type alias for a function that creates a boxed app and returns its preferred window dimensions.

```rust
pub(in ::apps) type AppConstructor = fn() -> (alloc::boxed::Box<dyn Runnable>, (usize, usize));
```

### Constants and Statics

#### Static `APP_REGISTRY`

The Registry: A static list of application names, their constructors, icons, and versions.

```rust
pub(crate) static APP_REGISTRY: &[(&str, fn() -> (alloc::boxed::Box<dyn Runnable>, (usize, usize)), [u32; 1024], &str)] = _;
```

## Module `input`

```rust
pub(crate) mod input { /* ... */ }
```

### Types

#### Enum `ScanCodeV2`

**Attributes:**

- `Repr(AttributeRepr { kind: Rust, align: None, packed: None, int: Some("u16") })`

Taken from uefi 0.36.1 input
A keyboard scan code

Codes 0x8000 -> 0xFFFF are reserved for future OEM extensibility, therefore
this C enum is **_not_** safe to model as a Rust enum (where the compiler must
know about all variants at compile time).

```rust
pub enum ScanCodeV2 {
    NULL = 0x00,
    UP = 0x01,
    DOWN = 0x02,
    RIGHT = 0x03,
    LEFT = 0x04,
    HOME = 0x05,
    END = 0x06,
    INSERT = 0x07,
    DELETE = 0x08,
    PAGE_UP = 0x09,
    PAGE_DOWN = 0x0A,
    FUNCTION_1 = 0x0B,
    FUNCTION_2 = 0x0C,
    FUNCTION_3 = 0x0D,
    FUNCTION_4 = 0x0E,
    FUNCTION_5 = 0x0F,
    FUNCTION_6 = 0x10,
    FUNCTION_7 = 0x11,
    FUNCTION_8 = 0x12,
    FUNCTION_9 = 0x13,
    FUNCTION_10 = 0x14,
    FUNCTION_11 = 0x15,
    FUNCTION_12 = 0x16,
    ESCAPE = 0x17,
    FUNCTION_13 = 0x68,
    FUNCTION_14 = 0x69,
    FUNCTION_15 = 0x6A,
    FUNCTION_16 = 0x6B,
    FUNCTION_17 = 0x6C,
    FUNCTION_18 = 0x6D,
    FUNCTION_19 = 0x6E,
    FUNCTION_20 = 0x6F,
    FUNCTION_21 = 0x70,
    FUNCTION_22 = 0x71,
    FUNCTION_23 = 0x72,
    FUNCTION_24 = 0x73,
    LEFT_CONTROL = 0x1D,
    LEFT_ALT = 0x38,
    MUTE = 0x7F,
    VOLUME_UP = 0x80,
    VOLUME_DOWN = 0x81,
    BRIGHTNESS_UP = 0x100,
    BRIGHTNESS_DOWN = 0x101,
    SUSPEND = 0x102,
    HIBERNATE = 0x103,
    TOGGLE_DISPLAY = 0x104,
    RECOVERY = 0x105,
    EJECT = 0x106,
}
```

##### Variants

###### `NULL`

Null scan code, indicates that the Unicode character should be used.

Discriminant: `0x00`

Discriminant value: `0`

###### `UP`

Move cursor up 1 row.

Discriminant: `0x01`

Discriminant value: `1`

###### `DOWN`

Move cursor down 1 row.

Discriminant: `0x02`

Discriminant value: `2`

###### `RIGHT`

Move cursor right 1 column.

Discriminant: `0x03`

Discriminant value: `3`

###### `LEFT`

Move cursor left 1 column.

Discriminant: `0x04`

Discriminant value: `4`

###### `HOME`

Discriminant: `0x05`

Discriminant value: `5`

###### `END`

Discriminant: `0x06`

Discriminant value: `6`

###### `INSERT`

Discriminant: `0x07`

Discriminant value: `7`

###### `DELETE`

Discriminant: `0x08`

Discriminant value: `8`

###### `PAGE_UP`

Discriminant: `0x09`

Discriminant value: `9`

###### `PAGE_DOWN`

Discriminant: `0x0A`

Discriminant value: `10`

###### `FUNCTION_1`

Discriminant: `0x0B`

Discriminant value: `11`

###### `FUNCTION_2`

Discriminant: `0x0C`

Discriminant value: `12`

###### `FUNCTION_3`

Discriminant: `0x0D`

Discriminant value: `13`

###### `FUNCTION_4`

Discriminant: `0x0E`

Discriminant value: `14`

###### `FUNCTION_5`

Discriminant: `0x0F`

Discriminant value: `15`

###### `FUNCTION_6`

Discriminant: `0x10`

Discriminant value: `16`

###### `FUNCTION_7`

Discriminant: `0x11`

Discriminant value: `17`

###### `FUNCTION_8`

Discriminant: `0x12`

Discriminant value: `18`

###### `FUNCTION_9`

Discriminant: `0x13`

Discriminant value: `19`

###### `FUNCTION_10`

Discriminant: `0x14`

Discriminant value: `20`

###### `FUNCTION_11`

Discriminant: `0x15`

Discriminant value: `21`

###### `FUNCTION_12`

Discriminant: `0x16`

Discriminant value: `22`

###### `ESCAPE`

Discriminant: `0x17`

Discriminant value: `23`

###### `FUNCTION_13`

Discriminant: `0x68`

Discriminant value: `104`

###### `FUNCTION_14`

Discriminant: `0x69`

Discriminant value: `105`

###### `FUNCTION_15`

Discriminant: `0x6A`

Discriminant value: `106`

###### `FUNCTION_16`

Discriminant: `0x6B`

Discriminant value: `107`

###### `FUNCTION_17`

Discriminant: `0x6C`

Discriminant value: `108`

###### `FUNCTION_18`

Discriminant: `0x6D`

Discriminant value: `109`

###### `FUNCTION_19`

Discriminant: `0x6E`

Discriminant value: `110`

###### `FUNCTION_20`

Discriminant: `0x6F`

Discriminant value: `111`

###### `FUNCTION_21`

Discriminant: `0x70`

Discriminant value: `112`

###### `FUNCTION_22`

Discriminant: `0x71`

Discriminant value: `113`

###### `FUNCTION_23`

Discriminant: `0x72`

Discriminant value: `114`

###### `FUNCTION_24`

Discriminant: `0x73`

Discriminant value: `115`

###### `LEFT_CONTROL`

Discriminant: `0x1D`

Discriminant value: `29`

###### `LEFT_ALT`

Discriminant: `0x38`

Discriminant value: `56`

###### `MUTE`

Discriminant: `0x7F`

Discriminant value: `127`

###### `VOLUME_UP`

Discriminant: `0x80`

Discriminant value: `128`

###### `VOLUME_DOWN`

Discriminant: `0x81`

Discriminant value: `129`

###### `BRIGHTNESS_UP`

Discriminant: `0x100`

Discriminant value: `256`

###### `BRIGHTNESS_DOWN`

Discriminant: `0x101`

Discriminant value: `257`

###### `SUSPEND`

Discriminant: `0x102`

Discriminant value: `258`

###### `HIBERNATE`

Discriminant: `0x103`

Discriminant value: `259`

###### `TOGGLE_DISPLAY`

Discriminant: `0x104`

Discriminant value: `260`

###### `RECOVERY`

Discriminant: `0x105`

Discriminant value: `261`

###### `EJECT`

Discriminant: `0x106`

Discriminant value: `262`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Pointee**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Functions

### Function `get_total_physical_memory_mb`

```rust
pub fn get_total_physical_memory_mb() -> u32 { /* ... */ }
```

### Function `main`

**Attributes:**

- `Other("#[allow(dead_code, unused, unused_must_use, non_camel_case_types,\nnonstandard_style)]")`
- `ExportName("efi_main")`

```rust
pub(crate) extern ""efiapi"" fn main(internal_image_handle: ::uefi::Handle, internal_system_table: *const ::core::ffi::c_void) -> Status { /* ... */ }
```

### Function `read_line`

```rust
pub(crate) fn read_line(buf: &mut alloc::string::String) { /* ... */ }
```

### Function `read_line_int`

```rust
pub(crate) fn read_line_int(buf: &mut alloc::string::String) -> i32 { /* ... */ }
```

### Function `enter`

```rust
pub(crate) fn enter(itm: &str) { /* ... */ }
```

### Function `start_kernel`

```rust
pub(crate) fn start_kernel(path: &str) { /* ... */ }
```

### Function `shutdown`

```rust
pub(crate) fn shutdown(mode: char) { /* ... */ }
```

### Function `handle_vm_command`

**Attributes:**

- `Other("#[allow(static_mut_refs)]")`

```rust
pub(crate) fn handle_vm_command(command: &[&str]) { /* ... */ }
```

### Function `handle_vmm_command`

**Attributes:**

- `Other("#[allow(static_mut_refs)]")`

```rust
pub(crate) fn handle_vmm_command(command: &[&str]) { /* ... */ }
```

### Function `boot_vm_with_media`

**Attributes:**

- `Other("#[allow(static_mut_refs)]")`

Boot a VM with an ISO, EFI file, or disk image

```rust
pub(crate) fn boot_vm_with_media(vm_id: u32, media_path: &str) { /* ... */ }
```

### Function `run_efi_application`

**Attributes:**

- `Other("#[allow(static_mut_refs)]")`

Run a standalone EFI application

```rust
pub(crate) fn run_efi_application(efi_path: &str, args: &[&str]) { /* ... */ }
```

### Function `attach_vm_console`

**Attributes:**

- `Other("#[allow(static_mut_refs, dead_code)]")`

Attach to a VM's console for interaction

```rust
pub(crate) fn attach_vm_console(vm_id: u32) { /* ... */ }
```

### Function `load_boot_media`

Load boot media (ISO, EFI, IMG files) from filesystem

```rust
pub(crate) fn load_boot_media(path: &str) -> Result<alloc::vec::Vec<u8>, &''static str> { /* ... */ }
```

### Function `read_boot_file`

**Attributes:**

- `Other("#[allow(dead_code)]")`

Helper function to read a file from the filesystem

```rust
pub(crate) fn read_boot_file(path: &str) -> Result<alloc::vec::Vec<u8>, &''static str> { /* ... */ }
```

### Function `init_mouse`

```rust
pub(crate) fn init_mouse() { /* ... */ }
```

### Function `init_mouse_deep_scan`

```rust
pub(crate) unsafe fn init_mouse_deep_scan() { /* ... */ }
```

### Function `calibrate_tsc`

```rust
pub fn calibrate_tsc() { /* ... */ }
```

## Constants and Statics

### Static `ALLOCATOR`

**Attributes:**

- `Other("#[allow(dead_code, unused)]")`

```rust
pub(crate) static ALLOCATOR: buddy_system_allocator::LockedHeap<32> = _;
```

### Static `HEAP_STORAGE`

**Attributes:**

- `Other("#[allow(dead_code, unused)]")`

```rust
pub(crate) static mut HEAP_STORAGE: [u8; 2097152] = _;
```

### Static `VIRT_STACK`

**Attributes:**

- `Other("#[allow(dead_code, unused)]")`

```rust
pub(crate) static mut VIRT_STACK: [u8; 268435456] = _;
```

### Static `HYPERVISOR`

```rust
pub(crate) static mut HYPERVISOR: Option<vmm::HypervisorManager> = None;
```

### Static `TOTAL_PHYSICAL_MEMORY_MB`

```rust
pub(crate) static mut TOTAL_PHYSICAL_MEMORY_MB: u32 = 0;
```

### Static `TSC_PER_US`

```rust
pub(crate) static mut TSC_PER_US: u64 = 0;
```

## Macros

### Macro `hpvm_log`

**Attributes:**

- `MacroExport`

```rust
pub macro_rules! hpvm_log {
    /* macro_rules! hpvm_log {
    ($color:expr, $prefix:expr, $($arg:tt)*) => { ... };
} */
}
```

### Macro `message`

**Attributes:**

- `MacroExport`

```rust
pub macro_rules! message {
    /* macro_rules! message {
    ($start:expr, $($arg:tt)*) => { ... };
} */
}
```

### Macro `hpvm_info`

**Attributes:**

- `MacroExport`

```rust
pub macro_rules! hpvm_info {
    /* macro_rules! hpvm_info {
    ($tag:expr, $($arg:tt)*) => { ... };
} */
}
```

### Macro `hpvm_warn`

**Attributes:**

- `MacroExport`

```rust
pub macro_rules! hpvm_warn {
    /* macro_rules! hpvm_warn {
    ($tag:expr, $($arg:tt)*) => { ... };
} */
}
```

### Macro `hpvm_error`

**Attributes:**

- `MacroExport`

```rust
pub macro_rules! hpvm_error {
    /* macro_rules! hpvm_error {
    ($tag:expr, $($arg:tt)*) => { ... };
} */
}
```

## Re-exports

### Re-export `lexer`

```rust
pub use crate::micro_c::lexer;
```

### Re-export `parser`

```rust
pub use crate::micro_c::parser;
```

### Re-export `ast`

```rust
pub use crate::micro_c::ast;
```

### Re-export `error`

```rust
pub use crate::micro_c::error;
```

### Re-export `arch`

```rust
pub use crate::micro_c::arch;
```

### Re-export `codegen_ir`

```rust
pub use crate::micro_c::codegen_ir;
```

### Re-export `ir`

```rust
pub use crate::micro_c::ir;
```

### Re-export `regalloc`

```rust
pub use crate::micro_c::regalloc;
```

### Re-export `stackframe`

```rust
pub use crate::micro_c::stackframe;
```

