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
pub struct Inner {
    pub x: i32,
}
impl Inner {
    pub unsafe fn new_1(this: *mut Inner) {}
    pub unsafe fn copy_from(this: *mut Inner, _a0: *const Inner) {
        (&raw mut (*this).x).write((*_a0).x);
    }
    pub unsafe fn move_from(this: *mut Inner, _a0: *mut Inner) {
        (&raw mut (*this).x).write((*_a0).x);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub data: [i32; 3],
    pub inner: Inner,
}
impl S {
    pub unsafe fn new_1(this: *mut S) {
        Inner::new_1((&raw mut (*this).inner));
    }
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).data).write(std::array::from_fn::<_, 3, _>(|__i: usize| {
            (*_a0).data[(__i)]
        }));
        Inner::copy_from((&raw mut (*this).inner), { &(*_a0).inner });
    }
    pub unsafe fn move_from(this: *mut S, _a0: *mut S) {
        (&raw mut (*this).data).write(std::array::from_fn::<_, 3, _>(|__i: usize| {
            (*_a0).data[(__i)]
        }));
        Inner::move_from((&raw mut (*this).inner), { &mut (*_a0).inner });
    }
}
pub unsafe fn operator_deref_0(s: *mut S) -> *mut Inner {
    return &mut (*s).inner;
}
pub unsafe fn operator_addr_1(s: *mut S) -> *mut i32 {
    return (&mut (*s).data[(0) as usize] as *mut i32);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut s;
        (&raw mut (*__d).data).write([1, 2, 3]);
        {
            let __d = (&raw mut (*__d).inner);
            (&raw mut (*__d).x).write(9);
        };
    };
    assert!(
        (((*(unsafe {
            let _s: *mut S = &mut s;
            operator_deref_0(_s)
        }))
        .x) == (9))
    );
    (*(unsafe {
        let _s: *mut S = &mut s;
        operator_deref_0(_s)
    }))
    .x = 10;
    assert!(((s.inner.x) == (10)));
    let mut p: *mut i32 = (unsafe {
        let _s: *mut S = &mut s;
        operator_addr_1(_s)
    });
    assert!(((*p) == (1)));
    (*p) = 5;
    assert!(((s.data[(0) as usize]) == (5)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
