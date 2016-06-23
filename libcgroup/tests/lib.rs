extern crate libcgroup;
use libcgroup::*;

#[test]
fn test_create_cgroup() {
    let cg = match CGroup::new("jimmi".to_string()) {
        Ok(cgroup) => cgroup,
        Err(err) => panic!("Should not have returned error: {}", err.description),
    };


    match cg.add_controller("memory".to_string()) {
        Ok(_) => println!("Working!"),
        Err(err) => println!("{}", err.description),
    };

    match cg.create() {
        Ok(_) => println!("Working!"),
        Err(err) => println!("{}", err.description),
    };
}
