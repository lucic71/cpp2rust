extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn run_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let steps: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            *steps.borrow_mut() = 0;
            if ((((*x.borrow()) < 0) as i32) != 0) {
                goto!('error);
            }
            (*steps.borrow_mut()) = 1;
            if ((((*x.borrow()) == 0) as i32) != 0) {
                goto!('done);
            }
            (*steps.borrow_mut()) = 2;
        }
        'error: {}
        'done: {
            (*steps.borrow_mut()) += 10;
            return (*steps.borrow());
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!((((({ run_0(-1_i32) }) == 10) as i32) != 0));
    assert!((((({ run_0(0) }) == 11) as i32) != 0));
    assert!((((({ run_0(5) }) == 12) as i32) != 0));
    return 0;
}
