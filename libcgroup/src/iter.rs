use ::init::init;
use ::result::{check_return, Result};

extern crate libcgroup_sys as ffi;
extern crate libc;

use std;
use std::ffi::{CStr, CString};
use std::vec::Vec;

#[derive(Debug)]
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

pub fn cgroup_mount_points_iter() -> CGroupMountIter {
    CGroupMountIter::new()
}

pub fn cgroup_mount_points() -> Result<Vec<CGroupMount>> {
    let mut vec = Vec::new();
    for c in cgroup_mount_points_iter() {
        let mp = match c {
            Ok(m) => m,
            Err(err) => return Err(err),
        };
        vec.push(mp);
    }
    Ok(vec)
}

#[derive(Debug)]
pub struct CGroupFileInfo {
    pub path: String,
    pub parent: String,
    pub full_path: String,
    pub depth: i16,
}

#[derive(Debug)]
pub enum CGroupFile {
    File(CGroupFileInfo),
    Dir(CGroupFileInfo),
    Other(CGroupFileInfo),
}

impl From<ffi::cgroup_file_info> for CGroupFile {
    fn from(cgf: ffi::cgroup_file_info) -> CGroupFile {
        let fi = CGroupFileInfo {
            path: unsafe {
                CStr::from_ptr(cgf.path as *const libc::c_char)
                    .to_string_lossy()
                    .into_owned()
            },
            parent: unsafe {
                CStr::from_ptr(cgf.parent as *const libc::c_char)
                    .to_string_lossy()
                    .into_owned()
            },
            full_path: unsafe {
                CStr::from_ptr(cgf.full_path as *const libc::c_char)
                    .to_string_lossy()
                    .into_owned()
            },
            depth: cgf.depth,
        };
        match cgf.file_type {
            ffi::cgroup_file_type::CGROUP_FILE_TYPE_FILE => CGroupFile::File(fi),
            ffi::cgroup_file_type::CGROUP_FILE_TYPE_DIR => CGroupFile::Dir(fi),
            ffi::cgroup_file_type::CGROUP_FILE_TYPE_OTHER => CGroupFile::Other(fi),
        }
    }
}

#[derive(Debug)]
pub struct CGroupWalkIter {
    handle: *const libc::c_void,
    base_level: libc::c_int,
    pub controller_name: String,
    pub base_path: String,
    pub depth: i32,
}

impl CGroupWalkIter {
    pub fn new<S>(controller_name: S) -> CGroupWalkIter
        where S: Into<String>
    {
        init();
        CGroupWalkIter {
            handle: std::ptr::null(),
            base_level: 0,
            controller_name: controller_name.into(),
            base_path: "".to_string(),
            depth: 0,
        }
    }

    pub fn new_with_options<S>(controller_name: S, base_path: S, depth: i32) -> CGroupWalkIter
        where S: Into<String>
    {
        init();
        CGroupWalkIter {
            handle: std::ptr::null(),
            base_level: 0,
            controller_name: controller_name.into(),
            base_path: base_path.into(),
            depth: depth,
        }
    }
}

impl Drop for CGroupWalkIter {
    fn drop(&mut self) {
        unsafe {
            ffi::cgroup_walk_tree_end(&self.handle);
        }
    }
}

impl Iterator for CGroupWalkIter {
    type Item = Result<CGroupFile>;

    fn next(&mut self) -> Option<Result<CGroupFile>> {
        let mut fi: ffi::cgroup_file_info = Default::default();
        let ret: i32;
        if self.handle.is_null() {
            ret = unsafe {
                ffi::cgroup_walk_tree_begin(CString::new(self.controller_name.clone())
                                                .unwrap()
                                                .as_ptr(),
                                            CString::new(self.base_path.clone()).unwrap().as_ptr(),
                                            self.depth,
                                            &self.handle as *const *const libc::c_void,
                                            &mut fi,
                                            &self.base_level as *const libc::c_int)
            };
        } else {
            ret = unsafe {
                ffi::cgroup_walk_tree_next(self.depth,
                                           &self.handle as *const *const libc::c_void,
                                           &mut fi,
                                           self.base_level)
            };
        }
        if ret == ffi::ECGEOF {
            return None;
        }
        Option::Some(check_return(ret, CGroupFile::from(fi)))
    }
}

pub fn cgroup_walk_tree_iter<S>(controller_name: S) -> CGroupWalkIter
    where S: Into<String>
{
    CGroupWalkIter::new(controller_name)
}
