#!/bin/bash

# Prepare Disk
echo "::group::Prepare Disk"
sudo mkdir -p /mnt/btrfs
sudo mkfs.btrfs /dev/sdb
sudo mount /dev/sdb /mnt/btrfs
echo "::endgroup::"

# Create "code" directory in the btrfs volume
sudo mkdir -p /mnt/btrfs/code

# Give ownership to the current user
sudo chown -R $USER:$USER /mnt/btrfs