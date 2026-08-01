extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn cmp_0(mut a: *const ::libc::c_void, mut b: *const ::libc::c_void) -> i32 {
    return libc::strcmp(
        (*(a as *const *const libc::c_char)),
        (*(b as *const *const libc::c_char)),
    );
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut items: [*const libc::c_char; 4] = [
        (c"pear".as_ptr().cast_mut()).cast_const(),
        (c"apple".as_ptr().cast_mut()).cast_const(),
        (c"fig".as_ptr().cast_mut()).cast_const(),
        (c"date".as_ptr().cast_mut()).cast_const(),
    ];
    libc::qsort(
        (items.as_mut_ptr() as *mut ::libc::c_void),
        4_usize,
        ::std::mem::size_of::<*const libc::c_char>(),
        Some(std::mem::transmute::<
            *const (),
            unsafe extern "C" fn(*const ::libc::c_void, *const ::libc::c_void) -> i32,
        >(cmp_0 as *const ())),
    );
    assert!(
        ((((libc::strcmp(
            items[((0) as usize)],
            (c"apple".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            items[((1) as usize)],
            (c"date".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            items[((2) as usize)],
            (c"fig".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            items[((3) as usize)],
            (c"pear".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    return 0;
}
