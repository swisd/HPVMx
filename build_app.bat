@echo off

set /p name=filename (no-ext):

clang -target x86_64-pc-win32-elf -mno-red-zone -ffreestanding -fno-stack-protector -fno-builtin -fno-unwind-tables -fno-asynchronous-unwind-tables -ffunction-sections -c executables/%name%.c -o executables/%name%.o
ld.lld -T executables/link/hpx_link.ld -o executables/%name%.elf executables/%name%.o
llvm-nm -n --defined-only executables/%name%.elf
llvm-objcopy -O binary executables/%name%.elf executables/%name%.bin

set /p userInput=step,draw,input (nospaces, full-len):
for /f "tokens=1-3 delims=," %%a in ("%userInput%") do (
    set "var1=%%a"
    set "var2=%%b"
    set "var3=%%c"
)

set /p width=width in px:
set /p height=height in px:

python tools/hpx_pack.py executables/%name%.bin executables/%name%.hpx --kind app --state-size 16 --step %var1% --draw %var2% --input %var3% --width %width% --height %height%