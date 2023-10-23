# Prepare Disk
Write-Host "::group::Prepare Disk"
$driveLetter = "Z:"
$volumeLabel = "ReFSVolume"
$diskNumber = 1
Initialize-Disk -Number $diskNumber -PartitionStyle GPT
New-Partition -DiskNumber $diskNumber -UseMaximumSize -AssignDriveLetter | Format-Volume -FileSystem ReFS -NewFileSystemLabel $volumeLabel
Write-Host "::endgroup::"

# Create "code" directory in the ReFS volume
Write-Host "::group::Create code directory"
New-Item -Path "$driveLetter\code" -ItemType Directory
Write-Host "::endgroup::"

# Install Rust
Write-Host "::group::Install Rust"
$rustupInit = (New-Object System.Net.WebClient).DownloadString("https://win.rustup.rs")
$rustupInit | Invoke-Expression
Write-Host "::endgroup::"

# Update PATH
Write-Host "::group::Update PATH"
$env:Path = "C:\Users\Administrator\.cargo\bin;$env:Path"
Write-Host "::endgroup::"