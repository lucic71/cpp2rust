extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Holder {
    pub v: (),
}
impl Holder {
    pub unsafe fn move_from(_a0: *mut Holder) -> Self {
        let mut this = Self {
            v: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Holder) -> *mut Holder {
        unimplemented!();
        return &mut (*(self as *mut Holder));
    }
}
pub unsafe fn write_through_0(mut p: *mut ()) {
    (*p) = 42_();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut v: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: *mut () = (&mut v as *mut ());
    let mut a: () = (*unimplemented!());
    (*unimplemented!()) = 30_();
    let mut h: Holder = <Holder>::default();
    unimplemented!();
    unimplemented!();
    let mut hp: *mut Holder = (&mut h as *mut Holder);
    let mut b: () = (*unimplemented!());
    (*unimplemented!()) = 60_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe { write_through_0((unimplemented!())) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
