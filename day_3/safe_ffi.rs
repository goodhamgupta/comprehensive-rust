#![allow(unused_imports, dead_code, unused_variables)]

mod ffi {
    use std::os::raw::{c_char, c_int, c_long, c_ulong, c_ushort};

    #[repr(C)]
    pub struct DIR {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    #[repr(C)]
    pub struct dirent {
        pub d_ino: c_long,
        pub d_off: c_ulong,
        pub d_reclen: c_ushort,
        pub d_type: c_char,
        pub d_name: [c_char; 256],
    }

    extern "C" {
        pub fn opendir(s: *const c_char) -> *mut DIR;
        pub fn readdir(s: *mut DIR) -> *const dirent;
        pub fn closedir(s: *mut DIR) -> c_int;
    }
}

use std::convert::TryInto;
use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    fn new(path: &str) -> Result<DirectoryIterator, String> {
        let c_str = CString::new(path).unwrap();
        unsafe {
            let dir = ffi::opendir(c_str.as_ptr());
            Ok(DirectoryIterator {
                path: CString::new(path).unwrap(),
                dir,
            })
        }
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        // call until we get a null pointer back
        unsafe {
            let dirent_record = ffi::readdir(self.dir);
            if dirent_record.is_null() {
                None
            } else {
                let d_name = CStr::from_ptr((*dirent_record).d_name.as_ptr());
                let os_str = OsStr::from_bytes(d_name.to_bytes());
                Some(os_str.to_owned())
            }
        }
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        // call closedir as needed
        if !self.dir.is_null() {
            if unsafe { ffi::closedir(self.dir) != 0 } {
                panic!("Could not close {:?}", self.path);
            }
        }
    }
}

fn main() -> Result<(), String> {
    let iter = DirectoryIterator::new(".");
    println!("files: {:#?}", iter?.collect::<Vec<_>>());
    Ok(())
}
