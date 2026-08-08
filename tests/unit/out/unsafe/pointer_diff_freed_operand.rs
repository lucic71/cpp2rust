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
pub struct Ctx {
    pub mark: *const libc::c_char,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    static mut text_0: [libc::c_char; 12] = unsafe { std::mem::transmute(*b"hello world\0") };;
    let mut c: Ctx = <Ctx>::default();
    c.mark = (&raw const text_0[((0) as usize)] as *const libc::c_char);
    let mut tmp: *mut libc::c_char = (libcc2rs::malloc_unsafe(8_usize) as *mut libc::c_char);
    {
        if 8_usize != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((c"abcdefg".as_ptr().cast_mut() as *const libc::c_char) as *const ::libc::c_void),
                ((tmp as *mut libc::c_char) as *mut ::libc::c_void),
                8_usize as usize,
            )
        }
        ((tmp as *mut libc::c_char) as *mut ::libc::c_void)
    };
    c.mark = (tmp.offset(((2) as isize))).cast_const();
    libcc2rs::free_unsafe(((tmp as *mut libc::c_char) as *mut ::libc::c_void));
    return ((((((&raw const text_0[((6) as usize)] as *const libc::c_char) as usize)
        - (c.mark as usize))
        / ::std::mem::size_of::<libc::c_char>()) as i64) as i32);
}
