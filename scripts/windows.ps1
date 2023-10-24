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

Write-Host "Start installing C++ build tools"
Invoke-WebRequest -Uri "https://aka.ms/vs/17/release/vs_community.exe" -OutFile "vs_installer.exe"
Start-Process -FilePath "vs_installer.exe" -ArgumentList "install --path install='C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools' --add Microsoft.VisualStudio.Workload.VCTools --quiet --norestart" -Wait
Write-Host "Finish installing C++ build tools"