# This script uses native Windows DiskPart to create a bootable VHD for VirtualBox
$VHD_PATH = "C:\RS\HPVMx\boot.vhd"
$MOUNT_LETTER = "X"
$EFI_PATH = "target\x86_64-unknown-uefi\release\HPVMx.efi"

# 1. Create a 64MB VHD
"create vdisk file=$VHD_PATH maximum=64 type=fixed",
"select vdisk file=$VHD_PATH",
"attach vdisk",
"convert gpt",
"create partition efi",
"format quick fs=fat32 label=SYSTEM",
"assign letter=$MOUNT_LETTER" | Out-File -FilePath diskpart.txt -Encoding ascii

Start-Process diskpart -ArgumentList "/s diskpart.txt" -Wait

# 2. Copy the binary
New-Item -Path "$($MOUNT_LETTER):\EFI\BOOT" -ItemType Directory -Force
Copy-Item $EFI_PATH "$($MOUNT_LETTER):\EFI\BOOT\BOOTX64.EFI"

# 3. Detach
"select vdisk file=$VHD_PATH",
"detach vdisk" | Out-File -FilePath diskpart.txt -Encoding ascii
Start-Process diskpart -ArgumentList "/s diskpart.txt" -Wait