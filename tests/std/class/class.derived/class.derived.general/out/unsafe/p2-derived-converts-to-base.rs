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
impl Default for DerivesDirectly {
    fn default() -> Self {
        unsafe { DerivesDirectly { own: 0 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DerivesIndirectly {}
pub unsafe fn incrementThroughPointer_0(mut p: *mut HasCounter) {
    (unsafe { HasCounter::increment(&mut (*p)) });
}
pub unsafe fn incrementThroughReference_1(r: *mut HasCounter) {
    (unsafe { HasCounter::increment(&mut (*r)) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: DerivesDirectly = <DerivesDirectly>::default();
    (unsafe { incrementThroughPointer_0((&mut d as *mut DerivesDirectly)) });
    assert!((((d as HasCounter).count) == (1)));
    (unsafe { incrementThroughReference_1(&mut d) });
    assert!((((d as HasCounter).count) == (2)));
    assert!(((d.own) == (0)));
    let mut p: *mut HasCounter = (&mut d as *mut DerivesDirectly);
    let r: *mut HasCounter = &mut d;
    assert!(((p) == (r)));
    (unsafe { HasCounter::increment(&mut (*p)) });
    assert!((((d as HasCounter).count) == (3)));
    (unsafe { HasCounter::increment(&mut (*r)) });
    assert!((((d as HasCounter).count) == (4)));
    (unsafe { HasCounter::increment(&mut (d as HasCounter)) });
    assert!((((*p).count) == (5)));
    assert!((((*r).count) == (5)));
    let mut i: DerivesIndirectly = <DerivesIndirectly>::default();
    (unsafe { incrementThroughPointer_0((&mut i as *mut DerivesIndirectly)) });
    assert!((((i as HasCounter).count) == (1)));
    (unsafe { incrementThroughReference_1(&mut i) });
    assert!((((i as HasCounter).count) == (2)));
    let mut asDirectBase: *mut DerivesDirectly = (&mut i as *mut DerivesIndirectly);
    (unsafe { HasCounter::increment(&mut (*(asDirectBase as *mut HasCounter))) });
    assert!((((i as HasCounter).count) == (3)));
    assert!((((d as HasCounter).count) == (5)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
