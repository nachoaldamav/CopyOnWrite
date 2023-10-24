#!/bin/bash

# Prepare Disk
echo "::group::Prepare Disk"
sudo mkdir -p /mnt/xfs
sudo mkfs.xfs /dev/sdb
sudo mount /dev/sdb /mnt/xfs
sudo xfs_io -c 'reflink on' /mnt/xfs
echo "::endgroup::"

# Create "code" directory in the xfs volume
sudo mkdir -p /mnt/xfs/code