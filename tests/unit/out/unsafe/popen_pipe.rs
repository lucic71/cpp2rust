extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut buf: [libc::c_char; 64] = [(0 as libc::c_char); 64];
    let mut in_: *mut ::libc::FILE = libc::popen(
        (c"echo hello".as_ptr().cast_mut()).cast_const(),
        (c"r".as_ptr().cast_mut()).cast_const(),
    );
    assert!((((!((in_).is_null())) as i32) != 0));
    assert!(
        (((!((libc::fgets(
            buf.as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_char; 64]>() as i32),
            in_
        ))
        .is_null())) as i32)
            != 0)
    );
    assert!(((((libcc2rs::pclose_unsafe(in_)) == (0)) as i32) != 0));
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"hello\n".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    let mut out: *mut ::libc::FILE = libc::popen(
        (c"cat > /dev/null".as_ptr().cast_mut()).cast_const(),
        (c"w".as_ptr().cast_mut()).cast_const(),
    );
    assert!((((!((out).is_null())) as i32) != 0));
    assert!(
        ((((libc::fputs((c"data\n".as_ptr().cast_mut()).cast_const(), out)) >= (0)) as i32) != 0)
    );
    assert!(((((libcc2rs::pclose_unsafe(out)) == (0)) as i32) != 0));
    (unsafe {
        libc::printf(
            (c"%s".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
            (buf.as_mut_ptr()),
        )
    });
    return 0;
}
