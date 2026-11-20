extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Level0_Level1_1_Level2_1_Level3_1 {
    pub x1: i32,
}
impl Level0_Level1_1_Level2_1_Level3_1 {
    pub unsafe fn new_1(this: *mut Level0_Level1_1_Level2_1_Level3_1) {}
    pub unsafe fn copy_from(
        this: *mut Level0_Level1_1_Level2_1_Level3_1,
        _a0: *const Level0_Level1_1_Level2_1_Level3_1,
    ) {
        (&raw mut (*this).x1).write((*_a0).x1);
    }
    pub unsafe fn move_from(
        this: *mut Level0_Level1_1_Level2_1_Level3_1,
        _a0: *mut Level0_Level1_1_Level2_1_Level3_1,
    ) {
        (&raw mut (*this).x1).write((*_a0).x1);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Level0_Level1_1_Level2_1_Level3_2 {
    pub x1: i32,
    pub x2: i32,
}
impl Level0_Level1_1_Level2_1_Level3_2 {
    pub unsafe fn new_1(this: *mut Level0_Level1_1_Level2_1_Level3_2) {}
    pub unsafe fn copy_from(
        this: *mut Level0_Level1_1_Level2_1_Level3_2,
        _a0: *const Level0_Level1_1_Level2_1_Level3_2,
    ) {
        (&raw mut (*this).x1).write((*_a0).x1);
        (&raw mut (*this).x2).write((*_a0).x2);
    }
    pub unsafe fn move_from(
        this: *mut Level0_Level1_1_Level2_1_Level3_2,
        _a0: *mut Level0_Level1_1_Level2_1_Level3_2,
    ) {
        (&raw mut (*this).x1).write((*_a0).x1);
        (&raw mut (*this).x2).write((*_a0).x2);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Level0_Level1_1_Level2_1 {
    pub x1: i32,
}
impl Level0_Level1_1_Level2_1 {
    pub unsafe fn new_1(this: *mut Level0_Level1_1_Level2_1) {}
    pub unsafe fn copy_from(
        this: *mut Level0_Level1_1_Level2_1,
        _a0: *const Level0_Level1_1_Level2_1,
    ) {
        (&raw mut (*this).x1).write((*_a0).x1);
    }
    pub unsafe fn move_from(
        this: *mut Level0_Level1_1_Level2_1,
        _a0: *mut Level0_Level1_1_Level2_1,
    ) {
        (&raw mut (*this).x1).write((*_a0).x1);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Level0_Level1_1 {
    pub x1: i32,
}
impl Level0_Level1_1 {
    pub unsafe fn new_1(this: *mut Level0_Level1_1) {}
    pub unsafe fn copy_from(this: *mut Level0_Level1_1, _a0: *const Level0_Level1_1) {
        (&raw mut (*this).x1).write((*_a0).x1);
    }
    pub unsafe fn move_from(this: *mut Level0_Level1_1, _a0: *mut Level0_Level1_1) {
        (&raw mut (*this).x1).write((*_a0).x1);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Level0_Level1_2 {
    pub x1: i32,
    pub x2: i32,
}
impl Level0_Level1_2 {
    pub unsafe fn new_1(this: *mut Level0_Level1_2) {}
    pub unsafe fn copy_from(this: *mut Level0_Level1_2, _a0: *const Level0_Level1_2) {
        (&raw mut (*this).x1).write((*_a0).x1);
        (&raw mut (*this).x2).write((*_a0).x2);
    }
    pub unsafe fn move_from(this: *mut Level0_Level1_2, _a0: *mut Level0_Level1_2) {
        (&raw mut (*this).x1).write((*_a0).x1);
        (&raw mut (*this).x2).write((*_a0).x2);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Level0 {}
impl Level0 {
    pub unsafe fn new_1(this: *mut Level0) {}
    pub unsafe fn copy_from(this: *mut Level0, _a0: *const Level0) {}
    pub unsafe fn move_from(this: *mut Level0, _a0: *mut Level0) {}
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x1: Level0_Level1_1 = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut x1;
        (&raw mut (*__d).x1).write(0);
    };
    let mut x2: Level0_Level1_2 = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut x2;
        (&raw mut (*__d).x1).write(1);
        (&raw mut (*__d).x2).write(2);
    };
    let mut x3: Level0_Level1_1_Level2_1 = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut x3;
        (&raw mut (*__d).x1).write(3);
    };
    let mut x4: Level0_Level1_1_Level2_1_Level3_1 = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut x4;
        (&raw mut (*__d).x1).write(4);
    };
    let mut x5: Level0_Level1_1_Level2_1_Level3_2 = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut x5;
        (&raw mut (*__d).x1).write(5);
        (&raw mut (*__d).x2).write(6);
    };
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
