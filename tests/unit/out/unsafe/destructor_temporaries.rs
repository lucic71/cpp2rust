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
    pub unsafe fn get(&self) -> i32 {
        return self.v;
    }
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Outer {
    pub inner: S,
    pub w: i32,
}
impl Outer {
    pub unsafe fn copy_from(this: *mut Outer, _a0: *const Outer) {
        S::copy_from((&raw mut (*this).inner), { &(*_a0).inner });
        (&raw mut (*this).w).write((*_a0).w);
    }
}
impl Outer {
    pub unsafe fn destructor(&mut self) {
        S::destructor(&mut self.inner);
    }
}
pub unsafe fn make_2(__ret: *mut S, mut v: i32) {
    {
        let __d = __ret.clone();
        (&raw mut (*__d).v).write(v);
    };
    return;
}
pub unsafe fn make_outer_3(__ret: *mut Outer) {
    {
        let __d = __ret.clone();
        {
            let __d = (&raw mut (*__d).inner);
            (&raw mut (*__d).v).write(7);
        };
        (&raw mut (*__d).w).write(8);
    };
    return;
}
pub unsafe fn use_4(s: *const S) -> i32 {
    assert!(((*(*&raw const destroyed_0).get()) == (0)));
    return (*s).v;
}
pub unsafe fn value_5(s: *const S) -> i32 {
    return (*s).v;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
        (unsafe { make_2(&raw mut __v, 1) });
        __v
    };
    assert!(((*(*&raw const destroyed_0).get()) == (1)) && ((*(*&raw const last_1).get()) == (1)));
    (*(*&raw const destroyed_0).get()) = 0;
    &({
        let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
        (unsafe { make_2(&raw mut __v, 2) });
        __v
    });
    assert!(((*(*&raw const destroyed_0).get()) == (1)) && ((*(*&raw const last_1).get()) == (2)));
    (*(*&raw const destroyed_0).get()) = 0;
    let mut x: i32 = {
        let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
        (unsafe { make_2(&raw mut __v, 3) });
        __v
    }
    .v;
    assert!(
        (((x) == (3)) && ((*(*&raw const destroyed_0).get()) == (1)))
            && ((*(*&raw const last_1).get()) == (3))
    );
    (*(*&raw const destroyed_0).get()) = 0;
    x = (unsafe {
        S::get(&mut *(|__p: *mut S| {
            (unsafe { make_2(__p, 4) });
            __p
        })(
            (&mut ::std::mem::MaybeUninit::<S>::uninit()).as_mut_ptr(),
        ))
    });
    assert!(
        (((x) == (4)) && ((*(*&raw const destroyed_0).get()) == (1)))
            && ((*(*&raw const last_1).get()) == (4))
    );
    (*(*&raw const destroyed_0).get()) = 0;
    x = {
        let mut __v: Outer = ::std::mem::MaybeUninit::uninit().assume_init();
        (unsafe { make_outer_3(&raw mut __v) });
        __v
    }
    .inner
    .v;
    assert!(
        (((x) == (7)) && ((*(*&raw const destroyed_0).get()) == (1)))
            && ((*(*&raw const last_1).get()) == (7))
    );
    (*(*&raw const destroyed_0).get()) = 0;
    x = S { v: 5 }.v;
    assert!(((x) == (5)) && ((*(*&raw const destroyed_0).get()) == (1)));
    (*(*&raw const destroyed_0).get()) = 0;
    x = (unsafe {
        let mut _s: S = ::std::mem::MaybeUninit::uninit().assume_init();
        (unsafe { make_2(&raw mut _s, 6) });
        use_4(&mut _s)
    });
    assert!(
        (((x) == (6)) && ((*(*&raw const destroyed_0).get()) == (1)))
            && ((*(*&raw const last_1).get()) == (6))
    );
    (*(*&raw const destroyed_0).get()) = 0;
    x = ((unsafe {
        let mut _s: S = ::std::mem::MaybeUninit::uninit().assume_init();
        {
            let __d = &raw mut _s;
            (&raw mut (*__d).v).write(1);
        };
        value_5(&mut _s)
    }) + (unsafe {
        let mut _s: S = ::std::mem::MaybeUninit::uninit().assume_init();
        {
            let __d = &raw mut _s;
            (&raw mut (*__d).v).write(2);
        };
        value_5(&mut _s)
    }));
    assert!(((x) == (3)) && ((*(*&raw const destroyed_0).get()) == (2)));
    (*(*&raw const destroyed_0).get()) = 0;
    x = (if ((x) == (3)) {
        {
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { make_2(&raw mut __v, 9) });
            __v
        }
    } else {
        S { v: 10 }
    })
    .v;
    assert!(
        (((x) == (9)) && ((*(*&raw const destroyed_0).get()) == (1)))
            && ((*(*&raw const last_1).get()) == (9))
    );
    (*(*&raw const destroyed_0).get()) = 0;
    {
        let mut __tmp_0: S = ::std::mem::MaybeUninit::uninit().assume_init();
        (unsafe { make_2(&raw mut __tmp_0, 11) });
        let r: *const S = &mut __tmp_0;
        assert!(((*(*&raw const destroyed_0).get()) == (0)) && (((*r).v) == (11)));
    }
    assert!(((*(*&raw const destroyed_0).get()) == (1)) && ((*(*&raw const last_1).get()) == (11)));
    (*(*&raw const destroyed_0).get()) = 0;
    {
        let mut s: S = ::std::mem::MaybeUninit::uninit().assume_init();
        {
            let __d = &raw mut s;
            (&raw mut (*__d).v).write(12);
        };
        let _dtor_s = std::mem::DropGuard::new(&raw mut s, |__p| unsafe { (*__p).destructor() });
        s = {
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { make_2(&raw mut __v, 13) });
            __v
        };
        assert!(
            (((*(*&raw const destroyed_0).get()) == (1))
                && ((*(*&raw const last_1).get()) == (13)))
                && ((s.v) == (13))
        );
    }
    assert!(((*(*&raw const destroyed_0).get()) == (2)) && ((*(*&raw const last_1).get()) == (13)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const destroyed_0).get();
    (*&raw const last_1).get();
}
