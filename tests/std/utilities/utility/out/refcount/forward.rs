extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    let mut source: i32 = 1;
    let mut target: i32 = source;
    assert!((target == 1));
    assert!((source == 1));
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct p9_move_trivially_copyable_Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
pub fn test_1() {
    let mut source: p9_move_trivially_copyable_Point =
        p9_move_trivially_copyable_Point { x: 1, y: 2 };
    let mut target: p9_move_trivially_copyable_Point = source;
    assert!((target.x == 1) && (target.y == 2));
    assert!((source.x == 1) && (source.y == 2));
    let mut assigned: p9_move_trivially_copyable_Point =
        p9_move_trivially_copyable_Point { x: 0, y: 0 };
    let mut other: p9_move_trivially_copyable_Point =
        p9_move_trivially_copyable_Point { x: 3, y: 4 };
    assigned = other;
    assert!((assigned.x == 3) && (assigned.y == 4));
    assert!((other.x == 3) && (other.y == 4));
    let mut points: Vec<p9_move_trivially_copyable_Point> = Vec::new();
    let mut element: p9_move_trivially_copyable_Point =
        p9_move_trivially_copyable_Point { x: 5, y: 6 };
    {
        let __a1 = element;
        points.push(__a1)
    };
    assert!((points[0_usize].x == 5) && (points[0_usize].y == 6));
    assert!((element.x == 5) && (element.y == 6));
}
#[derive(Record, ByteRepr, Uninit, FnPtrArg, MoveCtor, CppFrom, Default)]
#[placement]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct p10_move_constructor_A {
    #[offset(0)]
    pub value: i32,
}
impl p10_move_constructor_A {
    pub fn new(this: Ptr<p10_move_constructor_A>, mut value: i32) {
        field!(this, value).write(value);
    }
    pub fn move_from(this: Ptr<p10_move_constructor_A>, other: Ptr<p10_move_constructor_A>) {
        field!(this, value).write(other.with(|__s| __s.value));
        field!(other, value).write(0);
    }
}
pub fn test_2() {
    let source: Value<p10_move_constructor_A> =
        Rc::new(RefCell::new(<p10_move_constructor_A>::uninit()));
    p10_move_constructor_A::new(source.as_pointer(), { 1 });
    let target: Value<p10_move_constructor_A> =
        Rc::new(RefCell::new(<p10_move_constructor_A>::uninit()));
    p10_move_constructor_A::move_from(target.as_pointer(), { source.as_pointer() });
    assert!(({ ({ (*target.borrow()).value } == 1) }));
    assert!(({ ({ (*source.borrow()).value } == 0) }));
    let mut values: Vec<p10_move_constructor_A> = Vec::new();
    let element: Value<p10_move_constructor_A> =
        Rc::new(RefCell::new(<p10_move_constructor_A>::uninit()));
    p10_move_constructor_A::new(element.as_pointer(), { 2 });
    {
        let __a1 = Rc::try_unwrap({
            let __v: Value<p10_move_constructor_A> =
                Rc::new(RefCell::new(<p10_move_constructor_A>::uninit()));
            p10_move_constructor_A::move_from(__v.as_pointer(), { element.as_pointer() });
            __v
        })
        .ok()
        .unwrap()
        .into_inner();
        values.push(__a1)
    };
    assert!((values[0_usize].value == 2));
    assert!(({ ({ (*element.borrow()).value } == 0) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    ({ test_1() });
    ({ test_2() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
