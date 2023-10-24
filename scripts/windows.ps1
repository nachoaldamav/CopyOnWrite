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

$env:Path = "C:\Users\Administrator\.cargo\bin;$env:Path"
Write-Host "Added cargo to path"