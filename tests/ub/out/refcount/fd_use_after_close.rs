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
    let fd: Value<i32> = Rc::new(RefCell::new({
        let __mode = match &[].first() {
            Some(__m) => nix::sys::stat::Mode::from_bits_truncate(i32::get(__m) as ::libc::mode_t),
            None => nix::sys::stat::Mode::empty(),
        };
        match nix::fcntl::open(
            Ptr::from_string_literal(b"/dev/null\0")
                .to_rust_string()
                .as_str(),
            nix::fcntl::OFlag::from_bits_retain(::libc::O_RDONLY),
            __mode,
        ) {
            Ok(__ofd) => FdRegistry::register(__ofd),
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        }
    }));
    libcc2rs::close_refcount((*fd.borrow()));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..4).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    let n: Value<isize> = Rc::new(RefCell::new(libcc2rs::read_refcount(
        (*fd.borrow()),
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().clone(),
        4usize,
    )));
    return if (((*n.borrow()) == (-1_i32 as isize)) as i32) != 0 {
        0
    } else {
        1
    };
}
