extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl Point {
    pub fn new(this: Ptr<Point>, mut x: i32, mut y: i32) {
        field!(this, x).write(x);
        field!(this, y).write(y);
    }
    pub fn copy_from(this: Ptr<Point>, _a0: Ptr<Point>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
        field!(this, y).write(_a0.with(|__s| __s.y));
    }
    pub fn move_from(this: Ptr<Point>, _a0: Ptr<Point>) {
        field!(this, x).write({ (*_a0.upgrade().deref()).x });
        field!(this, y).write({ (*_a0.upgrade().deref()).y });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Option<Rc<dyn std::any::Any>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            Rc::try_unwrap({
                let __v: Value<Point> = Rc::new(RefCell::new(<Point>::uninit()));
                Point::new(__v.as_pointer(), { 3 }, { 4 });
                __v
            })
            .ok()
            .unwrap()
            .into_inner(),
        )))));
    assert!(({ (*a.borrow()).is_some() }));
    let mut p: Ptr<Point> = {
        let mut __tmp_0 = { (a.as_pointer()) };
        if __tmp_0.is_null() {
            Ptr::null()
        } else {
            __tmp_0.with(|__a: &Option<Rc<dyn std::any::Any>>| {
                match __a.clone().map(|__r| __r.downcast::<RefCell<Point>>()) {
                    Some(Ok(__v)) => __v.as_pointer(),
                    _ => Ptr::null(),
                }
            })
        }
    };
    assert!(!((p).is_null()));
    assert!((p.with(|__s| __s.x) == 3));
    assert!((p.with(|__s| __s.y) == 4));
    assert!(
        ({
            let mut __tmp_1 = { (a.as_pointer()) };
            if __tmp_1.is_null() {
                Ptr::null()
            } else {
                __tmp_1.with(|__a: &Option<Rc<dyn std::any::Any>>| {
                    match __a.clone().map(|__r| __r.downcast::<RefCell<i32>>()) {
                        Some(Ok(__v)) => __v.as_pointer(),
                        _ => Ptr::null(),
                    }
                })
            }
        })
        .is_null()
    );
    let q: Ptr<Point> = {
        let mut __tmp_2 = {
            Rc::try_unwrap({
                let __v: Value<Point> = Rc::new(RefCell::new(<Point>::uninit()));
                Point::new(__v.as_pointer(), { 5 }, { 6 });
                __v
            })
            .ok()
            .unwrap()
            .into_inner()
        };
        let __value: Value<Point> = Rc::new(RefCell::new(__tmp_2));
        let __ptr = __value.as_pointer();
        (*a.borrow_mut()) = Some(__value);
        __ptr
    };
    assert!((q.with(|__s| __s.x) == 5));
    assert!(
        ({
            let mut __tmp_3 = { (a.as_pointer()) };
            if __tmp_3.is_null() {
                Ptr::null()
            } else {
                __tmp_3.with(|__a: &Option<Rc<dyn std::any::Any>>| {
                    match __a.clone().map(|__r| __r.downcast::<RefCell<Point>>()) {
                        Some(Ok(__v)) => __v.as_pointer(),
                        _ => Ptr::null(),
                    }
                })
            }
        }
        .with(|__s| __s.y)
            == 6)
    );
    let b: Value<Option<Rc<dyn std::any::Any>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(7)))));
    assert!(
        (({
            let mut __tmp_4 = { (b.as_pointer()) };
            if __tmp_4.is_null() {
                Ptr::null()
            } else {
                __tmp_4.with(|__a: &Option<Rc<dyn std::any::Any>>| {
                    match __a.clone().map(|__r| __r.downcast::<RefCell<i32>>()) {
                        Some(Ok(__v)) => __v.as_pointer(),
                        _ => Ptr::null(),
                    }
                })
            }
        }
        .read())
            == 7)
    );
    (*b.borrow_mut()) = None;
    assert!(({ !((*b.borrow()).is_some()) }));
    assert!(
        ({
            let mut __tmp_5 = { (b.as_pointer()) };
            if __tmp_5.is_null() {
                Ptr::null()
            } else {
                __tmp_5.with(|__a: &Option<Rc<dyn std::any::Any>>| {
                    match __a.clone().map(|__r| __r.downcast::<RefCell<i32>>()) {
                        Some(Ok(__v)) => __v.as_pointer(),
                        _ => Ptr::null(),
                    }
                })
            }
        })
        .is_null()
    );
    let mut n: Ptr<Option<Rc<dyn std::any::Any>>> = Ptr::<Option<Rc<dyn std::any::Any>>>::null();
    assert!(
        (if n.is_null() {
            Ptr::null()
        } else {
            n.with(|__a: &Option<Rc<dyn std::any::Any>>| {
                match __a.clone().map(|__r| __r.downcast::<RefCell<i32>>()) {
                    Some(Ok(__v)) => __v.as_pointer(),
                    _ => Ptr::null(),
                }
            })
        })
        .is_null()
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
