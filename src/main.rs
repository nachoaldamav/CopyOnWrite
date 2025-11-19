mod common;
mod platform;

extern crate env_logger;

use futures::future::join_all;
use tokio::fs::{self, File};
use tokio::io::{self, AsyncWriteExt};
use tokio::time::Instant;

#[cfg(target_os = "windows")]
use platform::windows::reflink_sync;

#[cfg(target_os = "linux")]
use reflink_copy::reflink as reflink_sync;

#[cfg(target_os = "macos")]
use reflink_copy::reflink as reflink_sync;

use common::utils::absolute;
use log::{debug, info};
use std::path::PathBuf;

async fn create_and_fill_file(path: PathBuf, size: usize) -> io::Result<()> {
    let mut file = File::create(path).await?;
    let data = vec![0u8; size];
    file.write_all(&data).await?;
    file.sync_all().await
}

async fn benchmark_operation<F, Fut>(
    operation: F,
    src: PathBuf,
    dest: PathBuf,
    count: usize,
) -> Vec<std::time::Duration>
where
    F: Fn(PathBuf, PathBuf) -> Fut + Copy,
    Fut: std::future::Future<Output = io::Result<()>>,
{
    let mut durations = Vec::new();
    for _ in 0..count {
        let start = Instant::now();
        operation(src.clone(), dest.clone()).await.unwrap();
        durations.push(start.elapsed());
        fs::remove_file(dest.clone()).await.unwrap();
    }
    durations
}

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Starting up");

    let file_count = 10; // Number of files to test
    let file_size = 1 * 1024 * 1024; // Size of each file (1 MB)

    let mut src_files = Vec::new();
    let mut dest_files = Vec::new();

    // Prepare test files
    for i in 0..file_count {
        let src = absolute(&format!("testfile_{}.txt", i)).unwrap();
        let dest = absolute(&format!("testfile_{}_copy.txt", i)).unwrap();
        create_and_fill_file(src.clone(), file_size).await.unwrap();
        src_files.push(src);
        dest_files.push(dest);
    }

    // Benchmark Reflink
    let reflink_futures = src_files
        .iter()
        .zip(dest_files.iter())
        .map(|(src, dest)| {
            benchmark_operation(
                |src, dest| async move {
                    debug!("RefLinking {} to {}", src.display(), dest.display());
                    reflink_sync(src.to_str().unwrap(), dest.to_str().unwrap())
                },
                src.clone(),
                dest.clone(),
                file_count,
            )
        })
        .collect::<Vec<_>>();

    // Benchmark STD Copy
    let std_copy_futures = src_files
        .iter()
        .zip(dest_files.iter())
        .map(|(src, dest)| {
            benchmark_operation(
                |src, dest| async move {
                    debug!("Copying {} to {}", src.display(), dest.display());
                    fs::copy(src, dest).await.map(|_| ())
                },
                src.clone(),
                dest.clone(),
                file_count,
            )
        })
        .collect::<Vec<_>>();

    // Await all futures for completion
    let reflink_results = join_all(reflink_futures).await;
    let std_copy_results = join_all(std_copy_futures).await;

    // Print results
    print_results("RefLink", &reflink_results);
    print_results("STD Copy", &std_copy_results);

    // Clean up
    for src in src_files {
        fs::remove_file(src).await.unwrap();
    }

    info!("Shutting down");
}

fn print_results(operation_name: &str, results: &Vec<Vec<std::time::Duration>>) {
    let mut all_durations = Vec::new();
    for result in results {
        all_durations.extend(result.iter());
    }

    let total_durations: Vec<u128> = all_durations.iter().map(|&d: &std::time::Duration| d.as_millis()).collect();
    let average: u128 = total_durations.iter().sum::<u128>() / total_durations.len() as u128;
    let max = total_durations.iter().max().unwrap_or(&0);
    let min = total_durations.iter().min().unwrap_or(&0);

    println!("{} Results: Average = {} ms, Min = {} ms, Max = {} ms", operation_name, average, min, max);
}