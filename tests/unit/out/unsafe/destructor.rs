extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut global_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct S {}
impl S {
    pub unsafe fn destructor(&mut self) {
        (*(*&raw const global_0).get()).postfix_inc();
    }
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {}
    pub unsafe fn new_1(this: *mut S) {}
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Defaulted {
    pub s: S,
}
impl Defaulted {
    pub unsafe fn copy_from(this: *mut Defaulted, _a0: *const Defaulted) {
        S::copy_from((&raw mut (*this).s), { &(*_a0).s });
    }
}
impl Defaulted {
    pub unsafe fn destructor(&mut self) {
        S::destructor(&mut self.s);
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Middle {
    pub s: S,
}
impl Middle {
    pub unsafe fn copy_from(this: *mut Middle, _a0: *const Middle) {
        S::copy_from((&raw mut (*this).s), { &(*_a0).s });
    }
}
impl Middle {
    pub unsafe fn destructor(&mut self) {
        S::destructor(&mut self.s);
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Outer {
    pub m: Middle,
}
impl Outer {
    pub unsafe fn copy_from(this: *mut Outer, _a0: *const Outer) {
        Middle::copy_from((&raw mut (*this).m), { &(*_a0).m });
    }
}
impl Outer {
    pub unsafe fn destructor(&mut self) {
        Middle::destructor(&mut self.m);
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct ArrayMember {
    pub items: [S; 3],
}
impl ArrayMember {
    pub unsafe fn copy_from(this: *mut ArrayMember, _a0: *const ArrayMember) {
        {
            let __d = (&raw mut (*this).items).cast::<S>();
            for __i in 0..3_usize {
                S::copy_from(__d.offset(__i as isize), { &(*_a0).items[(__i)] });
            }
        };
    }
}
impl ArrayMember {
    pub unsafe fn destructor(&mut self) {
        for __e in self.items.iter_mut() {
            S::destructor(__e);
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct EmptyBody {
    pub s: S,
}
impl EmptyBody {
    pub unsafe fn destructor(&mut self) {
        S::destructor(&mut self.s);
    }
    pub unsafe fn copy_from(this: *mut EmptyBody, _a0: *const EmptyBody) {
        S::copy_from((&raw mut (*this).s), { &(*_a0).s });
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Templated_char_ {
    pub v: libc::c_char,
}
impl Templated_char_ {
    pub unsafe fn destructor(&mut self) {
        (*(*&raw const global_0).get()) = (((*(*&raw const global_0).get()) as usize)
            .wrapping_add((::std::mem::size_of::<libc::c_char>() as usize)))
            as i32;
    }
    pub unsafe fn copy_from(this: *mut Templated_char_, _a0: *const Templated_char_) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Templated_int_ {
    pub v: i32,
}
impl Templated_int_ {
    pub unsafe fn destructor(&mut self) {
        (*(*&raw const global_0).get()) = (((*(*&raw const global_0).get()) as usize)
            .wrapping_add((::std::mem::size_of::<i32>() as usize)))
            as i32;
    }
    pub unsafe fn copy_from(this: *mut Templated_int_, _a0: *const Templated_int_) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Copied {
    pub v: i32,
}
impl Copied {
    pub unsafe fn destructor(&mut self) {
        (*(*&raw const global_0).get()).postfix_inc();
    }
    pub unsafe fn copy_from(this: *mut Copied, _a0: *const Copied) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
pub static mut order_1: libcc2rs::LazyStorage<[i32; 3]> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write([0_i32; 3]) });
pub static mut order_count_2: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Tagged {
    pub tag: i32,
}
impl Tagged {
    pub unsafe fn destructor(&mut self) {
        (*(*&raw const order_1).get())
            [((*(*&raw const order_count_2).get()).postfix_inc()) as usize] = self.tag;
    }
    pub unsafe fn copy_from(this: *mut Tagged, _a0: *const Tagged) {
        (&raw mut (*this).tag).write((*_a0).tag);
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Ordered {
    pub first: Tagged,
    pub dummy1: i32,
    pub second: Tagged,
    pub dummy2: i32,
    pub third: Tagged,
}
impl Ordered {
    pub unsafe fn copy_from(this: *mut Ordered, _a0: *const Ordered) {
        Tagged::copy_from((&raw mut (*this).first), { &(*_a0).first });
        (&raw mut (*this).dummy1).write((*_a0).dummy1);
        Tagged::copy_from((&raw mut (*this).second), { &(*_a0).second });
        (&raw mut (*this).dummy2).write((*_a0).dummy2);
        Tagged::copy_from((&raw mut (*this).third), { &(*_a0).third });
    }
}
impl Ordered {
    pub unsafe fn destructor(&mut self) {
        Tagged::destructor(&mut self.third);
        Tagged::destructor(&mut self.second);
        Tagged::destructor(&mut self.first);
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
        };
        let _dtor_s = std::mem::DropGuard::new(&raw mut s, |__p| unsafe { (*__p).destructor() });
    }
    assert!(((*(*&raw const global_0).get()) == (1)));
    {
        let mut s: S = ::std::mem::MaybeUninit::uninit().assume_init();
        {
            let __d = &raw mut s;
        };
        let _dtor_s = std::mem::DropGuard::new(&raw mut s, |__p| unsafe { (*__p).destructor() });
    }
    assert!(((*(*&raw const global_0).get()) == (2)));
    {
        let mut d: Defaulted = ::std::mem::MaybeUninit::uninit().assume_init();
        {
            let __d = &raw mut d;
            {
                let __d = (&raw mut (*__d).s);
            };
        };
        let _dtor_d = std::mem::DropGuard::new(&raw mut d, |__p| unsafe { (*__p).destructor() });
    }
    assert!(((*(*&raw const global_0).get()) == (3)));
    {
        let mut o: Outer = ::std::mem::MaybeUninit::uninit().assume_init();
        {
            let __d = &raw mut o;
            {
                let __d = (&raw mut (*__d).m);
                {
                    let __d = (&raw mut (*__d).s);
                };
            };
        };
        let _dtor_o = std::mem::DropGuard::new(&raw mut o, |__p| unsafe { (*__p).destructor() });
    }
    assert!(((*(*&raw const global_0).get()) == (4)));
    {
        let mut am: ArrayMember = ::std::mem::MaybeUninit::uninit().assume_init();
        {
            let __d = &raw mut am;
            {
                let __d = (&raw mut (*__d).items).cast::<S>();
                {
                    let __d = __d.offset(0);
                };
                {
                    let __d = __d.offset(1);
                };
                {
                    let __d = __d.offset(2);
                };
            };
        };
        let _dtor_am = std::mem::DropGuard::new(&raw mut am, |__p| unsafe { (*__p).destructor() });
    }
    assert!(((*(*&raw const global_0).get()) == (7)));
    {
        let mut e: EmptyBody = ::std::mem::MaybeUninit::uninit().assume_init();
        {
            let __d = &raw mut e;
            {
                let __d = (&raw mut (*__d).s);
            };
        };
        let _dtor_e = std::mem::DropGuard::new(&raw mut e, |__p| unsafe { (*__p).destructor() });
    }
    assert!(((*(*&raw const global_0).get()) == (8)));
    {
        let mut tc: Templated_char_ = ::std::mem::MaybeUninit::uninit().assume_init();
        {
            let __d = &raw mut tc;
            (&raw mut (*__d).v).write((0 as libc::c_char));
        };
        let _dtor_tc = std::mem::DropGuard::new(&raw mut tc, |__p| unsafe { (*__p).destructor() });
        let mut ti: Templated_int_ = ::std::mem::MaybeUninit::uninit().assume_init();
        {
            let __d = &raw mut ti;
            (&raw mut (*__d).v).write(0_i32);
        };
        let _dtor_ti = std::mem::DropGuard::new(&raw mut ti, |__p| unsafe { (*__p).destructor() });
    }
    assert!(((*(*&raw const global_0).get()) == (13)));
    {
        let mut a: Copied = ::std::mem::MaybeUninit::uninit().assume_init();
        {
            let __d = &raw mut a;
            (&raw mut (*__d).v).write(5);
        };
        let _dtor_a = std::mem::DropGuard::new(&raw mut a, |__p| unsafe { (*__p).destructor() });
        let mut b: Copied = ::std::mem::MaybeUninit::uninit().assume_init();
        Copied::copy_from(&raw mut b, { &a });
        let _dtor_b = std::mem::DropGuard::new(&raw mut b, |__p| unsafe { (*__p).destructor() });
        assert!(((b.v) == (5)));
    }
    assert!(((*(*&raw const global_0).get()) == (15)));
    {
        let mut o: Ordered = ::std::mem::MaybeUninit::uninit().assume_init();
        {
            let __d = &raw mut o;
            {
                let __d = (&raw mut (*__d).first);
                (&raw mut (*__d).tag).write(1);
            };
            (&raw mut (*__d).dummy1).write(0);
            {
                let __d = (&raw mut (*__d).second);
                (&raw mut (*__d).tag).write(2);
            };
            (&raw mut (*__d).dummy2).write(0);
            {
                let __d = (&raw mut (*__d).third);
                (&raw mut (*__d).tag).write(3);
            };
        };
        let _dtor_o = std::mem::DropGuard::new(&raw mut o, |__p| unsafe { (*__p).destructor() });
    }
    assert!(((*(*&raw const order_count_2).get()) == (3)));
    assert!((((*(*&raw const order_1).get())[(0) as usize]) == (3)));
    assert!((((*(*&raw const order_1).get())[(1) as usize]) == (2)));
    assert!((((*(*&raw const order_1).get())[(2) as usize]) == (1)));
    let mut before: i32 = (*(*&raw const global_0).get());
    {
        let mut cs: S = ::std::mem::MaybeUninit::uninit().assume_init();
        S::new_1(&raw mut cs);
        let _dtor_cs = std::mem::DropGuard::new(&raw mut cs, |__p| unsafe { (*__p).destructor() });
    }
    assert!(((*(*&raw const global_0).get()) == ((before) + (1))));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const global_0).get();
    (*&raw const order_1).get();
    (*&raw const order_count_2).get();
}
