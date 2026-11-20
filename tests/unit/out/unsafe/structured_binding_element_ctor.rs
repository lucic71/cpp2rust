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
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[placement]
#[cpp_from(i32, new, value)]
pub struct Elem {
    pub v: i32,
}
impl Elem {
    pub unsafe fn new(this: *mut Elem, mut v: i32) {
        (&raw mut (*this).v).write(v);
    }
    pub unsafe fn copy_from(this: *mut Elem, other: *const Elem) {
        (&raw mut (*this).v).write((((*other).v) + (100)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
    }
    pub unsafe fn move_from(this: *mut Elem, other: *mut Elem) {
        (&raw mut (*this).v).write((((*other).v) + (1000)));
        (*other).v = -1_i32;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)).prefix_inc();
    }
}
impl Clone for Elem {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: Elem = ::std::mem::MaybeUninit::uninit().assume_init();
            Elem::copy_from(&raw mut __v, self as *const Elem);
            __v
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
#[placement]
pub struct Two {
    pub a: Elem,
    pub b: Elem,
}
impl Two {
    pub unsafe fn copy_from(this: *mut Two, _a0: *const Two) {
        Elem::copy_from((&raw mut (*this).a), { &(*_a0).a });
        Elem::copy_from((&raw mut (*this).b), { &(*_a0).b });
    }
    pub unsafe fn move_from(this: *mut Two, _a0: *mut Two) {
        Elem::move_from((&raw mut (*this).a), { &mut (*_a0).a });
        Elem::move_from((&raw mut (*this).b), { &mut (*_a0).b });
    }
}
pub unsafe fn make_two_2(__ret: *mut Two) {
    {
        let __d = __ret.clone();
        Elem::new((&raw mut (*__d).a), { 1 });
        Elem::new((&raw mut (*__d).b), { 2 });
    };
    return;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut t: Two = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut t;
        Elem::new((&raw mut (*__d).a), { 1 });
        Elem::new((&raw mut (*__d).b), { 2 });
    };
    let mut base_copies: i32 = (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0));
    let mut base_moves: i32 = (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1));
    let mut __decomp_3: Two = ::std::mem::MaybeUninit::uninit().assume_init();
    Two::copy_from(&raw mut __decomp_3, { &t });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == ((base_copies) + (2))));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (base_moves)));
    assert!(((__decomp_3.a.v) == (101)));
    assert!(((__decomp_3.b.v) == (102)));
    let __decomp_4: *mut Two = &mut t;
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == ((base_copies) + (2))));
    assert!((((*__decomp_4).a.v) == (1)));
    (*__decomp_4).a.v = 50;
    assert!(((t.a.v) == (50)));
    let mut __decomp_5: Two = ::std::mem::MaybeUninit::uninit().assume_init();
    Two::move_from(&raw mut __decomp_5, { &mut t });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == ((base_moves) + (2))));
    assert!(((__decomp_5.a.v) == (1050)));
    assert!(((__decomp_5.b.v) == (1002)));
    assert!(((t.a.v) == (-1_i32)));
    let mut before_copies: i32 = (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0));
    let mut before_moves: i32 = (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1));
    let mut __decomp_6: Two = ::std::mem::MaybeUninit::uninit().assume_init();
    (unsafe { make_two_2(&raw mut __decomp_6) });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (before_copies)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (before_moves)));
    assert!(((__decomp_6.a.v) == (1)));
    let mut p: (Elem, Elem) = (
        {
            let mut __v: Elem = ::std::mem::MaybeUninit::uninit().assume_init();
            Elem::new(&raw mut __v, { 7 });
            __v
        },
        {
            let mut __v: Elem = ::std::mem::MaybeUninit::uninit().assume_init();
            Elem::new(&raw mut __v, { 8 });
            __v
        },
    );
    before_copies = (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0));
    let mut __decomp_7: (Elem, Elem) = p.clone();
    let x: *mut Elem = &mut *__decomp_7.get_mut::<0>();
    let y: *mut Elem = &mut *__decomp_7.get_mut::<1>();
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == ((before_copies) + (2)))
    );
    assert!((((*x).v) == ((p.0.v) + (100))));
    assert!((((*y).v) == ((p.1.v) + (100))));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
    std::cell::LazyCell::force(&*&raw const moves_1);
}
