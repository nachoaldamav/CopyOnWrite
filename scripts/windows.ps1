$driveLetter = "D:"
$volumeLabel = "ReFSVolume"
$diskNumber = 1
Initialize-Disk -Number $diskNumber -PartitionStyle GPT
Write-Host "Initialize disk $diskNumber"
New-Partition -DiskNumber $diskNumber -UseMaximumSize -AssignDriveLetter | Format-Volume -FileSystem ReFS -NewFileSystemLabel $volumeLabel
Write-Host "Create partition on disk $diskNumber"

New-Item -Path "$driveLetter\code" -ItemType Directory
Write-Host "Create code directory at $driveLetter\code"

Write-Host "Start installing rustup"
Invoke-WebRequest -Uri "https://win.rustup.rs" -OutFile "rustup-init.exe"
.\rustup-init.exe -y
Write-Host "Finish installing rustup"

Write-Host "Start installing Chocolatey"
Invoke-WebRequest -Uri "https://chocolatey.org/install.ps1" -OutFile "choco-install.ps1"
.\choco-install.ps1
Write-Host "Finish installing Chocolatey"

Write-Host "Start installing C++ build tools"
choco install -y visualstudio2019buildtools --package-parameters "--add Microsoft.VisualStudio.Component.VC.Tools.x86.x64"
Write-Host "Finish installing C++ build tools"