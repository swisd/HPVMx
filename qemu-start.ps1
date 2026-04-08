# qemu-system-x86_64 `
#     -m 1G `
#     -drive if=pflash,format=raw,readonly=on,file=code.fd `
#     -drive if=pflash,format=raw,file=vars.fd `
#     -drive file=boot.vhd,format=vpc `
#     -device usb-ehci `
#     -device usb-tablet `
#     -net none

#-device qemu-xhci,id=xhci0 -device usb-tablet,bus=xhci0.0

# -display sdl

cmd.exe /c '"C:\Program Files\qemu\qemu-system-x86_64" -m 1G -drive if=pflash,format=raw,readonly=on,file=code.fd -drive if=pflash,format=raw,file=vars.fd -drive file=boot.vhd,format=vpc -device qemu-xhci,id=xhci0 -device usb-tablet,bus=xhci0.0 -netdev user,id=net0,dhcpstart=10.0.2.15 -device rtl8139,netdev=net0 '
