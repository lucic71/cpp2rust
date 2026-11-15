extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut sequence_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct StampsConstruction {
    pub stamp: i32,
}
impl StampsConstruction {
    pub unsafe fn new_2() -> Self {
        let mut this = Self {
            stamp: (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc(),
        };
        this
    }
    pub unsafe fn new_1(mut _a0: i32) -> Self {
        let mut this = Self {
            stamp: (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc(),
        };
        this
    }
}
impl Default for StampsConstruction {
    fn default() -> Self {
        unsafe { StampsConstruction::new_2() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FirstBase {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SecondBase {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct InitializesMembersOutOfOrder {
    pub firstMember: StampsConstruction,
    pub secondMember: StampsConstruction,
    pub thirdMember: StampsConstruction,
    pub bodyStamp: i32,
}
impl InitializesMembersOutOfOrder {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            firstMember: StampsConstruction::new_1({ 0 }),
            secondMember: StampsConstruction::new_1({ 0 }),
            thirdMember: StampsConstruction::new_1({ 0 }),
            bodyStamp: 0_i32,
        };
        this.bodyStamp = (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc();
        this
    }
}
impl Default for InitializesMembersOutOfOrder {
    fn default() -> Self {
        unsafe { InitializesMembersOutOfOrder::new() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: InitializesMembersOutOfOrder = InitializesMembersOutOfOrder::new();
    assert!(((((d as FirstBase) as StampsConstruction).stamp) == (1)));
    assert!(((((d as SecondBase) as StampsConstruction).stamp) == (2)));
    assert!(((d.firstMember.stamp) == (3)));
    assert!(((d.secondMember.stamp) == (4)));
    assert!(((d.thirdMember.stamp) == (5)));
    assert!(((d.bodyStamp) == (6)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)) == (6)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const sequence_0);
}
