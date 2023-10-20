use std::io;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
fn sanitize_windows_path(path: PathBuf) -> io::Result<PathBuf> {
    let mut path_str = path.to_string_lossy().to_string();

    if path_str.len() >= 260 {
        // Handling code for long paths on Windows
        path_str = format!("\\\\?\\{}", path_str);
    }

    Ok(PathBuf::from(path_str))
}

#[cfg(not(target_os = "windows"))]
fn sanitize_windows_path(path: PathBuf) -> io::Result<PathBuf> {
    // No-op for non-Windows platforms
    Ok(path)
}

fn to_absolute_path(relative_path: &str) -> io::Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    Ok(current_dir.join(relative_path))
}

pub fn absolute(path: &str) -> io::Result<PathBuf> {
    let path_buf = PathBuf::from(path);
    let final_path = if path_buf.is_relative() {
        to_absolute_path(path)?
    } else {
        path_buf
    };

    sanitize_windows_path(final_path)
}

// Testing code
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_absolute() {
        let test_path = "some/relative/path";
        let abs_path = absolute(test_path);
        assert!(abs_path.is_ok());
        assert!(abs_path.unwrap().is_absolute());
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_absolute_long_path() {
        let test_path = String::from_utf8(vec![b'a'; 300]).unwrap();
        let abs_path = absolute(&test_path);
        assert!(abs_path.is_ok());
        assert!(abs_path.unwrap().to_string_lossy().len() >= 260);
    }
}
