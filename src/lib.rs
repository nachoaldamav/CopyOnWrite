mod common;
mod platform;

use common::utils::absolute;
use std::path::PathBuf;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::Builder;

    fn create_file(path: &str) {
        let mut file = File::create(path).unwrap();
        file.write_all(b"Hello, world!").unwrap();
    }

    #[test]
    fn test_reflink_file_sync() {
        // Create a temporary directory inside the Current Working Directory
        let tmp_dir = Builder::new()
            .prefix("test_reflink_file_sync")
            .tempdir()
            .unwrap();
        let src = tmp_dir.path().join("test_reflink_file_sync_src");
        let dest = tmp_dir.path().join("test_reflink_file_sync_dest");

        // Create a file
        create_file(src.to_str().unwrap());

        // Reflink the file
        reflink_file_sync(src.to_str().unwrap(), dest.to_str().unwrap()).unwrap();

        // Cleanup
        tmp_dir.close().unwrap();
    }
}
