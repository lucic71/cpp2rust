extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct Counted {
    pub x: i32,
    pub y: i32,
}
impl Counted {
    pub unsafe fn new(this: *mut Counted, mut x: i32, mut y: i32) {
        (&raw mut (*this).x).write(x);
        (&raw mut (*this).y).write(y);
    }
    pub unsafe fn copy_from(this: *mut Counted, other: *const Counted) {
        (&raw mut (*this).x).write((((*other).x) * (2)));
        (&raw mut (*this).y).write((((*other).y) * (2)));
        (*(*&raw const copies_0).get()).prefix_inc();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Counted) -> *mut Counted {
        self.x = (*_a0).x;
        self.y = (*_a0).y;
        return &mut (*(self as *mut Counted));
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
pub unsafe fn make_counted_1(__ret: *mut Counted) {
    Counted::new(__ret.clone(), { 5 }, { 6 });
    return;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: Counted = ::std::mem::MaybeUninit::uninit().assume_init();
    Counted::new(&raw mut s, { 1 }, { 2 });
    let mut __decomp_2: Counted = ::std::mem::MaybeUninit::uninit().assume_init();
    Counted::copy_from(&raw mut __decomp_2, { &s });
    assert!(((*(*&raw const copies_0).get()) == (1)));
    assert!(((__decomp_2.x) == (2)));
    assert!(((__decomp_2.y) == (4)));
    let __decomp_3: *mut Counted = &mut s;
    assert!(((*(*&raw const copies_0).get()) == (1)));
    assert!((((*__decomp_3).x) == (1)));
    assert!((((*__decomp_3).y) == (2)));
    let __decomp_4: *const Counted = &s;
    assert!(((*(*&raw const copies_0).get()) == (1)));
    assert!((((*__decomp_4).x) == (1)));
    let mut __decomp_5: Counted = ::std::mem::MaybeUninit::uninit().assume_init();
    (unsafe { make_counted_1(&raw mut __decomp_5) });
    assert!(((*(*&raw const copies_0).get()) == (1)));
    assert!(((__decomp_5.x) == (5)));
    assert!(((__decomp_5.y) == (6)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const copies_0).get();
}
