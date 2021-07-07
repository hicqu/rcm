extern crate libc;
use libc::{c_char, c_int, c_short, c_void, gid_t, mode_t, pid_t, uid_t};
use std::mem;

pub const ECGEOF: c_int = 50023;
pub const ECGFAIL: c_int = 50013;
pub const ECGROUPNOTEQUAL: c_int = 50017;
pub const ECGCONTROLLERNOTEQUAL: c_int = 50018;
pub const FILENAME_MAX: usize = 4096;
pub const CG_VALUE_MAX: usize = 100;

#[allow(non_camel_case_types)]
pub enum cgroup {}
#[allow(non_camel_case_types)]
pub enum cgroup_controller {}

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

#[repr(C)]
#[derive(Copy)]
pub struct cgroup_stat {
    pub name: [c_char; FILENAME_MAX],
    pub value: [c_char; CG_VALUE_MAX],
}

impl Clone for cgroup_stat {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for cgroup_stat {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct cgroup_mount_point {
    pub name: [c_char; FILENAME_MAX],
    pub path: [c_char; FILENAME_MAX],
}

impl Clone for cgroup_mount_point {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for cgroup_mount_point {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Copy, Clone)]
pub enum group_walk_type {
    CGROUP_WALK_TYPE_PRE_DIR = 0x1,
    CGROUP_WALK_TYPE_POST_DIR = 0x2,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Copy, Clone)]
pub enum cgroup_file_type {
    CGROUP_FILE_TYPE_FILE,
    CGROUP_FILE_TYPE_DIR,
    CGROUP_FILE_TYPE_OTHER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_file_info {
    pub file_type: cgroup_file_type,
    pub path: *const c_char,
    pub parent: *const c_char,
    pub full_path: *const c_char,
    pub depth: c_short,
}

impl Default for cgroup_file_info {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

extern "C" {
    // error.h
    pub fn cgroup_strerror(code: c_int) -> *const c_char;
    pub fn cgroup_get_last_errno() -> c_int;

    // init.h
    pub fn cgroup_init() -> c_int;
    pub fn cgroup_get_subsys_mount_point(
        controller: *const c_char,
        mount_point: *const *const c_char,
    ) -> c_int;

    // iterators.h
    pub fn cgroup_walk_tree_begin(
        controller: *const c_char,
        base_path: *const c_char,
        depth: c_int,
        handle: *const *const c_void,
        info: *mut cgroup_file_info,
        base_level: *const c_int,
    ) -> c_int;
    pub fn cgroup_walk_tree_next(
        depth: c_int,
        handle: *const *const c_void,
        info: *mut cgroup_file_info,
        base_level: c_int,
    ) -> c_int;
    pub fn cgroup_walk_tree_end(handle: *const *const c_void) -> c_int;
    pub fn cgroup_walk_tree_set_flags(handle: *const *const c_void, flags: c_int) -> c_int;

    pub fn cgroup_read_value_begin(
        controller: *const c_char,
        path: *const c_char,
        name: *mut c_char,
        h: *const *const c_void,
        buffer: *mut c_char,
        max: c_int,
    ) -> c_int;
    pub fn cgroup_read_value_next(
        handle: *const *const c_void,
        buffer: *mut c_char,
        max: c_int,
    ) -> c_int;
    pub fn cgroup_read_value_end(handle: *const *const c_void) -> c_int;

    pub fn cgroup_read_stats_begin(
        controller: *const c_char,
        path: *const c_char,
        handle: *const *const c_void,
        stat: *mut cgroup_stat,
    ) -> c_int;
    pub fn cgroup_read_stats_next(handle: *const *const c_void, stat: *mut cgroup_stat) -> c_int;
    pub fn cgroup_read_stats_end(handle: *const *const c_void) -> c_int;

    pub fn cgroup_get_task_begin(
        cgroup: *const c_char,
        controller: *const c_char,
        handle: *const *const c_void,
        pid: *mut pid_t,
    ) -> c_int;
    pub fn cgroup_get_task_next(handle: *const *const c_void, pid: *mut pid_t) -> c_int;
    pub fn cgroup_get_task_end(handle: *const *const c_void) -> c_int;

    pub fn cgroup_get_controller_begin(
        handle: *const *const c_void,
        info: *mut cgroup_mount_point,
    ) -> c_int;
    pub fn cgroup_get_controller_next(
        handle: *const *const c_void,
        info: *mut cgroup_mount_point,
    ) -> c_int;
    pub fn cgroup_get_controller_end(handle: *const *const c_void) -> c_int;

    pub fn cgroup_get_all_controller_begin(
        handle: *const *const c_void,
        info: *mut controller_data,
    ) -> c_int;
    pub fn cgroup_get_all_controller_next(
        handle: *const *const c_void,
        info: *mut controller_data,
    ) -> c_int;
    pub fn cgroup_get_all_controller_end(handle: *const *const c_void) -> c_int;

    pub fn cgroup_get_subsys_mount_point_begin(
        controller: *const c_char,
        handle: *const *const c_void,
        path: *mut c_char,
    );
    pub fn cgroup_get_subsys_mount_point_next(handle: *const *const c_void, path: *mut c_char);
    pub fn cgroup_get_subsys_mount_point_end(handle: *const *const c_void);

    // groups.h
    pub fn cgroup_new_cgroup(name: *const c_char) -> *mut cgroup;
    pub fn cgroup_add_controller(
        cgroup: *const cgroup,
        name: *const c_char,
    ) -> *const cgroup_controller;
    pub fn cgroup_get_controller(
        cgroup: *const cgroup,
        name: *const c_char,
    ) -> *const cgroup_controller;
    pub fn cgroup_free(cgroup: *const *const cgroup);
    pub fn cgroup_free_controllers(cgroup: *const cgroup);
    pub fn cgroup_create_cgroup(cgroup: *const cgroup, ignore_ownership: c_int) -> c_int;
    pub fn cgroup_create_cgroup_from_parent(
        cgroup: *const cgroup,
        ignore_ownership: c_int,
    ) -> c_int;
    pub fn cgroup_modify_cgroup(cgroup: *const cgroup) -> c_int;
    pub fn cgroup_delete_cgroup(cgroup: *const cgroup, ignore_ownership: c_int) -> c_int;
    pub fn cgroup_delete_cgroup_ext(cgroup: *const cgroup, flags: c_int) -> c_int;
    pub fn cgroup_get_cgroup(cgroup: *const cgroup) -> c_int;
    pub fn cgroup_copy_cgroup(dst: *const cgroup, src: *const cgroup) -> c_int;
    pub fn cgroup_compare_cgroup(cgroup_a: *const cgroup, cgroup_b: *const cgroup) -> c_int;
    pub fn cgroup_compare_controllers(
        cgca: *const cgroup_controller,
        cgcb: *const cgroup_controller,
    ) -> c_int;
    pub fn cgroup_set_uid_gid(
        cgroup: *const cgroup,
        tasks_uid: uid_t,
        tasks_gid: gid_t,
        control_uid: uid_t,
        control_gid: gid_t,
    ) -> c_int;
    pub fn cgroup_get_uid_gid(
        cgroup: *const cgroup,
        tasks_uid: *mut uid_t,
        tasks_gid: *mut gid_t,
        control_uid: *mut uid_t,
        control_gid: *mut gid_t,
    ) -> c_int;
    pub fn cgroup_set_permissions(
        cgroup: *const cgroup,
        control_dperm: mode_t,
        control_fperm: mode_t,
        task_fperm: mode_t,
    );
    pub fn cgroup_add_value_string(
        controller: *const cgroup_controller,
        name: *const c_char,
        value: *const c_char,
    ) -> c_int;
    pub fn cgroup_add_value_int64(
        controller: *const cgroup_controller,
        name: *const c_char,
        value: i64,
    ) -> c_int;
    pub fn cgroup_add_value_uint64(
        controller: *const cgroup_controller,
        name: *const c_char,
        value: u64,
    ) -> c_int;
    pub fn cgroup_add_value_bool(
        controller: *const cgroup_controller,
        name: *const c_char,
        value: bool,
    ) -> c_int;
    pub fn cgroup_get_value_string(
        controller: *const cgroup_controller,
        name: *const c_char,
        value: *const *const c_char,
    ) -> c_int;
    pub fn cgroup_get_value_int64(
        controller: *const cgroup_controller,
        name: *const c_char,
        value: *const i64,
    ) -> c_int;
    pub fn cgroup_get_value_uint64(
        controller: *const cgroup_controller,
        name: *const c_char,
        value: *const u64,
    ) -> c_int;
    pub fn cgroup_get_value_bool(
        controller: *const cgroup_controller,
        name: *const c_char,
        value: *const bool,
    ) -> c_int;
    pub fn cgroup_set_value_string(
        controller: *const cgroup_controller,
        name: *const c_char,
        value: *const c_char,
    ) -> c_int;
    pub fn cgroup_set_value_int64(
        controller: *const cgroup_controller,
        name: *const c_char,
        value: i64,
    ) -> c_int;
    pub fn cgroup_set_value_uint64(
        controller: *const cgroup_controller,
        name: *const c_char,
        value: u64,
    ) -> c_int;
    pub fn cgroup_set_value_bool(
        controller: *const cgroup_controller,
        name: *const c_char,
        value: bool,
    ) -> c_int;
    pub fn cgroup_get_value_name_count(controller: *const cgroup_controller) -> c_int;
    pub fn cgroup_get_value_name(
        controller: *const cgroup_controller,
        index: c_int,
    ) -> *const c_char;
    pub fn cgroup_get_procs(
        name: *const c_char,
        controller: *const c_char,
        pids: *const *const pid_t,
        size: *mut c_int,
    ) -> c_int;
    pub fn cg_chmod_recursive(
        cgroup: *const cgroup,
        dir_mode: mode_t,
        dirm_change: c_int,
        file_mode: mode_t,
        filem_change: c_int,
    ) -> c_int;
    pub fn cgroup_get_cgroup_name(cgroup: *const cgroup) -> *const c_char;

    // tasks.h
    pub fn cgroup_attach_task(cgroup: *const cgroup) -> c_int;
    pub fn cgroup_attach_task_pid(cgroup: *const cgroup, tid: pid_t) -> c_int;
    pub fn cgroup_get_current_controller_path(
        pid: pid_t,
        controller: *const c_char,
        current_path: *const *const c_char,
    ) -> c_int;

    // config.h
    pub fn cgroup_config_load_config(pathname: *const c_char) -> c_int;
    pub fn cgroup_unload_cgroups() -> c_int;
    pub fn cgroup_config_unload_config(pathname: *const c_char, flags: c_int) -> c_int;
    pub fn cgroup_config_set_default(new_default: *mut cgroup) -> c_int;
    pub fn cgroup_init_templates_cache(pathname: *const c_char) -> c_int;
    pub fn cgroup_reload_cached_templates(pathname: *const c_char) -> c_int;
    pub fn cgroup_config_create_template_group(
        cgroup: *mut cgroup,
        template_name: *mut c_char,
        flags: c_int,
    ) -> c_int;
}
