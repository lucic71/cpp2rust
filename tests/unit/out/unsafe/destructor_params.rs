extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut destroyed_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut last_1: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn destructor(&mut self) {
        (*(*&raw const destroyed_0).get()).postfix_inc();
        (*(*&raw const last_1).get()) = self.v;
    }
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
pub unsafe fn by_value_2(s: *mut S) -> i32 {
    assert!(((*(*&raw const destroyed_0).get()) == (0)));
    return (*s).v;
}
pub unsafe fn with_default_3(s: *mut S) -> i32 {
    return (*s).v;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Holder {
    pub v: i32,
}
impl Holder {
    pub unsafe fn new(this: *mut Holder, s: *mut S) {
        (&raw mut (*this).v).write((*s).v);
    }
    pub unsafe fn add(&self, s: *mut S) -> i32 {
        return ((self.v) + ((*s).v));
    }
    pub unsafe fn copy_from(this: *mut Holder, _a0: *const Holder) {
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
    {
        let mut s: S = ::std::mem::MaybeUninit::uninit().assume_init();
        {
            let __d = &raw mut s;
            (&raw mut (*__d).v).write(1);
        };
        let _dtor_s = std::mem::DropGuard::new(&raw mut s, |__p| unsafe { (*__p).destructor() });
        assert!(
            ((unsafe {
                by_value_2((|__p: *mut S| {
                    S::copy_from(__p, { &s });
                    __p
                })(
                    (&mut ::std::mem::MaybeUninit::<S>::uninit()).as_mut_ptr()
                ))
            }) == (1))
        );
        assert!(
            ((*(*&raw const destroyed_0).get()) == (1)) && ((*(*&raw const last_1).get()) == (1))
        );
    }
    assert!(((*(*&raw const destroyed_0).get()) == (2)));
    (*(*&raw const destroyed_0).get()) = 0;
    assert!(
        ((unsafe {
            by_value_2((|__p: *mut S| {
                {
                    let __d = __p;
                    (&raw mut (*__d).v).write(2);
                };
                __p
            })(
                (&mut ::std::mem::MaybeUninit::<S>::uninit()).as_mut_ptr()
            ))
        }) == (2))
    );
    assert!(((*(*&raw const destroyed_0).get()) == (1)) && ((*(*&raw const last_1).get()) == (2)));
    (*(*&raw const destroyed_0).get()) = 0;
    assert!(
        ((unsafe {
            with_default_3((|__p: *mut S| {
                {
                    let __d = __p;
                    (&raw mut (*__d).v).write(5);
                };
                __p
            })(
                (&mut ::std::mem::MaybeUninit::<S>::uninit()).as_mut_ptr()
            ))
        }) == (5))
    );
    assert!(((*(*&raw const destroyed_0).get()) == (1)) && ((*(*&raw const last_1).get()) == (5)));
    (*(*&raw const destroyed_0).get()) = 0;
    {
        let mut h: Holder = ::std::mem::MaybeUninit::uninit().assume_init();
        Holder::new(&raw mut h, {
            (|__p: *mut S| {
                {
                    let __d = __p;
                    (&raw mut (*__d).v).write(3);
                };
                __p
            })((&mut ::std::mem::MaybeUninit::<S>::uninit()).as_mut_ptr())
        });
        assert!(
            ((*(*&raw const destroyed_0).get()) == (1)) && ((*(*&raw const last_1).get()) == (3))
        );
        assert!(
            ((unsafe {
                Holder::add(
                    &h,
                    (|__p: *mut S| {
                        {
                            let __d = __p;
                            (&raw mut (*__d).v).write(4);
                        };
                        __p
                    })(
                        (&mut ::std::mem::MaybeUninit::<S>::uninit()).as_mut_ptr()
                    ),
                )
            }) == (7))
        );
        assert!(
            ((*(*&raw const destroyed_0).get()) == (2)) && ((*(*&raw const last_1).get()) == (4))
        );
    }
    assert!(((*(*&raw const destroyed_0).get()) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const destroyed_0).get();
    (*&raw const last_1).get();
}
