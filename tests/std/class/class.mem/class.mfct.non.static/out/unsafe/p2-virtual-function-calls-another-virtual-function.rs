extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe trait DeclaresVirtuals {
    unsafe fn first(&mut self);
    unsafe fn second(&mut self);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FirstCallsSecond {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OverridesSecond {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: FirstCallsSecond = <FirstCallsSecond>::default();
    let mut base: *mut dyn DeclaresVirtuals = (&mut d as *mut FirstCallsSecond);
    (unsafe { (*base).first() });
    assert!((((d as DeclaresVirtuals).steps) == (11)));
    let mut e: OverridesSecond = <OverridesSecond>::default();
    base = (&mut e as *mut OverridesSecond);
    (unsafe { (*base).first() });
    assert!((((e as DeclaresVirtuals).steps) == (101)));
    assert!((((d as DeclaresVirtuals).steps) == (11)));
    return 0;
}
unsafe impl DeclaresVirtuals for FirstCallsSecond {
    unsafe fn first(&mut self) {
        self.steps.prefix_inc();
        (unsafe { self.second() });
    }
    unsafe fn second(&mut self) {
        self.steps += 10;
    }
}
unsafe impl FirstCallsSecond for OverridesSecond {
    unsafe fn second(&mut self) {
        self.steps += 100;
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
