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
#[placement]
pub struct StructWithCtor {
    x1_: i32,
    x2_: i32,
}
impl StructWithCtor {
    pub unsafe fn new(this: *mut StructWithCtor, mut x1: i32, mut x2: i32) {
        (&raw mut (*this).x1_).write(x1);
        (&raw mut (*this).x2_).write(x2);
        (*this).x1_.prefix_inc();
        (*this).x2_.prefix_dec();
    }
    pub unsafe fn x1(&self) -> *const i32 {
        return &self.x1_;
    }
    pub unsafe fn x2(&self) -> *const i32 {
        return &self.x2_;
    }
    pub unsafe fn copy_from(this: *mut StructWithCtor, _a0: *const StructWithCtor) {
        (&raw mut (*this).x1_).write((*_a0).x1_);
        (&raw mut (*this).x2_).write((*_a0).x2_);
    }
}
pub unsafe fn foo_0(x: *mut i32) -> *mut i32 {
    return x;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[placement]
#[cpp_from(i32, new, value)]
pub struct Value_ {
    pub v: i32,
}
impl Value_ {
    pub unsafe fn new(this: *mut Value_, mut u: i32) {
        (&raw mut (*this).v).write(u);
    }
    pub unsafe fn copy_from(this: *mut Value_, _a0: *const Value_) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
#[placement]
pub struct Ptr_ {
    pub v1: Value_,
    pub v2: Value_,
}
impl Ptr_ {
    pub unsafe fn new(this: *mut Ptr_) {
        Value_::new((&raw mut (*this).v1), { 11 });
        Value_::new((&raw mut (*this).v2), { 22 });
    }
    pub unsafe fn copy_from(this: *mut Ptr_, _a0: *const Ptr_) {
        Value_::copy_from((&raw mut (*this).v1), { &(*_a0).v1 });
        Value_::copy_from((&raw mut (*this).v2), { &(*_a0).v2 });
    }
}
impl Default for Ptr_ {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: Ptr_ = ::std::mem::MaybeUninit::uninit().assume_init();
                Ptr_::new(&raw mut __v);
                __v
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
    let mut struct_with_ctor: StructWithCtor = ::std::mem::MaybeUninit::uninit().assume_init();
    StructWithCtor::new(&raw mut struct_with_ctor, { 1 }, { 2 });
    let mut x: i32 = 3;
    assert!(
        (((*(unsafe { foo_0(&mut x,) })) == (3))
            && ((*(unsafe { StructWithCtor::x1(&struct_with_ctor,) })) == (2)))
            && ((*(unsafe { StructWithCtor::x2(&struct_with_ctor,) })) == (1))
    );
    let mut p: Ptr_ = ::std::mem::MaybeUninit::uninit().assume_init();
    Ptr_::new(&raw mut p);
    assert!(((p.v1.v) == (11)));
    assert!(((p.v2.v) == (22)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
