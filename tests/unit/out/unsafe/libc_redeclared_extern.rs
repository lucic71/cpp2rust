extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fileno_0(mut stream: *mut ::libc::FILE) -> i32 {
    &(stream);
    return 42;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sink {
    pub in_: *mut ::libc::FILE,
    pub closer: Option<unsafe fn(*mut ::libc::FILE) -> i32>,
}
impl Default for sink {
    fn default() -> Self {
        sink {
            in_: std::ptr::null_mut(),
            closer: None,
        }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((unsafe { fileno_0(libcc2rs::stdout_unsafe()) }) == (42)) as i32) != 0));
    let mut s: *const libc::c_char = (c"hello".as_ptr().cast_mut()).cast_const();
    assert!(((((libc::strlen(s)) == (5_usize)) as i32) != 0));
    assert!(((((libc::strlen((c"".as_ptr().cast_mut()).cast_const())) == (0_usize)) as i32) != 0));
    let mut tty: i32 = libc::isatty(1);
    assert!(((((tty) == (0)) as i32) != 0));
    let mut k: sink = <sink>::default();
    k.in_ = libc::popen(
        (c"exit 7".as_ptr().cast_mut()).cast_const(),
        (c"r".as_ptr().cast_mut()).cast_const(),
    );
    assert!((((!((k.in_).is_null())) as i32) != 0));
    (k.closer) = Some(libcc2rs::pclose_unsafe);
    assert!(
        ((((unsafe {
            let _arg0: *mut ::libc::FILE = k.in_;
            (k.closer).unwrap()(_arg0)
        }) == ((7) * (256))) as i32)
            != 0)
    );
    k.in_ = libc::fopen(
        (c"/dev/null".as_ptr().cast_mut()).cast_const(),
        (c"r".as_ptr().cast_mut()).cast_const(),
    );
    assert!((((!((k.in_).is_null())) as i32) != 0));
    (k.closer) = Some(libcc2rs::fclose_unsafe);
    assert!(
        ((((unsafe {
            let _arg0: *mut ::libc::FILE = k.in_;
            (k.closer).unwrap()(_arg0)
        }) == (0)) as i32)
            != 0)
    );
    return 0;
}
