extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn acc_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    thread_local!(
        static total_1: Value<i32> = Rc::new(RefCell::new(5));
    );
    thread_local!(
        static limit_2: Value<i32> = Rc::new(RefCell::new(10));
    );
    goto_block!({
        '__entry: {
            if (((*x.borrow()) < 0) as i32) != 0 {
                goto!('done);
            }
            (*total_1.with(Value::clone).borrow_mut()) += (*x.borrow());
            if (((*total_1.with(Value::clone).borrow()) > (*limit_2.with(Value::clone).borrow()))
                as i32)
                != 0
            {
                (*total_1.with(Value::clone).borrow_mut()) = (*limit_2.with(Value::clone).borrow());
            }
        }
        'done: {
            return (*total_1.with(Value::clone).borrow());
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!((((({ acc_0(3) }) == 8) as i32) != 0));
    assert!((((({ acc_0(-1_i32) }) == 8) as i32) != 0));
    assert!((((({ acc_0(4) }) == 10) as i32) != 0));
    return 0;
}
