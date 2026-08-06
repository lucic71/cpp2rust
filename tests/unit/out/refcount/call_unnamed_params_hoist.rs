extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn pick_0(a: Ptr<u8>, b: Ptr<u8>, n: i32) -> i32 {
    let a: Value<Ptr<u8>> = Rc::new(RefCell::new(a));
    let b: Value<Ptr<u8>> = Rc::new(RefCell::new(b));
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return {
        let _lhs = {
            let _lhs = (if ((({
                let _lhs = (*a.borrow()).clone();
                _lhs == (*b.borrow()).clone()
            }) as i32)
                != 0)
            {
                10
            } else {
                20
            });
            _lhs + (*n.borrow())
        };
        _lhs + ((((*a.borrow()).offset(((0) as isize)).read()) as i32) - ('a' as i32))
    };
}
pub fn total_1(x: Ptr<i32>, y: Ptr<i32>) -> i32 {
    let x: Value<Ptr<i32>> = Rc::new(RefCell::new(x));
    let y: Value<Ptr<i32>> = Rc::new(RefCell::new(y));
    {
        let _ptr = (*x.borrow()).clone();
        _ptr.write((_ptr.read()) + 1)
    };
    return {
        let _lhs = ((*x.borrow()).read());
        _lhs + ((*y.borrow()).read())
    };
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"abc")));
    let t: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"bcd")));
    let n: Value<i32> = Rc::new(RefCell::new(5));
    let v: Value<i32> = Rc::new(RefCell::new(4));
    assert!(
        (((({
            let _a: Ptr<u8> = (*s.borrow()).clone();
            let _b: Ptr<u8> = (*s.borrow()).clone();
            pick_0(_a, _b, (*n.borrow()))
        }) == 15) as i32)
            != 0)
    );
    assert!(
        (((({ pick_0((*s.borrow()).clone(), (*t.borrow()).clone(), (*n.borrow())) }) == 25)
            as i32)
            != 0)
    );
    assert!(
        (((({
            let _x: Ptr<i32> = (v.as_pointer());
            let _y: Ptr<i32> = (v.as_pointer());
            total_1(_x, _y)
        }) == 10) as i32)
            != 0)
    );
    assert!(((((*v.borrow()) == 5) as i32) != 0));
    return 0;
}
