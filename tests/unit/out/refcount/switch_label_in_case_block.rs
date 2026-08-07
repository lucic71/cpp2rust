extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn f_0(op: i32, v: i32) -> i32 {
    let op: Value<i32> = Rc::new(RefCell::new(op));
    let v: Value<i32> = Rc::new(RefCell::new(v));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    let a: Value<i32> = <Value<i32>>::default();
    let a__1: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
    switch!(match (*op.borrow()) {
        __v if __v == 1 => {
            *a.borrow_mut() = ((*v.borrow()) * 4);
            if ((*v.borrow()) != 0) {
                goto!('l1);
            }
            break;
        }
        __v if false => 'l1: {
            (*r.borrow_mut()) = (*a.borrow());
            break;
        }
        __v if __v == 2 => {
            *a__1.borrow_mut() = Ptr::from_string_literal(b"abcd\0");
            if ((*v.borrow()) != 0) {
                goto!('l2);
            }
            break;
        }
        __v if false => 'l2: {
            (*r.borrow_mut()) = ((*a__1.borrow()).to_c_string_iterator().count() as i32);
            break;
        }
        _ => {}
    });
    return (*r.borrow());
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!((((({ f_0(1, 3) }) == 12) as i32) != 0));
    assert!((((({ f_0(2, 1) }) == 4) as i32) != 0));
    assert!((((({ f_0(1, 0) }) == 0) as i32) != 0));
    return 0;
}
