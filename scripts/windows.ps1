# Prepare Disk
Write-Host "::group::Prepare Disk"
$driveLetter = "Z:"
$volumeLabel = "ReFSVolume"
$diskNumber = 1 # Assume the ReFS disk is Disk 1. Modify as needed.
Initialize-Disk -Number $diskNumber -PartitionStyle GPT
New-Partition -DiskNumber $diskNumber -UseMaximumSize -AssignDriveLetter | Format-Volume -FileSystem ReFS -NewFileSystemLabel $volumeLabel
Write-Host "::endgroup::"

# Prepare User
Write-Host "::group::Prepare User"
$userName = "ghaction"
$password = ConvertTo-SecureString "YourPasswordHere" -AsPlainText -Force
New-LocalUser -Name $userName -Password $password
Add-LocalGroupMember -Group "Administrators" -Member $userName
Write-Host "::endgroup::"

# SSH Setup for the new user
Write-Host "::group::SSH Setup for the new user"
New-Item -Path "C:\Users\$userName\.ssh" -ItemType Directory
# Add authorized SSH keys if needed
# Set-Content -Path "C:\Users\$userName\.ssh\authorized_keys" -Value "YourPublicKeyHere"
Write-Host "::endgroup::"

# Ensure that the user has access to the ReFS volume
icacls "$driveLetter\" /grant "$userName:F"

# Create "code" directory in the ReFS volume
New-Item -Path "$driveLetter\code" -ItemType Directory
icacls "$driveLetter\code" /grant "$userName:F"
