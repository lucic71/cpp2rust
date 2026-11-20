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
pub struct S_2_ {
    pub v: i32,
    pub w: i32,
}
impl S_2_ {
    pub unsafe fn get(&self) -> i32 {
        return ((self.v) - (self.w));
    }
    pub unsafe fn new_1(this: *mut S_2_) {}
    pub unsafe fn copy_from(this: *mut S_2_, _a0: *const S_2_) {
        (&raw mut (*this).v).write((*_a0).v);
        (&raw mut (*this).w).write((*_a0).w);
    }
    pub unsafe fn move_from(this: *mut S_2_, _a0: *mut S_2_) {
        (&raw mut (*this).v).write((*_a0).v);
        (&raw mut (*this).w).write((*_a0).w);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_1_ {
    pub v: i32,
}
impl S_1_ {
    pub unsafe fn get(&self) -> i32 {
        return ((self.v) * (1));
    }
    pub unsafe fn new_1(this: *mut S_1_) {}
    pub unsafe fn copy_from(this: *mut S_1_, _a0: *const S_1_) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut S_1_, _a0: *mut S_1_) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_3_ {
    pub v: i32,
}
impl S_3_ {
    pub unsafe fn get(&self) -> i32 {
        return ((self.v) * (3));
    }
    pub unsafe fn new_1(this: *mut S_3_) {}
    pub unsafe fn copy_from(this: *mut S_3_, _a0: *const S_3_) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut S_3_, _a0: *mut S_3_) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
pub unsafe fn scale_0(mut x: i32) -> i32 {
    return ((x) * (1));
}
pub unsafe fn scale_1(mut x: i32) -> i32 {
    return ((x) * (3));
}
pub unsafe fn scale_2(mut x: i32) -> i32 {
    return ((x) - (2));
}
pub unsafe fn get_float_3() -> f32 {
    return f32::NAN;
}
pub unsafe fn get_float_4() -> f32 {
    return f32::NEG_INFINITY;
}
pub unsafe fn get_double_5() -> f64 {
    return f64::NAN;
}
pub unsafe fn get_double_6() -> f64 {
    return f64::NEG_INFINITY;
}
pub unsafe fn get_long_double_7() -> f64 {
    return f64::NAN;
}
pub unsafe fn get_long_double_8() -> f64 {
    return f64::NEG_INFINITY;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s1: S_1_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut s1;
        (&raw mut (*__d).v).write(5);
    };
    let mut s2: S_2_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut s2;
        (&raw mut (*__d).v).write(5);
        (&raw mut (*__d).w).write(1);
    };
    let mut s3: S_3_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut s3;
        (&raw mut (*__d).v).write(5);
    };
    assert!(((unsafe { S_1_::get(&s1,) }) == (5)));
    assert!(((unsafe { S_2_::get(&s2,) }) == (4)));
    assert!(((unsafe { S_3_::get(&s3,) }) == (15)));
    assert!(((unsafe { scale_0(7,) }) == (7)));
    assert!(((unsafe { scale_2(7,) }) == (5)));
    assert!(((unsafe { scale_1(7,) }) == (21)));
    let mut f_nan: f32 = (unsafe { get_float_3() });
    let mut f_inf: f32 = (unsafe { get_float_4() });
    assert!(((f_nan) != (f_nan)));
    assert!(((f_inf) < (0_f32)));
    let mut d_nan: f64 = (unsafe { get_double_5() });
    let mut d_inf: f64 = (unsafe { get_double_6() });
    assert!(((d_nan) != (d_nan)));
    assert!(((d_inf) < (0_f64)));
    let mut ld_nan: f64 = (unsafe { get_long_double_7() });
    let mut ld_inf: f64 = (unsafe { get_long_double_8() });
    assert!(((ld_nan) != (ld_nan)));
    assert!(((ld_inf) < (0_f64)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
