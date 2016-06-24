extern crate libcgroup_sys as ffi;

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
    pub fn new<S>(name: S) -> Result<CGroup>
        where S: Into<String>
    {
        C_LIB_INITIALIZED.call_once(|| {
            unsafe {
                ffi::cgroup_init();
            }
        });
        let cg = unsafe { ffi::cgroup_new_cgroup(CString::new(name.into()).unwrap().as_ptr()) };
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

    pub fn add_controller<S>(&self, controller: S) -> Result<*const CGroup>
        where S: Into<String>
    {
        let ctrlr = unsafe {
            ffi::cgroup_add_controller(self.cgroup,
                                       CString::new(controller.into()).unwrap().as_ptr())
        };
        if ctrlr.is_null() {
            return check_return(ffi::ECGFAIL, self);
        }
        Ok(self)
    }
}

impl Drop for CGroup {
    fn drop(&mut self) {
        unsafe {
            ffi::cgroup_free(&self.cgroup);
        }
    }
}
