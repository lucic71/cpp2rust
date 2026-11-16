extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe trait DeclaresVirtual {
    unsafe fn setValue(&mut self, mut v: i32) {
        self.value = v;
    }
    unsafe fn f(&mut self);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct CallsNonVirtualMembers {
    pub own: i32,
}
impl CallsNonVirtualMembers {
    pub unsafe fn setOwn(&mut self, mut v: i32) {
        self.own = v;
    }
}
impl Default for CallsNonVirtualMembers {
    fn default() -> Self {
        unsafe { CallsNonVirtualMembers { own: 0 } }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: CallsNonVirtualMembers = <CallsNonVirtualMembers>::default();
    let mut base: *mut dyn DeclaresVirtual = (&mut d as *mut CallsNonVirtualMembers);
    assert!((((d as DeclaresVirtual).value) == (0)) && ((d.own) == (0)));
    (unsafe { (*base).f() });
    assert!((((d as DeclaresVirtual).value) == (1)) && ((d.own) == (2)));
    return 0;
}
unsafe impl DeclaresVirtual for CallsNonVirtualMembers {
    unsafe fn f(&mut self) {
        (unsafe { DeclaresVirtual::setValue(self, 1) });
        (unsafe { CallsNonVirtualMembers::setOwn(self, 2) });
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
