extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let vec_: Value<Vec<u8>> = Rc::new(RefCell::new(Vec::from([195_u8, 167_u8])));
    let mut i: i32 = 27;
    let mut str: Vec<i8> = {
        let mut __bytes = Ptr::<i8>::from_string_literal(b"bar.").to_c_bytes();
        __bytes.push(0);
        __bytes
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
