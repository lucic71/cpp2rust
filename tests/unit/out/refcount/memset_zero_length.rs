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
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4])));
    let end: Value<Ptr<i32>> = Rc::new(RefCell::new(
        (arr.as_pointer() as Ptr<i32>).offset(((4) as isize)),
    ));
    {
        ((*end.borrow()).clone() as Ptr<i32>)
            .to_any()
            .memset((0) as u8, 0_usize as usize);
        ((*end.borrow()).clone() as Ptr<i32>).to_any().clone()
    };
    {
        ((*end.borrow()).clone() as Ptr<i32>).to_any().memcpy(
            &((arr.as_pointer() as Ptr<i32>) as Ptr<i32>).to_any(),
            0_usize as usize,
        );
        ((*end.borrow()).clone() as Ptr<i32>).to_any().clone()
    };
    assert!(((((*arr.borrow())[(0) as usize] == 1) as i32) != 0));
    assert!(((((*arr.borrow())[(3) as usize] == 4) as i32) != 0));
    return 0;
}
