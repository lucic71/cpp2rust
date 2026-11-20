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
    pub unsafe fn copy_from(this: *mut AssertsDtorCountAtExit, _a0: *const AssertsDtorCountAtExit) {
        (&raw mut (*this).expected).write((*_a0).expected);
    }
}
pub unsafe fn count_4() -> i32 {
    return (*(*(*&raw const n_6).get()).prefix_inc());
}
pub unsafe fn count_7() -> i32 {
    return (*(*(*&raw const n_9).get()).prefix_inc());
}
pub unsafe fn count_10() -> i32 {
    return (*(*(*&raw const n_12).get()).prefix_inc());
}
static mut t_5: libcc2rs::LazyStorage<LifetimeTracked> = libcc2rs::LazyStorage::new(|__p| unsafe {
    {
        let __static = LifetimeTracked::new(__p);
        libcc2rs::register_static_destructor(|| (*(*&raw const t_5).get()).destructor());
        __static
    }
});
static mut n_6: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
static mut t_8: libcc2rs::LazyStorage<LifetimeTracked> = libcc2rs::LazyStorage::new(|__p| unsafe {
    {
        let __static = LifetimeTracked::new(__p);
        libcc2rs::register_static_destructor(|| (*(*&raw const t_8).get()).destructor());
        __static
    }
});
static mut n_9: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
static mut t_11: libcc2rs::LazyStorage<LifetimeTracked> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __static = LifetimeTracked::new(__p);
            libcc2rs::register_static_destructor(|| (*(*&raw const t_11).get()).destructor());
            __static
        }
    });
static mut n_12: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut check_13: libcc2rs::LazyStorage<AssertsDtorCountAtExit> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __static = {
                let __d = __p;
                (&raw mut (*__d).expected).write(4);
            };
            libcc2rs::register_static_destructor(|| (*(*&raw const check_13).get()).destructor());
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
    assert!(((unsafe { count_4() }) == (1)));
    assert!(((unsafe { b_count_local_15() }) == (1)));
    assert!(((unsafe { count_4() }) == (2)));
    assert!(((unsafe { count_7() }) == (1)));
    assert!(((unsafe { b_count_int_16() }) == (2)));
    assert!(((unsafe { count_7() }) == (3)));
    assert!(((*(*&raw const ctorCount_0).get()) == (3)));
    assert!(((unsafe { count_10() }) == (1)));
    assert!(((unsafe { b_count_long_17() }) == (2)));
    assert!(((*(*&raw const ctorCount_0).get()) == (4)));
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_14 {}
impl S_14 {
    pub unsafe fn copy_from(this: *mut S_14, _a0: *const S_14) {}
}
pub unsafe fn count_18() -> i32 {
    return (*(*(*&raw const n_20).get()).prefix_inc());
}
static mut t_19: libcc2rs::LazyStorage<LifetimeTracked> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __static = LifetimeTracked::new(__p);
            libcc2rs::register_static_destructor(|| (*(*&raw const t_19).get()).destructor());
            __static
        }
    });
static mut n_20: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub unsafe fn b_count_int_16() -> i32 {
    return (unsafe { count_7() });
}
pub unsafe fn b_count_long_17() -> i32 {
    return (unsafe { count_10() });
}
pub unsafe fn b_count_local_15() -> i32 {
    return (unsafe { count_18() });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_21 {}
impl S_21 {
    pub unsafe fn copy_from(this: *mut S_21, _a0: *const S_21) {}
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const ctorCount_0).get();
    (*&raw const copyCount_1).get();
    (*&raw const dtorCount_2).get();
    (*&raw const useCount_3).get();
    (*&raw const check_13).get();
}
