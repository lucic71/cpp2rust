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
pub struct HasMember {
    pub value: i32,
}
impl Default for HasMember {
    fn default() -> Self {
        unsafe { HasMember { value: 7 } }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct HasNonTrivialDestructor {}
impl HasNonTrivialDestructor {
    pub unsafe fn destructor(&mut self) {
        self.value = 0;
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: *mut HasNonTrivialDestructor =
        (Box::leak(Box::new(<HasNonTrivialDestructor>::default())) as *mut HasNonTrivialDestructor);
    (unsafe { HasNonTrivialDestructor::destructor(&mut (*p)) });
    return (*(p as *mut HasMember)).value;
}
pub unsafe fn __cpp2rust_init_globals() {}
