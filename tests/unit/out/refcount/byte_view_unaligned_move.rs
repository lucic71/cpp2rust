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
    let words: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        0_u32,
        <u32>::default(),
        <u32>::default(),
        <u32>::default(),
    ])));
    let bytes: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (words.as_pointer() as Ptr<u32>).reinterpret_cast::<u8>(),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((*i.borrow()) < 6) as i32) != 0) {
        {
            let __rhs = (((*i.borrow()) + 1) as u8);
            (*bytes.borrow())
                .offset(((6 + (*i.borrow())) as isize))
                .write(__rhs)
        };
        (*i.borrow_mut()).postfix_inc();
    }
    (*bytes.borrow()).offset(((12) as isize)).write(170_u8);
    {
        ((*bytes.borrow()).offset(((7) as isize)) as Ptr<u8>)
            .to_any()
            .memcpy(
                &((*bytes.borrow()).offset(((6) as isize)) as Ptr<u8>).to_any(),
                6_usize as usize,
            );
        ((*bytes.borrow()).offset(((7) as isize)) as Ptr<u8>)
            .to_any()
            .clone()
    };
    assert!(((((((*bytes.borrow()).offset(((6) as isize)).read()) as i32) == 1) as i32) != 0));
    assert!(((((((*bytes.borrow()).offset(((7) as isize)).read()) as i32) == 1) as i32) != 0));
    assert!(((((((*bytes.borrow()).offset(((8) as isize)).read()) as i32) == 2) as i32) != 0));
    assert!(((((((*bytes.borrow()).offset(((9) as isize)).read()) as i32) == 3) as i32) != 0));
    assert!(((((((*bytes.borrow()).offset(((10) as isize)).read()) as i32) == 4) as i32) != 0));
    assert!(((((((*bytes.borrow()).offset(((11) as isize)).read()) as i32) == 5) as i32) != 0));
    assert!(((((((*bytes.borrow()).offset(((12) as isize)).read()) as i32) == 6) as i32) != 0));
    assert!(((((((*bytes.borrow()).offset(((13) as isize)).read()) as i32) == 0) as i32) != 0));
    let src: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        9_u8, 8_u8, 7_u8, 6_u8, 5_u8, 4_u8, 90_u8,
    ])));
    {
        ((*bytes.borrow()).offset(((2) as isize)) as Ptr<u8>)
            .to_any()
            .memcpy(
                &((src.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                7_usize as usize,
            );
        ((*bytes.borrow()).offset(((2) as isize)) as Ptr<u8>)
            .to_any()
            .clone()
    };
    assert!(((((((*bytes.borrow()).offset(((1) as isize)).read()) as i32) == 0) as i32) != 0));
    assert!(((((((*bytes.borrow()).offset(((2) as isize)).read()) as i32) == 9) as i32) != 0));
    assert!(((((((*bytes.borrow()).offset(((7) as isize)).read()) as i32) == 4) as i32) != 0));
    assert!(((((((*bytes.borrow()).offset(((8) as isize)).read()) as i32) == 90) as i32) != 0));
    assert!(((((((*bytes.borrow()).offset(((9) as isize)).read()) as i32) == 3) as i32) != 0));
    return 0;
}
