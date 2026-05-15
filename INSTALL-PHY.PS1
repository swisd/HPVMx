# This script uses native Windows DiskPart to create a bootable VHD
$env:CC = "clang"

Write-Output "Running Build Script..."
./build.bat


Write-Output "Updating docs"
#cargo +nightly rustdoc -p HPVMx -- -Z unstable-options --document-private-items
#cargo +nightly rustdoc -p HPVMx -- -Z unstable-options --output-format json --document-private-items
#rustdoc-md --path ./target/doc/HPVMx.json --output hpvmx.md

Write-Output "Installing..."

$MOUNT_LETTER = "X"
$EFI_PATH = "target\x86_64-unknown-uefi\release\HPVMx.efi"




# 2. Copy the binary
Write-Output "Copying Binary EFI"
New-Item -Path "$($MOUNT_LETTER):\EFI\BOOT" -ItemType Directory -Force
Copy-Item $EFI_PATH "$($MOUNT_LETTER):\EFI\BOOT\BOOTX64.EFI"
New-Item -Path "$($MOUNT_LETTER):\docs\man" -ItemType Directory -Force
Copy-Item "./doc/manual.md" "$($MOUNT_LETTER):\docs\man\manual.md"

Dismount-DiskImage -ImagePath $VHD_PATH

Write-Output "Build and Install Complete!"
Write-Output "finished compile/install at $(Get-Date)"