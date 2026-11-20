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
pub struct Outer_Named {
    pub a: i32,
    pub b: i32,
}
impl Outer_Named {
    pub unsafe fn copy_from(this: *mut Outer_Named, _a0: *const Outer_Named) {
        (&raw mut (*this).a).write((*_a0).a);
        (&raw mut (*this).b).write((*_a0).b);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_0 {
    pub c: i32,
    pub d: i32,
}
impl anon_0 {
    pub unsafe fn copy_from(this: *mut anon_0, _a0: *const anon_0) {
        (&raw mut (*this).c).write((*_a0).c);
        (&raw mut (*this).d).write((*_a0).d);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_1 {
    pub g: i32,
    pub h: i32,
}
impl anon_1 {
    pub unsafe fn copy_from(this: *mut anon_1, _a0: *const anon_1) {
        (&raw mut (*this).g).write((*_a0).g);
        (&raw mut (*this).h).write((*_a0).h);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_2 {
    pub e: i32,
    pub f: i32,
}
impl anon_2 {
    pub unsafe fn copy_from(this: *mut anon_2, _a0: *const anon_2) {
        (&raw mut (*this).e).write((*_a0).e);
        (&raw mut (*this).f).write((*_a0).f);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_4 {
    pub j: i32,
}
impl anon_4 {
    pub unsafe fn copy_from(this: *mut anon_4, _a0: *const anon_4) {
        (&raw mut (*this).j).write((*_a0).j);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_5 {
    pub k: i32,
}
impl anon_5 {
    pub unsafe fn copy_from(this: *mut anon_5, _a0: *const anon_5) {
        (&raw mut (*this).k).write((*_a0).k);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_3 {
    pub i: i32,
    pub inner_named: anon_4,
    pub anon_5: anon_5,
}
impl anon_3 {
    pub unsafe fn copy_from(this: *mut anon_3, _a0: *const anon_3) {
        (&raw mut (*this).i).write((*_a0).i);
        anon_4::copy_from((&raw mut (*this).inner_named), { &(*_a0).inner_named });
        anon_5::copy_from((&raw mut (*this).anon_5), { &(*_a0).anon_5 });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer {
    pub named: Outer_Named,
    pub anonymous_named_0: anon_0,
    pub anonymous_named_1: anon_1,
    pub anon_2: anon_2,
    pub anon_3: anon_3,
}
impl Outer {
    pub unsafe fn copy_from(this: *mut Outer, _a0: *const Outer) {
        Outer_Named::copy_from((&raw mut (*this).named), { &(*_a0).named });
        anon_0::copy_from((&raw mut (*this).anonymous_named_0), {
            &(*_a0).anonymous_named_0
        });
        anon_1::copy_from((&raw mut (*this).anonymous_named_1), {
            &(*_a0).anonymous_named_1
        });
        anon_2::copy_from((&raw mut (*this).anon_2), { &(*_a0).anon_2 });
        anon_3::copy_from((&raw mut (*this).anon_3), { &(*_a0).anon_3 });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut o: Outer = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut o;
        {
            let __d = (&raw mut (*__d).named);
            (&raw mut (*__d).a).write(0_i32);
            (&raw mut (*__d).b).write(0_i32);
        };
        {
            let __d = (&raw mut (*__d).anonymous_named_0);
            (&raw mut (*__d).c).write(0_i32);
            (&raw mut (*__d).d).write(0_i32);
        };
        {
            let __d = (&raw mut (*__d).anonymous_named_1);
            (&raw mut (*__d).g).write(0_i32);
            (&raw mut (*__d).h).write(0_i32);
        };
        {
            let __d = (&raw mut (*__d).anon_2);
            (&raw mut (*__d).e).write(0_i32);
            (&raw mut (*__d).f).write(0_i32);
        };
        {
            let __d = (&raw mut (*__d).anon_3);
            (&raw mut (*__d).i).write(0_i32);
            {
                let __d = (&raw mut (*__d).inner_named);
                (&raw mut (*__d).j).write(0_i32);
            };
            {
                let __d = (&raw mut (*__d).anon_5);
                (&raw mut (*__d).k).write(0_i32);
            };
        };
    };
    o.named.a = 1;
    o.named.b = 2;
    o.anonymous_named_0.c = 3;
    o.anonymous_named_0.d = 4;
    o.anonymous_named_1.g = 5;
    o.anonymous_named_1.h = 6;
    o.anon_2.e = 7;
    o.anon_2.f = 8;
    o.anon_3.i = 9;
    o.anon_3.inner_named.j = 10;
    o.anon_3.anon_5.k = 11;
    assert!(((o.named.a) == (1)));
    assert!(((o.named.b) == (2)));
    assert!(((o.anonymous_named_0.c) == (3)));
    assert!(((o.anonymous_named_0.d) == (4)));
    assert!(((o.anonymous_named_1.g) == (5)));
    assert!(((o.anonymous_named_1.h) == (6)));
    assert!(((o.anon_2.e) == (7)));
    assert!(((o.anon_2.f) == (8)));
    assert!(((o.anon_3.i) == (9)));
    assert!(((o.anon_3.inner_named.j) == (10)));
    assert!(((o.anon_3.anon_5.k) == (11)));
    let mut s: anon_6 = ::std::mem::MaybeUninit::uninit().assume_init();
    anon_6::new_1(&raw mut s);
    s.x = 1;
    s.z = 2;
    assert!(
        ({
            s.x = 1;
            s.x
        } != 0)
    );
    assert!(
        ({
            s.z = 2;
            s.z
        } != 0)
    );
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_6 {
    pub x: i32,
    pub z: i32,
}
impl anon_6 {
    pub unsafe fn new_1(this: *mut anon_6) {}
    pub unsafe fn copy_from(this: *mut anon_6, _a0: *const anon_6) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).z).write((*_a0).z);
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
