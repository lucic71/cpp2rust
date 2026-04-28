extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(str: Ptr<u8>) {
    let str: Value<Ptr<u8>> = Rc::new(RefCell::new(str));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mutable_strings: Value<Box<[Ptr<u8>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::from_string_literal("a"),
        Ptr::from_string_literal("b"),
        Ptr::from_string_literal("c"),
    ])));
    let immutable_strings: Value<Box<[Ptr<u8>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::from_string_literal("a"),
        Ptr::from_string_literal("b"),
        Ptr::from_string_literal("c"),
    ])));
    let mutable_string: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal("hello")));
    let immutable_string: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal("hello")));
    ({
        let _str: Ptr<u8> = Ptr::from_string_literal("world");
        foo_0(_str)
    });
    ({
        let _str: Ptr<u8> = (*mutable_string.borrow()).clone();
        foo_0(_str)
    });
    return 0;
}
