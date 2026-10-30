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
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr1: [i32; 5] = [5, 2, 8, 1, 3];
    {
        let __first = arr1.as_mut_ptr();
        let __n = arr1
            .as_mut_ptr()
            .offset((5) as isize)
            .iter_distance(&__first);
        let __cmp = FnPtr::<fn(i32, i32) -> bool>::new(|mut x: i32, mut y: i32| -> bool {
            unsafe {
                return ((x) < (y));
            }
        });
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            let __a = __first.iter_at(__x) as *const _;
            let __b = __first.iter_at(__y) as *const _;
            if __cmp.call(__a, __b) {
                std::cmp::Ordering::Less
            } else if __cmp.call(__b, __a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__first.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__first.iter_at(__i), __v);
        }
    };
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
