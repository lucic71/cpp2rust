extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct CountsFirst {
    pub first: i32,
}
impl CountsFirst {
    pub unsafe fn touchFirst(&mut self) {
        self.first.prefix_inc();
    }
}
impl Default for CountsFirst {
    fn default() -> Self {
        unsafe { CountsFirst { first: 0 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct CountsSecond {
    pub second: i32,
}
impl CountsSecond {
    pub unsafe fn touchSecond(&mut self) {
        self.second.prefix_inc();
    }
}
impl Default for CountsSecond {
    fn default() -> Self {
        unsafe { CountsSecond { second: 0 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct CountsThird {
    pub third: i32,
}
impl CountsThird {
    pub unsafe fn touchThird(&mut self) {
        self.third.prefix_inc();
    }
}
impl Default for CountsThird {
    fn default() -> Self {
        unsafe { CountsThird { third: 0 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasThreeBases {}
impl HasThreeBases {
    pub unsafe fn touchAll(&mut self) {
        (unsafe { CountsFirst::touchFirst(self) });
        (unsafe { CountsSecond::touchSecond(self) });
        (unsafe { CountsThird::touchThird(self) });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: HasThreeBases = <HasThreeBases>::default();
    (unsafe { HasThreeBases::touchAll(&mut d) });
    assert!(
        ((((d as CountsFirst).first) == (1)) && (((d as CountsSecond).second) == (1)))
            && (((d as CountsThird).third) == (1))
    );
    (unsafe { CountsSecond::touchSecond(&mut (d as CountsSecond)) });
    assert!(
        ((((d as CountsFirst).first) == (1)) && (((d as CountsSecond).second) == (2)))
            && (((d as CountsThird).third) == (1))
    );
    let mut first: *mut CountsFirst = (&mut d as *mut HasThreeBases);
    (unsafe { CountsFirst::touchFirst(&mut (*first)) });
    assert!(
        ((((d as CountsFirst).first) == (2)) && (((d as CountsSecond).second) == (2)))
            && (((d as CountsThird).third) == (1))
    );
    let mut second: *mut CountsSecond = (&mut d as *mut HasThreeBases);
    (unsafe { CountsSecond::touchSecond(&mut (*second)) });
    assert!(
        ((((d as CountsFirst).first) == (2)) && (((d as CountsSecond).second) == (3)))
            && (((d as CountsThird).third) == (1))
    );
    let mut third: *mut CountsThird = (&mut d as *mut HasThreeBases);
    (unsafe { CountsThird::touchThird(&mut (*third)) });
    assert!(
        ((((d as CountsFirst).first) == (2)) && (((d as CountsSecond).second) == (3)))
            && (((d as CountsThird).third) == (2))
    );
    assert!((((*first).first) == (2)));
    assert!((((*second).second) == (3)));
    assert!((((*third).third) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
