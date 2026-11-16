extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe trait DeclaresVirtual {
    unsafe fn id(&mut self) -> i32;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct HoldsId {
    pub held: i32,
}
impl Default for HoldsId {
    fn default() -> Self {
        unsafe { HoldsId { held: 0 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct HasPolymorphicMembers {
    pub before: i32,
    pub first: HoldsId,
    pub second: HoldsId,
}
impl Default for HasPolymorphicMembers {
    fn default() -> Self {
        unsafe {
            HasPolymorphicMembers {
                before: 0,
                first: <HoldsId>::default(),
                second: <HoldsId>::default(),
            }
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut owner: HasPolymorphicMembers = <HasPolymorphicMembers>::default();
    owner.first.held = 4;
    owner.second.held = 5;
    let mut base: *mut dyn DeclaresVirtual = (&mut owner.first as *mut HoldsId);
    assert!(((unsafe { (*base).id() }) == (4)));
    base = (&mut owner.second as *mut HoldsId);
    assert!(((unsafe { (*base).id() }) == (5)));
    owner.second.held = 6;
    assert!(((unsafe { (*base).id() }) == (6)));
    return 0;
}
unsafe impl DeclaresVirtual for HoldsId {
    unsafe fn id(&mut self) -> i32 {
        return self.held;
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
