use alloc::format;
use alloc::string::{String, ToString};
use crate::error::error;

pub fn open_file_or_lib(path: &str) -> String {
    match path {
        "Sys" | "sys" => {
            let mut s = String::new();
            s.push_str("extern fn printf(fmt, value);\n");
            s.push_str("extern fn malloc(size);\n");
            s.push_str("extern fn free(ptr);\n");
            s.push_str("extern fn exit(code);\n");
            s.push_str("extern fn memcpy(dest, src, n);\n");
            s.push_str("extern fn memset(s, c, n);\n");
            s
        }
        "Refs" | "refs" => {
            let mut s = String::new();
            s.push_str("extern fn ref_inc(ptr);\n");
            s.push_str("extern fn ref_dec(ptr);\n");
            s.push_str("extern fn ref_get(ptr);\n");
            s
        }
        "FS" | "fs" => {
            let mut s = String::new();
            s.push_str("extern fn fs_read_file(path);\n");
            s.push_str("extern fn fs_write_file(path, data);\n");
            s.push_str("extern fn fs_list_files();\n");
            s.push_str("extern fn fs_remove(path);\n");
            s.push_str("extern fn fs_mkdir(path);\n");
            s
        }
        "OS" | "os" => {
            let mut s = String::new();
            s.push_str("struct VM {\n");
            s.push_str("    id: i64;\n");
            s.push_str("    state: i64;\n");
            s.push_str("    memory_mb: i64;\n");
            s.push_str("    vcpu_count: i64;\n");
            s.push_str("}\n");
            s.push_str("extern fn os_get_vm(id);\n");
            s.push_str("extern fn os_save_vm(vm);\n");
            s.push_str("extern fn os_list_vms();\n");
            s
        }
        "UI" | "ui" => {
            let mut s = String::new();
            s.push_str("extern fn ui_draw_pixel(x, y, color);\n");
            s.push_str("extern fn ui_fill_rect(x, y, w, h, color);\n");
            s.push_str("extern fn ui_draw_line(x1, y1, x2, y2, color);\n");
            s.push_str("extern fn ui_draw_text(x, y, text, color);\n");
            s.push_str("extern fn ui_clear(color);\n");
            s.push_str("extern fn ui_get_resolution_x();\n");
            s.push_str("extern fn ui_get_resolution_y();\n");
            s
        }
        _ => {
            error(&format!("cant open file {}", path));
            "".to_string()
        }
    }
}