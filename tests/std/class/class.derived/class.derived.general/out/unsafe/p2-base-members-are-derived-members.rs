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
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct DerivesDirectly {
    pub own: i32,
}
impl DerivesDirectly {
    pub unsafe fn incrementBoth(&mut self) {
        (unsafe { HasCounter::increment(self) });
        self.own.prefix_inc();
    }
}
impl Default for DerivesDirectly {
    fn default() -> Self {
        unsafe { DerivesDirectly { own: 0 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DerivesIndirectly {}
impl DerivesIndirectly {
    pub unsafe fn incrementTwice(&mut self) {
        (unsafe { HasCounter::increment(self) });
        (unsafe { DerivesDirectly::incrementBoth(self) });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: DerivesDirectly = <DerivesDirectly>::default();
    (unsafe { HasCounter::increment(&mut (d as HasCounter)) });
    assert!((((d as HasCounter).count) == (1)) && ((d.own) == (0)));
    (unsafe { DerivesDirectly::incrementBoth(&mut d) });
    assert!((((d as HasCounter).count) == (2)) && ((d.own) == (1)));
    (d as HasCounter).count += 5;
    assert!((((d as HasCounter).count) == (7)));
    (unsafe { HasCounter::increment(&mut (d as HasCounter)) });
    assert!((((d as HasCounter).count) == (8)));
    let mut i: DerivesIndirectly = <DerivesIndirectly>::default();
    (unsafe { DerivesIndirectly::incrementTwice(&mut i) });
    assert!((((i as HasCounter).count) == (2)) && (((i as DerivesDirectly).own) == (1)));
    (unsafe { HasCounter::increment(&mut (i as HasCounter)) });
    assert!((((i as HasCounter).count) == (3)) && (((i as DerivesDirectly).own) == (1)));
    assert!((((d as HasCounter).count) == (8)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
