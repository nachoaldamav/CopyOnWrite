use copy_on_write::reflink_file_sync;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use tempfile::Builder;

#[test]
fn benchmark_copy_vs_reflink() {
    // 100 MB file size
    const FILE_SIZE: usize = 100 * 1024 * 1024;
    
    // Use current dir to ensure we are on the same volume (likely Dev Drive or ReFS if user is testing this)
    let tmp_dir = Builder::new()
        .prefix("cow_perf_test")
        .tempdir_in(std::env::current_dir().unwrap())
        .expect("Failed to create temp dir");

    let src_path = tmp_dir.path().join("source.dat");
    let dest_copy_path = tmp_dir.path().join("dest_copy.dat");
    let dest_reflink_path = tmp_dir.path().join("dest_reflink.dat");

    println!("Generating {} MB file...", FILE_SIZE / 1024 / 1024);
    {
        let mut file = File::create(&src_path).expect("Failed to create source file");
        // Write in chunks to avoid massive memory usage
        let chunk_size = 1024 * 1024; // 1 MB
        let chunk = vec![0u8; chunk_size];
        for _ in 0..(FILE_SIZE / chunk_size) {
            file.write_all(&chunk).expect("Failed to write to source file");
        }
    }
    
    // Warm up / ensure file is flushed
    let _ = std::fs::metadata(&src_path).unwrap();

    // Benchmark std::fs::copy
    let start_copy = Instant::now();
    std::fs::copy(&src_path, &dest_copy_path).expect("std::fs::copy failed");
    let duration_copy = start_copy.elapsed();
    println!("std::fs::copy time: {:?}", duration_copy);

    // Benchmark reflink_file_sync
    let start_reflink = Instant::now();
    match reflink_file_sync(src_path.to_str().unwrap(), dest_reflink_path.to_str().unwrap()) {
        Ok(_) => {
            let duration_reflink = start_reflink.elapsed();
            println!("reflink_file_sync time: {:?}", duration_reflink);
            
            if duration_reflink < duration_copy {
                 println!("CoW was {:.2}x faster than std::fs::copy", duration_copy.as_secs_f64() / duration_reflink.as_secs_f64());
            } else {
                 println!("CoW was slower or equal to std::fs::copy (might not be supported on this drive)");
            }
        },
        Err(e) => {
            println!("reflink_file_sync failed: {}", e);
            // Don't fail the test if CoW isn't supported, just report it
            if e.to_string().contains("not support copy-on-write") {
                println!("Skipping CoW check as volume does not support it.");
            } else {
                panic!("reflink_file_sync failed unexpectedly: {}", e);
            }
        }
    }

    // Cleanup happens automatically when tmp_dir goes out of scope
}
