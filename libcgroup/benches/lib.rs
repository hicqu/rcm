#![feature(test)]

extern crate test;

extern crate libcgroup;

use libcgroup::*;

use test::Bencher;

#[bench]
fn bench_iterate_cgroup_tree(b: &mut Bencher) {
    init();
    b.iter(|| {
        let mut found = false;
        for c in cgroup_walk_tree_iter("devices") {
            found = match c {
                Ok(_) => {
                    /*match f {
                        CGroupFile::Dir(fi) => { println!("{:?}", fi); },
                        _ => {}
                    }*/
                    true
                },
                Err(err) => panic!("{}", err.description),
            }
        }
        assert!(found);
    })
}
