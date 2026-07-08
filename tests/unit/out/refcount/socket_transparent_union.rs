extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let fd: Value<i32> = Rc::new(RefCell::new(0));
    let ssloc: Value<libc::sockaddr_storage> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
    let slen: Value<u32> = Rc::new(RefCell::new((128usize as u32)));
    assert!(
        ((({
            let result: i32 = todo!("getsockname is not implemented yet");
            result
        } == -1_i32) as i32)
            != 0)
    );
    let sin: Value<::libc::sockaddr_in> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
    let inlen: Value<u32> = Rc::new(RefCell::new((16usize as u32)));
    assert!(
        ((({
            let result: i32 = todo!("getsockname is not implemented yet");
            result
        } == -1_i32) as i32)
            != 0)
    );
    return 0;
}
