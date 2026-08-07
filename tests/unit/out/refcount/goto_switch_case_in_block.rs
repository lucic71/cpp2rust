extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn route_0(op: i32, v: i32) -> i32 {
    let op: Value<i32> = Rc::new(RefCell::new(op));
    let v: Value<i32> = Rc::new(RefCell::new(v));
    let out: Value<i32> = Rc::new(RefCell::new(0));
    let base: Value<i32> = <Value<i32>>::default();
    switch!(match (*op.borrow()) {
        __v if __v == 1 => {
            let base: Value<i32> = Rc::new(RefCell::new(((*v.borrow()) * 10)));
            if ((((*v.borrow()) > 3) as i32) != 0) {
                (*out.borrow_mut()) = ((*base.borrow()) + 1);
                goto!('tail);
            }
            (*v.borrow_mut()) = (*base.borrow());
        }
        __v if __v == 2 => {
            (*base.borrow_mut()) = ((*v.borrow()) + 7);
            (*out.borrow_mut()) = ((*base.borrow()) * 2);
        }
        __v if false => 'tail: {
            (*out.borrow_mut()) += 3;
            break;
        }
        __v if __v == 3 => {
            (*out.borrow_mut()) = -(*v.borrow());
            break;
        }
        __v if false => '__default_1: {
            (*out.borrow_mut()) = 99;
            break;
        }
        _ => {
            goto!('__default_1);
        }
    });
    return (*out.borrow());
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!((((({ route_0(1, 5) }) == 54) as i32) != 0));
    assert!((((({ route_0(1, 2) }) == 57) as i32) != 0));
    assert!((((({ route_0(2, 10) }) == 37) as i32) != 0));
    assert!((((({ route_0(3, 4) }) == -4_i32) as i32) != 0));
    assert!((((({ route_0(9, 0) }) == 99) as i32) != 0));
    return 0;
}
