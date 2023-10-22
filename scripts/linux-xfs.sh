#!/bin/bash

# Update the package list and install essential packages
echo "::group::Update the package list and install essential packages"
sudo apt-get update
sudo apt-get install -y build-essential git curl xfsprogs
echo "::endgroup::"

# Prepare Disk
echo "::group::Prepare Disk"
sudo mkdir -p /mnt/xfs
sudo mkfs.xfs /dev/sdb
sudo mount /dev/sdb /mnt/xfs
sudo xfs_io -c 'reflink on' /mnt/xfs
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

# Ensure that the user has access to the xfs volume
sudo chown ghaction:ghaction /mnt/xfs

# Create "code" directory in the xfs volume
sudo mkdir -p /mnt/xfs/code
sudo chown ghaction:ghaction /mnt/xfs/code
