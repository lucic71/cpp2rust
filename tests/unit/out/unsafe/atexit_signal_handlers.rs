extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut got_0: i32 = unsafe { 0 };
pub unsafe fn on_signal_1(mut sig: i32) {
    got_0 = sig;
}
pub unsafe fn first_exit_2() {
    printf((c"first\n".as_ptr().cast_mut()).cast_const() as *const i8);
}
pub unsafe fn second_exit_3() {
    printf((c"second\n".as_ptr().cast_mut()).cast_const() as *const i8);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        (((({
            let __handler = match Some(on_signal_1) {
                None => 0_usize,
                Some(__f) => __f as usize,
            };
            match libc::signal(10, __handler) {
                0 => None,
                __prev => Some(std::mem::transmute::<usize, unsafe fn(i32)>(__prev)),
            }
        })
        .is_none()) as i32)
            != 0)
    );
    assert!(((((libc::raise(10)) == (0)) as i32) != 0));
    assert!(((((got_0) == (10)) as i32) != 0));
    got_0 = 0;
    let mut prev: Option<unsafe fn(i32)> = {
        let __handler = match (std::mem::transmute::<usize, Option<unsafe fn(i32)>>((1 as usize))) {
            None => 0_usize,
            Some(__f) => __f as usize,
        };
        match libc::signal(10, __handler) {
            0 => None,
            __prev => Some(std::mem::transmute::<usize, unsafe fn(i32)>(__prev)),
        }
    };
    assert!(((((prev) == (Some(on_signal_1))) as i32) != 0));
    assert!(((((libc::raise(10)) == (0)) as i32) != 0));
    assert!(((((got_0) == (0)) as i32) != 0));
    assert!(
        ((((libc::atexit(std::mem::transmute::<*const (), extern "C" fn()>(
            Some(first_exit_2).expect("atexit: null handler") as *const (),
        ))) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::atexit(std::mem::transmute::<*const (), extern "C" fn()>(
            Some(second_exit_3).expect("atexit: null handler") as *const (),
        ))) == (0)) as i32)
            != 0)
    );
    printf((c"main\n".as_ptr().cast_mut()).cast_const() as *const i8);
    return 0;
}
