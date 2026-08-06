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
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"cpp2rust_fstat_test.tmp",
    )));
    let fd: Value<i32> = Rc::new(RefCell::new({
        let __mode = match &[(420).into()].first() {
            Some(__m) => nix::sys::stat::Mode::from_bits_truncate(i32::get(__m) as ::libc::mode_t),
            None => nix::sys::stat::Mode::empty(),
        };
        match nix::fcntl::open(
            (*path.borrow()).to_rust_string().as_str(),
            nix::fcntl::OFlag::from_bits_retain(
                ((::libc::O_RDWR | ::libc::O_CREAT) | ::libc::O_TRUNC),
            ),
            __mode,
        ) {
            Ok(__ofd) => FdRegistry::register(__ofd),
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        }
    }));
    assert!(((((*fd.borrow()) >= 0) as i32) != 0));
    assert!(
        (((libcc2rs::write_refcount(
            (*fd.borrow()),
            Ptr::from_string_literal(b"hello").to_any().clone(),
            5_usize
        ) == 5_isize) as i32)
            != 0)
    );
    let st: Value<libcc2rs::Stat> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((libcc2rs::fstat_refcount((*fd.borrow()), (st.as_pointer()).clone()) == 0) as i32) != 0)
    );
    assert!(((((*st.borrow()).st_size == 5_i64) as i32) != 0));
    assert!((((libcc2rs::close_refcount((*fd.borrow())) == 0) as i32) != 0));
    assert!((((libcc2rs::unlink_refcount((*path.borrow()).clone()) == 0) as i32) != 0));
    return 0;
}
