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
    pub a: [i32; 1],
    pub v: Vec<i32>,
}
impl UserDefined {
    pub unsafe fn copy_from(this: *mut UserDefined, _a0: *const UserDefined) {
        (&raw mut (*this).a).write((*_a0).a.clone());
        (&raw mut (*this).v).write((*_a0).v.clone());
    }
    pub unsafe fn move_from(this: *mut UserDefined, _a0: *mut UserDefined) {
        (&raw mut (*this).a).write((*_a0).a.clone());
        (&raw mut (*this).v).write(std::mem::take(&mut (*_a0).v));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut UserDefined) -> *mut UserDefined {
        {
            for __i in 0..1 {
                *&mut self.a[__i] = (*&mut (&mut (*_a0)).a[__i]);
            }
        };
        {
            self.v = std::mem::take(&mut (*_a0).v);
        };
        return &mut (*(self as *mut UserDefined));
    }
    pub unsafe fn new_1(this: *mut UserDefined) {
        (&raw mut (*this).a).write(std::array::from_fn::<_, 1, _>(|_| 0_i32));
        (&raw mut (*this).v).write(Vec::new());
    }
}
impl Default for UserDefined {
    fn default() -> Self {
        unsafe {
            UserDefined {
                a: std::array::from_fn::<_, 1, _>(|_| 0_i32),
                v: Default::default(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct FieldIsLibcType {
    pub addr: libc::sockaddr,
}
impl FieldIsLibcType {
    pub unsafe fn new_1(this: *mut FieldIsLibcType) {
        (&raw mut (*this).addr).write(std::mem::zeroed());
    }
    pub unsafe fn copy_from(this: *mut FieldIsLibcType, _a0: *const FieldIsLibcType) {
        (&raw mut (*this).addr).write((*_a0).addr);
    }
    pub unsafe fn move_from(this: *mut FieldIsLibcType, _a0: *mut FieldIsLibcType) {
        (&raw mut (*this).addr).write((*_a0).addr);
    }
}
impl Default for FieldIsLibcType {
    fn default() -> Self {
        unsafe {
            FieldIsLibcType {
                addr: std::mem::zeroed(),
            }
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: ::libc::pollfd = std::mem::zeroed();
    p.fd = -1_i32;
    p.events = 0_i16;
    p.revents = 2_i16;
    assert!(((p.fd) == (-1_i32)));
    assert!(((p.events as i32) == (0)));
    assert!(((p.revents as i32) == (2)));
    let mut ia: ::libc::in_addr = std::mem::zeroed();
    ia.s_addr = (1 as ::libc::in_addr_t);
    assert!(((ia.s_addr) == (1 as ::libc::in_addr_t)));
    let mut t: ::libc::tm = std::mem::zeroed();
    t.tm_year = 124;
    t.tm_mon = 5;
    t.tm_mday = 15;
    assert!(((t.tm_year) == (124)));
    assert!(((t.tm_mon) == (5)));
    assert!(((t.tm_mday) == (15)));
    let mut st: ::libc::stat = std::mem::zeroed();
    st.st_size = (1024 as ::libc::off_t);
    assert!(((st.st_size) == (1024 as ::libc::off_t)));
    let mut ud: UserDefined = ::std::mem::MaybeUninit::uninit().assume_init();
    UserDefined::new_1(&raw mut ud);
    ud.a[(0_usize)] = 3;
    assert!(((ud.a[(0_usize)]) == (3)));
    assert!(((ud.v.len()) == (0_usize)));
    let mut filt: FieldIsLibcType = ::std::mem::MaybeUninit::uninit().assume_init();
    FieldIsLibcType::new_1(&raw mut filt);
    filt.addr.sa_family = (2 as libc::sa_family_t);
    assert!(((filt.addr.sa_family as i32) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
