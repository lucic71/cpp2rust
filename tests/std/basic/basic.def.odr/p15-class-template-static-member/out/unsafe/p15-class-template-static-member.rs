extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut ctorCount_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut copyCount_1: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut dtorCount_2: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut useCount_3: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct LifetimeTracked {
    pub uses: i32,
}
impl LifetimeTracked {
    pub unsafe fn reset() {
        (*(*&raw const ctorCount_0).get()) = {
            (*(*&raw const copyCount_1).get()) = {
                (*(*&raw const dtorCount_2).get()) = {
                    (*(*&raw const useCount_3).get()) = 0;
                    (*(*&raw const useCount_3).get())
                };
                (*(*&raw const dtorCount_2).get())
            };
            (*(*&raw const copyCount_1).get())
        };
    }
    pub unsafe fn new(this: *mut LifetimeTracked) {
        (&raw mut (*this).uses).write(0);
        (*(*&raw const ctorCount_0).get()).prefix_inc();
    }
    pub unsafe fn copy_from(this: *mut LifetimeTracked, _a0: *const LifetimeTracked) {
        (&raw mut (*this).uses).write(0);
        (*(*&raw const copyCount_1).get()).prefix_inc();
    }
    pub unsafe fn destructor(&mut self) {
        (*(*&raw const dtorCount_2).get()).prefix_inc();
    }
    pub unsafe fn use_(&self) {
        (*(&raw const self.uses).cast_mut()).prefix_inc();
        (*(*&raw const useCount_3).get()).prefix_inc();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const LifetimeTracked) -> *mut LifetimeTracked {
        self.uses = (*_a0).uses;
        return &mut (*(self as *mut LifetimeTracked));
    }
}
impl Clone for LifetimeTracked {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: LifetimeTracked = ::std::mem::MaybeUninit::uninit().assume_init();
            LifetimeTracked::copy_from(&raw mut __v, self as *const LifetimeTracked);
            __v
        }
    }
}
impl Default for LifetimeTracked {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: LifetimeTracked = ::std::mem::MaybeUninit::uninit().assume_init();
                LifetimeTracked::new(&raw mut __v);
                __v
            }
        }
    }
}
pub static mut member_4: libcc2rs::LazyStorage<LifetimeTracked> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __static = LifetimeTracked::new(__p);
            libcc2rs::register_static_destructor(|| (*(*&raw const member_4).get()).destructor());
            __static
        }
    });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasStaticMember_int_ {}
impl HasStaticMember_int_ {
    pub unsafe fn new_1(this: *mut HasStaticMember_int_) {}
    pub unsafe fn copy_from(this: *mut HasStaticMember_int_, _a0: *const HasStaticMember_int_) {}
    pub unsafe fn move_from(this: *mut HasStaticMember_int_, _a0: *mut HasStaticMember_int_) {}
}
pub static mut member_5: libcc2rs::LazyStorage<LifetimeTracked> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __static = LifetimeTracked::new(__p);
            libcc2rs::register_static_destructor(|| (*(*&raw const member_5).get()).destructor());
            __static
        }
    });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasStaticMember_long_ {}
impl HasStaticMember_long_ {
    pub unsafe fn new_1(this: *mut HasStaticMember_long_) {}
    pub unsafe fn copy_from(this: *mut HasStaticMember_long_, _a0: *const HasStaticMember_long_) {}
    pub unsafe fn move_from(this: *mut HasStaticMember_long_, _a0: *mut HasStaticMember_long_) {}
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { LifetimeTracked::use_(&(*(*&raw const member_4).get())) });
    (unsafe { b_use_int_6() });
    (unsafe { b_use_long_7() });
    assert!((((*(*&raw const member_4).get()).uses) == (2)));
    assert!((((*(*&raw const member_5).get()).uses) == (1)));
    assert!(
        ((&raw mut (*(*&raw const member_4).get()) as *mut LifetimeTracked)
            == (unsafe { b_int_address_8() }))
    );
    assert!(((*(*&raw const ctorCount_0).get()) == (2)));
    assert!(((*(*&raw const copyCount_1).get()) == (0)));
    return 0;
}
pub unsafe fn b_use_int_6() {
    (unsafe { LifetimeTracked::use_(&(*(*&raw const member_4).get())) });
}
pub unsafe fn b_use_long_7() {
    (unsafe { LifetimeTracked::use_(&(*(*&raw const member_5).get())) });
}
pub unsafe fn b_int_address_8() -> *mut LifetimeTracked {
    return (&raw mut (*(*&raw const member_4).get()) as *mut LifetimeTracked);
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const ctorCount_0).get();
    (*&raw const copyCount_1).get();
    (*&raw const dtorCount_2).get();
    (*&raw const useCount_3).get();
    (*&raw const member_4).get();
    (*&raw const member_5).get();
}
