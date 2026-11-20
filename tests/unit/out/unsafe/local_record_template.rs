extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn get_0(t: *mut Local_1) -> i32 {
    return ((*t).x as i32);
}
pub unsafe fn get_2(t: *mut Local_3) -> i32 {
    return (*t).x;
}
pub unsafe fn get_4(t: *mut Local_5) -> i32 {
    return (*t).x;
}
pub unsafe fn get_6(t: *mut Local_7) -> i32 {
    return ((*t).x as i32);
}
pub unsafe fn twice_8(t: *mut Local_3) -> i32 {
    return (((*t).x) * (2));
}
pub unsafe fn wrap_9(mut v: i32) -> i32 {
    let mut l: Local_5 = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut l;
        (&raw mut (*__d).x).write(v);
    };
    return (unsafe {
        get_4((|__p: *mut Local_5| {
            Local_5::copy_from(__p, { &l });
            __p
        })(
            (&mut ::std::mem::MaybeUninit::<Local_5>::uninit()).as_mut_ptr(),
        ))
    });
}
pub unsafe fn wrap_10(mut v: i64) -> i32 {
    let mut l: Local_7 = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut l;
        (&raw mut (*__d).x).write(v);
    };
    return (unsafe {
        get_6((|__p: *mut Local_7| {
            Local_7::copy_from(__p, { &l });
            __p
        })(
            (&mut ::std::mem::MaybeUninit::<Local_7>::uninit()).as_mut_ptr(),
        ))
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Local_5 {
    pub x: i32,
}
impl Local_5 {
    pub unsafe fn copy_from(this: *mut Local_5, _a0: *const Local_5) {
        (&raw mut (*this).x).write((*_a0).x);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Local_7 {
    pub x: i64,
}
impl Local_7 {
    pub unsafe fn copy_from(this: *mut Local_7, _a0: *const Local_7) {
        (&raw mut (*this).x).write((*_a0).x);
    }
}
pub unsafe fn other_11() -> i32 {
    let mut l: Local_1 = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut l;
        (&raw mut (*__d).x).write(3_i64);
        (&raw mut (*__d).y).write(4_i64);
    };
    return ((unsafe {
        get_0((|__p: *mut Local_1| {
            Local_1::copy_from(__p, { &l });
            __p
        })(
            (&mut ::std::mem::MaybeUninit::<Local_1>::uninit()).as_mut_ptr(),
        ))
    }) + (l.y as i32));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Local_1 {
    pub x: i64,
    pub y: i64,
}
impl Local_1 {
    pub unsafe fn copy_from(this: *mut Local_1, _a0: *const Local_1) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).y).write((*_a0).y);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut l: Local_3 = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut l;
        (&raw mut (*__d).x).write(7);
    };
    assert!(
        ((unsafe {
            get_2((|__p: *mut Local_3| {
                Local_3::copy_from(__p, { &l });
                __p
            })(
                (&mut ::std::mem::MaybeUninit::<Local_3>::uninit()).as_mut_ptr(),
            ))
        }) == (7))
    );
    assert!(
        ((unsafe {
            twice_8((|__p: *mut Local_3| {
                Local_3::copy_from(__p, { &l });
                __p
            })(
                (&mut ::std::mem::MaybeUninit::<Local_3>::uninit()).as_mut_ptr(),
            ))
        }) == (14))
    );
    assert!(((unsafe { other_11() }) == (7)));
    assert!(((unsafe { wrap_9(5,) }) == (5)));
    assert!(((unsafe { wrap_10(6_i64,) }) == (6)));
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Local_3 {
    pub x: i32,
}
impl Local_3 {
    pub unsafe fn copy_from(this: *mut Local_3, _a0: *const Local_3) {
        (&raw mut (*this).x).write((*_a0).x);
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
