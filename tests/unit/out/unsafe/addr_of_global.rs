extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Inner {
    pub value: i32,
}
impl Inner {
    pub unsafe fn copy_from(this: *mut Inner, _a0: *const Inner) {
        (&raw mut (*this).value).write((*_a0).value);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer {
    pub p: *mut Inner,
}
impl Outer {
    pub unsafe fn copy_from(this: *mut Outer, _a0: *const Outer) {
        (&raw mut (*this).p).write((*_a0).p);
    }
}
pub static mut alpha_0: libcc2rs::LazyStorage<Inner> = libcc2rs::LazyStorage::new(|__p| unsafe {
    {
        let __d = __p;
        (&raw mut (*__d).value).write(1);
    }
});
pub static mut beta_1: libcc2rs::LazyStorage<Inner> = libcc2rs::LazyStorage::new(|__p| unsafe {
    {
        let __d = __p;
        (&raw mut (*__d).value).write(2);
    }
});
pub static mut shared_2: libcc2rs::LazyStorage<Inner> = libcc2rs::LazyStorage::new(|__p| unsafe {
    {
        let __d = __p;
        (&raw mut (*__d).value).write(42);
    }
});
pub static mut items_3: libcc2rs::LazyStorage<[*mut Inner; 2]> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        __p.write([
            (&raw mut (*(*&raw const alpha_0).get()) as *mut Inner),
            (&raw mut (*(*&raw const beta_1).get()) as *mut Inner),
        ])
    });
pub static mut obj_4: libcc2rs::LazyStorage<Outer> = libcc2rs::LazyStorage::new(|__p| unsafe {
    {
        let __d = __p;
        (&raw mut (*__d).p).write((&raw mut (*(*&raw const shared_2).get()) as *mut Inner));
    }
});
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((((*(*(*&raw const items_3).get())[(0) as usize]).value) == (1)));
    assert!((((*(*(*&raw const items_3).get())[(1) as usize]).value) == (2)));
    assert!((((*(*(*&raw const obj_4).get()).p).value) == (42)));
    assert!((((*(*(*&raw const cache_5).get())[(0) as usize]).value) == (1)));
    assert!((((*(*(*&raw const cache_5).get())[(1) as usize]).value) == (2)));
    return 0;
}
static mut cache_5: libcc2rs::LazyStorage<[*mut Inner; 2]> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        __p.write([
            (&raw mut (*(*&raw const alpha_0).get()) as *mut Inner),
            (&raw mut (*(*&raw const beta_1).get()) as *mut Inner),
        ])
    });
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const alpha_0).get();
    (*&raw const beta_1).get();
    (*&raw const shared_2).get();
    (*&raw const items_3).get();
    (*&raw const obj_4).get();
}
