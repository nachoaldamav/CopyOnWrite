#!/bin/bash

# Install required package for OCFS2
echo "::group::Install OCFS2 Tools"
sudo apt-get update
sudo apt-get install -y ocfs2-tools ocfs2console
echo "::endgroup::"

# Configure Cluster
echo "::group::Configure Cluster"
sudo o2cb init-node test-node  
sudo o2cb add-cluster test-cluster
sudo o2cb add-node test-cluster test-node --ip 127.0.0.1
sudo o2cb heartbeat-mode test-cluster global
sudo o2cb online-cluster test-cluster
echo "::endgroup::"

# Prepare Disk
echo "::group::Prepare Disk"
sudo mkdir -p /mnt/ocfs2
sudo mkfs.ocfs2 -b 4K -C 32K -L "OCFS2Volume" /dev/sdb
sudo mount -t ocfs2 /dev/sdb /mnt/ocfs2
echo "::endgroup::"

# Create "code" directory in the OCFS2 volume
sudo mkdir -p /mnt/ocfs2/code

# Give ownership to the current user
sudo chown -R $USER:$USER /mnt/ocfs2/code

# Give full permissions to the current user
sudo chmod -R 777 /mnt/ocfs2/code

