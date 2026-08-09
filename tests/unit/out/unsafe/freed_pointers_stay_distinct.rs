extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct pair {
    pub a: *mut ::libc::FILE,
    pub b: *mut ::libc::FILE,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: *mut pair =
        (libcc2rs::calloc_unsafe(1_usize, ::std::mem::size_of::<pair>()) as *mut pair);
    (*p).a = libc::fopen(
        (c"/dev/null".as_ptr().cast_mut()).cast_const(),
        (c"w".as_ptr().cast_mut()).cast_const(),
    );
    (*p).b = libc::fopen(
        (c"/dev/null".as_ptr().cast_mut()).cast_const(),
        (c"w".as_ptr().cast_mut()).cast_const(),
    );
    assert!((((((*p).a) != ((*p).b)) as i32) != 0));
    libcc2rs::fclose_unsafe((*p).a);
    libcc2rs::fclose_unsafe((*p).b);
    let mut i: i32 = 0;
    'loop_: while (((i) < (64)) as i32) != 0 {
        let mut q: *mut libc::c_char = (libcc2rs::malloc_unsafe(16_usize) as *mut libc::c_char);
        (*q.offset(((0) as isize))) = (i as libc::c_char);
        libcc2rs::free_unsafe(((q as *mut libc::c_char) as *mut ::libc::c_void));
        i.postfix_inc();
    }
    assert!((((((*p).a) != ((*p).b)) as i32) != 0));
    libcc2rs::free_unsafe(((p as *mut pair) as *mut ::libc::c_void));
    return 0;
}
