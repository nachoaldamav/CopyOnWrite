mod common;
mod platform;

extern crate env_logger;

#[cfg(target_os = "windows")]
use platform::windows::reflink_sync;

#[cfg(target_os = "linux")]
use reflink_copy::reflink as reflink_sync;

#[cfg(target_os = "macos")]
use reflink_copy::reflink as reflink_sync;

use common::utils::absolute;
use log::info;
use std::io::{self, Read};

fn main() {
    env_logger::init();

    info!("Starting up");

    // Convert relative paths to absolute paths
    let src_absolute = absolute("my-file.txt").unwrap();
    let dest_absolute = absolute("my-file-copy.txt").unwrap();

    // Create a file with some data (1 MB)
    info!("Creating a file with some data (1 MB)");
    let mut file = std::fs::File::create(&src_absolute).unwrap();
    let mut src = io::repeat(65).take(1 * 1024 * 1024); // 1 MB
    io::copy(&mut src, &mut file).unwrap();

    // Close the file
    info!("Closing the file");
    drop(file);

    // Wait 2 seconds to warm up the disk cache
    info!("Warming up the disk cache");
    std::thread::sleep(std::time::Duration::from_secs(2));

    let start_time = std::time::Instant::now();

    let result = reflink_sync(
        src_absolute.to_str().unwrap_or_default(),
        dest_absolute.to_str().unwrap_or_default(),
    );

    let elapsed = start_time.elapsed();

    match result {
        Ok(_) => println!("Success! Elapsed time: {:?}", elapsed),
        Err(e) => println!("Error: {}", e),
    }

    // Remove both files
    std::fs::remove_file(src_absolute).unwrap();
    std::fs::remove_file(dest_absolute).unwrap();
}
