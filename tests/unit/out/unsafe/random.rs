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
pub struct Pair {
    pub x: i32,
    pub y: i32,
    pub a: [i32; 5],
    pub r: *mut i32,
    pub p: *mut i32,
    pub pair: *mut Pair,
    pub ap: [*mut i32; 2],
}
impl Pair {
    pub unsafe fn method(&mut self) {
        self.x.postfix_inc();
        self.y.prefix_inc();
        self.a[(4) as usize] = 1;
        (*self.r) = 1;
        self.p = std::ptr::null_mut();
        self.pair = std::ptr::null_mut();
        self.ap[(0) as usize] = std::ptr::null_mut();
    }
    pub unsafe fn as_val(&mut self) -> i32 {
        return self.x;
    }
    pub unsafe fn as_ref(&mut self) -> *mut i32 {
        return &mut self.x;
    }
    pub unsafe fn as_ptr(&mut self) -> *mut i32 {
        return (&mut self.x as *mut i32);
    }
    pub unsafe fn copy_from(this: *mut Pair, _a0: *const Pair) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).y).write((*_a0).y);
        (&raw mut (*this).a).write(std::array::from_fn::<_, 5, _>(|__i: usize| (*_a0).a[(__i)]));
        (&raw mut (*this).r).write((*_a0).r);
        (&raw mut (*this).p).write((*_a0).p);
        (&raw mut (*this).pair).write((*_a0).pair);
        (&raw mut (*this).ap).write(std::array::from_fn::<_, 2, _>(|__i: usize| {
            (*_a0).ap[(__i)]
        }));
    }
}
pub unsafe fn zero_0() -> i32 {
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct X1 {}
impl X1 {
    pub unsafe fn copy_from(this: *mut X1, _a0: *const X1) {}
}
pub unsafe fn foo_1(mut x1: i32, x2: *mut i32, mut x3: *mut i32, p2: *mut Pair, mut p3: *mut Pair) {
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x1: i32 = 1;
    let mut c1: i32 = x1;
    let rx1: *mut i32 = &mut x1;
    let mut px1: *mut i32 = (&mut x1 as *mut i32);
    let mut x2: i32 = (*rx1);
    let rx2: *mut i32 = rx1;
    let mut px2: *mut i32 = (rx1);
    let mut x3: i32 = (*px1);
    let rx3: *mut i32 = &mut (*px1);
    let mut px3: *mut i32 = px1;
    let mut res: i32 = ((x1) + (x2));
    res = ((x1) + (x2));
    let mut y1: Pair = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut y1;
        (&raw mut (*__d).x).write(1);
        (&raw mut (*__d).y).write(2);
        (&raw mut (*__d).a).write([1, 2, 3, 4, 5]);
        (&raw mut (*__d).r).write(&mut x1);
        (&raw mut (*__d).p).write(std::ptr::null_mut());
        (&raw mut (*__d).pair).write(std::ptr::null_mut());
        (&raw mut (*__d).ap).write([std::ptr::null_mut(), std::ptr::null_mut()]);
    };
    let mut y4: Pair = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut y4;
        (&raw mut (*__d).x).write(y1.x);
        (&raw mut (*__d).y).write(y1.y);
        (&raw mut (*__d).a).write([
            y1.a[(0) as usize],
            y1.a[(1) as usize],
            y1.a[(2) as usize],
            y1.a[(3) as usize],
            y1.a[(4) as usize],
        ]);
        (&raw mut (*__d).r).write(y1.r);
        (&raw mut (*__d).p).write(y1.p);
        (&raw mut (*__d).pair).write(y1.pair);
        (&raw mut (*__d).ap).write([y1.ap[(0) as usize], y1.ap[(1) as usize]]);
    };
    let ry1: *mut Pair = &mut y1;
    let mut py1: *mut Pair = (&mut y1 as *mut Pair);
    let mut y2: Pair = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut y2;
        (&raw mut (*__d).x).write((*ry1).x);
        (&raw mut (*__d).y).write((*ry1).y);
        (&raw mut (*__d).a).write([
            (*ry1).a[(0) as usize],
            (*ry1).a[(1) as usize],
            (*ry1).a[(2) as usize],
            (*ry1).a[(3) as usize],
            (*ry1).a[(4) as usize],
        ]);
        (&raw mut (*__d).r).write((*ry1).r);
        (&raw mut (*__d).p).write((*ry1).p);
        (&raw mut (*__d).pair).write((*ry1).pair);
        (&raw mut (*__d).ap).write([(*ry1).ap[(0) as usize], (*ry1).ap[(1) as usize]]);
    };
    let ry2: *mut Pair = ry1;
    let mut py2: *mut Pair = (ry1);
    let mut y3: Pair = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut y3;
        (&raw mut (*__d).x).write((*py1).x);
        (&raw mut (*__d).y).write((*py1).y);
        (&raw mut (*__d).a).write([
            (*py1).a[(0) as usize],
            (*py1).a[(1) as usize],
            (*py1).a[(2) as usize],
            (*py1).a[(3) as usize],
            (*py1).a[(4) as usize],
        ]);
        (&raw mut (*__d).r).write((*py1).r);
        (&raw mut (*__d).p).write((*py1).p);
        (&raw mut (*__d).pair).write((*py1).pair);
        (&raw mut (*__d).ap).write([(*py1).ap[(0) as usize], (*py1).ap[(1) as usize]]);
    };
    let ry3: *mut Pair = &mut (*py1);
    let mut py3: *mut Pair = py1;
    py3 = std::ptr::null_mut();
    let mut ptr2pair: *mut Pair = py3;
    (unsafe {
        let _x1: i32 = x1;
        let _x2: *mut i32 = &mut x1;
        let _x3: *mut i32 = (&mut x1 as *mut i32);
        let _p2: *mut Pair = &mut y1;
        let _p3: *mut Pair = (&mut y1 as *mut Pair);
        foo_1(_x1, _x2, _x3, _p2, _p3)
    });
    (unsafe {
        let _x1: i32 = (*rx1);
        let _x2: *mut i32 = rx1;
        let _x3: *mut i32 = (rx1);
        let _p2: *mut Pair = ry1;
        let _p3: *mut Pair = (ry1);
        foo_1(_x1, _x2, _x3, _p2, _p3)
    });
    (unsafe {
        let _x1: i32 = (*px1);
        let _x2: *mut i32 = &mut (*px1);
        let _x3: *mut i32 = px1;
        let _p2: *mut Pair = &mut (*py1);
        let _p3: *mut Pair = py1;
        foo_1(_x1, _x2, _x3, _p2, _p3)
    });
    let cr1: *mut i32 = &mut c1;
    let mut cp1: *mut i32 = (&mut c1 as *mut i32);
    x1 = c1;
    x1 = 1;
    x1 = (*cr1);
    x1 = (*cp1);
    (*rx1) = c1;
    (*rx2) = (*cr1);
    (*rx3) = (*cp1);
    (*px1) = c1;
    (*px2) = (*cr1);
    (*px3) = (*cp1);
    px1 = (&mut c1 as *mut i32);
    px2 = (cr1);
    px3 = cp1;
    y1.x = 2;
    y1.y = 3;
    y1.a[(0) as usize] = 100;
    (*y1.r) = 10;
    y1.p = px3;
    px3 = px2;
    y1.pair = (&mut y3 as *mut Pair);
    (*y1.pair).x = 100;
    (*y1.pair).pair = (&mut y2 as *mut Pair);
    (*(*y1.pair).pair).x = 100;
    y1.ap[(0) as usize] = (&mut x1 as *mut i32);
    y1.ap[(1) as usize] = (&mut x2 as *mut i32);
    (*y1.ap[(0) as usize]) = 0;
    c1 = ((x1) + (1));
    let mut j: i32 = 0;
    let mut new_y: Pair = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut new_y;
        (&raw mut (*__d).x).write(1);
        (&raw mut (*__d).y).write(2);
        (&raw mut (*__d).a).write([1, 2, 3, 4, 5]);
        (&raw mut (*__d).r).write(&mut j);
        (&raw mut (*__d).p).write(std::ptr::null_mut());
        (&raw mut (*__d).pair).write(std::ptr::null_mut());
        (&raw mut (*__d).ap).write([std::ptr::null_mut(), std::ptr::null_mut()]);
    };
    y1.x = new_y.x;
    let mut i: u32 = 1_u32;
    y1.a[(i) as usize] = -1_i32;
    x1.postfix_inc();
    x1.prefix_inc();
    y1.x.postfix_inc();
    (*y1.pair).pair = (&mut y2 as *mut Pair);
    (*(*y1.pair).pair).x = 10;
    (unsafe { Pair::method(&mut y1) });
    y1.pair = (&mut y2 as *mut Pair);
    y2.pair = (&mut y3 as *mut Pair);
    (unsafe { Pair::method(&mut (*(*y1.pair).pair)) });
    let mut x: X1 = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut x;
    };
    let mut y: X1 = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut y;
    };
    x1 = ((unsafe { zero_0() }) + (y1.x));
    y1.x = ((unsafe { zero_0() }) + (5));
    let mut ptr2ptr_1: *mut *mut i32 = (&mut px1 as *mut *mut i32);
    let mut ptr2ptr_2: *mut *mut Pair = (&mut py1 as *mut *mut Pair);
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
