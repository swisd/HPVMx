# This script uses native Windows DiskPart to create a bootable VHD for VirtualBox

./build.bat

echo Installing...

$VHD_PATH = "C:\RS\HPVMx\boot.vhd"
$MOUNT_LETTER = "X"
$EFI_PATH = "target\x86_64-unknown-uefi\release\HPVMx.efi"

# the vhd must have the id X:\ and be at least 256 MB
Mount-DiskImage -ImagePath $VHD_PATH

# 2. Copy the binary
New-Item -Path "$($MOUNT_LETTER):\EFI\BOOT" -ItemType Directory -Force
Copy-Item $EFI_PATH "$($MOUNT_LETTER):\EFI\BOOT\BOOTX64.EFI"

# 3. Detach
"select vdisk file=$VHD_PATH",
"detach vdisk" | Out-File -FilePath diskpart.txt -Encoding ascii
Start-Process diskpart -ArgumentList "/s diskpart.txt" -Wait

Dismount-DiskImage -ImagePath $VHD_PATH