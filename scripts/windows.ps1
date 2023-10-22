# Prepare Disk
Write-Host "::group::Prepare Disk"
$driveLetter = "Z:"
$volumeLabel = "ReFSVolume"
$diskNumber = 1 # Assume the ReFS disk is Disk 1. Modify as needed.
Initialize-Disk -Number $diskNumber -PartitionStyle GPT
New-Partition -DiskNumber $diskNumber -UseMaximumSize -AssignDriveLetter | Format-Volume -FileSystem ReFS -NewFileSystemLabel $volumeLabel
Write-Host "::endgroup::"

# Create "code" directory in the ReFS volume
Write-Host "::group::Create code directory"
New-Item -Path "$driveLetter\code" -ItemType Directory
Write-Host "::endgroup::"