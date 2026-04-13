@echo off
echo copying files...
copy %cd%\src\graphics.rs %cd%\src\ui\graphics.rs
copy %cd%\src\hardware\vmx.rs %cd%\src\hardware\cpu\vmx.rs
robocopy %cd%\..\Micro-C\src %cd%\src\micro_c\ /E /XF "error.rs" "main.rs"
echo building...
cargo build --target x86_64-unknown-uefi --release