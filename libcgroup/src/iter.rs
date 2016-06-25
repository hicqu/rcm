use ::init::init;
use ::result::{check_return, Result};

extern crate libcgroup_sys as ffi;
extern crate libc;

use std;
use std::ffi::CStr;

pub struct CGroupMount {
    pub controller_name: String,
    pub path: String,
}

impl From<ffi::cgroup_mount_point> for CGroupMount {
    fn from(mp: ffi::cgroup_mount_point) -> CGroupMount {
        CGroupMount {
            controller_name: unsafe {
                CStr::from_ptr(&mp.name as *const libc::c_char)
                    .to_string_lossy()
                    .into_owned()
            },
            path: unsafe {
                CStr::from_ptr(&mp.path as *const libc::c_char).to_string_lossy().into_owned()
            },
        }
    }
}

pub struct CGroupMountIter {
    handle: *const libc::c_void,
}

impl CGroupMountIter {
    pub fn new() -> CGroupMountIter {
        init();
        CGroupMountIter { handle: std::ptr::null() }
    }
}

impl Drop for CGroupMountIter {
    fn drop(&mut self) {
        unsafe {
            ffi::cgroup_get_controller_end(&self.handle);
        }
    }
}

impl Iterator for CGroupMountIter {
    type Item = Result<CGroupMount>;

    fn next(&mut self) -> Option<Result<CGroupMount>> {
        let mut mp: ffi::cgroup_mount_point = Default::default();
        let ret: i32;
        if self.handle.is_null() {
            ret = unsafe {
                ffi::cgroup_get_controller_begin(&self.handle as *const *const libc::c_void,
                                                 &mut mp)
            };
        } else {
            ret = unsafe {
                ffi::cgroup_get_controller_next(&self.handle as *const *const libc::c_void, &mut mp)
            };
        }
        if ret == ffi::ECGEOF {
            return None;
        }
        Option::Some(check_return(ret, CGroupMount::from(mp)))
    }
}

pub fn cgroup_mount_points() -> CGroupMountIter {
    CGroupMountIter::new()
}
