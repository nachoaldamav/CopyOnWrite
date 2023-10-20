mod common;
mod platform;

use common::utils::absolute;

#[cfg(target_os = "windows")]
use platform::windows::reflink_sync;

#[cfg(target_os = "linux")]
use reflink_copy::reflink as reflink_sync;

#[cfg(target_os = "macos")]
use reflink::reflink as reflink_sync;

pub fn reflink_file_sync(src: &str, dest: &str) -> std::io::Result<()> {
    reflink_sync(
        absolute(src)?.to_str().unwrap_or_default(),
        absolute(dest)?.to_str().unwrap_or_default(),
    )
}
