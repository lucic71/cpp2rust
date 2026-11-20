extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    let mut source: i32 = 1;
    let mut target: i32 = source;
    assert!(((target) == (1)));
    assert!(((source) == (1)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p9_move_trivially_copyable_Point {
    pub x: i32,
    pub y: i32,
}
impl p9_move_trivially_copyable_Point {
    pub unsafe fn new_1(this: *mut p9_move_trivially_copyable_Point) {}
    pub unsafe fn copy_from(
        this: *mut p9_move_trivially_copyable_Point,
        _a0: *const p9_move_trivially_copyable_Point,
    ) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).y).write((*_a0).y);
    }
    pub unsafe fn move_from(
        this: *mut p9_move_trivially_copyable_Point,
        _a0: *mut p9_move_trivially_copyable_Point,
    ) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).y).write((*_a0).y);
    }
}
pub unsafe fn test_1() {
    let mut source: p9_move_trivially_copyable_Point =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut source;
        (&raw mut (*__d).x).write(1);
        (&raw mut (*__d).y).write(2);
    };
    let mut target: p9_move_trivially_copyable_Point =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p9_move_trivially_copyable_Point::move_from(&raw mut target, { &mut source });
    assert!(((target.x) == (1)) && ((target.y) == (2)));
    assert!(((source.x) == (1)) && ((source.y) == (2)));
    let mut assigned: p9_move_trivially_copyable_Point =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut assigned;
        (&raw mut (*__d).x).write(0);
        (&raw mut (*__d).y).write(0);
    };
    let mut other: p9_move_trivially_copyable_Point =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut other;
        (&raw mut (*__d).x).write(3);
        (&raw mut (*__d).y).write(4);
    };
    assigned = other;
    assert!(((assigned.x) == (3)) && ((assigned.y) == (4)));
    assert!(((other.x) == (3)) && ((other.y) == (4)));
    let mut points: Vec<p9_move_trivially_copyable_Point> = Vec::new();
    let mut element: p9_move_trivially_copyable_Point =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut element;
        (&raw mut (*__d).x).write(5);
        (&raw mut (*__d).y).write(6);
    };
    {
        let __a1 = element;
        points.push(__a1)
    };
    assert!(((points[(0_usize)].x) == (5)) && ((points[(0_usize)].y) == (6)));
    assert!(((element.x) == (5)) && ((element.y) == (6)));
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct p10_move_constructor_A {
    pub value: i32,
}
impl p10_move_constructor_A {
    pub unsafe fn new(this: *mut p10_move_constructor_A, mut value: i32) {
        (&raw mut (*this).value).write(value);
    }
    pub unsafe fn move_from(this: *mut p10_move_constructor_A, other: *mut p10_move_constructor_A) {
        (&raw mut (*this).value).write((*other).value);
        (*other).value = 0;
    }
}
pub unsafe fn test_2() {
    let mut source: p10_move_constructor_A = ::std::mem::MaybeUninit::uninit().assume_init();
    p10_move_constructor_A::new(&raw mut source, { 1 });
    let mut target: p10_move_constructor_A = ::std::mem::MaybeUninit::uninit().assume_init();
    p10_move_constructor_A::move_from(&raw mut target, { &mut source });
    assert!(((target.value) == (1)));
    assert!(((source.value) == (0)));
    let mut values: Vec<p10_move_constructor_A> = Vec::new();
    let mut element: p10_move_constructor_A = ::std::mem::MaybeUninit::uninit().assume_init();
    p10_move_constructor_A::new(&raw mut element, { 2 });
    {
        let __a1 = {
            let mut __v: p10_move_constructor_A = ::std::mem::MaybeUninit::uninit().assume_init();
            p10_move_constructor_A::move_from(&raw mut __v, { &mut element });
            __v
        };
        values.push(__a1)
    };
    assert!(((values[(0_usize)].value) == (2)));
    assert!(((element.value) == (0)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    (unsafe { test_1() });
    (unsafe { test_2() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
