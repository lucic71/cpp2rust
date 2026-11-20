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
pub struct Probe {}
impl Probe {
    pub unsafe fn copy_from(this: *mut Probe, _a0: *const Probe) {}
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Wrapper_Probe_ {
    pub base_: Probe,
    pub tag: i32,
}
impl Wrapper_Probe_ {
    pub unsafe fn copy_from(this: *mut Wrapper_Probe_, _a0: *const Wrapper_Probe_) {
        Probe::copy_from((&raw mut (*this).base_), { &(*_a0).base_ });
        (&raw mut (*this).tag).write((*_a0).tag);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Wrapper_Probe_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut a;
        {
            let __d = (&raw mut (*__d).base_);
        };
        (&raw mut (*__d).tag).write(0_i32);
    };
    a.tag = 3;
    let mut b: Wrapper_Probe_ = ::std::mem::MaybeUninit::uninit().assume_init();
    Wrapper_Probe_::copy_from(&raw mut b, { &a });
    assert!(((b.tag) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
