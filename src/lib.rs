mod common;
mod platform;

use common::utils::absolute;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
use platform::windows::reflink_sync;

#[cfg(not(target_os = "windows"))]
use reflink_copy::reflink as reflink_sync;

pub fn reflink_file_sync(src: impl AsRef<str>, dest: impl AsRef<str>) -> std::io::Result<()> {
    // Clone the src and dest before moving them
    let src_clone = src.as_ref().to_string();
    let dest_clone = dest.as_ref().to_string();

    // Convert to absolute paths
    let src_abs: PathBuf = absolute(&src_clone)?;
    let dest_abs: PathBuf = absolute(&dest_clone)?;

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
            .prefix("test_reflink")
            .tempdir_in(std::env::current_dir().unwrap())
            .unwrap();
        let src = tmp_dir.path().join("src.txt");
        let dest = tmp_dir.path().join("dest.txt");

        // Create a file
        create_file(src.to_str().unwrap());

        // Reflink the file
        reflink_file_sync(src.to_str().unwrap(), dest.to_str().unwrap()).unwrap();

        // Cleanup
        tmp_dir.close().unwrap();
    }
}
