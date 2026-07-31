extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut names_0: [*const libc::c_char; 4] = unsafe {
    [
        (c"alpha".as_ptr().cast_mut()).cast_const(),
        (c"beta".as_ptr().cast_mut()).cast_const(),
        (c"gamma".as_ptr().cast_mut()).cast_const(),
        std::ptr::null(),
    ]
};
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut count: usize = 0_usize;
    let mut walk: *const *const libc::c_char = std::ptr::null();
    walk = (names_0.as_mut_ptr()).cast_const();
    'loop_: while !(*walk).is_null() {
        count.prefix_inc();
        walk.postfix_inc();
    }
    assert!(((((count) == (3_usize)) as i32) != 0));
    assert!(
        ((((::std::mem::size_of::<[*const libc::c_char; 4]>())
            == ((::std::mem::size_of::<*const libc::c_char>() as usize).wrapping_mul(4_usize)))
            as i32)
            != 0)
    );
    let mut copy: *mut *const libc::c_char = (libcc2rs::malloc_unsafe(
        ((::std::mem::size_of::<*const libc::c_char>() as u64)
            .wrapping_mul((((count).wrapping_add(1_usize)) as u64)) as usize),
    ) as *mut *const libc::c_char);
    assert!((((!((copy).is_null())) as i32) != 0));
    {
        if ((::std::mem::size_of::<*const libc::c_char>() as u64).wrapping_mul((count as u64))
            as usize)
            != 0
        {
            ::std::ptr::copy_nonoverlapping(
                ((names_0.as_mut_ptr() as *const *const libc::c_char) as *const ::libc::c_void),
                (copy as *mut ::libc::c_void),
                ((::std::mem::size_of::<*const libc::c_char>() as u64).wrapping_mul((count as u64))
                    as usize) as usize,
            )
        }
        (copy as *mut ::libc::c_void)
    };
    (*copy.offset(((count) as isize))) = std::ptr::null();
    assert!(
        ((((libc::strcmp(
            (*copy.offset(((0) as isize))),
            (c"alpha".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (*copy.offset(((1) as isize))),
            (c"beta".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (*copy.offset(((2) as isize))),
            (c"gamma".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(((((*copy.offset(((3) as isize))).is_null()) as i32) != 0));
    count = 0_usize;
    walk = (copy).cast_const();
    'loop_: while !(*walk).is_null() {
        count.prefix_inc();
        walk.postfix_inc();
    }
    assert!(((((count) == (3_usize)) as i32) != 0));
    libcc2rs::free_unsafe(((copy as *mut *const libc::c_char) as *mut ::libc::c_void));
    return 0;
}
