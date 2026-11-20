extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static total_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Destructor, Default)]
#[placement]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: i32,
}
impl S {
    pub fn new(this: Ptr<S>, mut init: i32) {
        field!(this, v).write(init);
        ({ SImpl::mut_method(&this) });
        total_0.with(|rc| *rc.borrow_mut() += ({ SImpl::const_method(&this) }));
    }
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom)]
#[placement]
#[cpp_from(i32, new_1, value)]
#[byte_size(8)]
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl Point {
    pub fn new_2(this: Ptr<Point>, mut x: i32, mut y: i32) {
        field!(this, x).write(x);
        field!(this, y).write(y);
    }
    pub fn new_1(this: Ptr<Point>, mut v: i32) {
        Point::new_2(this.clone(), { v }, { (v + 1) });
        {
            field!(this, y).with_mut(|__v| *__v = *__v * 10)
        };
    }
    pub fn new_3(this: Ptr<Point>) {
        Point::new_1(this.clone(), { 4 });
        {
            field!(this, x).with_mut(|__v| *__v = *__v + 100)
        };
    }
    pub fn copy_from(this: Ptr<Point>, _a0: Ptr<Point>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
        field!(this, y).write(_a0.with(|__s| __s.y));
    }
}
impl Default for Point {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<Point> = Rc::new(RefCell::new(<Point>::uninit()));
            Point::new_3(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    {
        let s: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
        S::new(s.as_pointer(), { 3 });
        let _dtor_s = std::mem::DropGuard::new(s.clone(), |__p| __p.as_pointer().destructor());
        assert!(({ ({ (*s.borrow()).v } == 4) }));
        assert!((total_0.with(|rc| *rc.borrow()) == 8));
    }
    assert!((total_0.with(|rc| *rc.borrow()) == 18));
    let p: Value<Point> = Rc::new(RefCell::new(<Point>::uninit()));
    Point::new_3(p.as_pointer());
    assert!(({ ({ (*p.borrow()).x } == 104) }));
    assert!(({ ({ (*p.borrow()).y } == 50) }));
    let q: Value<Point> = Rc::new(RefCell::new(<Point>::uninit()));
    Point::new_1(q.as_pointer(), { 7 });
    assert!(({ ({ (*q.borrow()).x } == 7) }));
    assert!(({ ({ (*q.borrow()).y } == 80) }));
    return 0;
}
pub trait SImpl {
    fn const_method(&self) -> i32;
    fn mut_method(&self);
    fn destructor(&self);
}
impl SImpl for Ptr<S> {
    fn const_method(&self) -> i32 {
        return ((*self).with(|__s| __s.v) * 2);
    }
    fn mut_method(&self) {
        {
            field!((*self), v).with_mut(|__v| *__v = *__v + 1)
        };
    }
    fn destructor(&self) {
        ({ SImpl::mut_method(self) });
        total_0.with(|rc| *rc.borrow_mut() += ({ SImpl::const_method(self) }));
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = total_0.with(|_| ());
}
