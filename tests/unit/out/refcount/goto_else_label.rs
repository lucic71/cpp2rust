extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static fails_0: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn fail_mark_1() -> i32 {
    (*fails_0.with(Value::clone).borrow_mut()).postfix_inc();
    return -1_i32;
}
pub fn helper_2(mode: i32, v: i32) -> i32 {
    let mode: Value<i32> = Rc::new(RefCell::new(mode));
    let v: Value<i32> = Rc::new(RefCell::new(v));
    let r: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            *r.borrow_mut() = 0;
            if !((((*mode.borrow()) == 1) as i32) != 0) {
                goto!('__f1_else);
            }
        }
        '__f2_then: {
            if !((((*v.borrow()) < 0) as i32) != 0) {
                goto!('__f3_join);
            }
        }
        '__f4_then: {
            goto!('bad_input);
        }
        '__f3_join: {
            (*r.borrow_mut()) = ((*v.borrow()) * 2);
            goto!('__f0_join);
        }
        '__f1_else: {
            if !((((*mode.borrow()) == 2) as i32) != 0) {
                goto!('__f6_else);
            }
        }
        '__f7_then: {
            if !((((*v.borrow()) == 0) as i32) != 0) {
                goto!('__f8_join);
            }
        }
        '__f9_then: {
            goto!('bad_input);
        }
        '__f8_join: {
            (*r.borrow_mut()) = (100 / (*v.borrow()));
            goto!('__f5_join);
        }
        '__f6_else: {}
        'bad_input: {
            (*r.borrow_mut()) = ({ fail_mark_1() });
        }
        '__f5_join: {}
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
    assert!((((({ helper_2(1, 4) }) == 8) as i32) != 0));
    assert!((((({ helper_2(1, -1_i32) }) == -1_i32) as i32) != 0));
    assert!((((({ helper_2(2, 5) }) == 20) as i32) != 0));
    assert!((((({ helper_2(2, 0) }) == -1_i32) as i32) != 0));
    assert!((((({ helper_2(7, 3) }) == -1_i32) as i32) != 0));
    assert!(((((*fails_0.with(Value::clone).borrow()) == 3) as i32) != 0));
    return 0;
}
