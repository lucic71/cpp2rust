extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_mut_0(mut str: *mut u8) {}
pub unsafe fn foo_const_1(mut str: *const u8) {}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut mutable_strings: [*mut u8; 3] = [
        b"a\0".as_ptr().cast_mut(),
        b"b\0".as_ptr().cast_mut(),
        b"c\0".as_ptr().cast_mut(),
    ];
    let mut immutable_strings: [*const u8; 3] = [
        (b"a\0".as_ptr().cast_mut()).cast_const(),
        (b"b\0".as_ptr().cast_mut()).cast_const(),
        (b"c\0".as_ptr().cast_mut()).cast_const(),
    ];
    let mut mutable_string: *mut u8 = b"hello\0".as_ptr().cast_mut();
    let mut immutable_string: *const u8 = (b"hello\0".as_ptr().cast_mut()).cast_const();
    let mut mutable_string_arr: [u8; 9] = *b"papanasi\0";
    let immutable_string_arr: [u8; 9] = *b"papanasi\0";
    let mut mutable_empty: *mut u8 = b"\0".as_ptr().cast_mut();
    let mut immutable_empty: *const u8 = (b"\0".as_ptr().cast_mut()).cast_const();
    let mut mutable_empty_arr: [u8; 1] = [0u8; 1];
    let immutable_empty_arr: [u8; 1] = [0u8; 1];
    (unsafe { foo_mut_0(b"world\0".as_ptr().cast_mut()) });
    (unsafe { foo_mut_0(mutable_string) });
    (unsafe { foo_mut_0(mutable_string_arr.as_mut_ptr()) });
    (unsafe { foo_const_1((b"world\0".as_ptr().cast_mut()).cast_const()) });
    (unsafe { foo_const_1((mutable_string).cast_const()) });
    (unsafe { foo_const_1(immutable_string) });
    (unsafe { foo_const_1((mutable_string_arr.as_mut_ptr()).cast_const()) });
    (unsafe { foo_const_1(immutable_string_arr.as_ptr()) });
    (unsafe { foo_const_1((b"\0".as_ptr().cast_mut()).cast_const()) });
    (unsafe { foo_const_1((mutable_empty).cast_const()) });
    (unsafe { foo_const_1(immutable_empty) });
    (unsafe { foo_const_1((mutable_empty_arr.as_mut_ptr()).cast_const()) });
    (unsafe { foo_const_1(immutable_empty_arr.as_ptr()) });
    let inited_through_init_list: [u8; 21] = *b"papanasi cu smantana\0";
    (unsafe { foo_const_1(inited_through_init_list.as_ptr()) });
    return 0;
}
