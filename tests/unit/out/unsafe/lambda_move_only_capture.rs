extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, DestructorUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct Owner {
    pub p: *mut i32,
}
impl Owner {
    pub unsafe fn new(this: *mut Owner, mut v: i32) {
        (&raw mut (*this).p).write((Box::leak(Box::new(v)) as *mut i32));
    }
    pub unsafe fn move_from(this: *mut Owner, o: *mut Owner) {
        (&raw mut (*this).p).write((*o).p);
        (*o).p = std::ptr::null_mut();
    }
    pub unsafe fn destructor(&mut self) {
        {
            let __p = self.p;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(__p))
            }
        };
    }
    pub unsafe fn take(&mut self) -> FnPtr<fn() -> i32> {
        return lambda_unsafe!(
            {
                let self_: Owner = {
                    let mut __v: Owner = ::std::mem::MaybeUninit::uninit().assume_init();
                    Owner::move_from(&raw mut __v, { &mut (*(self as *mut Owner)) });
                    __v
                };
            },
            || -> i32 {
                return (*self_.p);
            }
        );
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut o: Owner = ::std::mem::MaybeUninit::uninit().assume_init();
    Owner::new(&raw mut o, { 5 });
    let _dtor_o = std::mem::DropGuard::new(&raw mut o, |__p| unsafe { (*__p).destructor() });
    let mut f: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let h: Owner = {
                let mut __v: Owner = ::std::mem::MaybeUninit::uninit().assume_init();
                Owner::move_from(&raw mut __v, { &mut o });
                __v
            };
        },
        || -> i32 {
            return (*h.p);
        }
    );
    let _dtor_f = std::mem::DropGuard::new(&raw mut f, |__p| unsafe { (*__p).destroy() });
    assert!((o.p).is_null());
    assert!(((unsafe { f.call() }) == (5)));
    let mut g: FnPtr<fn() -> i32> = f.move_from();
    let _dtor_g = std::mem::DropGuard::new(&raw mut g, |__p| unsafe { (*__p).destroy() });
    assert!(((unsafe { g.call() }) == (5)));
    let mut total: i32 = 0;
    let mut consume: FnPtr<fn()> = lambda_unsafe!(
        {
            let h: Owner = {
                let mut __v: Owner = ::std::mem::MaybeUninit::uninit().assume_init();
                Owner::new(&raw mut __v, { 7 });
                __v
            };
            let total: *mut i32 = &mut total;
        },
        || {
            (*total) += (*h.p);
            (*h.p) = 0;
        }
    );
    let _dtor_consume =
        std::mem::DropGuard::new(&raw mut consume, |__p| unsafe { (*__p).destroy() });
    (unsafe { consume.call() });
    (unsafe { consume.call() });
    assert!(((total) == (7)));
    let mut o2: Owner = ::std::mem::MaybeUninit::uninit().assume_init();
    Owner::new(&raw mut o2, { 9 });
    let _dtor_o2 = std::mem::DropGuard::new(&raw mut o2, |__p| unsafe { (*__p).destructor() });
    let mut t: FnPtr<fn() -> i32> = (unsafe { Owner::take(&mut o2) });
    let _dtor_t = std::mem::DropGuard::new(&raw mut t, |__p| unsafe { (*__p).destroy() });
    assert!((o2.p).is_null());
    assert!(((unsafe { t.call() }) == (9)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
