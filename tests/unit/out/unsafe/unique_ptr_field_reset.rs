extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Data {
    pub v: i32,
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Holder {
    pub data: Option<Box<Data>>,
    pub n: i32,
}
impl Holder {
    pub unsafe fn set(&mut self, mut p: *mut Data) {
        {
            let _a0: *mut Data = p;
            self.data = if _a0.is_null() {
                None
            } else {
                Some(Box::from_raw(_a0))
            }
        };
    }
    pub unsafe fn move_from(this: *mut Holder, _a0: *mut Holder) {
        (&raw mut (*this).data).write((*_a0).data.take());
        (&raw mut (*this).n).write((*_a0).n);
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Holder) -> *mut Holder {
        {
            self.data = (*_a0).data.take();
        };
        self.n = (*_a0).n;
        return &mut (*(self as *mut Holder));
    }
    pub unsafe fn new_1(this: *mut Holder) {
        (&raw mut (*this).data).write(None);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut h: Holder = ::std::mem::MaybeUninit::uninit().assume_init();
    Holder::new_1(&raw mut h);
    h.n = 1;
    let mut hp: *mut Holder = (&mut h as *mut Holder);
    {
        let _a0: *mut Data = (Box::leak(Box::new(Data { v: 3 })) as *mut Data);
        (*hp).data = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    assert!(
        (((*h
            .data
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const Data as *mut Data))
        .v) == (3))
    );
    (unsafe { Holder::set(&mut h, (Box::leak(Box::new(Data { v: 4 })) as *mut Data)) });
    assert!(
        (((*(*hp)
            .data
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const Data as *mut Data))
        .v) == (4))
    );
    (*(*hp)
        .data
        .as_deref()
        .map_or(::std::ptr::null_mut(), |v| v as *const Data as *mut Data))
    .v += (*hp).n;
    assert!(
        (((*h
            .data
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const Data as *mut Data))
        .v) == (5))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
