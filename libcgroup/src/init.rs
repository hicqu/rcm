extern crate libcgroup_sys as ffi;

use std::sync::{Once, ONCE_INIT};

static C_LIB_INITIALIZED: Once = ONCE_INIT;

pub fn init() {
    C_LIB_INITIALIZED.call_once(|| {
        unsafe {
            ffi::cgroup_init();
        }
    });
}
