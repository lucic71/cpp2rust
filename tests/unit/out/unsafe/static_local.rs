extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0() -> i32 {
    (*(*&raw const kX1_4).get()) += 1;
    return (((*(*&raw const kX1_4).get()) + (*(*&raw const kX2_5).get()))
        + (*(*&raw const static_i_1).get()));
}
static mut static_i_1: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
static mut static_f_2: libcc2rs::LazyStorage<f32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_f32) });
static mut static_b_3: libcc2rs::LazyStorage<bool> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(false) });
static mut kX1_4: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(1) });
static mut kX2_5: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(2) });
pub unsafe fn from_local_class_6() -> i32 {
    return ((unsafe {
        S_8::get(&mut *(|__p: *mut S_8| {
            {
                let __d = __p;
            };
            __p
        })(
            (&mut ::std::mem::MaybeUninit::<S_8>::uninit()).as_mut_ptr(),
        ))
    }) + (unsafe {
        S_8::get(&mut *(|__p: *mut S_8| {
            {
                let __d = __p;
            };
            __p
        })(
            (&mut ::std::mem::MaybeUninit::<S_8>::uninit()).as_mut_ptr(),
        ))
    }));
}
static mut x_7: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(3) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_8 {}
impl S_8 {
    pub unsafe fn get(&self) -> i32 {
        return (*(*(*&raw const x_7).get()).prefix_inc());
    }
    pub unsafe fn copy_from(this: *mut S_8, _a0: *const S_8) {}
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((unsafe { foo_0() }) + (unsafe { foo_0() })) + (unsafe { foo_0() })) == (15)));
    assert!(((unsafe { from_local_class_6() }) == (9)));
    assert!(((unsafe { from_local_class_6() }) == (13)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
