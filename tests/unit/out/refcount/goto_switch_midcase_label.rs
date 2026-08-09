extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn classify_0(kind: i32, x: i32) -> i32 {
    let kind: Value<i32> = Rc::new(RefCell::new(kind));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let len: Value<i32> = Rc::new(RefCell::new(0));
    let width: Value<i32> = Rc::new(RefCell::new(0));
    switch!(match (*kind.borrow()) {
        __v if __v == 0 => {
            (*width.borrow_mut()) = ((*x.borrow()) * 2);
            goto!('finish_width);
        }
        __v if __v == 1 => {
            (*len.borrow_mut()) = ((*x.borrow()) + 1);
            if (((*len.borrow()) > 10) as i32) != 0 {
                (*len.borrow_mut()) = 10;
            }
        }
        __v if false => 'finish_width: {
            (*width.borrow_mut()) += (*len.borrow());
            (*width.borrow_mut()).postfix_inc();
            break;
        }
        __v if __v == 2 => {
            (*len.borrow_mut()) = 50;
            goto!('finish_width);
        }
        __v if false => '__default_1: {
            (*width.borrow_mut()) = -1_i32;
            break;
        }
        _ => {
            goto!('__default_1);
        }
    });
    return (*width.borrow());
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!((((({ classify_0(0, 4) }) == 9) as i32) != 0));
    assert!((((({ classify_0(1, 2) }) == 4) as i32) != 0));
    assert!((((({ classify_0(1, 42) }) == 11) as i32) != 0));
    assert!((((({ classify_0(2, 0) }) == 51) as i32) != 0));
    assert!((((({ classify_0(7, 0) }) == -1_i32) as i32) != 0));
    return 0;
}
