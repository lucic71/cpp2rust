extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct UserDefined {
    pub a: (),
    pub v: (),
}
impl UserDefined {
    pub unsafe fn move_from(_a0: *mut UserDefined) -> Self {
        let mut this = Self {
            a: (*_a0).a,
            v: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut UserDefined) -> *mut UserDefined {
        self.a = ((*_a0).a).clone();
        unimplemented!();
        return &mut (*(self as *mut UserDefined));
    }
}
impl Default for UserDefined {
    fn default() -> Self {
        unsafe {
            UserDefined {
                a: unimplemented!(),
                v: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct FieldIsLibcType {
    pub addr: (),
}
impl Default for FieldIsLibcType {
    fn default() -> Self {
        unsafe {
            FieldIsLibcType {
                addr: unimplemented!(),
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
unsafe fn main_0() -> () {
    let mut p: () = unimplemented!();
    p.fd = -1_();
    p.events = 0_();
    p.revents = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut ia: () = unimplemented!();
    ia.s_addr = 1_();
    unimplemented!();
    let mut t: () = unimplemented!();
    t.tm_year = 124_();
    t.tm_mon = 5_();
    t.tm_mday = 15_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut st: () = unimplemented!();
    st.st_size = 1024_();
    unimplemented!();
    let mut ud: UserDefined = <UserDefined>::default();
    unimplemented!();
    unimplemented!();
    let mut filt: FieldIsLibcType = <FieldIsLibcType>::default();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
