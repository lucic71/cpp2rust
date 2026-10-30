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
    let mut v: Vec<bool> = Vec::from([true]);
    assert!(
        ({
            let __idx = 0_usize as usize;
            let __v: &Vec<bool> = &v;
            __v[__idx]
        } as bool)
    );
    let r: Value<bool> = Rc::new(RefCell::new({
        let __idx = 0_usize as usize;
        let __v: &Vec<bool> = &v;
        __v[__idx]
    }));
    let copy: Value<bool> = Rc::new(RefCell::new((*r.borrow())));
    assert!(((*copy.borrow()) as bool));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
