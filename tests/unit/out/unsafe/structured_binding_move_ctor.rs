extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut moves_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct Movable {
    pub x: i32,
    pub y: i32,
}
impl Movable {
    pub unsafe fn new(this: *mut Movable, mut x: i32, mut y: i32) {
        (&raw mut (*this).x).write(x);
        (&raw mut (*this).y).write(y);
    }
    pub unsafe fn copy_from(this: *mut Movable, other: *const Movable) {
        (&raw mut (*this).x).write((*other).x);
        (&raw mut (*this).y).write((*other).y);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
    }
    pub unsafe fn move_from(this: *mut Movable, other: *mut Movable) {
        (&raw mut (*this).x).write((*other).x);
        (&raw mut (*this).y).write((*other).y);
        (*other).x = 0;
        (*other).y = 0;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)).prefix_inc();
    }
}
impl Clone for Movable {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: Movable = ::std::mem::MaybeUninit::uninit().assume_init();
            Movable::copy_from(&raw mut __v, self as *const Movable);
            __v
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Holder {
    pub xs: Vec<i32>,
    pub ys: Vec<i32>,
}
impl Holder {
    pub unsafe fn extract(&mut self, __ret: *mut Holder) {
        {
            let __d = __ret.clone();
            (&raw mut (*__d).xs).write(std::mem::take(&mut self.xs));
            (&raw mut (*__d).ys).write(std::mem::take(&mut self.ys));
        };
        return;
    }
    pub unsafe fn copy_from(this: *mut Holder, _a0: *const Holder) {
        (&raw mut (*this).xs).write((*_a0).xs.clone());
        (&raw mut (*this).ys).write((*_a0).ys.clone());
    }
    pub unsafe fn move_from(this: *mut Holder, _a0: *mut Holder) {
        (&raw mut (*this).xs).write(std::mem::take(&mut (*_a0).xs));
        (&raw mut (*this).ys).write(std::mem::take(&mut (*_a0).ys));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Holder) -> *mut Holder {
        {
            self.xs = std::mem::take(&mut (*_a0).xs);
        };
        {
            self.ys = std::mem::take(&mut (*_a0).ys);
        };
        return &mut (*(self as *mut Holder));
    }
    pub unsafe fn new_1(this: *mut Holder) {
        (&raw mut (*this).xs).write(Vec::new());
        (&raw mut (*this).ys).write(Vec::new());
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut m: Movable = ::std::mem::MaybeUninit::uninit().assume_init();
    Movable::new(&raw mut m, { 3 }, { 4 });
    let mut __decomp_2: Movable = {
        let mut __v: Movable = ::std::mem::MaybeUninit::uninit().assume_init();
        Movable::move_from(&raw mut __v, { &mut m });
        __v
    };
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    assert!(((__decomp_2.x) == (3)));
    assert!(((__decomp_2.y) == (4)));
    assert!(((m.x) == (0)));
    assert!(((m.y) == (0)));
    let mut h: Holder = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut h;
        (&raw mut (*__d).xs).write(Vec::from([1, 2, 3]));
        (&raw mut (*__d).ys).write(Vec::from([4, 5]));
    };
    let mut __decomp_3: Holder = {
        let mut __v: Holder = ::std::mem::MaybeUninit::uninit().assume_init();
        (unsafe { Holder::extract(&mut h, &raw mut __v) });
        __v
    };
    assert!(((__decomp_3.xs.len()) == (3_usize)));
    assert!(((__decomp_3.ys.len()) == (2_usize)));
    assert!(((__decomp_3.xs[(2_usize)]) == (3)));
    assert!(((__decomp_3.ys[(0_usize)]) == (4)));
    assert!(h.xs.is_empty());
    assert!(h.ys.is_empty());
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
    std::cell::LazyCell::force(&*&raw const moves_1);
}
