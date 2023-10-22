#!/bin/bash

# Update the package list and install essential packages
sudo apt-get update
sudo apt-get install -y build-essential git curl

# Install Rust and Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Prepare Disk
sudo mkdir -p /mnt/btrfs
sudo mkfs.btrfs /dev/sdb
sudo mount /dev/sdb /mnt/btrfs

# Prepare User
sudo useradd -m ghaction

# SSH Setup for the new user
sudo mkdir -p ~ghaction/.ssh
sudo touch ~ghaction/.ssh/authorized_keys
sudo chown -R ghaction:ghaction ~ghaction/.ssh
sudo chmod 700 ~ghaction/.ssh
sudo chmod 600 ~ghaction/.ssh/authorized_keys

# Ensure that the user has access to the btrfs volume
sudo chown ghaction:ghaction /mnt/btrfs
