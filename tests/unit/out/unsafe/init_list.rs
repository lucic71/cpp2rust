extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn f_0(mut list: Vec<i32>) {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Pair {
    pub a: i32,
    pub b: i32,
}
impl Pair {
    pub unsafe fn copy_from(this: *mut Pair, _a0: *const Pair) {
        (&raw mut (*this).a).write((*_a0).a);
        (&raw mut (*this).b).write((*_a0).b);
    }
    pub unsafe fn move_from(this: *mut Pair, _a0: *mut Pair) {
        (&raw mut (*this).a).write((*_a0).a);
        (&raw mut (*this).b).write((*_a0).b);
    }
    pub unsafe fn new_1(this: *mut Pair) {}
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Holder {
    pub p: Pair,
}
impl Holder {
    pub unsafe fn operator_assign_1(&mut self, o: *mut Pair) -> *mut Holder {
        self.p = (*o);
        return &mut (*(self as *mut Holder));
    }
    pub unsafe fn new_1(this: *mut Holder) {
        {
            let __d = (&raw mut (*this).p);
            (&raw mut (*__d).a).write(0);
            (&raw mut (*__d).b).write(0);
        };
    }
    pub unsafe fn copy_from(this: *mut Holder, _a0: *const Holder) {
        Pair::copy_from((&raw mut (*this).p), { &(*_a0).p });
    }
    pub unsafe fn move_from(this: *mut Holder, _a0: *mut Holder) {
        Pair::move_from((&raw mut (*this).p), { &mut (*_a0).p });
    }
}
impl Default for Holder {
    fn default() -> Self {
        unsafe {
            Holder {
                p: Pair { a: 0, b: 0 },
            }
        }
    }
}
pub unsafe fn sum_1(p: *const Pair) -> i32 {
    return (((*p).a) + ((*p).b));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Seven {
    pub v: i32,
}
impl Seven {
    pub unsafe fn new_1(this: *mut Seven) {
        (&raw mut (*this).v).write(7);
    }
    pub unsafe fn copy_from(this: *mut Seven, _a0: *const Seven) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut Seven, _a0: *mut Seven) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
impl Default for Seven {
    fn default() -> Self {
        unsafe { Seven { v: 7 } }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut i1: i32 = 3;
    let mut i2: i32 = 0_i32;
    let mut carr1: [i32; 2] = [1, 2];
    let mut carr2: [i32; 3] = [1, 0_i32, 0_i32];
    let mut arr: [i32; 3] = [1, 2, 3];
    let mut vec_: Vec<i32> = Vec::from([1, 2, 3]);
    (unsafe { f_0(Vec::from([1, 2, 3, 4])) });
    let mut p: Pair = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut p;
        (&raw mut (*__d).a).write(1);
        (&raw mut (*__d).b).write(2);
    };
    let mut h: Holder = ::std::mem::MaybeUninit::uninit().assume_init();
    Holder::new_1(&raw mut h);
    (unsafe { Holder::operator_assign_1(&mut h, &mut p) });
    assert!(((h.p.a) == (1)) && ((h.p.b) == (2)));
    assert!(((unsafe { sum_1(&p,) }) == (3)));
    let mut sevens: [Seven; 4] = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = (&raw mut sevens).cast::<Seven>();
        {
            let __d = __d.offset(0);
            (&raw mut (*__d).v).write(7);
        };
        {
            let __d = __d.offset(1);
            (&raw mut (*__d).v).write(7);
        };
        {
            let __d = __d.offset(2);
            (&raw mut (*__d).v).write(7);
        };
        {
            let __d = __d.offset(3);
            (&raw mut (*__d).v).write(7);
        };
    };
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (4)) {
            assert!(((sevens[(i) as usize].v) == (7)));
            i.postfix_inc();
        }
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
