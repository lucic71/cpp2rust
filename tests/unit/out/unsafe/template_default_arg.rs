extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[placement]
#[cpp_from(i32, new_1, value)]
pub struct A {
    pub v: i32,
}
impl A {
    pub unsafe fn new_2(this: *mut A) {
        (&raw mut (*this).v).write(1);
    }
    pub unsafe fn new_1(this: *mut A, mut v: i32) {
        (&raw mut (*this).v).write(v);
    }
    pub unsafe fn copy_from(this: *mut A, _a0: *const A) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
impl Default for A {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: A = ::std::mem::MaybeUninit::uninit().assume_init();
                A::new_2(&raw mut __v);
                __v
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
#[placement]
pub struct B {
    pub v: i32,
}
impl B {
    pub unsafe fn new(this: *mut B) {
        (&raw mut (*this).v).write(2);
    }
    pub unsafe fn copy_from(this: *mut B, _a0: *const B) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
impl Default for B {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: B = ::std::mem::MaybeUninit::uninit().assume_init();
                B::new(&raw mut __v);
                __v
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[placement]
#[cpp_from(i32, new, value)]
pub struct NoDefault {
    pub v: i32,
}
impl NoDefault {
    pub unsafe fn new(this: *mut NoDefault, mut v: i32) {
        (&raw mut (*this).v).write(v);
    }
    pub unsafe fn copy_from(this: *mut NoDefault, _a0: *const NoDefault) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
pub unsafe fn used_0(x: *mut A) -> i32 {
    return (*x).v;
}
pub unsafe fn used_1(x: *mut B) -> i32 {
    return (*x).v;
}
pub unsafe fn scaled_2(x: *mut A, mut n: i32) -> i32 {
    return (((*x).v) * (n));
}
pub unsafe fn always_given_3(x: *mut NoDefault) -> i32 {
    return (*x).v;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[placement]
#[cpp_from(i32, new, value)]
pub struct S_NoDefault_ {
    pub v: i32,
}
impl S_NoDefault_ {
    pub unsafe fn new(this: *mut S_NoDefault_, mut v: i32) {
        (&raw mut (*this).v).write(v);
    }
    pub unsafe fn get(&self, t: *mut NoDefault) -> i32 {
        return ((self.v) + ((*t).v));
    }
    pub unsafe fn copy_from(this: *mut S_NoDefault_, _a0: *const S_NoDefault_) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((unsafe {
            used_0((|__p: *mut A| {
                A::new_2(__p);
                __p
            })(
                (&mut ::std::mem::MaybeUninit::<A>::uninit()).as_mut_ptr()
            ))
        }) == (1))
    );
    assert!(
        ((unsafe {
            used_0((|__p: *mut A| {
                A::new_1(__p, { 5 });
                __p
            })(
                (&mut ::std::mem::MaybeUninit::<A>::uninit()).as_mut_ptr()
            ))
        }) == (5))
    );
    assert!(
        ((unsafe {
            used_1((|__p: *mut B| {
                B::new(__p);
                __p
            })(
                (&mut ::std::mem::MaybeUninit::<B>::uninit()).as_mut_ptr()
            ))
        }) == (2))
    );
    assert!(
        ((unsafe {
            scaled_2(
                (|__p: *mut A| {
                    A::new_1(__p, { 3 });
                    __p
                })((&mut ::std::mem::MaybeUninit::<A>::uninit()).as_mut_ptr()),
                (::std::mem::size_of::<A>() as i32),
            )
        }) == ((3) * (::std::mem::size_of::<A>() as i32)))
    );
    assert!(
        ((unsafe {
            scaled_2(
                (|__p: *mut A| {
                    A::new_1(__p, { 3 });
                    __p
                })((&mut ::std::mem::MaybeUninit::<A>::uninit()).as_mut_ptr()),
                2,
            )
        }) == (6))
    );
    assert!(
        ((unsafe {
            always_given_3((|__p: *mut NoDefault| {
                NoDefault::new(__p, { 3 });
                __p
            })(
                (&mut ::std::mem::MaybeUninit::<NoDefault>::uninit()).as_mut_ptr(),
            ))
        }) == (3))
    );
    let mut s: S_NoDefault_ = ::std::mem::MaybeUninit::uninit().assume_init();
    S_NoDefault_::new(&raw mut s, { 1 });
    assert!(
        ((unsafe {
            S_NoDefault_::get(
                &s,
                (|__p: *mut NoDefault| {
                    NoDefault::new(__p, { 4 });
                    __p
                })(
                    (&mut ::std::mem::MaybeUninit::<NoDefault>::uninit()).as_mut_ptr()
                ),
            )
        }) == (5))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
