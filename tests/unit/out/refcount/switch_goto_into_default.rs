extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn direct_label_0(x: i32, y: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let y: Value<i32> = Rc::new(RefCell::new(y));
    switch!(match (*x.borrow()) {
        __v if __v == 1 => {
            if ((*y.borrow()) != 0) {
                goto!('other);
            }
            return 10;
        }
        __v if __v == 2 => {
            return 30;
        }
        __v if false => 'other: {
            return 20;
        }
        _ => {
            goto!('other);
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn braced_label_1(x: i32, y: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let y: Value<i32> = Rc::new(RefCell::new(y));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    switch!(match (*x.borrow()) {
        __v if __v == 1 => {
            if ((*y.borrow()) != 0) {
                goto!('other);
            }
            (*r.borrow_mut()) = 10;
            break;
        }
        __v if __v == 2 => {
            (*r.borrow_mut()) = 30;
            break;
        }
        __v if false => '__default_1: {}
        __v if false => 'other: {
            (*r.borrow_mut()) = 20;
            break;
        }
        _ => {
            goto!('__default_1);
        }
    });
    return (*r.borrow());
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!((((({ direct_label_0(1, 0) }) == 10) as i32) != 0));
    assert!((((({ direct_label_0(1, 1) }) == 20) as i32) != 0));
    assert!((((({ direct_label_0(2, 0) }) == 30) as i32) != 0));
    assert!((((({ direct_label_0(5, 0) }) == 20) as i32) != 0));
    assert!((((({ direct_label_0(0, 1) }) == 20) as i32) != 0));
    assert!((((({ braced_label_1(1, 0) }) == 10) as i32) != 0));
    assert!((((({ braced_label_1(1, 1) }) == 20) as i32) != 0));
    assert!((((({ braced_label_1(2, 0) }) == 30) as i32) != 0));
    assert!((((({ braced_label_1(5, 0) }) == 20) as i32) != 0));
    assert!((((({ braced_label_1(0, 1) }) == 20) as i32) != 0));
    return 0;
}
