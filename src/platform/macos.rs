extern crate libc;

use std::ffi::CString;
use std::fs::File;
use std::io::{Error, Result};

extern "C" {
    fn fcntl(fd: libc::c_int, cmd: libc::c_int, arg: libc::c_ulong) -> libc::c_int;
}

const F_CLONEFILE: libc::c_int = 0x4000;

pub fn reflink_sync(src: &str, dest: &str) -> Result<()> {
    let c_src = CString::new(src)?;
    let c_dest = CString::new(dest)?;

    let src_fd = unsafe { libc::open(c_src.as_ptr(), libc::O_RDONLY) };
    let dest_fd = unsafe { libc::open(c_dest.as_ptr(), libc::O_WRONLY | libc::O_CREAT, 0o644) };

    if src_fd < 0 || dest_fd < 0 {
        return Err(Error::last_os_error());
    }

    let result = unsafe { fcntl(dest_fd, F_CLONEFILE, src_fd as libc::c_ulong) };

    unsafe {
        libc::close(src_fd);
        libc::close(dest_fd);
    }

    if result < 0 {
        return Err(Error::last_os_error());
    }

    Ok(())
}