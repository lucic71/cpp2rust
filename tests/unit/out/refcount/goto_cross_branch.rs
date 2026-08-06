extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn compute_0(op: i32, a: i32, b: i32) -> i32 {
    let op: Value<i32> = Rc::new(RefCell::new(op));
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    let r: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            *r.borrow_mut() = 0;
            if !((((*a.borrow()) > 0) as i32) != 0) {
                goto!('__f1_else);
            }
        }
        '__f2_then: {}
        'int_path: {
            (*r.borrow_mut()) = ((*a.borrow()) + (*b.borrow()));
            if !((*op.borrow()) != 0) {
                goto!('__f3_join);
            }
        }
        '__f4_then: {
            goto!('fp_path);
        }
        '__f3_join: {
            goto!('__f0_join);
        }
        '__f1_else: {
            if !((((*b.borrow()) > 0) as i32) != 0) {
                goto!('__f5_join);
            }
        }
        '__f6_then: {
            goto!('int_path);
        }
        '__f5_join: {}
        'fp_path: {
            (*r.borrow_mut()) = ((*a.borrow()) * (*b.borrow()));
        }
        '__f0_join: {
            return (*r.borrow());
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!((((({ compute_0(0, 5, 3) }) == 8) as i32) != 0));
    assert!((((({ compute_0(1, 5, 3) }) == 15) as i32) != 0));
    assert!((((({ compute_0(0, -2_i32, 4) }) == 2) as i32) != 0));
    assert!((((({ compute_0(0, -2_i32, -4_i32) }) == 8) as i32) != 0));
    assert!((((({ compute_0(1, -2_i32, -4_i32) }) == 8) as i32) != 0));
    return 0;
}
