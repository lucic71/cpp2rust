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
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct AssertsDtorCountAtExit {
    pub expected: i32,
}
impl AssertsDtorCountAtExit {
    pub unsafe fn destructor(&mut self) {
        assert!(((*(*&raw const dtorCount_2).get()) == (self.expected)));
    }
    pub unsafe fn new_1(this: *mut AssertsDtorCountAtExit) {}
    pub unsafe fn copy_from(this: *mut AssertsDtorCountAtExit, _a0: *const AssertsDtorCountAtExit) {
        (&raw mut (*this).expected).write((*_a0).expected);
    }
}
pub unsafe fn instance_4() -> *mut LifetimeTracked {
    return &mut (*(*&raw const t_5).get());
}
static mut t_5: libcc2rs::LazyStorage<LifetimeTracked> = libcc2rs::LazyStorage::new(|__p| unsafe {
    {
        let __static = LifetimeTracked::new(__p);
        libcc2rs::register_static_destructor(|| (*(*&raw const t_5).get()).destructor());
        __static
    }
});
pub static mut check_6: libcc2rs::LazyStorage<AssertsDtorCountAtExit> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __static = {
                let __d = __p;
                (&raw mut (*__d).expected).write(1);
            };
            libcc2rs::register_static_destructor(|| (*(*&raw const check_6).get()).destructor());
            __static
        }
    });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((*(*&raw const ctorCount_0).get()) == (0)));
    (unsafe { LifetimeTracked::use_(&(*(unsafe { instance_4() }))) });
    (unsafe { b_use_7() });
    assert!(((*(*&raw const ctorCount_0).get()) == (1)));
    assert!(((*(*&raw const useCount_3).get()) == (2)));
    assert!((((*(unsafe { instance_4() })).uses) == (2)));
    assert!(((unsafe { instance_4() }) == (unsafe { b_address_8() })));
    assert!(((*(*&raw const ctorCount_0).get()) == (1)));
    assert!(((*(*&raw const copyCount_1).get()) == (0)));
    return 0;
}
pub unsafe fn b_use_7() {
    (unsafe { LifetimeTracked::use_(&(*(unsafe { instance_4() }))) });
}
pub unsafe fn b_address_8() -> *mut LifetimeTracked {
    return (unsafe { instance_4() });
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const ctorCount_0).get();
    (*&raw const copyCount_1).get();
    (*&raw const dtorCount_2).get();
    (*&raw const useCount_3).get();
    (*&raw const check_6).get();
}
