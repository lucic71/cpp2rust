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
pub struct S {
    pub head: i32,
    pub tail: [i32; 3],
    pub buf: [libc::c_char; 4],
}
pub static mut s_0: libcc2rs::LazyStorage<S> = libcc2rs::LazyStorage::new(|__p| unsafe {
    __p.write(S {
        head: 5,
        tail: [0; 3],
        buf: [0; 4],
    })
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct P {
    pub x: i32,
    pub y: i32,
}
pub static mut g_1: libcc2rs::LazyStorage<[i32; 5]> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write([0_i32; 5]) });
pub static mut k_2: libcc2rs::LazyStorage<[i32; 8]> = libcc2rs::LazyStorage::new(|__p| unsafe {
    __p.write([3, 0, 4, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32])
});
pub static mut counter_3: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub unsafe fn next_counter_4() -> i32 {
    return (*(*(*&raw const counter_3).get()).prefix_inc());
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((((*(*&raw const s_0).get()).head) == (5)));
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (3)) {
            assert!((((*(*&raw const s_0).get()).tail[(i) as usize]) == (0)));
            i.postfix_inc();
        }
    }
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (4)) {
            assert!((((*(*&raw const s_0).get()).buf[(i) as usize] as i32) == (0)));
            i.postfix_inc();
        }
    }
    let mut a: [i32; 10] = [0_i32; 10];
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (10)) {
            assert!(((a[(i) as usize]) == (0)));
            i.postfix_inc();
        }
    }
    let mut seven: [i32; 4] = [7, 0_i32, 0_i32, 0_i32];
    assert!(((seven[(0) as usize]) == (7)));
    {
        let mut i: i32 = 1;
        'loop_: while ((i) < (4)) {
            assert!(((seven[(i) as usize]) == (0)));
            i.postfix_inc();
        }
    }
    let mut b: [i32; 6] = [1, 0, 2, 0_i32, 0_i32, 0_i32];
    assert!(
        ((((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (0))) && ((b[(2) as usize]) == (2)))
    );
    {
        let mut i: i32 = 3;
        'loop_: while ((i) < (6)) {
            assert!(((b[(i) as usize]) == (0)));
            i.postfix_inc();
        }
    }
    let mut d: [f64; 4] = [-0_f64, 0_f64, 0_f64, 0_f64];
    assert!((((d[(0) as usize]) == (0_f64)) && ((d[(3) as usize]) == (0_f64))));
    let mut p: [P; 4] = [
        P { x: 1, y: 2 },
        <P>::default(),
        <P>::default(),
        <P>::default(),
    ];
    assert!((((p[(0) as usize].x) == (1)) && ((p[(0) as usize].y) == (2))));
    assert!((((p[(3) as usize].x) == (0)) && ((p[(3) as usize].y) == (0))));
    let mut c: [i32; 5] = [
        (unsafe { next_counter_4() }),
        (unsafe { next_counter_4() }),
        0_i32,
        0_i32,
        0_i32,
    ];
    assert!(((((c[(0) as usize]) + (c[(1) as usize])) == (3)) && ((c[(4) as usize]) == (0))));
    let mut t: [i32; 2] = [0_i32; 2];
    assert!((((t[(0) as usize]) == (0)) && ((t[(1) as usize]) == (0))));
    let mut u: [i32; 3] = [5, 6, 0_i32];
    assert!(
        ((((u[(0) as usize]) == (5)) && ((u[(1) as usize]) == (6))) && ((u[(2) as usize]) == (0)))
    );
    assert!((((*(*&raw const g_1).get())[(4) as usize]) == (0)));
    assert!(
        ((((((*(*&raw const k_2).get())[(0) as usize]) == (3))
            && (((*(*&raw const k_2).get())[(1) as usize]) == (0)))
            && (((*(*&raw const k_2).get())[(2) as usize]) == (4)))
            && (((*(*&raw const k_2).get())[(7) as usize]) == (0)))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const s_0).get();
    (*&raw const g_1).get();
    (*&raw const k_2).get();
    (*&raw const counter_3).get();
}
