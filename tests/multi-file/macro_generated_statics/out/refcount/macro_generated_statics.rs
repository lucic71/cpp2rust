extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn scaleA_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((*x.borrow()) * 2);
}
pub fn shiftA_1(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((*x.borrow()) - 2);
}
pub fn scaleB_2(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((*x.borrow()) * 3);
}
pub fn shiftB_3(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((*x.borrow()) - 3);
}
pub fn pmin_int_4(a: i32, b: i32) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return if ((((*a.borrow()) < (*b.borrow())) as i32) != 0) {
        (*a.borrow())
    } else {
        (*b.borrow())
    };
}
pub fn pmax_int_5(a: i32, b: i32) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return if ((((*a.borrow()) > (*b.borrow())) as i32) != 0) {
        (*a.borrow())
    } else {
        (*b.borrow())
    };
}
pub fn pmin_long_6(a: i64, b: i64) -> i64 {
    let a: Value<i64> = Rc::new(RefCell::new(a));
    let b: Value<i64> = Rc::new(RefCell::new(b));
    return if ((((*a.borrow()) < (*b.borrow())) as i32) != 0) {
        (*a.borrow())
    } else {
        (*b.borrow())
    };
}
pub fn pmax_long_7(a: i64, b: i64) -> i64 {
    let a: Value<i64> = Rc::new(RefCell::new(a));
    let b: Value<i64> = Rc::new(RefCell::new(b));
    return if ((((*a.borrow()) > (*b.borrow())) as i32) != 0) {
        (*a.borrow())
    } else {
        (*b.borrow())
    };
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!((((({ scaleA_0(5) }) == 10) as i32) != 0));
    assert!((((({ scaleB_2(5) }) == 15) as i32) != 0));
    assert!((((({ shiftA_1(5) }) == 3) as i32) != 0));
    assert!((((({ shiftB_3(5) }) == 2) as i32) != 0));
    assert!((((({ pmin_int_4(3, 4) }) == 3) as i32) != 0));
    assert!((((({ pmax_int_5(3, 4) }) == 4) as i32) != 0));
    assert!((((({ pmin_long_6(3_i64, 4_i64) }) == 3_i64) as i32) != 0));
    assert!((((({ pmax_long_7(3_i64, 4_i64) }) == 4_i64) as i32) != 0));
    assert!((((({ combine_8(5) }) == 35) as i32) != 0));
    return 0;
}
pub fn combine_8(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((((({ scaleA_0((*x.borrow())) }) + ({ scaleB_2((*x.borrow())) }))
        + ({ shiftA_1((*x.borrow())) }))
        + ({ shiftB_3((*x.borrow())) }))
        + (({ pmax_long_7(((*x.borrow()) as i64), 0_i64) }) as i32));
}
