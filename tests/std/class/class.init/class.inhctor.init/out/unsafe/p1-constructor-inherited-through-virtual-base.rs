extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut int_ctors_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct VirtualBase {
    pub value: i32,
}
impl VirtualBase {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut int_ctors_0)).prefix_inc();
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DeletesDefaultConstructor {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct InheritsOnce {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct InheritsThroughTwoLevels {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut twoLevels: InheritsThroughTwoLevels = InheritsThroughTwoLevels::new_1({ 0 });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut int_ctors_0)) == (1)));
    assert!((((twoLevels as VirtualBase).value) == (0)));
    let mut once: InheritsOnce = InheritsOnce::new_1({ 4 });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut int_ctors_0)) == (2)));
    assert!((((once as VirtualBase).value) == (4)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const int_ctors_0);
}
