# CopyOnWrite (CoW) in Rust

## Status

| OS          | File System | Status                                                                                                                                                                           |
|-------------|-------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Linux       | btrfs       | [![E2E Linux (btrfs)](https://github.com/nachoaldamav/CopyOnWrite/actions/workflows/linux.yml/badge.svg?branch=main)](https://github.com/nachoaldamav/CopyOnWrite/actions/workflows/linux.yml)  |
| Linux       | xfs         | [![E2E Linux (xfs)](https://github.com/nachoaldamav/CopyOnWrite/actions/workflows/linux.yml/badge.svg?branch=main)](https://github.com/nachoaldamav/CopyOnWrite/actions/workflows/linux.yml)        |
| Windows     | ReFS        | [![E2E Windows (ReFS)](https://github.com/nachoaldamav/CopyOnWrite/actions/workflows/windows.yml/badge.svg?branch=main)](https://github.com/nachoaldamav/CopyOnWrite/actions/workflows/windows.yml) |
| MacOS       | APFS        | |

> Note: All the tests run in Google Cloud VMs with the correct filesystem, this way we ensure the tests run correctly.

## Description

This project is an implementation of Copy-On-Write (CoW) or reflinks in Rust. It provides file copy functionalities that are optimized for different file systems and operating systems.

### Features

- Unix/MacOS: Uses the `reflink-copy` crate for CoW support.
- Windows: Native implementation supporting ReFS drives (Windows Server 2016+ and Windows 11 Dev Drives).

## Usage Demo

Here's a simple demo showing how to use this library.

```rust
use copy_on_write::reflink_file_sync;

reflink_file_sync(src, dst);
```
