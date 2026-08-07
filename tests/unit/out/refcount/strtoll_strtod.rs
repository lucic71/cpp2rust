extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let end: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
    (*s.borrow_mut()) = Ptr::from_string_literal(b"42\0");
    assert!(
        ((((((libcc2rs::strtoll_refcount((*s.borrow()).clone(), (end.as_pointer()).clone(), 10)
            == 42_i64) as i32)
            != 0)
            && ((((((*end.borrow()).clone() - (*s.borrow()).clone()) as i64) == 2_i64) as i32)
                != 0)) as i32)
            != 0)
    );
    (*s.borrow_mut()) = Ptr::from_string_literal(b"  -17abc\0");
    assert!(
        ((((((libcc2rs::strtoll_refcount((*s.borrow()).clone(), (end.as_pointer()).clone(), 10)
            == (-17_i32 as i64)) as i32)
            != 0)
            && ((((((*end.borrow()).clone() - (*s.borrow()).clone()) as i64) == 5_i64) as i32)
                != 0)) as i32)
            != 0)
    );
    (*s.borrow_mut()) = Ptr::from_string_literal(b"0xff\0");
    assert!(
        ((((((libcc2rs::strtoll_refcount((*s.borrow()).clone(), (end.as_pointer()).clone(), 16)
            == 255_i64) as i32)
            != 0)
            && ((((((*end.borrow()).clone() - (*s.borrow()).clone()) as i64) == 4_i64) as i32)
                != 0)) as i32)
            != 0)
    );
    assert!(
        ((((((libcc2rs::strtoll_refcount((*s.borrow()).clone(), (end.as_pointer()).clone(), 0)
            == 255_i64) as i32)
            != 0)
            && ((((((*end.borrow()).clone() - (*s.borrow()).clone()) as i64) == 4_i64) as i32)
                != 0)) as i32)
            != 0)
    );
    (*s.borrow_mut()) = Ptr::from_string_literal(b"0755\0");
    assert!(
        ((((((libcc2rs::strtoll_refcount((*s.borrow()).clone(), (end.as_pointer()).clone(), 0)
            == 493_i64) as i32)
            != 0)
            && ((((((*end.borrow()).clone() - (*s.borrow()).clone()) as i64) == 4_i64) as i32)
                != 0)) as i32)
            != 0)
    );
    (*s.borrow_mut()) = Ptr::from_string_literal(b"0x\0");
    assert!(
        ((((((libcc2rs::strtoll_refcount((*s.borrow()).clone(), (end.as_pointer()).clone(), 16)
            == 0_i64) as i32)
            != 0)
            && ((((((*end.borrow()).clone() - (*s.borrow()).clone()) as i64) == 1_i64) as i32)
                != 0)) as i32)
            != 0)
    );
    (*s.borrow_mut()) = Ptr::from_string_literal(b"9223372036854775808\0");
    assert!(
        ((((((libcc2rs::strtoll_refcount((*s.borrow()).clone(), (end.as_pointer()).clone(), 10)
            == 9223372036854775807_i64) as i32)
            != 0)
            && ((((((*end.borrow()).clone() - (*s.borrow()).clone()) as i64) == 19_i64) as i32)
                != 0)) as i32)
            != 0)
    );
    (*s.borrow_mut()) = Ptr::from_string_literal(b"-9223372036854775809\0");
    assert!(
        ((((((libcc2rs::strtoll_refcount((*s.borrow()).clone(), (end.as_pointer()).clone(), 10)
            == (-9223372036854775807_i64 - 1_i64)) as i32)
            != 0)
            && ((((((*end.borrow()).clone() - (*s.borrow()).clone()) as i64) == 20_i64) as i32)
                != 0)) as i32)
            != 0)
    );
    (*s.borrow_mut()) = Ptr::from_string_literal(b"junk\0");
    assert!(
        ((((((libcc2rs::strtoll_refcount((*s.borrow()).clone(), (end.as_pointer()).clone(), 10)
            == 0_i64) as i32)
            != 0)
            && ((({
                let _lhs = (*end.borrow()).clone();
                _lhs == (*s.borrow()).clone()
            }) as i32)
                != 0)) as i32)
            != 0)
    );
    (*s.borrow_mut()) = Ptr::from_string_literal(b"z\0");
    assert!(
        ((((((libcc2rs::strtoll_refcount((*s.borrow()).clone(), (end.as_pointer()).clone(), 36)
            == 35_i64) as i32)
            != 0)
            && ((((((*end.borrow()).clone() - (*s.borrow()).clone()) as i64) == 1_i64) as i32)
                != 0)) as i32)
            != 0)
    );
    assert!(
        (((libcc2rs::strtoll_refcount(
            Ptr::from_string_literal(b"55\0").clone(),
            Ptr::<Ptr::<u8>>::null().clone(),
            10
        ) == 55_i64) as i32)
            != 0)
    );
    (*s.borrow_mut()) = Ptr::from_string_literal(b"3.14\0");
    assert!(
        ((((((libcc2rs::strtod_refcount((*s.borrow()).clone(), (end.as_pointer()).clone())
            == 3.1400000000000001E+0) as i32)
            != 0)
            && ((((((*end.borrow()).clone() - (*s.borrow()).clone()) as i64) == 4_i64) as i32)
                != 0)) as i32)
            != 0)
    );
    (*s.borrow_mut()) = Ptr::from_string_literal(b"  -2.5e3xyz\0");
    assert!(
        ((((((libcc2rs::strtod_refcount((*s.borrow()).clone(), (end.as_pointer()).clone())
            == -2.5E+3) as i32)
            != 0)
            && ((((((*end.borrow()).clone() - (*s.borrow()).clone()) as i64) == 8_i64) as i32)
                != 0)) as i32)
            != 0)
    );
    (*s.borrow_mut()) = Ptr::from_string_literal(b"1.e5\0");
    assert!(
        ((((((libcc2rs::strtod_refcount((*s.borrow()).clone(), (end.as_pointer()).clone())
            == 1.0E+5) as i32)
            != 0)
            && ((((((*end.borrow()).clone() - (*s.borrow()).clone()) as i64) == 4_i64) as i32)
                != 0)) as i32)
            != 0)
    );
    (*s.borrow_mut()) = Ptr::from_string_literal(b".5\0");
    assert!(
        ((((((libcc2rs::strtod_refcount((*s.borrow()).clone(), (end.as_pointer()).clone())
            == 5.0E-1) as i32)
            != 0)
            && ((((((*end.borrow()).clone() - (*s.borrow()).clone()) as i64) == 2_i64) as i32)
                != 0)) as i32)
            != 0)
    );
    (*s.borrow_mut()) = Ptr::from_string_literal(b"1e\0");
    assert!(
        ((((((libcc2rs::strtod_refcount((*s.borrow()).clone(), (end.as_pointer()).clone())
            == 1.0E+0) as i32)
            != 0)
            && ((((((*end.borrow()).clone() - (*s.borrow()).clone()) as i64) == 1_i64) as i32)
                != 0)) as i32)
            != 0)
    );
    (*s.borrow_mut()) = Ptr::from_string_literal(b"junk\0");
    assert!(
        ((((((libcc2rs::strtod_refcount((*s.borrow()).clone(), (end.as_pointer()).clone())
            == 0.0E+0) as i32)
            != 0)
            && ((({
                let _lhs = (*end.borrow()).clone();
                _lhs == (*s.borrow()).clone()
            }) as i32)
                != 0)) as i32)
            != 0)
    );
    assert!(
        (((libcc2rs::strtod_refcount(
            Ptr::from_string_literal(b"+0.375e-1\0").clone(),
            Ptr::<Ptr::<u8>>::null().clone()
        ) == 3.7499999999999999E-2) as i32)
            != 0)
    );
    return 0;
}
