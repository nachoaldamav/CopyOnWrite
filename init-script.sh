#!/bin/bash

# Update the package list and install essential packages
sudo apt-get update
sudo apt-get install -y build-essential git curl

# Install Rust and Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env