use std::collections::HashMap;
use std::sync::Mutex;
use std::{
    ffi::c_void,
    mem::{size_of, MaybeUninit},
};
use widestring::U16CString;
use windows::Win32::Foundation::CloseHandle;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{GENERIC_READ, GENERIC_WRITE, HANDLE},
        Storage::FileSystem::{
            CreateFileW, FileEndOfFileInfo, FileStandardInfo, GetDiskFreeSpaceW,
            GetFileInformationByHandleEx, GetFileSizeEx, GetVolumeInformationW,
            SetFileInformationByHandle, CREATE_NEW, FILE_ATTRIBUTE_NORMAL, FILE_BASIC_INFO,
            FILE_END_OF_FILE_INFO, FILE_FLAG_NO_BUFFERING, FILE_SHARE_DELETE, FILE_SHARE_READ,
            FILE_SHARE_WRITE, OPEN_EXISTING,
        },
        System::{
            Ioctl::{
                DUPLICATE_EXTENTS_DATA, FSCTL_DUPLICATE_EXTENTS_TO_FILE,
                FSCTL_GET_INTEGRITY_INFORMATION, FSCTL_GET_INTEGRITY_INFORMATION_BUFFER,
                FSCTL_SET_INTEGRITY_INFORMATION, FSCTL_SET_SPARSE,
            },
            IO::DeviceIoControl,
        },
    },
};

pub fn reflink_sync(src: &str, dest: &str) -> std::io::Result<()> {
    // Open source file
    let source_file_handle = open_file(src)?;

    // Get source volume info
    let source_volume = get_volume_info_for_path(src)?;

    // Check if the source supports copy-on-write
    if !source_volume.supports_cow {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Source volume does not support copy-on-write",
        ));
    }

    // Create empty destination file
    let dest_file_handle = create_file(dest)?;

    // Set destination as sparse
    let mut bytes_returned: u32 = 0;
    set_sparse(dest_file_handle, &mut bytes_returned)?;

    // Get source file size and sparse status
    let source_file_size = get_file_size(source_file_handle)?;
    let source_file_sparse = get_file_sparse_status(source_file_handle)?;

    // Clone integrity info from source to destination
    let integrity_info = get_integrity_info(source_file_handle, &mut bytes_returned)?;
    let _ = set_integrity_info(dest_file_handle, &mut bytes_returned, integrity_info);

    // Set destination file size
    set_file_size(dest_file_handle, source_file_size)?;

    // Duplicate extents
    duplicate_extents(
        dest_file_handle,
        source_file_handle,
        source_file_size as i64,
        source_volume,
    )?;

    // Unset sparse
    if !source_file_sparse {
        unset_sparse(dest_file_handle, &mut bytes_returned)?;
    }

    // Close handles
    close_handle(source_file_handle)?;
    close_handle(dest_file_handle)?;

    Ok(())
}

fn open_file(file_path: &str) -> Result<HANDLE, windows::core::Error> {
    let wide_file_path = U16CString::from_str(file_path).unwrap();

    unsafe {
        // Open the file
        let file_handle = CreateFileW(
            PCWSTR(wide_file_path.as_ptr() as _),
            GENERIC_READ.0,
            FILE_SHARE_READ | FILE_SHARE_DELETE,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL | FILE_FLAG_NO_BUFFERING,
            HANDLE(0),
        )?;

        if file_handle.is_invalid() {
            Err(windows::core::Error::from_win32())
        } else {
            Ok(file_handle)
        }
    }
}

fn create_file(file_path: &str) -> Result<HANDLE, windows::core::Error> {
    let wide_file_path = U16CString::from_str(file_path).unwrap();

    unsafe {
        // Create the file
        let file_handle = CreateFileW(
            PCWSTR(wide_file_path.as_ptr() as _),
            GENERIC_WRITE.0 | GENERIC_READ.0,
            FILE_SHARE_DELETE | FILE_SHARE_READ | FILE_SHARE_WRITE,
            None,
            CREATE_NEW,
            FILE_ATTRIBUTE_NORMAL,
            HANDLE(0),
        )?;

        if file_handle.is_invalid() {
            Err(windows::core::Error::from_win32())
        } else {
            Ok(file_handle)
        }
    }
}

fn get_file_size(file_handle: HANDLE) -> Result<u64, windows::core::Error> {
    let mut file_size = 0;

    unsafe {
        let result = GetFileSizeEx(file_handle, &mut file_size);

        if result.is_err() {
            return Err(windows::core::Error::from_win32());
        }
    }

    Ok(file_size as u64)
}

fn get_file_sparse_status(file_handle: HANDLE) -> Result<bool, windows::core::Error> {
    let mut info: FILE_BASIC_INFO = unsafe { std::mem::zeroed() };

    let result = unsafe {
        GetFileInformationByHandleEx(
            file_handle,
            FileStandardInfo,
            &mut info as *mut _ as *mut c_void,
            size_of::<FILE_BASIC_INFO>().try_into().unwrap(),
        )
    };

    if result.is_err() {
        return Err(windows::core::Error::from_win32());
    }

    Ok(info.FileAttributes & 0x00004000 != 0)
}

fn set_sparse(file_handle: HANDLE, bytes_returned: &mut u32) -> Result<(), windows::core::Error> {
    unsafe {
        DeviceIoControl(
            file_handle,
            FSCTL_SET_SPARSE,
            None,
            0,
            None,
            0,
            Some(bytes_returned as *mut _ as *mut _),
            None,
        )
    }
}

fn unset_sparse(file_handle: HANDLE, bytes_returned: &mut u32) -> Result<(), windows::core::Error> {
    unsafe {
        DeviceIoControl(
            file_handle,
            FSCTL_SET_SPARSE,
            Some(&mut 0 as *mut _ as *mut c_void),
            0,
            None,
            0,
            Some(bytes_returned as *mut _ as *mut _),
            None,
        )
    }
}

fn get_integrity_info(
    file_handle: HANDLE,
    bytes_returned: &mut u32,
) -> Result<FSCTL_GET_INTEGRITY_INFORMATION_BUFFER, windows::core::Error> {
    let mut integrity_info: MaybeUninit<FSCTL_GET_INTEGRITY_INFORMATION_BUFFER> =
        MaybeUninit::uninit();

    unsafe {
        DeviceIoControl(
            file_handle,
            FSCTL_GET_INTEGRITY_INFORMATION,
            None,
            0,
            Some(integrity_info.as_mut_ptr() as *mut c_void),
            size_of::<FSCTL_GET_INTEGRITY_INFORMATION_BUFFER>()
                .try_into()
                .unwrap(),
            Some(bytes_returned as *mut _ as *mut _),
            None,
        )?;

        Ok(integrity_info.assume_init())
    }
}

fn set_integrity_info(
    file_handle: HANDLE,
    bytes_returned: &mut u32,
    integrity_info: FSCTL_GET_INTEGRITY_INFORMATION_BUFFER,
) -> Result<(), windows::core::Error> {
    unsafe {
        DeviceIoControl(
            file_handle,
            FSCTL_SET_INTEGRITY_INFORMATION,
            Some(&integrity_info as *const _ as *mut c_void),
            size_of::<FSCTL_GET_INTEGRITY_INFORMATION_BUFFER>()
                .try_into()
                .unwrap(),
            None,
            0,
            Some(bytes_returned as *mut _ as *mut _),
            None,
        )?;

        Ok(())
    }
}

fn set_file_size(file_handle: HANDLE, file_size: u64) -> Result<(), windows::core::Error> {
    let mut eof_info: FILE_END_OF_FILE_INFO = FILE_END_OF_FILE_INFO {
        EndOfFile: file_size.try_into().unwrap(),
    };
    unsafe {
        SetFileInformationByHandle(
            file_handle,
            FileEndOfFileInfo,
            &mut eof_info as *mut _ as *mut c_void,
            size_of::<u64>().try_into().unwrap(),
        )
    }
}

// 2GB
const MAX_CHUNK_SIZE: i64 = 2 * 1024 * 1024 * 1024;

#[derive(Clone)]
struct VolumeInfo {
    supports_cow: bool,
    cluster_size: u64,
}

impl VolumeInfo {
    pub fn new(supports_cow: bool, cluster_size: u64) -> Self {
        Self {
            supports_cow,
            cluster_size,
        }
    }
}

fn round_up_to_power_of_2(n: i64, m: i64) -> i64 {
    (n + m - 1) & !(m - 1)
}

lazy_static::lazy_static! {
    static ref VOLUME_INFO_CACHE: Mutex<HashMap<String, VolumeInfo>> = Mutex::new(HashMap::new());
}

fn get_volume_info_for_path(path: &str) -> Result<VolumeInfo, windows::core::Error> {
    let drive_letter = match path.chars().next() {
        Some(c) => c,
        None => return Err(windows::core::Error::from_win32()),
    };
    let drive_root = format!("{}:\\", drive_letter);

    // Attempt to get the information from cache first.
    {
        let cache = VOLUME_INFO_CACHE.lock().unwrap();
        if let Some(cached_info) = cache.get(&drive_root) {
            return Ok(cached_info.clone());
        }
    }

    // If it's not in the cache, proceed with fetching the information.
    let wide_path = U16CString::from_str(&drive_root).unwrap();

    let mut volume_name = vec![0u16; 260]; // MAX_PATH = 260
    let mut volume_serial_number: u32 = 0;
    let mut maximum_component_length: u32 = 0;
    let mut file_system_flags: u32 = 0;
    let mut file_system_name = vec![0u16; 260]; // MAX_PATH = 260

    unsafe {
        GetVolumeInformationW(
            PCWSTR(wide_path.as_ptr() as _),
            Some(&mut volume_name),
            Some(&mut volume_serial_number),
            Some(&mut maximum_component_length),
            Some(&mut file_system_flags),
            Some(&mut file_system_name),
        )?;
    }

    let supports_cow = (file_system_flags & 0x00000040) != 0;

    let mut sectors_per_cluster = 0;
    let mut bytes_per_sector = 0;
    let mut number_of_free_clusters = 0;
    let mut total_number_of_clusters = 0;

    unsafe {
        GetDiskFreeSpaceW(
            PCWSTR(wide_path.as_ptr() as _),
            Some(&mut sectors_per_cluster),
            Some(&mut bytes_per_sector),
            Some(&mut number_of_free_clusters),
            Some(&mut total_number_of_clusters),
        )?;
    }

    let cluster_size = sectors_per_cluster as u64 * bytes_per_sector as u64;

    let volume_info = VolumeInfo::new(supports_cow, cluster_size);

    // Cache the information for future use.
    {
        let mut cache = VOLUME_INFO_CACHE.lock().unwrap();
        cache.insert(drive_root, volume_info.clone());
    }

    Ok(volume_info)
}

fn duplicate_extents(
    dest_file_handle: HANDLE,
    source_file_handle: HANDLE,
    source_file_length: i64,
    source_volume: VolumeInfo,
) -> std::io::Result<()> {
    let file_size_rounded_up_to_cluster_boundary =
        round_up_to_power_of_2(source_file_length, source_volume.cluster_size as i64);
    let mut source_offset = 0;

    if source_volume.cluster_size != 4096 && source_volume.cluster_size != 65536 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Cluster size of source must either be 4K or 64K (restricted by ReFS)",
        ));
    }

    while source_offset < source_file_length {
        let this_chunk_size = std::cmp::min(
            file_size_rounded_up_to_cluster_boundary - source_offset,
            MAX_CHUNK_SIZE,
        );

        let mut duplicate_extents_data = DUPLICATE_EXTENTS_DATA {
            FileHandle: source_file_handle,
            SourceFileOffset: source_offset,
            TargetFileOffset: source_offset,
            ByteCount: this_chunk_size,
        };

        unsafe {
            DeviceIoControl(
                dest_file_handle,
                FSCTL_DUPLICATE_EXTENTS_TO_FILE,
                Some(&mut duplicate_extents_data as *mut _ as *const c_void),
                std::mem::size_of::<DUPLICATE_EXTENTS_DATA>()
                    .try_into()
                    .unwrap(),
                None,
                0,
                None,
                None,
            )?;
        };

        source_offset += this_chunk_size;
    }

    Ok(())
}

fn close_handle(handle: HANDLE) -> Result<(), windows::core::Error> {
    unsafe {
        CloseHandle(handle)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile;
    use tempfile::Builder;
    use std::{env, io::{Read, Write}};

    #[test]
    fn should_open_file() {
        let file_path = "C:\\Windows\\System32\\notepad.exe";
        let file_handle = open_file(file_path);
        assert!(file_handle.is_ok());

        // Close the handle
        file_handle.unwrap();
    }

    #[test]
    fn should_fail_to_open_nonexistent_file() {
        let file_path = "C:\\Windows\\System32\\nonexistent.exe";
        let file_handle = open_file(file_path);
        assert!(file_handle.is_err());
    }

    #[test]
    fn should_create_file() {
        // The file should be located in the temp directory
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let file_handle = create_file(file_path.to_str().unwrap());

        // Close the handle
        close_handle(file_handle.unwrap()).unwrap();

        // Try to open the file
        let file_handle = open_file(file_path.to_str().unwrap());

        // Close the handle
        close_handle(file_handle.unwrap()).unwrap();

        // Clean up
        temp_dir.close().unwrap();
    }

    #[test]
    fn should_fail_to_create_existing_file() {
        // The file should be located in the temp directory
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        // Create the file
        let file_handle = create_file(file_path.to_str().unwrap());

        // Close the handle
        close_handle(file_handle.unwrap()).unwrap();

        // Try to create the file again
        let file_handle = create_file(file_path.to_str().unwrap());

        assert!(file_handle.is_err());

        // Clean up
        temp_dir.close().unwrap();
    }

    #[test]
    fn should_get_file_size() {
        // Create a tmp file, write some data (Using the standard library), open that file and check the size, then close the file, compare the sizes
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        // Create the file
        std::fs::File::create(&file_path).unwrap();

        // Write some data
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .open(&file_path)
            .unwrap();

        let data = "Hello, world!";

        file.write_all(data.as_bytes()).unwrap();

        // Close the file
        std::mem::drop(file);

        // Open the file
        let file_handle = open_file(file_path.to_str().unwrap()).unwrap();

        // Get the file size
        let file_size = get_file_size(file_handle).unwrap();

        // Close the file
        close_handle(file_handle).unwrap();

        // Compare the sizes
        assert_eq!(file_size, data.len() as u64);

        // Clean up
        temp_dir.close().unwrap();
    }

    #[test]
    fn should_close_handle() {
        let file_path = "C:\\Windows\\System32\\notepad.exe";
        let file_handle = open_file(file_path).unwrap();
        let result = close_handle(file_handle);
        assert!(result.is_ok());
    }

    #[test]
    fn should_get_current_volume_info() {
        let current_dir = std::env::current_dir().unwrap();
        let current_dir_str = current_dir.to_str().unwrap();

        let volume_info = get_volume_info_for_path(current_dir_str).unwrap();

        assert!(volume_info.supports_cow);
        assert_eq!(volume_info.cluster_size > 0, true);
    }

    #[test]
    fn should_get_volume_info_for_path() {
        let file_path = "C:\\Windows\\System32\\notepad.exe";
        let volume_info = get_volume_info_for_path(file_path).unwrap();

        assert!(volume_info.supports_cow);
        assert_eq!(volume_info.cluster_size > 0, true);
    }

    #[test]
    fn should_get_file_sparse_status() {
        let file_path = "C:\\Windows\\System32\\notepad.exe";
        let file_handle = open_file(file_path).unwrap();
        let sparse_status = get_file_sparse_status(file_handle).unwrap();
        close_handle(file_handle).unwrap();
        assert_eq!(sparse_status, false);
    }
}

