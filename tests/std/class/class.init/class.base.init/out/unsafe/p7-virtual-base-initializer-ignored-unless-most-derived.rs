extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut default_ctors_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut int_ctors_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct VirtualBase {
    pub value: i32,
}
impl VirtualBase {
    pub unsafe fn new_2() -> Self {
        let mut this = Self { value: -1_i32 };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut default_ctors_0)).prefix_inc();
        this
    }
    pub unsafe fn new_1(mut i: i32) -> Self {
        let mut this = Self { value: i };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut int_ctors_1)).prefix_inc();
        this
    }
}
impl Default for VirtualBase {
    fn default() -> Self {
        unsafe { VirtualBase::new_2() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct PassesArgumentToVirtualBase {}
impl PassesArgumentToVirtualBase {
    pub unsafe fn new(mut i: i32) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct HasNoInitializerForVirtualBase {}
impl HasNoInitializerForVirtualBase {
    pub unsafe fn new(mut _a0: i32) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct JoinsBoth {}
impl JoinsBoth {
    pub unsafe fn new(mut i: i32) -> Self {
        let mut this = Self {};
        this
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut base: VirtualBase = VirtualBase::new_1({ 1 });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut default_ctors_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut int_ctors_1)) == (1))
    );
    assert!(((base.value) == (1)));
    let mut passes: PassesArgumentToVirtualBase = PassesArgumentToVirtualBase::new({ 2 });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut default_ctors_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut int_ctors_1)) == (2))
    );
    assert!((((passes as VirtualBase).value) == (2)));
    let mut hasNone: HasNoInitializerForVirtualBase = HasNoInitializerForVirtualBase::new({ 3 });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut default_ctors_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut int_ctors_1)) == (2))
    );
    assert!((((hasNone as VirtualBase).value) == (-1_i32)));
    let mut joined: JoinsBoth = JoinsBoth::new({ 4 });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut default_ctors_0)) == (2))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut int_ctors_1)) == (2))
    );
    assert!((((joined as VirtualBase).value) == (-1_i32)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const default_ctors_0);
    std::cell::LazyCell::force(&*&raw const int_ctors_1);
}
