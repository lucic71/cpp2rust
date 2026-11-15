extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut defaultCtors_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut intCtors_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct CountsConstructors {
    pub value: i32,
}
impl CountsConstructors {
    pub unsafe fn new_2() -> Self {
        let mut this = Self { value: -1_i32 };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut defaultCtors_0)).prefix_inc();
        this
    }
    pub unsafe fn new_1(mut v: i32) -> Self {
        let mut this = Self { value: v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut intCtors_1)).prefix_inc();
        this
    }
}
impl Default for CountsConstructors {
    fn default() -> Self {
        unsafe { CountsConstructors::new_2() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct OtherBase {
    pub other: i32,
}
impl OtherBase {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { other: v };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct HasNoCtorInitializer {}
impl HasNoCtorInitializer {
    pub unsafe fn new_2() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_1(mut _a0: i32) -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for HasNoCtorInitializer {
    fn default() -> Self {
        unsafe { HasNoCtorInitializer::new_2() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct InitializesOnlyOtherBase {}
impl InitializesOnlyOtherBase {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for InitializesOnlyOtherBase {
    fn default() -> Self {
        unsafe { InitializesOnlyOtherBase::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasImplicitConstructor {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct InitializesBase {}
impl InitializesBase {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for InitializesBase {
    fn default() -> Self {
        unsafe { InitializesBase::new() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: HasNoCtorInitializer = HasNoCtorInitializer::new_2();
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut defaultCtors_0)) == (1)));
    assert!((((a as CountsConstructors).value) == (-1_i32)));
    let mut b: HasNoCtorInitializer = HasNoCtorInitializer::new_1({ 1 });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut defaultCtors_0)) == (2)));
    assert!((((b as CountsConstructors).value) == (-1_i32)));
    let mut c: InitializesOnlyOtherBase = InitializesOnlyOtherBase::new();
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut defaultCtors_0)) == (3)));
    assert!((((c as CountsConstructors).value) == (-1_i32)) && (((c as OtherBase).other) == (5)));
    let mut d: HasImplicitConstructor = <HasImplicitConstructor>::default();
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut defaultCtors_0)) == (4)));
    assert!((((d as CountsConstructors).value) == (-1_i32)));
    let mut e: InitializesBase = InitializesBase::new();
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut defaultCtors_0)) == (4)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut intCtors_1)) == (1)));
    assert!((((e as CountsConstructors).value) == (7)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const defaultCtors_0);
    std::cell::LazyCell::force(&*&raw const intCtors_1);
}
