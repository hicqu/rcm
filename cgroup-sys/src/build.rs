extern crate gcc;

use std::env;

fn main() {
    // CGROUP_LINKAGE (dylib, static, framework) is used to specify how
    // to link against libtls (see rustc-link-lib) - default is dylib
    let mode = env::var("CGROUP_LINKAGE").unwrap_or("dylib".to_owned());

    // If available use the paths in LIBTLS_LIBRARY_PATH to search for libraries.
    if let Ok(e_libpath) = env::var("CGROUP_LIBRARY_PATH") {
        for path in env::split_paths(&e_libpath) {
            println!("cargo:rustc-link-search=native={}", &path.to_string_lossy());
        }
    }

    if let Ok(e_libs) = env::var("CGROUP_LIBS") {
        // Link against the libraries in CGROUP_LIBS, multiple
        // libraries can specified, separated by semicolon(;)
        for lib in e_libs.split(";") {
            println!("cargo:rustc-link-lib={}={}", mode, lib);
        }
    } else {
        println!("cargo:rustc-link-lib={}=cgroup", mode);
    }
}
