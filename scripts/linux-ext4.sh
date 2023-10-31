#!/bin/bash

# Prepare Disk
echo "::group::Prepare Disk"

# Unmount the disk if it's mounted
sudo umount /dev/sdb1 2>/dev/null || true

# Create a new ext4 filesystem on the drive
sudo mkfs.ext4 /dev/sdb1

# Create a directory to serve as the mount point
sudo mkdir -p /mnt/ext4

# Mount the drive
sudo mount /dev/sdb1 /mnt/ext4

echo "::endgroup::"

# Create a "code" directory in the ext4 volume
sudo mkdir -p /mnt/ext4/code

# Give ownership to the current user
sudo chown -R $USER:$USER /mnt/ext4/code

# Give full permissions to the current user
sudo chmod -R 777 /mnt/ext4/code

