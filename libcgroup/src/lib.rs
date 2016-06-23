extern crate cgroup_sys as ffi;

use std::ffi::{CStr, CString};
use std::ptr;
use std::result;
use std::sync::{Once, ONCE_INIT};

static C_LIB_INITIALIZED: Once = ONCE_INIT;

pub struct CGroupError {
    pub code: i32,
    pub description: String,
}

pub type Result<T> = result::Result<T, CGroupError>;

fn check_return<T>(ret: i32, val: T) -> Result<T> {
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

pub struct CGroup {
    cgroup: *const ffi::cgroup,
}

impl CGroup {
    pub fn new(name: String) -> Result<CGroup> {
        C_LIB_INITIALIZED.call_once(|| {
            unsafe {
                ffi::cgroup_init();
            }
        });
        let cg = unsafe { ffi::cgroup_new_cgroup(CString::new(name).unwrap().as_ptr()) };
        if cg.is_null() {
            check_return(ffi::ECGFAIL, CGroup { cgroup: ptr::null() })
        } else {
            Ok(CGroup { cgroup: cg })
        }
    }

    pub fn create(&self) -> Result<*const CGroup> {
        let ret = unsafe { ffi::cgroup_create_cgroup(self.cgroup, 0) };
        check_return(ret, self)
    }

    pub fn add_controller(&self, controller: String) -> Result<*const CGroup> {
        let ctrlr = unsafe {
            ffi::cgroup_add_controller(self.cgroup, CString::new(controller).unwrap().as_ptr())
        };
        if ctrlr.is_null() {
            check_return(ffi::ECGFAIL, self)
        } else {
            Ok(self)
        }
    }
}

impl Drop for CGroup {
    fn drop(&mut self) {
        unsafe {
            ffi::cgroup_free(&self.cgroup);
        }
    }
}
