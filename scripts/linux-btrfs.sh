#!/bin/bash

# Prepare Disk
echo "::group::Prepare Disk"
sudo mkdir -p /mnt/btrfs
sudo mkfs.btrfs /dev/sdb
sudo mount /dev/sdb /mnt/btrfs
echo "::endgroup::"

# Prepare User
echo "::group::Prepare User"
sudo useradd -m ghaction
echo "::endgroup::"

# SSH Setup for the new user
echo "::group::SSH Setup for the new user"
sudo mkdir -p ~ghaction/.ssh
sudo touch ~ghaction/.ssh/authorized_keys
sudo chown -R ghaction:ghaction ~ghaction/.ssh
sudo chmod 700 ~ghaction/.ssh
sudo chmod 600 ~ghaction/.ssh/authorized_keys
echo "::endgroup::"

# Ensure that the user has access to the btrfs volume
sudo chown ghaction:ghaction /mnt/btrfs

# Create "code" directory in the btrfs volume
sudo mkdir -p /mnt/btrfs/code
sudo chown ghaction:ghaction /mnt/btrfs/code