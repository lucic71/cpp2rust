extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe trait DeclaresVirtual {
    unsafe fn add(&mut self, amount: i32);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct AddsTwice {
    pub calls: i32,
}
impl Default for AddsTwice {
    fn default() -> Self {
        unsafe { AddsTwice { calls: 0 } }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: AddsTwice = <AddsTwice>::default();
    let mut base: *mut dyn DeclaresVirtual = (&mut d as *mut AddsTwice);
    let mut derived: *mut AddsTwice = (&mut d as *mut AddsTwice);
    (unsafe { (*base).add(3) });
    assert!((((d as DeclaresVirtual).total) == (6)) && ((d.calls) == (1)));
    assert!((((*(derived as *mut dyn DeclaresVirtual)).total) == (6)));
    (unsafe { (*base).add(1) });
    assert!((((*base).total) == (8)));
    assert!((((*derived).calls) == (2)));
    return 0;
}
unsafe impl DeclaresVirtual for AddsTwice {
    unsafe fn add(&mut self, amount: i32) {
        self.total += ((2) * (amount));
        self.calls.prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
