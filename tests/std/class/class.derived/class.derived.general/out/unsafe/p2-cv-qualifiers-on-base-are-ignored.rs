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
pub struct HasCounter {
    pub count: i32,
}
impl HasCounter {
    pub unsafe fn increment(&mut self) {
        self.count.prefix_inc();
    }
}
impl Default for HasCounter {
    fn default() -> Self {
        unsafe { HasCounter { count: 0 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DerivesFromConstTypedef {}
impl DerivesFromConstTypedef {
    pub unsafe fn incrementTwice(&mut self) {
        (unsafe { HasCounter::increment(self) });
        (unsafe { HasCounter::increment(self) });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: DerivesFromConstTypedef = <DerivesFromConstTypedef>::default();
    (unsafe { HasCounter::increment(&mut (d as HasCounter)) });
    assert!((((d as HasCounter).count) == (1)));
    (unsafe { DerivesFromConstTypedef::incrementTwice(&mut d) });
    assert!((((d as HasCounter).count) == (3)));
    let mut p: *mut HasCounter = (&mut d as *mut DerivesFromConstTypedef);
    (unsafe { HasCounter::increment(&mut (*p)) });
    assert!((((d as HasCounter).count) == (4)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
