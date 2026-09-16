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
    let mut arr: [i32; 5] = [5, 2, 8, 1, 3];
    libc::qsort(
        (arr.as_mut_ptr() as *mut i32 as *mut ::libc::c_void),
        5_usize,
        ::std::mem::size_of::<i32>(),
        Some(std::mem::transmute::<
            *const (),
            unsafe extern "C" fn(*const ::libc::c_void, *const ::libc::c_void) -> i32,
        >(
            ((|a: *const ::libc::c_void, b: *const ::libc::c_void| {
                return ((*(b as *const i32)) - (*(a as *const i32)));
            }) as unsafe fn(*const ::libc::c_void, *const ::libc::c_void) -> i32)
                as *const (),
        )),
    );
    assert!(((arr[(0) as usize]) == (8)));
    return 0;
}
