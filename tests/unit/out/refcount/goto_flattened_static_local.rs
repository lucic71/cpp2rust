extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn step_0(mode: i32, v: i32) -> i32 {
    let mode: Value<i32> = Rc::new(RefCell::new(mode));
    let v: Value<i32> = Rc::new(RefCell::new(v));
    thread_local!(
        static base_1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([100, 200])));
    );
    thread_local!(
        static calls_2: Value<i32> = Rc::new(RefCell::new(0));
    );
    let r: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            *r.borrow_mut() = 0;
            (*calls_2.with(Value::clone).borrow_mut()).postfix_inc();
            if !((((*v.borrow()) > 0) as i32) != 0) {
                goto!('__f1_else);
            }
        }
        '__f2_then: {}
        'from_positive: {
            (*r.borrow_mut()) =
                ((*base_1.with(Value::clone).borrow())[(0) as usize] + (*v.borrow()));
            if !((((*mode.borrow()) == 1) as i32) != 0) {
                goto!('__f3_join);
            }
        }
        '__f4_then: {
            goto!('from_negative);
        }
        '__f3_join: {
            goto!('__f0_join);
        }
        '__f1_else: {
            if !((((*mode.borrow()) == 2) as i32) != 0) {
                goto!('__f5_join);
            }
        }
        '__f6_then: {
            goto!('from_positive);
        }
        '__f5_join: {}
        'from_negative: {
            (*r.borrow_mut()) =
                ((*base_1.with(Value::clone).borrow())[(1) as usize] - (*v.borrow()));
        }
        '__f0_join: {
            return (((*r.borrow()) * 10) + (*calls_2.with(Value::clone).borrow()));
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!((((({ step_0(0, 5) }) == 1051) as i32) != 0));
    assert!((((({ step_0(1, 5) }) == 1952) as i32) != 0));
    assert!((((({ step_0(0, -2_i32) }) == 2023) as i32) != 0));
    assert!((((({ step_0(2, -2_i32) }) == 984) as i32) != 0));
    return 0;
}
