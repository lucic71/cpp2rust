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
    pub a: i32,
    pub b: i32,
}
impl Inner {
    pub unsafe fn new_1(this: *mut Inner) {}
    pub unsafe fn copy_from(this: *mut Inner, _a0: *const Inner) {
        (&raw mut (*this).a).write((*_a0).a);
        (&raw mut (*this).b).write((*_a0).b);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Cookie {
    pub x: i32,
    pub data: *mut i32,
    pub in_: *const i32,
    pub inner: Inner,
    pub arr: [i32; 4],
}
impl Cookie {
    pub unsafe fn new_1(this: *mut Cookie) {
        Inner::new_1((&raw mut (*this).inner));
    }
    pub unsafe fn copy_from(this: *mut Cookie, _a0: *const Cookie) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).data).write((*_a0).data);
        (&raw mut (*this).in_).write((*_a0).in_);
        Inner::copy_from((&raw mut (*this).inner), { &(*_a0).inner });
        (&raw mut (*this).arr).write(std::array::from_fn::<_, 4, _>(|__i: usize| {
            (*_a0).arr[(__i)]
        }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Counter {
    pub n: i32,
}
impl Counter {
    pub unsafe fn inc(&mut self) {
        self.n.prefix_inc();
    }
    pub unsafe fn new_1(this: *mut Counter) {
        (&raw mut (*this).n).write(0);
    }
    pub unsafe fn copy_from(this: *mut Counter, _a0: *const Counter) {
        (&raw mut (*this).n).write((*_a0).n);
    }
}
impl Default for Counter {
    fn default() -> Self {
        unsafe { Counter { n: 0 } }
    }
}
pub unsafe fn set_0(mut p: *mut i32, mut v: i32) {
    (*p) = v;
}
pub unsafe fn get_1(c: *const Cookie) -> i32 {
    return (*c).x;
}
pub unsafe fn first_2(mut p: *const i32) -> i32 {
    return (*p.offset((0) as isize));
}
pub unsafe fn consume_3(cookie: *const Cookie) -> i32 {
    let mut c: Cookie = ::std::mem::MaybeUninit::uninit().assume_init();
    Cookie::copy_from(&raw mut c, { cookie });
    let mut sum: i32 = 0;
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (4)) {
            (*c.data.offset((i) as isize)) = ((*c.in_.offset((i) as isize)) * (c.x));
            sum += (*c.data.offset((i) as isize));
            i.prefix_inc();
        }
    }
    c.x = sum;
    c.inner.a += c.x;
    c.arr[(1) as usize] = c.inner.a;
    (*c.data) = ((c.arr[(1) as usize]) + (unsafe { first_2(c.in_) }));
    (unsafe {
        let _p: *mut i32 = c.data.offset((1) as isize);
        let _v: i32 = (unsafe { first_2((c.data).cast_const()) });
        set_0(_p, _v)
    });
    return ((c.x) + (c.inner.b));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut data: [i32; 4] = [0, 0, 0, 0];
    let mut in_: [i32; 4] = [1, 2, 3, 4];
    let mut cookie: Cookie = ::std::mem::MaybeUninit::uninit().assume_init();
    Cookie::new_1(&raw mut cookie);
    cookie.x = 2;
    cookie.data = data.as_mut_ptr();
    cookie.in_ = (in_.as_mut_ptr()).cast_const();
    cookie.inner.a = 1;
    cookie.inner.b = 3;
    assert!(((unsafe { consume_3(&cookie,) }) == (23)));
    assert!(
        (((data[(0) as usize]) == (22)) && ((data[(1) as usize]) == (22)))
            && ((data[(3) as usize]) == (8))
    );
    assert!(((unsafe { get_1(&cookie,) }) == (2)));
    let mut local: Inner = ::std::mem::MaybeUninit::uninit().assume_init();
    Inner::new_1(&raw mut local);
    local.a = 4;
    local.b = ((local.a) * (2));
    local.a.postfix_inc();
    assert!((((local.a) + (local.b)) == (13)));
    let mut field_addr: Inner = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut field_addr;
        (&raw mut (*__d).a).write(0);
        (&raw mut (*__d).b).write(0);
    };
    (unsafe { set_0((&mut field_addr.b as *mut i32), 7) });
    assert!(((field_addr.b) == (7)));
    let mut counter: Counter = ::std::mem::MaybeUninit::uninit().assume_init();
    Counter::new_1(&raw mut counter);
    (unsafe { Counter::inc(&mut counter) });
    assert!(((counter.n) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
