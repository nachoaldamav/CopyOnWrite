mod common;
mod platform;

use std::path::PathBuf;
use common::utils::absolute;

#[cfg(target_os = "windows")]
use platform::windows::reflink_sync;

#[cfg(not(target_os = "windows"))]
use reflink_copy::reflink as reflink_sync;

pub fn reflink_file_sync(src: &str, dest: &str) -> std::io::Result<()> {
    // Convert to absolute paths
    let src_abs: PathBuf = absolute(src)?;
    let dest_abs: PathBuf = absolute(dest)?;

    #[cfg(target_os = "windows")]
    {
        let src_str = src_abs.to_str().unwrap_or_default();
        let dest_str = dest_abs.to_str().unwrap_or_default();
        reflink_sync(src_str, dest_str)
    }

    #[cfg(not(target_os = "windows"))]
    {
        reflink_sync(&src_abs, &dest_abs)
    }
}
