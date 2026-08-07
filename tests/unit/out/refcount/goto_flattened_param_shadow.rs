extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn dispatch_0(op: i32, flags: i32) -> i32 {
    let op: Value<i32> = Rc::new(RefCell::new(op));
    let flags: Value<i32> = Rc::new(RefCell::new(flags));
    let r: Value<i32> = <Value<i32>>::default();
    let flags__1: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            *r.borrow_mut() = 0;
            if !((((*op.borrow()) == 1) as i32) != 0) {
                goto!('__f1_else);
            }
        }
        '__f2_then: {}
        'from_op: {
            (*flags__1.borrow_mut()) = 7;
            (*r.borrow_mut()) += (*flags__1.borrow());
            goto!('__f0_join);
        }
        '__f1_else: {
            if !((((*op.borrow()) == 2) as i32) != 0) {
                goto!('__f3_join);
            }
        }
        '__f4_then: {
            goto!('from_op);
        }
        '__f3_join: {
            (*r.borrow_mut()) += 100;
        }
        '__f0_join: {
            if !(((*flags.borrow()) & 4) != 0) {
                goto!('__f5_join);
            }
        }
        '__f6_then: {
            (*r.borrow_mut()) += 1000;
        }
        '__f5_join: {
            return (*r.borrow());
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!((((({ dispatch_0(1, 4) }) == 1007) as i32) != 0));
    assert!((((({ dispatch_0(0, 4) }) == 1100) as i32) != 0));
    assert!((((({ dispatch_0(2, 4) }) == 1007) as i32) != 0));
    assert!((((({ dispatch_0(1, 0) }) == 7) as i32) != 0));
    return 0;
}
