extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn reduce_0(rule: i32, v: i32) -> i32 {
    let rule: Value<i32> = Rc::new(RefCell::new(rule));
    let v: Value<i32> = Rc::new(RefCell::new(v));
    let acc: Value<i32> = Rc::new(RefCell::new(0));
    let tmp: Value<i32> = <Value<i32>>::default();
    let wide: Value<i64> = <Value<i64>>::default();
    'switch: {
        let __match_cond = (*rule.borrow());
        match __match_cond {
            __v if __v == 0 => {
                (*tmp.borrow_mut()) = ((*v.borrow()) * 2);
                (*acc.borrow_mut()) = ((*tmp.borrow()) + 1);
                break 'switch;
            }
            __v if __v == 1 => {
                (*wide.borrow_mut()) = (((*v.borrow()) as i64) + 10_i64);
                (*acc.borrow_mut()) = (((*wide.borrow()) * 2_i64) as i32);
                break 'switch;
            }
            __v if __v == 2 => {
                (*tmp.borrow_mut()) = ((*v.borrow()) - 1);
                (*wide.borrow_mut()) = ((*tmp.borrow()) as i64);
                (*acc.borrow_mut()) = (((*wide.borrow()) as i32) * 3);
                break 'switch;
            }
            _ => {
                (*acc.borrow_mut()) = -1_i32;
                break 'switch;
            }
        }
    };
    return (*acc.borrow());
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!((((({ reduce_0(0, 5) }) == 11) as i32) != 0));
    assert!((((({ reduce_0(1, 5) }) == 30) as i32) != 0));
    assert!((((({ reduce_0(2, 5) }) == 12) as i32) != 0));
    assert!((((({ reduce_0(9, 5) }) == -1_i32) as i32) != 0));
    return 0;
}
