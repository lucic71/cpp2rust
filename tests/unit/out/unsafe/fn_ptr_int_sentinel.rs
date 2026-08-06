extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut freed_0: i32 = unsafe { 0 };
pub unsafe fn real_free_1(mut p: *mut ::libc::c_void) {
    &(p);
    freed_0.postfix_inc();
}
pub unsafe fn consume_2(
    mut data: *mut ::libc::c_void,
    mut d: Option<unsafe fn(*mut ::libc::c_void)>,
) -> i32 {
    if ((((d).is_none()) as i32) != 0) {
        return 1;
    }
    if ((((d)
        == (std::mem::transmute::<usize, Option<unsafe fn(*mut ::libc::c_void)>>(
            (-1_i32 as usize),
        ))) as i32)
        != 0)
    {
        return 2;
    }
    (unsafe { (d).unwrap()(data) });
    return 3;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 7;
    assert!(
        ((((unsafe {
            consume_2(
                (((&raw mut x as *mut i32) as *mut i32) as *mut ::libc::c_void),
                (std::mem::transmute::<usize, Option<unsafe fn(*mut ::libc::c_void)>>(
                    (0 as usize),
                )),
            )
        }) == (1)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            consume_2(
                (((&raw mut x as *mut i32) as *mut i32) as *mut ::libc::c_void),
                (std::mem::transmute::<usize, Option<unsafe fn(*mut ::libc::c_void)>>(
                    (-1_i32 as usize),
                )),
            )
        }) == (2)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            consume_2(
                (((&raw mut x as *mut i32) as *mut i32) as *mut ::libc::c_void),
                Some(real_free_1),
            )
        }) == (3)) as i32)
            != 0)
    );
    assert!(((((freed_0) == (1)) as i32) != 0));
    return 0;
}
