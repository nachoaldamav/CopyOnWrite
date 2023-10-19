use std::env;
use std::path::PathBuf;

fn to_absolute_path(relative_path: &str) -> std::io::Result<PathBuf> {
    let current_dir = env::current_dir()?;
    Ok(current_dir.join(relative_path))
}

pub fn absolute(path: &str) -> std::io::Result<PathBuf> {
    if PathBuf::from(path).is_absolute() {
        Ok(PathBuf::from(path))
    } else {
        to_absolute_path(path)
    }
}