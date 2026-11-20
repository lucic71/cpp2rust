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
    pub v: i32,
    pub name: *const libc::c_char,
}
impl Inner {
    pub unsafe fn new_1(this: *mut Inner) {}
    pub unsafe fn copy_from(this: *mut Inner, _a0: *const Inner) {
        (&raw mut (*this).v).write((*_a0).v);
        (&raw mut (*this).name).write((*_a0).name);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer {
    pub p1: *mut i32,
    pub p2: *const i32,
    pub arr: [*mut i32; 3],
    pub cp: *const libc::c_char,
    pub pp: *mut *mut i32,
    pub inner: Inner,
    pub x: i32,
    pub fn_: Option<unsafe fn(i32) -> i32>,
}
impl Outer {
    pub unsafe fn new_1(this: *mut Outer) {
        Inner::new_1((&raw mut (*this).inner));
    }
    pub unsafe fn copy_from(this: *mut Outer, _a0: *const Outer) {
        (&raw mut (*this).p1).write((*_a0).p1);
        (&raw mut (*this).p2).write((*_a0).p2);
        (&raw mut (*this).arr).write(std::array::from_fn::<_, 3, _>(|__i: usize| {
            (*_a0).arr[(__i)]
        }));
        (&raw mut (*this).cp).write((*_a0).cp);
        (&raw mut (*this).pp).write((*_a0).pp);
        Inner::copy_from((&raw mut (*this).inner), { &(*_a0).inner });
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).fn_).write(((*_a0).fn_));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Foo {
    pub s1: *const libc::c_char,
    pub s2: *const libc::c_char,
    pub fn1: Option<unsafe fn(i32) -> i32>,
    pub fn2: Option<unsafe fn(i32) -> i32>,
    pub n: i32,
}
impl Foo {
    pub unsafe fn copy_from(this: *mut Foo, _a0: *const Foo) {
        (&raw mut (*this).s1).write((*_a0).s1);
        (&raw mut (*this).s2).write((*_a0).s2);
        (&raw mut (*this).fn1).write(((*_a0).fn1));
        (&raw mut (*this).fn2).write(((*_a0).fn2));
        (&raw mut (*this).n).write((*_a0).n);
    }
}
pub static mut static_fn_0: libcc2rs::LazyStorage<Option<unsafe fn(i32) -> i32>> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(None) });
pub static mut static_outer_1: libcc2rs::LazyStorage<Outer> =
    libcc2rs::LazyStorage::new(|__p| unsafe { Outer::new_1(__p) });
pub static mut static_inner_array_2: libcc2rs::LazyStorage<[Inner; 2]> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __d = __p.cast::<Inner>();
            for __i in 0..2_isize {
                Inner::new_1(__d.offset(__i));
            }
        }
    });
pub static mut static_foo_3: libcc2rs::LazyStorage<Foo> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __d = __p;
            (&raw mut (*__d).s1).write(c"hello".as_ptr());
            (&raw mut (*__d).s2).write(std::ptr::null());
            (&raw mut (*__d).fn1).write(None);
            (&raw mut (*__d).fn2).write(None);
            (&raw mut (*__d).n).write(42);
        }
    });
pub static mut static_foo_array_4: libcc2rs::LazyStorage<[Foo; 2]> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __d = __p.cast::<Foo>();
            {
                let __d = __d.offset(0);
                (&raw mut (*__d).s1).write(c"first".as_ptr());
                (&raw mut (*__d).s2).write(std::ptr::null());
                (&raw mut (*__d).fn1).write(None);
                (&raw mut (*__d).fn2).write(None);
                (&raw mut (*__d).n).write(1);
            };
            {
                let __d = __d.offset(1);
                (&raw mut (*__d).s1).write(c"second".as_ptr());
                (&raw mut (*__d).s2).write(std::ptr::null());
                (&raw mut (*__d).fn1).write(None);
                (&raw mut (*__d).fn2).write(None);
                (&raw mut (*__d).n).write(2);
            };
        }
    });
pub unsafe fn check_local_static_5() {
    assert!(((*(*&raw const local_outer_6).get()).p1).is_null());
    assert!(((*(*&raw const local_outer_6).get()).fn_).is_none());
    assert!((*(*&raw const local_fn_7).get()).is_none());
    assert!((*(*&raw const local_p_8).get()).is_null());
}
static mut local_outer_6: libcc2rs::LazyStorage<Outer> =
    libcc2rs::LazyStorage::new(|__p| unsafe { Outer::new_1(__p) });
static mut local_fn_7: libcc2rs::LazyStorage<Option<unsafe fn(i32) -> i32>> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(None) });
static mut local_p_8: libcc2rs::LazyStorage<*mut i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(std::ptr::null_mut()) });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((*(*&raw const static_fn_0).get()).is_none());
    assert!(((*(*&raw const static_outer_1).get()).p1).is_null());
    assert!(((*(*&raw const static_outer_1).get()).p2).is_null());
    assert!(((*(*&raw const static_outer_1).get()).cp).is_null());
    assert!(((*(*&raw const static_outer_1).get()).pp).is_null());
    assert!(((*(*&raw const static_outer_1).get()).fn_).is_none());
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (3)) {
            assert!(((*(*&raw const static_outer_1).get()).arr[(i) as usize]).is_null());
            i.prefix_inc();
        }
    }
    assert!(((*(*&raw const static_outer_1).get()).inner.name).is_null());
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (2)) {
            assert!(((*(*&raw const static_inner_array_2).get())[(i) as usize].name).is_null());
            i.prefix_inc();
        }
    }
    assert!(((*(*&raw const static_foo_3).get()).s2).is_null());
    assert!(((*(*&raw const static_foo_3).get()).fn1).is_none());
    assert!(((*(*&raw const static_foo_3).get()).fn2).is_none());
    assert!((((*(*&raw const static_foo_3).get()).n) == (42)));
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (2)) {
            assert!(((*(*&raw const static_foo_array_4).get())[(i) as usize].s2).is_null());
            assert!(((*(*&raw const static_foo_array_4).get())[(i) as usize].fn1).is_none());
            assert!(((*(*&raw const static_foo_array_4).get())[(i) as usize].fn2).is_none());
            i.prefix_inc();
        }
    }
    (unsafe { check_local_static_5() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const static_fn_0).get();
    (*&raw const static_outer_1).get();
    (*&raw const static_inner_array_2).get();
    (*&raw const static_foo_3).get();
    (*&raw const static_foo_array_4).get();
}
