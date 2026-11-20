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
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub unsafe fn new_1(this: *mut Point) {}
    pub unsafe fn copy_from(this: *mut Point, _a0: *const Point) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).y).write((*_a0).y);
    }
    pub unsafe fn move_from(this: *mut Point, _a0: *mut Point) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).y).write((*_a0).y);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Shape {
    pub id: i32,
    pub coords: [i32; 4],
    pub points: [Point; 3],
    pub tail: i32,
}
impl Shape {
    pub unsafe fn new_1(this: *mut Shape) {
        {
            let __d = (&raw mut (*this).points).cast::<Point>();
            for __i in 0..3_isize {
                Point::new_1(__d.offset(__i));
            }
        };
    }
    pub unsafe fn copy_from(this: *mut Shape, _a0: *const Shape) {
        (&raw mut (*this).id).write((*_a0).id);
        (&raw mut (*this).coords).write(std::array::from_fn::<_, 4, _>(|__i: usize| {
            (*_a0).coords[(__i)]
        }));
        {
            let __d = (&raw mut (*this).points).cast::<Point>();
            for __i in 0..3_usize {
                Point::copy_from(__d.offset(__i as isize), { &(*_a0).points[(__i)] });
            }
        };
        (&raw mut (*this).tail).write((*_a0).tail);
    }
    pub unsafe fn move_from(this: *mut Shape, _a0: *mut Shape) {
        (&raw mut (*this).id).write((*_a0).id);
        (&raw mut (*this).coords).write(std::array::from_fn::<_, 4, _>(|__i: usize| {
            (*_a0).coords[(__i)]
        }));
        {
            let __d = (&raw mut (*this).points).cast::<Point>();
            for __i in 0..3_usize {
                Point::move_from(__d.offset(__i as isize), { &mut (*_a0).points[(__i)] });
            }
        };
        (&raw mut (*this).tail).write((*_a0).tail);
    }
}
pub unsafe fn sum_0(mut p: *const i32, mut n: i32) -> i32 {
    let mut s: i32 = 0;
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (n)) {
            s += (*p.offset((i) as isize));
            i.prefix_inc();
        }
    }
    return s;
}
pub unsafe fn set_y_1(mut p: *mut Point, mut y: i32) {
    (*p).y = y;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: Shape = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut s;
        (&raw mut (*__d).id).write(1);
        (&raw mut (*__d).coords).write([10, 20, 30, 40]);
        {
            let __d = (&raw mut (*__d).points).cast::<Point>();
            {
                let __d = __d.offset(0);
                (&raw mut (*__d).x).write(1);
                (&raw mut (*__d).y).write(2);
            };
            {
                let __d = __d.offset(1);
                (&raw mut (*__d).x).write(3);
                (&raw mut (*__d).y).write(4);
            };
            {
                let __d = __d.offset(2);
                (&raw mut (*__d).x).write(5);
                (&raw mut (*__d).y).write(6);
            };
        };
        (&raw mut (*__d).tail).write(99);
    };
    let mut c: *mut i32 = (&mut s.coords[(1) as usize] as *mut i32);
    assert!(((*c) == (20)));
    (*c) = 21;
    assert!(((s.coords[(1) as usize]) == (21)));
    c = (c).wrapping_add((2 as i32) as usize);
    assert!(((*c) == (40)));
    assert!(
        ((((c as usize - (&mut s.coords[(0) as usize] as *mut i32) as usize)
            / ::std::mem::size_of::<i32>()) as i64)
            == (3_i64))
    );
    assert!(((*c.offset((-1_i32) as isize)) == (30)));
    assert!(((unsafe { sum_0((s.coords.as_mut_ptr()).cast_const(), 4,) }) == (101)));
    assert!(
        ((unsafe { sum_0((&mut s.coords[(2) as usize] as *mut i32).cast_const(), 2,) }) == (70))
    );
    let mut p: *mut Point = (&mut s.points[(1) as usize] as *mut Point);
    assert!((((*p).x) == (3)));
    (unsafe { set_y_1(p.offset((1) as isize), 60) });
    assert!(((s.points[(2) as usize].y) == (60)));
    let mut py: *mut i32 = (&mut s.points[(0) as usize].y as *mut i32);
    (*py) = 7;
    assert!(((s.points[(0) as usize].y) == (7)));
    let mut px: *mut i32 = (&mut (*(p.offset((1) as isize))).x as *mut i32);
    (*px) += 50;
    assert!(((s.points[(2) as usize].x) == (55)));
    let mut sp: *mut Shape = (&mut s as *mut Shape);
    let mut d: *mut i32 = (*sp).coords.as_mut_ptr().offset((3) as isize);
    (*d) = 41;
    assert!(((s.coords[(3) as usize]) == (41)));
    (*sp).points[(1) as usize].y = (((*sp).coords[(0) as usize]) + ((*sp).points[(0) as usize].x));
    assert!(((s.points[(1) as usize].y) == (11)));
    let mut q: *mut Point = (*sp).points.as_mut_ptr();
    (*q.offset((2) as isize)).x = 8;
    assert!((((*sp).points[(2) as usize].x) == (8)));
    let mut t: Shape = ::std::mem::MaybeUninit::uninit().assume_init();
    Shape::copy_from(&raw mut t, { &s });
    t.coords[(0) as usize] = 0;
    t.points[(0) as usize].x = 0;
    assert!(((s.coords[(0) as usize]) == (10)) && ((s.points[(0) as usize].x) == (1)));
    assert!(
        (((t.coords[(1) as usize]) == (21)) && ((t.points[(2) as usize].y) == (60)))
            && ((t.tail) == (99))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
