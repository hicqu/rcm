extern crate libc;
use libc::{c_int, c_void, c_char};
use std::mem;

const FILENAME_MAX: usize = libc::FILENAME_MAX as usize;

pub type Handle = *const c_void;

#[allow(dead_code)]
pub const ECGEOF: c_int = 50023;

#[repr(C)]
#[derive(Copy)]
pub struct controller_data {
    pub name: [c_char; FILENAME_MAX],
    pub hierarchy: c_int,
    pub num_cgroups: c_int,
    pub enabled: c_int,
}

impl Clone for controller_data {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for controller_data {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub fn new_controller_data() -> controller_data {
    Default::default()
}

extern "C" {
    pub fn cgroup_init() -> c_int;

    pub fn cgroup_strerror(code: c_int) -> *const c_char;

    pub fn cgroup_get_all_controller_begin(h: *const Handle, info: *mut controller_data) -> c_int;
    pub fn cgroup_get_all_controller_next(h: *const Handle, info: *mut controller_data) -> c_int;
    pub fn cgroup_get_all_controller_end(h: *const Handle) -> c_int;
}

// A minimal test, enough to force a sanity check on the linkage
#[test]
fn test_init() {
    use std::ffi::CStr;

    unsafe {
        cgroup_init();

        let mut controllers = new_controller_data();
        let h = 0 as Handle;

        let mut ret = cgroup_get_all_controller_begin(&h as *const *const c_void, &mut controllers);

        while ret == 0 {
            println!("{}",
                     CStr::from_ptr(controllers.name.as_ptr()).to_string_lossy().into_owned());
            println!("{}", controllers.num_cgroups);

            ret = cgroup_get_all_controller_next(&h as *const *const c_void, &mut controllers);
        }

        if ret != ECGEOF {
            println!("{}",
                     CStr::from_ptr(cgroup_strerror(ret)).to_string_lossy().into_owned());
        }

        ret = cgroup_get_all_controller_end(&h as *const *const libc::c_void);
        if ret != 0 {
            println!("{}",
                     CStr::from_ptr(cgroup_strerror(ret)).to_string_lossy().into_owned());
        }
    }
}
