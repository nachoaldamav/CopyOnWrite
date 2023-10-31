#!/bin/bash

# Prepare Disk
echo "::group::Prepare Disk"
sudo mkdir -p /mnt/ext4
sudo mkfs.ext4 /dev/sdb
sudo mount /dev/sdb /mnt/ext4
echo "::endgroup::"

sudo mkdir -p /mnt/ext4/code
sudo chown -R $USER:$USER /mnt/ext4/code
sudo chmod -R 777 /mnt/ext4/code
