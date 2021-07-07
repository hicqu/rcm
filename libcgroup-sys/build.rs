use std::path::Path;
use std::process::Command;

fn main() {
    let status = Command::new("./bootstrap.sh")
        .current_dir("libcgroup")
        .spawn()
        .and_then(|mut c| c.wait())
        .unwrap();
    if !status.success() {
        panic!("bootstrap.sh fails");
    }

    let status = Command::new("./configure")
        .args(&["--disable-pam"])
        .current_dir("libcgroup")
        .spawn()
        .and_then(|mut c| c.wait())
        .unwrap();
    if !status.success() {
        panic!("configure fails");
    }

    let status = Command::new("make")
        .current_dir("libcgroup")
        .spawn()
        .and_then(|mut c| c.wait())
        .unwrap();
    if !status.success() {
        panic!("make fails");
    }

    let out_dir = Path::new("libcgroup/src/.libs").canonicalize().unwrap();
    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static=cgroup");
    println!("cargo:rerun-if-changed=libcgroup/src");
}
