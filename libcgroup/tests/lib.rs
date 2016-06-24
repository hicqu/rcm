extern crate libcgroup;
use libcgroup::*;

#[test]
fn test_create_cgroup() {
    let cg = match CGroup::new("jimmi") {
        Ok(cgroup) => cgroup,
        Err(err) => panic!("Should not have returned error: {}", err.description),
    };


    match cg.add_controller("memory") {
        Ok(_) => println!("Working!"),
        Err(err) => panic!("{}", err.description),
    };

    match cg.create() {
        Ok(_) => println!("Working!"),
        Err(err) => panic!("{}", err.description),
    };
}
