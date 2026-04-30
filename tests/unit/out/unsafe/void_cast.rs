extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn unused_param_0(mut x: i32) {
    let _ = x.clone();
}
pub static mut side_effect_counter: i32 = 0;
pub unsafe fn bump_and_return_1() -> i32 {
    side_effect_counter.prefix_inc();
    return side_effect_counter;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Holder {
    pub field: i32,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe {
        let _x: i32 = 42;
        unused_param_0(_x)
    });
    let mut y: i32 = 5;
    let _ = y.clone();
    let mut z: i32 = {
        let _ = y.clone();
        7
    };
    assert!(((z) == (7)));
    let mut counter: i32 = 0;
    let mut w: i32 = {
        let _ = counter.clone();
        counter = 3;
        counter
    };
    assert!(((w) == (3)));
    assert!(((counter) == (3)));
    let _ = (unsafe { bump_and_return_1() });
    assert!(((side_effect_counter) == (1)));
    let mut v: i32 = {
        let _ = (unsafe { bump_and_return_1() });
        99
    };
    assert!(((side_effect_counter) == (2)));
    assert!(((v) == (99)));
    let _ = 0;
    let _ = (0);
    let _ = (y).clone();
    {
        let _ = 0;
    };
    {
        let _ = (y).clone();
    };
    let mut err: i32 = 0;
    {
        let _ = (err = 42).clone();
    };
    assert!(((err) == (42)));
    let mut chosen: i32 = {
        let _ = (err = 7).clone();
        123
    };
    assert!(((err) == (7)));
    assert!(((chosen) == (123)));
    let _ = bump_and_return_1.clone();
    assert!(((side_effect_counter) == (2)));
    let _ = (Some(bump_and_return_1));
    assert!(((side_effect_counter) == (2)));
    let _ = (std::mem::transmute::<Option<unsafe fn() -> i32>, Option<unsafe fn() -> i32>>(
        (Some(bump_and_return_1)),
    ));
    assert!(((side_effect_counter) == (2)));
    let mut storage: i32 = 11;
    let mut p: *mut i32 = (&mut storage as *mut i32);
    let _ = (*p).clone();
    let mut arr: [i32; 3] = [1, 2, 3];
    let _ = (arr[(1) as usize]).clone();
    let mut h: Holder = Holder { field: 17 };
    let _ = (h.field).clone();
    let mut hp: *mut Holder = (&mut h as *mut Holder);
    let _ = ((*hp).field).clone();
    return 0;
}
