extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn for_test_0(mut n: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(0));
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    'loop_: while (i < n) {
        (*x.borrow_mut()) += 1;
        if ((*x.borrow()) == 100) {
            break;
        }
        let mut k: i32 = 0;
        let mut w: i32 = 0;
        'loop_: while (w < j) {
            break;
            {
                {
                    w += 1;
                    k += 1
                };
                i += k
            };
        }
        (*x.borrow_mut()) = { ((*x.borrow()) + 1) };
        {
            j = i;
            i += 1
        };
    }
    return (*x.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ for_test_0(200,) }) == 400));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
