extern crate libcgroup_sys;
use libcgroup_sys::*;

extern crate libc;
use libc::{c_char, c_void};

use std::ffi::{CStr, CString};

#[test]
fn test_init() {
    unsafe {
        cgroup_init();
    }
}

#[test]
fn test_iterate_all_controllers() {
    let mut controllers = Default::default();
    let h: *const c_void = std::ptr::null();

    unsafe {
        cgroup_init();

        let mut ret = cgroup_get_all_controller_begin(&h as *const *const c_void, &mut controllers);
        assert!(ret == 0);

        while ret == 0 {
            println!(
                "{}",
                CStr::from_ptr(controllers.name.as_ptr())
                    .to_string_lossy()
                    .into_owned()
            );
            println!("{}", controllers.num_cgroups);

            ret = cgroup_get_all_controller_next(&h as *const *const c_void, &mut controllers);
        }

        if ret != ECGEOF {
            println!(
                "{}",
                CStr::from_ptr(cgroup_strerror(ret))
                    .to_string_lossy()
                    .into_owned()
            );
        }
        assert!(ret == ECGEOF);

        ret = cgroup_get_all_controller_end(&h as *const *const c_void);
        if ret != 0 {
            println!(
                "{}",
                CStr::from_ptr(cgroup_strerror(ret))
                    .to_string_lossy()
                    .into_owned()
            );
        }
        assert!(ret == 0);
    }
}

#[cfg(feature = "nightly")]
extern crate procinfo;

#[test]
#[cfg(feature = "nightly")]
fn test_cgroup_get_current_controller_path() {
    unsafe {
        cgroup_init();

        let current_path: *const c_char = std::ptr::null();

        let stat = procinfo::pid::stat_self().unwrap();

        let ret = cgroup_get_current_controller_path(
            stat.pid,
            CString::new("memory").unwrap().as_ptr(),
            &current_path as *const *const c_char,
        );
        if ret != 0 {
            println!(
                "{}",
                CStr::from_ptr(cgroup_strerror(ret))
                    .to_string_lossy()
                    .into_owned()
            );
        } else {
            println!(
                "{}",
                CStr::from_ptr(current_path).to_string_lossy().into_owned()
            );
        }
        assert!(ret == 0);
    }
}
