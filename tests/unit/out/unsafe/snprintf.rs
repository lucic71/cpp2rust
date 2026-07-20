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
    let mut buf: [libc::c_char; 32] = [(0 as libc::c_char); 32];
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize,
                (c"x=%d y=%u".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (-3_i32),
                (7_u32),
            )
        }) == (8)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"x=-3 y=7".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                4_usize as usize,
                (c"%s".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (c"hello".as_ptr().cast_mut()),
            )
        }) == (5)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"hel".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize,
                (c"%05d|%02x".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (42),
                (255),
            )
        }) == (8)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"00042|ff".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize,
                (c"%c%%".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (65),
            )
        }) == (2)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"A%".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize,
                (c"%ld %lu %zu".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (-1_i64),
                (1_u64),
                (9 as usize),
            )
        }) == (6)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"-1 1 9".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    buf[(0) as usize] = (('Z' as i32) as libc::c_char);
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                0_usize as usize,
                (c"%d".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (123),
            )
        }) == (3)) as i32)
            != 0)
    );
    assert!(((((buf[(0) as usize] as i32) == ('Z' as i32)) as i32) != 0));
    return 0;
}
