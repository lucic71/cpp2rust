extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn strchr_0(s: Ptr<u8>, c: i32) -> Ptr<u8> {
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(s));
    let c: Value<i32> = Rc::new(RefCell::new(c));
    return Ptr::<u8>::null();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ({
            let _s: Ptr<u8> = Ptr::from_string_literal("hello");
            let _c: i32 = ('l' as i32);
            strchr_0(_s, _c)
        }),
    ));
    assert!(((((*p.borrow()).is_null()) as i32) != 0));
    return 0;
}
