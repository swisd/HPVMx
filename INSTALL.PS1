# This script uses native Windows DiskPart to create a bootable VHD
$env:CC = "clang"

Write-Output "Running Build Script..."
./build.bat


Write-Output "Updating docs"
#cargo +nightly rustdoc -p HPVMx -- -Z unstable-options --document-private-items
#cargo +nightly rustdoc -p HPVMx -- -Z unstable-options --output-format json --document-private-items
#rustdoc-md --path ./target/doc/HPVMx.json --output hpvmx.md

Write-Output "Installing..."

$VHD_PATH = ".\boot.vhd"
$MOUNT_LETTER = "X"
$EFI_PATH = "target\x86_64-unknown-uefi\release\HPVMx.efi"

Write-Output "Mounting VHD at $VHD_PATH"

# the vhd must have the id X:\ and be at least 256 MB
#Mount-DiskImage -ImagePath $VHD_PATH -PassThru | Get-Disk | Get-Partition | Where-Object { $_.DriveLetter -ne $MOUNT_LETTER } | Set-Partition -NewDriveLetter $MOUNT_LETTER

# Create a temporary script for diskpart
$dpScript = @"
select vdisk file="$VHD_PATH"
attach vdisk
select partition 1
assign letter=$MOUNT_LETTER
"@

$dpScript | diskpart



# 2. Copy the binary
Write-Output "Copying Binary EFI"
New-Item -Path "$($MOUNT_LETTER):\EFI\BOOT" -ItemType Directory -Force
Copy-Item $EFI_PATH "$($MOUNT_LETTER):\EFI\BOOT\BOOTX64.EFI"
New-Item -Path "$($MOUNT_LETTER):\docs\man" -ItemType Directory -Force
Copy-Item "./doc/manual.md" "$($MOUNT_LETTER):\docs\man\manual.md"

# 3. Detach
Write-Output "Detaching VHD $MOUNT_LETTER"
"select vdisk file=$VHD_PATH",
"detach vdisk" | Out-File -FilePath diskpart.txt -Encoding ascii
Start-Process diskpart -ArgumentList "/s diskpart.txt" -Wait
Dismount-DiskImage -ImagePath $VHD_PATH

Write-Output "Build and Install Complete!"
Write-Output "finished compile/install at $(Get-Date)"