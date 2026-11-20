extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut ctorCount_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut copyCount_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut dtorCount_2: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut useCount_3: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
#[placement]
pub struct LifetimeTracked {
    pub uses: i32,
}
impl LifetimeTracked {
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) = {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_1)) = {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)) = {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_3)) = 0;
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_3))
                };
                (*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2))
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_1))
        };
    }
    pub unsafe fn new(this: *mut LifetimeTracked) {
        (&raw mut (*this).uses).write(0);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)).prefix_inc();
    }
    pub unsafe fn copy_from(this: *mut LifetimeTracked, _a0: *const LifetimeTracked) {
        (&raw mut (*this).uses).write(0);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_1)).prefix_inc();
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)).prefix_inc();
    }
    pub unsafe fn use_(&self) {
        (*(&raw const self.uses).cast_mut()).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_3)).prefix_inc();
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
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)) == (self.expected)));
    }
}
pub unsafe fn f_4(mut cond: bool, mut p: Option<unsafe fn()>) {
    if cond {
        (unsafe {
            f_4(
                false,
                Some(|| unsafe {
                    (unsafe {
                        LifetimeTracked::use_(
                            &(*std::cell::LazyCell::force_mut(&mut *&raw mut local_5)),
                        )
                    });
                }),
            )
        });
    } else {
        (unsafe { (p).unwrap()() });
    }
}
static mut local_5: std::cell::LazyCell<LifetimeTracked> = std::cell::LazyCell::new(|| unsafe {
    {
        let __static = {
            let mut __v: LifetimeTracked = ::std::mem::MaybeUninit::uninit().assume_init();
            LifetimeTracked::new(&raw mut __v);
            __v
        };
        libcc2rs::register_static_destructor(|| {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut local_5)).destructor()
        });
        __static
    }
});
pub static mut check_6: std::cell::LazyCell<AssertsDtorCountAtExit> =
    std::cell::LazyCell::new(|| unsafe {
        {
            let __static = AssertsDtorCountAtExit { expected: 1 };
            libcc2rs::register_static_destructor(|| {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut check_6)).destructor()
            });
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
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (0)));
    (unsafe { f_4(true, None) });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_3)) == (1)));
    (unsafe { b_call_7() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_3)) == (2)));
    return 0;
}
pub unsafe fn b_call_7() {
    (unsafe { f_4(true, None) });
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const ctorCount_0);
    std::cell::LazyCell::force(&*&raw const copyCount_1);
    std::cell::LazyCell::force(&*&raw const dtorCount_2);
    std::cell::LazyCell::force(&*&raw const useCount_3);
    std::cell::LazyCell::force(&*&raw const check_6);
}
