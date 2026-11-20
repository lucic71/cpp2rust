extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct Counted {
    pub copies: i32,
    pub moves: i32,
}
impl Counted {
    pub unsafe fn new(this: *mut Counted) {
        (&raw mut (*this).copies).write(0);
        (&raw mut (*this).moves).write(0);
    }
    pub unsafe fn copy_from(this: *mut Counted, o: *const Counted) {
        (&raw mut (*this).copies).write((((*o).copies) + (1)));
        (&raw mut (*this).moves).write((*o).moves);
    }
    pub unsafe fn move_from(this: *mut Counted, o: *mut Counted) {
        (&raw mut (*this).copies).write((*o).copies);
        (&raw mut (*this).moves).write((((*o).moves) + (1)));
    }
}
impl Clone for Counted {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: Counted = ::std::mem::MaybeUninit::uninit().assume_init();
            Counted::copy_from(&raw mut __v, self as *const Counted);
            __v
        }
    }
}
impl Default for Counted {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: Counted = ::std::mem::MaybeUninit::uninit().assume_init();
                Counted::new(&raw mut __v);
                __v
            }
        }
    }
}
pub static mut drops_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct Dropped {}
impl Dropped {
    pub unsafe fn new(this: *mut Dropped) {}
    pub unsafe fn copy_from(this: *mut Dropped, _a0: *const Dropped) {}
    pub unsafe fn move_from(this: *mut Dropped, _a0: *mut Dropped) {}
    pub unsafe fn destructor(&mut self) {
        (*(*&raw const drops_0).get()).postfix_inc();
    }
}
impl Clone for Dropped {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: Dropped = ::std::mem::MaybeUninit::uninit().assume_init();
            Dropped::copy_from(&raw mut __v, self as *const Dropped);
            __v
        }
    }
}
impl Default for Dropped {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: Dropped = ::std::mem::MaybeUninit::uninit().assume_init();
                Dropped::new(&raw mut __v);
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
    let mut c: Counted = ::std::mem::MaybeUninit::uninit().assume_init();
    Counted::new(&raw mut c);
    let mut f: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let c: Counted = {
                let mut __v: Counted = ::std::mem::MaybeUninit::uninit().assume_init();
                Counted::copy_from(&raw mut __v, { &c });
                __v
            };
        },
        || -> i32 {
            return (((c.copies) * (10)) + (c.moves));
        }
    );
    assert!(((unsafe { f.call() }) == (10)));
    let mut g: FnPtr<fn() -> i32> = f.copy_from();
    assert!(((unsafe { g.call() }) == (20)));
    assert!(((unsafe { f.call() }) == (10)));
    let mut h: FnPtr<fn() -> i32> = f.move_from();
    assert!(((unsafe { h.call() }) == (11)));
    let mut returned: i32 = {
        let mut __v: Counted = ::std::mem::MaybeUninit::uninit().assume_init();
        (unsafe {
            lambda_unsafe!(
                {
                    let c: Counted = {
                        let mut __v: Counted = ::std::mem::MaybeUninit::uninit().assume_init();
                        Counted::copy_from(&raw mut __v, { &c });
                        __v
                    };
                },
                |__ret: *mut Counted| {
                    Counted::copy_from(__ret.clone(), { &c });
                    return;
                }
            )
            .call(&raw mut __v)
        });
        __v
    }
    .copies;
    assert!(((returned) == (2)));
    let mut arr: [Counted; 2] = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = (&raw mut arr).cast::<Counted>();
        for __i in 0..2_isize {
            Counted::new(__d.offset(__i));
        }
    };
    let mut a: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let arr: [Counted; 2] = std::array::from_fn::<_, 2, _>(|__i: usize| {
                let mut __v: Counted = ::std::mem::MaybeUninit::uninit().assume_init();
                Counted::copy_from(&raw mut __v, { &arr[(__i)] });
                __v
            });
        },
        || -> i32 {
            return ((arr[(0) as usize].copies) + (arr[(1) as usize].copies));
        }
    );
    assert!(((unsafe { a.call() }) == (2)));
    let mut a2: FnPtr<fn() -> i32> = a.copy_from();
    assert!(((unsafe { a2.call() }) == (4)));
    let mut a3: FnPtr<fn() -> i32> = a.move_from();
    assert!(((unsafe { a3.call() }) == (2)));
    {
        let mut m: FnPtr<fn()> = lambda_unsafe!(
            {
                let d: Dropped = {
                    let mut __v: Dropped = ::std::mem::MaybeUninit::uninit().assume_init();
                    Dropped::new(&raw mut __v);
                    __v
                };
            },
            || {}
        );
        let _dtor_m = std::mem::DropGuard::new(&raw mut m, |__p| unsafe { (*__p).destroy() });
        let mut m2: FnPtr<fn()> = m.move_from();
        let _dtor_m2 = std::mem::DropGuard::new(&raw mut m2, |__p| unsafe { (*__p).destroy() });
    }
    assert!(((*(*&raw const drops_0).get()) == (2)));
    {
        let mut k: FnPtr<fn()> = lambda_unsafe!(
            {
                let d: Dropped = {
                    let mut __v: Dropped = ::std::mem::MaybeUninit::uninit().assume_init();
                    Dropped::new(&raw mut __v);
                    __v
                };
            },
            || {}
        );
        let _dtor_k = std::mem::DropGuard::new(&raw mut k, |__p| unsafe { (*__p).destroy() });
        let mut k2: FnPtr<fn()> = k.copy_from();
        let _dtor_k2 = std::mem::DropGuard::new(&raw mut k2, |__p| unsafe { (*__p).destroy() });
    }
    assert!(((*(*&raw const drops_0).get()) == (4)));
    {
        let mut inner: FnPtr<fn()> = lambda_unsafe!(
            {
                let d: Dropped = {
                    let mut __v: Dropped = ::std::mem::MaybeUninit::uninit().assume_init();
                    Dropped::new(&raw mut __v);
                    __v
                };
            },
            || {}
        );
        let _dtor_inner =
            std::mem::DropGuard::new(&raw mut inner, |__p| unsafe { (*__p).destroy() });
        let mut outer: FnPtr<fn()> = lambda_unsafe!(
            {
                let inner: FnPtr<fn()> = inner.copy_from();
            },
            || {}
        );
        let _dtor_outer =
            std::mem::DropGuard::new(&raw mut outer, |__p| unsafe { (*__p).destroy() });
    }
    assert!(((*(*&raw const drops_0).get()) == (6)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const drops_0).get();
}
