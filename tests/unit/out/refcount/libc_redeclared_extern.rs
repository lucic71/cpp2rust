extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fileno_0(stream: Ptr<CFile>) -> i32 {
    let stream: Value<Ptr<CFile>> = Rc::new(RefCell::new(stream));
    (*stream.borrow()).clone();
    return 42;
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ fileno_0((libcc2rs::c_stdout()).clone()) }) == 42) as i32) != 0));
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"hello")));
    assert!(((((*s.borrow()).to_c_string_iterator().count() == 5_usize) as i32) != 0));
    assert!(
        (((Ptr::from_string_literal(b"").to_c_string_iterator().count() == 0_usize) as i32) != 0)
    );
    let tty: Value<i32> = Rc::new(RefCell::new(
        match FdRegistry::with_fd(1, |__fd| nix::unistd::isatty(__fd)) {
            Ok(__tty) => __tty as i32,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                0
            }
        },
    ));
    assert!(((((*tty.borrow()) == 0) as i32) != 0));
    return 0;
}
