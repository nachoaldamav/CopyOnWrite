mod platform;
mod common;

use common::utils::absolute;

#[cfg(target_os = "windows")]
use platform::windows::reflink_sync;

#[cfg(target_os = "linux")]
use platform::linux::reflink_sync;

#[cfg(target_os = "macos")]
use platform::macos::reflink_sync;

pub fn reflink_file_sync(src: &str, dest: &str) -> std::io::Result<()> {
    let src_path = absolute(src)?;
    let dest_path = absolute(dest)?;

    reflink_sync(
        src_path.to_str().unwrap_or_default(),
        dest_path.to_str().unwrap_or_default(),
    )
}
