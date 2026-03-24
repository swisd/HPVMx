# qemu-system-x86_64 `
#     -m 1G `
#     -drive if=pflash,format=raw,readonly=on,file=code.fd `
#     -drive if=pflash,format=raw,file=vars.fd `
#     -drive file=boot.vhd,format=vpc `
#     -device usb-ehci `
#     -device usb-tablet `
#     -net none
cmd.exe /c '"C:\Program Files\qemu\qemu-system-x86_64" -m 1G -drive if=pflash,format=raw,readonly=on,file=code.fd -drive if=pflash,format=raw,file=vars.fd -drive file=boot.vhd,format=vpc -device qemu-xhci -device usb-host,vendorid=0x045E,productid=0x07B2 -net none'


