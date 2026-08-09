extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn pick_0(op: i32, x: i32) -> i32 {
    let op: Value<i32> = Rc::new(RefCell::new(op));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    switch!(match (*op.borrow()) {
        __v if __v == 1 => {
            if (((*x.borrow()) == 0) as i32) != 0 {
                (*r.borrow_mut()) = 5;
                break;
            }
            goto!('shared);
        }
        __v if __v == 2 => 'shared: {
            let t: Value<i32> = Rc::new(RefCell::new(((*x.borrow()) * 3)));
            (*r.borrow_mut()) = ((*t.borrow()) + 1);
            break;
        }
        __v if false => '__default_1: {
            (*r.borrow_mut()) = -1_i32;
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
    assert!((((({ pick_0(1, 0) }) == 5) as i32) != 0));
    assert!((((({ pick_0(1, 4) }) == 13) as i32) != 0));
    assert!((((({ pick_0(2, 2) }) == 7) as i32) != 0));
    assert!((((({ pick_0(0, 9) }) == -1_i32) as i32) != 0));
    return 0;
}
