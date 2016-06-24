extern crate libcgroup_sys as ffi;
extern crate libc;

use std::ffi::{CStr, CString};
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

pub struct CGroupController {
    controller: *const ffi::cgroup_controller,
}

pub struct CGroupUidGid {
    pub tasks_uid: u32,
    pub tasks_gid: u32,
    pub control_uid: u32,
    pub control_gid: u32,
}

pub struct CGroupPermissions {
    pub control_dperm: u32,
    pub control_fperm: u32,
    pub task_fperm: u32,
}

impl Drop for CGroup {
    fn drop(&mut self) {
        unsafe {
            ffi::cgroup_free(&self.cgroup);
        }
    }
}

pub enum Compare {
    Equal,
    NotEqual,
    ControllersNotEqual,
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
        let cg = CGroup {
            cgroup: unsafe { ffi::cgroup_new_cgroup(CString::new(name.into()).unwrap().as_ptr()) },
        };
        if cg.cgroup.is_null() {
            return check_return(ffi::ECGFAIL, cg);
        }
        Ok(cg)
    }

    pub fn add_controller<S>(&self, controller: S) -> Result<CGroupController>
        where S: Into<String>
    {
        let ctrlr = unsafe {
            ffi::cgroup_add_controller(self.cgroup,
                                       CString::new(controller.into()).unwrap().as_ptr())
        };
        let cgroup_controller = CGroupController { controller: ctrlr };
        if ctrlr.is_null() {
            return check_return(ffi::ECGFAIL, cgroup_controller);
        }
        Ok(cgroup_controller)
    }

    pub fn get_controller<S>(&self, controller: S) -> Result<CGroupController>
        where S: Into<String>
    {
        let ctrlr = unsafe {
            ffi::cgroup_get_controller(self.cgroup,
                                       CString::new(controller.into()).unwrap().as_ptr())
        };
        let cgroup_controller = CGroupController { controller: ctrlr };
        if ctrlr.is_null() {
            return check_return(ffi::ECGFAIL, cgroup_controller);
        }
        Ok(cgroup_controller)
    }

    pub fn create(&self) -> Result<()> {
        let ret = unsafe { ffi::cgroup_create_cgroup(self.cgroup, 0) };
        check_return(ret, ())
    }

    pub fn create_from_parent(&self) -> Result<()> {
        let ret = unsafe { ffi::cgroup_create_cgroup_from_parent(self.cgroup, 0) };
        check_return(ret, ())
    }

    pub fn modify(&self) -> Result<()> {
        let ret = unsafe { ffi::cgroup_modify_cgroup(self.cgroup) };
        check_return(ret, ())
    }

    pub fn delete(&self) -> Result<()> {
        let ret = unsafe { ffi::cgroup_delete_cgroup(self.cgroup, 0) };
        check_return(ret, ())
    }

    pub fn get(&self) -> Result<()> {
        let ret = unsafe { ffi::cgroup_get_cgroup(self.cgroup) };
        check_return(ret, ())
    }

    pub fn copy(&self, dest: &CGroup) -> Result<()> {
        let ret = unsafe { ffi::cgroup_copy_cgroup(self.cgroup, dest.cgroup) };
        check_return(ret, ())
    }

    pub fn compare(&self, other: &CGroup) -> Compare {
        match unsafe { ffi::cgroup_compare_cgroup(self.cgroup, other.cgroup) } {
            ffi::ECGCONTROLLERNOTEQUAL => Compare::ControllersNotEqual,
            ffi::ECGROUPNOTEQUAL => Compare::NotEqual,
            0 => Compare::Equal,
            x => panic!("Invalid return: {}", x),
        }
    }

    pub fn set_uid_gid(&self, uid_gid: CGroupUidGid) -> Result<()> {
        let ret = unsafe {
            ffi::cgroup_set_uid_gid(self.cgroup,
                                    uid_gid.tasks_uid,
                                    uid_gid.tasks_gid,
                                    uid_gid.control_uid,
                                    uid_gid.control_gid)
        };
        check_return(ret, ())
    }

    pub fn get_uid_gid(&self) -> Result<CGroupUidGid> {
        let (mut tasks_uid, mut tasks_gid, mut control_uid, mut control_gid) =
            (0u32, 0u32, 0u32, 0u32);
        let ret = unsafe {
            ffi::cgroup_get_uid_gid(self.cgroup,
                                    &mut tasks_uid,
                                    &mut tasks_gid,
                                    &mut control_uid,
                                    &mut control_gid)
        };
        check_return(ret,
                     CGroupUidGid {
                         tasks_uid: tasks_uid,
                         tasks_gid: tasks_gid,
                         control_uid: control_uid,
                         control_gid: control_gid,
                     })
    }

    pub fn set_permissions(&self, perms: CGroupPermissions) {
        unsafe {
            ffi::cgroup_set_permissions(self.cgroup,
                                        perms.control_dperm,
                                        perms.control_fperm,
                                        perms.task_fperm)
        };
    }
}

impl CGroupController {
    pub fn compare(&self, other: &CGroupController) -> i32 {
        unsafe { ffi::cgroup_compare_controllers(self.controller, other.controller) }
    }

    pub fn add_value_string<S>(&self, name: S, value: S) -> Result<()>
        where S: Into<String>
    {
        let ret = unsafe {
            ffi::cgroup_add_value_string(self.controller,
                                         CString::new(name.into()).unwrap().as_ptr(),
                                         CString::new(value.into()).unwrap().as_ptr())
        };
        check_return(ret, ())
    }

    pub fn add_value_int64<S>(&self, name: S, value: i64) -> Result<()>
        where S: Into<String>
    {
        let ret = unsafe {
            ffi::cgroup_add_value_int64(self.controller,
                                        CString::new(name.into()).unwrap().as_ptr(),
                                        value)
        };
        check_return(ret, ())
    }

    pub fn add_value_uint64<S>(&self, name: S, value: u64) -> Result<()>
        where S: Into<String>
    {
        let ret = unsafe {
            ffi::cgroup_add_value_uint64(self.controller,
                                         CString::new(name.into()).unwrap().as_ptr(),
                                         value)
        };
        check_return(ret, ())
    }

    pub fn add_value_bool<S>(&self, name: S, value: bool) -> Result<()>
        where S: Into<String>
    {
        let ret = unsafe {
            ffi::cgroup_add_value_bool(self.controller,
                                       CString::new(name.into()).unwrap().as_ptr(),
                                       value)
        };
        check_return(ret, ())
    }

    pub fn get_value_string<S>(&self, name: S) -> Result<String>
        where S: Into<String>
    {
        let value: *const libc::c_char = std::ptr::null();
        let ret = unsafe {
            ffi::cgroup_get_value_string(self.controller,
                                         CString::new(name.into()).unwrap().as_ptr(),
                                         &value as *const *const libc::c_char)
        };
        check_return(ret,
                     unsafe { CStr::from_ptr(value).to_string_lossy().into_owned() })
    }

    pub fn get_value_int64<S>(&self, name: S) -> Result<i64>
        where S: Into<String>
    {
        let value = 0i64;
        let ret = unsafe {
            ffi::cgroup_get_value_int64(self.controller,
                                        CString::new(name.into()).unwrap().as_ptr(),
                                        &value)
        };
        check_return(ret, value)
    }

    pub fn get_value_uint64<S>(&self, name: S) -> Result<u64>
        where S: Into<String>
    {
        let value = 0u64;
        let ret = unsafe {
            ffi::cgroup_get_value_uint64(self.controller,
                                         CString::new(name.into()).unwrap().as_ptr(),
                                         &value)
        };
        check_return(ret, value)
    }

    pub fn get_value_bool<S>(&self, name: S) -> Result<bool>
        where S: Into<String>
    {
        let value = false;
        let ret = unsafe {
            ffi::cgroup_get_value_bool(self.controller,
                                       CString::new(name.into()).unwrap().as_ptr(),
                                       &value)
        };
        check_return(ret, value)
    }

    pub fn set_value_string<S>(&self, name: S, value: S) -> Result<*const CGroupController>
        where S: Into<String>
    {
        let ret = unsafe {
            ffi::cgroup_set_value_string(self.controller,
                                         CString::new(name.into()).unwrap().as_ptr(),
                                         CString::new(value.into()).unwrap().as_ptr())
        };
        check_return(ret, self)
    }

    pub fn set_value_int64<S>(&self, name: S, value: i64) -> Result<*const CGroupController>
        where S: Into<String>
    {
        let ret = unsafe {
            ffi::cgroup_set_value_int64(self.controller,
                                        CString::new(name.into()).unwrap().as_ptr(),
                                        value)
        };
        check_return(ret, self)
    }

    pub fn set_value_uint64<S>(&self, name: S, value: u64) -> Result<*const CGroupController>
        where S: Into<String>
    {
        let ret = unsafe {
            ffi::cgroup_set_value_uint64(self.controller,
                                         CString::new(name.into()).unwrap().as_ptr(),
                                         value)
        };
        check_return(ret, self)
    }

    pub fn set_value_bool<S>(&self, name: S, value: bool) -> Result<*const CGroupController>
        where S: Into<String>
    {
        let ret = unsafe {
            ffi::cgroup_set_value_bool(self.controller,
                                       CString::new(name.into()).unwrap().as_ptr(),
                                       value)
        };
        check_return(ret, self)
    }
}
