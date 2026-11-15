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
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct StampsConstruction {
    pub stamp: i32,
}
impl StampsConstruction {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            stamp: (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc(),
        };
        this
    }
}
impl Default for StampsConstruction {
    fn default() -> Self {
        unsafe { StampsConstruction::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct VirtualOfLeft {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct VirtualOfRight {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DirectVirtual {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NonVirtualFirst {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Left {
    pub leftStamp: i32,
}
impl Left {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            leftStamp: (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc(),
        };
        this
    }
}
impl Default for Left {
    fn default() -> Self {
        unsafe { Left::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Right {
    pub rightStamp: i32,
}
impl Right {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            rightStamp: (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc(),
        };
        this
    }
}
impl Default for Right {
    fn default() -> Self {
        unsafe { Right::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct MostDerived {
    pub ownStamp: i32,
}
impl MostDerived {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            ownStamp: (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc(),
        };
        this
    }
}
impl Default for MostDerived {
    fn default() -> Self {
        unsafe { MostDerived::new() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: MostDerived = MostDerived::new();
    assert!(((((d as VirtualOfLeft) as StampsConstruction).stamp) == (1)));
    assert!(((((d as VirtualOfRight) as StampsConstruction).stamp) == (2)));
    assert!(((((d as DirectVirtual) as StampsConstruction).stamp) == (3)));
    assert!(((((d as NonVirtualFirst) as StampsConstruction).stamp) == (4)));
    assert!((((d as Left).leftStamp) == (5)));
    assert!((((d as Right).rightStamp) == (6)));
    assert!(((d.ownStamp) == (7)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)) == (7)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)) = 0;
    let mut r: Right = Right::new();
    assert!(((((r as VirtualOfRight) as StampsConstruction).stamp) == (1)));
    assert!(((((r as VirtualOfLeft) as StampsConstruction).stamp) == (2)));
    assert!(((r.rightStamp) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const sequence_0);
}
