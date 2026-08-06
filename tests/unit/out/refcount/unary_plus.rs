extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(5));
    let d: Value<f64> = Rc::new(RefCell::new(5.0E-1));
    let b: Value<i32> = Rc::new(RefCell::new(-3_i32));
    assert!(((((*a.borrow()) == 5) as i32) != 0));
    assert!(((((*b.borrow()) == -3_i32) as i32) != 0));
    assert!((((((*a.borrow()) + (*b.borrow())) == 2) as i32) != 0));
    assert!((((((*d.borrow()) * 4.0E+0) == 2.0E+0) as i32) != 0));
    assert!(
        ((((if ((((*a.borrow()) > 0) as i32) != 0) {
            1
        } else {
            -1_i32
        }) == 1) as i32)
            != 0)
    );
    return 0;
}
