extern crate libc;

use std::ffi::CString;
use std::fs::File;
use std::io::{Error, Result};
use std::os::unix::io::AsRawFd;

extern "C" {
    fn ioctl(fd: libc::c_int, request: libc::c_ulong, ...) -> libc::c_int;
}

const FICLONE: libc::c_ulong = 0x40049409;

pub fn reflink_sync(src: &str, dest: &str) -> Result<()> {
    // Open source and destination files
    let src_file = File::open(src)?;
    let dest_file = File::create(dest)?;

    // Extract raw file descriptors
    let src_fd = src_file.as_raw_fd();
    let dest_fd = dest_file.as_raw_fd();

    // Perform the cloning operation
    let ret = unsafe { ioctl(dest_fd, FICLONE, src_fd) };

    if ret == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}
