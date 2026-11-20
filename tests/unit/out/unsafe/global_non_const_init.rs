extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn next_0() -> i32 {
    return (*(*(*&raw const counter_1).get()).prefix_inc());
}
static mut counter_1: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub unsafe fn marker_2(mut tag: u8) -> u8 {
    return ((((tag as i32) << (3)) | (2)) as u8);
}
pub static mut signature_3: libcc2rs::LazyStorage<[u8; 3]> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        __p.write([
            (unsafe { marker_2(1_u8) }),
            4_u8,
            (('B' as libc::c_char) as u8),
        ])
    });
pub static mut single_4: libcc2rs::LazyStorage<u8> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write((unsafe { marker_2(2_u8) })) });
pub static mut from_call_5: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write((unsafe { next_0() })) });
pub static mut depends_on_call_6: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        __p.write(((*(*&raw const from_call_5).get()) + (1)))
    });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Ctor {
    pub v: i32,
}
impl Ctor {
    pub unsafe fn new_1(this: *mut Ctor) {
        (&raw mut (*this).v).write((unsafe { next_0() }));
    }
    pub unsafe fn new_2(this: *mut Ctor, mut x: i32) {
        (&raw mut (*this).v).write(x);
    }
    pub unsafe fn copy_from(this: *mut Ctor, _a0: *const Ctor) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
impl Default for Ctor {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: Ctor = ::std::mem::MaybeUninit::uninit().assume_init();
                Ctor::new_1(&raw mut __v);
                __v
            }
        }
    }
}
pub static mut default_ctor_7: libcc2rs::LazyStorage<Ctor> =
    libcc2rs::LazyStorage::new(|__p| unsafe { Ctor::new_1(__p) });
pub static mut arg_ctor_8: libcc2rs::LazyStorage<Ctor> =
    libcc2rs::LazyStorage::new(|__p| unsafe { Ctor::new_2(__p, { 7 }) });
pub static mut str_9: libcc2rs::LazyStorage<Vec<libc::c_char>> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        __p.write(
            std::slice::from_raw_parts(
                c"abc".as_ptr(),
                (0..).take_while(|&i| *c"abc".as_ptr().add(i) != 0).count() + 1,
            )
            .to_vec(),
        )
    });
pub static mut inline_member_11: libcc2rs::LazyStorage<Ctor> =
    libcc2rs::LazyStorage::new(|__p| unsafe { Ctor::new_2(__p, { 5 }) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Holder {}
impl Holder {
    pub unsafe fn copy_from(this: *mut Holder, _a0: *const Holder) {}
}
pub static mut member_10: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write((unsafe { next_0() })) });
pub unsafe fn local_static_12() -> i32 {
    return ((*(*&raw const once_13).get()) + ((*(*&raw const local_ctor_14).get()).v));
}
static mut once_13: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write((unsafe { next_0() })) });
static mut local_ctor_14: libcc2rs::LazyStorage<Ctor> =
    libcc2rs::LazyStorage::new(|__p| unsafe { Ctor::new_2(__p, { 3 }) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Singleton {
    pub hits: i32,
}
impl Singleton {
    pub unsafe fn new(this: *mut Singleton) {
        (&raw mut (*this).hits).write(0);
    }
    pub unsafe fn instance() -> *mut Singleton {
        return &mut (*(*&raw const s_15).get());
    }
    pub unsafe fn copy_from(this: *mut Singleton, _a0: *const Singleton) {
        (&raw mut (*this).hits).write((*_a0).hits);
    }
}
impl Default for Singleton {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: Singleton = ::std::mem::MaybeUninit::uninit().assume_init();
                Singleton::new(&raw mut __v);
                __v
            }
        }
    }
}
static mut s_15: libcc2rs::LazyStorage<Singleton> =
    libcc2rs::LazyStorage::new(|__p| unsafe { Singleton::new(__p) });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((((*(*&raw const signature_3).get())[(0) as usize] as i32) == (10)));
    assert!((((*(*&raw const signature_3).get())[(1) as usize] as i32) == (4)));
    assert!((((*(*&raw const single_4).get()) as i32) == (18)));
    assert!(((*(*&raw const from_call_5).get()) == (1)));
    assert!(((*(*&raw const depends_on_call_6).get()) == (2)));
    assert!((((*(*&raw const default_ctor_7).get()).v) == (2)));
    assert!((((*(*&raw const arg_ctor_8).get()).v) == (7)));
    assert!(
        (*(*&raw const str_9).get()) == {
            std::slice::from_raw_parts(
                c"abc".as_ptr(),
                (0..).take_while(|&i| *c"abc".as_ptr().add(i) != 0).count() + 1,
            )
            .to_vec()
        }
    );
    assert!(((*(*&raw const member_10).get()) == (3)));
    assert!((((*(*&raw const inline_member_11).get()).v) == (5)));
    assert!(((unsafe { local_static_12() }) == (7)));
    assert!(((unsafe { local_static_12() }) == (7)));
    (*(unsafe { Singleton::instance() })).hits.postfix_inc();
    (*(unsafe { Singleton::instance() })).hits.postfix_inc();
    assert!((((*(unsafe { Singleton::instance() })).hits) == (2)));
    assert!(((unsafe { Singleton::instance() }) == (unsafe { Singleton::instance() })));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const signature_3).get();
    (*&raw const single_4).get();
    (*&raw const from_call_5).get();
    (*&raw const depends_on_call_6).get();
    (*&raw const default_ctor_7).get();
    (*&raw const arg_ctor_8).get();
    (*&raw const str_9).get();
    (*&raw const inline_member_11).get();
    (*&raw const member_10).get();
}
