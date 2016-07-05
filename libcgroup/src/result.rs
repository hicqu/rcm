extern crate libcgroup_sys as ffi;

use std;
use std::ffi::CStr;

#[derive(Debug)]
pub struct CGroupError {
    pub code: i32,
    pub description: String,
}

pub type Result<T> = std::result::Result<T, CGroupError>;

pub fn check_return<T>(ret: i32, val: T) -> Result<T> {
    match ret {
        0 => Ok(val),
        _ => {
            let desc =
                unsafe { CStr::from_ptr(ffi::cgroup_strerror(ret)).to_string_lossy().into_owned() };
            Err(CGroupError {
                code: ret,
                description: desc,
            })
        }
    }
}
