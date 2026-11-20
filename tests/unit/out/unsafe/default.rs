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
pub struct Pointers {
    pub x1: *mut i32,
    pub x2: *const i32,
    pub x3: [*mut i32; 5],
    pub x4: [*const i32; 10],
    pub x5: i32,
}
impl Pointers {
    pub unsafe fn new_1(this: *mut Pointers) {}
    pub unsafe fn copy_from(this: *mut Pointers, _a0: *const Pointers) {
        (&raw mut (*this).x1).write((*_a0).x1);
        (&raw mut (*this).x2).write((*_a0).x2);
        (&raw mut (*this).x3).write(std::array::from_fn::<_, 5, _>(|__i: usize| {
            (*_a0).x3[(__i)]
        }));
        (&raw mut (*this).x4).write(std::array::from_fn::<_, 10, _>(|__i: usize| {
            (*_a0).x4[(__i)]
        }));
        (&raw mut (*this).x5).write((*_a0).x5);
    }
    pub unsafe fn move_from(this: *mut Pointers, _a0: *mut Pointers) {
        (&raw mut (*this).x1).write((*_a0).x1);
        (&raw mut (*this).x2).write((*_a0).x2);
        (&raw mut (*this).x3).write(std::array::from_fn::<_, 5, _>(|__i: usize| {
            (*_a0).x3[(__i)]
        }));
        (&raw mut (*this).x4).write(std::array::from_fn::<_, 10, _>(|__i: usize| {
            (*_a0).x4[(__i)]
        }));
        (&raw mut (*this).x5).write((*_a0).x5);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SmallArrays {
    pub a: [i32; 32],
    pub f: Option<unsafe fn(i32) -> i32>,
    pub fs: [Option<unsafe fn(i32) -> i32>; 2],
    pub p: [Pointers; 2],
}
impl SmallArrays {
    pub unsafe fn new_1(this: *mut SmallArrays) {
        {
            let __d = (&raw mut (*this).p).cast::<Pointers>();
            for __i in 0..2_isize {
                Pointers::new_1(__d.offset(__i));
            }
        };
    }
    pub unsafe fn copy_from(this: *mut SmallArrays, _a0: *const SmallArrays) {
        (&raw mut (*this).a).write(std::array::from_fn::<_, 32, _>(|__i: usize| {
            (*_a0).a[(__i)]
        }));
        (&raw mut (*this).f).write(((*_a0).f));
        (&raw mut (*this).fs).write(std::array::from_fn::<_, 2, _>(|__i: usize| {
            (*_a0).fs[(__i)]
        }));
        {
            let __d = (&raw mut (*this).p).cast::<Pointers>();
            for __i in 0..2_usize {
                Pointers::copy_from(__d.offset(__i as isize), { &(*_a0).p[(__i)] });
            }
        };
    }
    pub unsafe fn move_from(this: *mut SmallArrays, _a0: *mut SmallArrays) {
        (&raw mut (*this).a).write(std::array::from_fn::<_, 32, _>(|__i: usize| {
            (*_a0).a[(__i)]
        }));
        (&raw mut (*this).f).write(((*_a0).f));
        (&raw mut (*this).fs).write(std::array::from_fn::<_, 2, _>(|__i: usize| {
            (*_a0).fs[(__i)]
        }));
        {
            let __d = (&raw mut (*this).p).cast::<Pointers>();
            for __i in 0..2_usize {
                Pointers::move_from(__d.offset(__i as isize), { &mut (*_a0).p[(__i)] });
            }
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct BigArray {
    pub a: [i32; 33],
}
impl BigArray {
    pub unsafe fn new_1(this: *mut BigArray) {}
    pub unsafe fn copy_from(this: *mut BigArray, _a0: *const BigArray) {
        (&raw mut (*this).a).write(std::array::from_fn::<_, 33, _>(|__i: usize| {
            (*_a0).a[(__i)]
        }));
    }
    pub unsafe fn move_from(this: *mut BigArray, _a0: *mut BigArray) {
        (&raw mut (*this).a).write(std::array::from_fn::<_, 33, _>(|__i: usize| {
            (*_a0).a[(__i)]
        }));
    }
}
impl Default for BigArray {
    fn default() -> Self {
        unsafe { BigArray { a: [0_i32; 33] } }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut default_pointers: *mut Pointers = Box::leak(
        (0..10_usize)
            .map(|_| <Pointers>::default())
            .collect::<Box<[Pointers]>>(),
    )
    .as_mut_ptr();
    {
        let __p = default_pointers;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<Pointers>(),
            )))
        }
    };
    let mut small: *mut SmallArrays = Box::leak(
        (0..2_usize)
            .map(|_| <SmallArrays>::default())
            .collect::<Box<[SmallArrays]>>(),
    )
    .as_mut_ptr();
    {
        let __p = small;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<SmallArrays>(),
            )))
        }
    };
    let mut big: *mut BigArray = Box::leak(
        (0..2_usize)
            .map(|_| <BigArray>::default())
            .collect::<Box<[BigArray]>>(),
    )
    .as_mut_ptr();
    {
        let __p = big;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<BigArray>(),
            )))
        }
    };
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
