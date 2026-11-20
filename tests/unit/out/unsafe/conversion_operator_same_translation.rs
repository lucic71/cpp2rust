extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(libc::c_char, to_libcc_char)]
#[cpp_into(u8, to_u8)]
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn to_libcc_char(&self) -> libc::c_char {
        return (self.v as libc::c_char);
    }
    pub unsafe fn to_u8(&self) -> u8 {
        return (((self.v) + (1)) as u8);
    }
    pub unsafe fn new_1(this: *mut S) {}
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut S, _a0: *mut S) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(i64, to_i64_1)]
pub struct T {
    pub v: i32,
}
impl T {
    pub unsafe fn to_i64_1(&self) -> i64 {
        return (self.v as i64);
    }
    pub unsafe fn to_i64_2(&self) -> i64 {
        return (((self.v) + (1)) as i64);
    }
    pub unsafe fn new_1(this: *mut T) {}
    pub unsafe fn copy_from(this: *mut T, _a0: *const T) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut T, _a0: *mut T) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut s;
        (&raw mut (*__d).v).write(65);
    };
    let mut a: libc::c_char = (unsafe { S::to_libcc_char(&s) });
    let mut b: u8 = (unsafe { S::to_u8(&s) });
    assert!(((a as i32) == (('A' as libc::c_char) as i32)));
    assert!(((b as i32) == ((66 as u8) as i32)));
    assert!(
        ((((unsafe { S::to_libcc_char(&s,) }) as i32) + ((unsafe { S::to_u8(&s,) }) as i32))
            == (131))
    );
    let mut t: T = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut t;
        (&raw mut (*__d).v).write(3);
    };
    let mut c: i64 = (unsafe { T::to_i64_1(&t) });
    let mut d: i64 = (unsafe { T::to_i64_2(&t) });
    assert!(((c) == (3_i64)));
    assert!(((d) == (4_i64)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
