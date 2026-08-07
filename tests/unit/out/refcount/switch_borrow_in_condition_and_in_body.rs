extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn borrow_in_condition_and_in_body_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    switch!(match (*x.borrow()) {
        __v if __v == 0 => {}
        __v if false => '__default_1: {
            return ((*x.borrow()) + 1);
        }
        _ => {
            goto!('__default_1);
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!((({ borrow_in_condition_and_in_body_0(0) }) == 1));
    assert!((({ borrow_in_condition_and_in_body_0(1) }) == 2));
    return 0;
}
