extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut a: i32, mut b: i32) -> i32 {
    return ((a) + (b));
}
pub unsafe fn baz_1(mut a: *mut i32, mut b: *mut i32) -> bool {
    return ((a) == (b));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new, value)]
pub struct Bar {
    pub v: i32,
}
impl Bar {
    pub unsafe fn new(this: *mut Bar, mut v: i32) {
        (&raw mut (*this).v).write(v);
    }
}
impl Default for Bar {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: Bar = ::std::mem::MaybeUninit::uninit().assume_init();
                Bar::new(&raw mut __v, { 1 });
                __v
            }
        }
    }
}
pub static mut counter_2: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn next_3() -> i32 {
    return (*(*std::cell::LazyCell::force_mut(&mut *&raw mut counter_2)).prefix_inc());
}
pub unsafe fn lazy_4(mut x: i32) -> i32 {
    return x;
}
pub unsafe fn by_ref_5(b: *const Bar) -> i32 {
    return (*b).v;
}
pub static mut global_bar_6: std::cell::LazyCell<Bar> = std::cell::LazyCell::new(|| unsafe {
    {
        let mut __v: Bar = ::std::mem::MaybeUninit::zeroed().assume_init();
        Bar::new(&raw mut __v, { 9 });
        __v
    }
});
pub unsafe fn by_global_ref_7(b: *const Bar) -> i32 {
    return (*b).v;
}
pub unsafe fn shifted_8(mut x: i32, mut y: i32) -> i32 {
    return ((x) + (y));
}
pub unsafe fn first_9(r: *mut [i32; 3]) -> i32 {
    return (*r)[(0) as usize];
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Holder_int_ {
    pub v: i32,
}
impl Holder_int_ {
    pub unsafe fn new(this: *mut Holder_int_, mut v: i32) {
        (&raw mut (*this).v).write(v);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { foo_0(1, 10,) }) == (11)));
    assert!(((unsafe { foo_0(1, 2,) }) == (3)));
    let mut a: i32 = 0;
    assert!(
        (((unsafe { baz_1((&mut a as *mut i32), std::ptr::null_mut(),) }) as i32)
            == (false as i32))
    );
    assert!(
        (((unsafe {
            let _a: *mut i32 = (&mut a as *mut i32);
            let _b: *mut i32 = (&mut a as *mut i32);
            baz_1(_a, _b)
        }) as i32)
            == (true as i32))
    );
    let mut b: Bar = ::std::mem::MaybeUninit::uninit().assume_init();
    Bar::new(&raw mut b, { 1 });
    assert!(((b.v) == (1)));
    assert!(
        (({
            let mut __v: Bar = ::std::mem::MaybeUninit::uninit().assume_init();
            Bar::new(&raw mut __v, { 2 });
            __v
        }
        .v) == (2))
    );
    let mut arr: [Bar; 3] = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = (&raw mut arr).cast::<Bar>();
        Bar::new(__d.offset(0), { 1 });
        Bar::new(__d.offset(1), { 1 });
        Bar::new(__d.offset(2), { 1 });
    };
    assert!(((arr[(0) as usize].v) == (1)));
    assert!(((arr[(2) as usize].v) == (1)));
    assert!(((unsafe { lazy_4(5,) }) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut counter_2)) == (0)));
    assert!(((unsafe { lazy_4((unsafe { next_3() }),) }) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut counter_2)) == (1)));
    assert!(
        ((unsafe {
            let mut _b: Bar = ::std::mem::MaybeUninit::uninit().assume_init();
            let __tmp_0 = &raw mut _b;
            Bar::new(__tmp_0, { 7 });
            by_ref_5(&mut _b)
        }) == (7))
    );
    assert!(
        ((unsafe {
            let mut _b: Bar = ::std::mem::MaybeUninit::uninit().assume_init();
            let __tmp_1 = &raw mut _b;
            Bar::new(__tmp_1, { 3 });
            by_ref_5(&mut _b)
        }) == (3))
    );
    assert!(
        ((unsafe {
            by_global_ref_7(&(*std::cell::LazyCell::force_mut(&mut *&raw mut global_bar_6)))
        }) == (9))
    );
    assert!(
        ((unsafe {
            let mut _b: Bar = ::std::mem::MaybeUninit::uninit().assume_init();
            let __tmp_2 = &raw mut _b;
            Bar::new(__tmp_2, { 4 });
            by_global_ref_7(&mut _b)
        }) == (4))
    );
    let mut h: Holder_int_ = ::std::mem::MaybeUninit::uninit().assume_init();
    Holder_int_::new(&raw mut h, { 4 });
    assert!(((h.v) == (4)));
    assert!(((unsafe { shifted_8(3, -1_i32,) }) == (2)));
    assert!(((unsafe { shifted_8(3, 4,) }) == (7)));
    assert!(
        ((unsafe {
            let mut _r: [i32; 3] = [4, 5, 6];
            first_9(&mut _r)
        }) == (4))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const counter_2);
    std::cell::LazyCell::force(&*&raw const global_bar_6);
}
