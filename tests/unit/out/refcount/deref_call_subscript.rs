extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static g_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
pub fn get_1() -> Ptr<()> {
    return g_0.with(|v| v.as_pointer());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!().write(4_());
    let mut v: () = unimplemented!();
    unimplemented!().write(5_());
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = g_0.with(|_| ());
}
