extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Handler {
    #[offset(0)]
    pub tag: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub cb: FnPtr<fn(i32) -> i32>,
}
impl Handler {
    pub fn new_1(this: Ptr<Handler>) {}
    pub fn copy_from(this: Ptr<Handler>, _a0: Ptr<Handler>) {
        field!(this, tag).write(_a0.with(|__s| __s.tag));
        field!(this, cb).write(_a0.with(|__s| __s.cb.clone()));
    }
    pub fn move_from(this: Ptr<Handler>, _a0: Ptr<Handler>) {
        field!(this, tag).write({ (*_a0.upgrade().deref()).tag });
        field!(this, cb).write({ (*_a0.upgrade().deref()).cb.clone() });
    }
}
pub fn double_it_0(mut x: i32) -> i32 {
    return (x * 2);
}
pub fn negate_1(mut x: i32) -> i32 {
    return -x;
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct S {}
impl S {
    pub fn pick_1(mut x: i32) -> i32 {
        return (x + 1);
    }
    pub fn pick_2(mut x: i64) -> i32 {
        return ((x as i32) + 2);
    }
    pub fn solo(mut x: i32) -> i32 {
        return (x + 3);
    }
    pub fn new_1(this: Ptr<S>) {}
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {}
    pub fn move_from(this: Ptr<S>, _a0: Ptr<S>) {}
}
#[derive(Record, ByteRepr, Uninit, FnPtrArg, MoveCtor, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    pub data_: i32,
}
impl MoveOnly {
    pub fn new(this: Ptr<MoveOnly>, mut data: i32) {
        field!(this, data_).write(data);
    }
    pub fn move_from(this: Ptr<MoveOnly>, x: Ptr<MoveOnly>) {
        field!(this, data_).write(x.with(|__s| __s.data_));
        field!(x, data_).write(0);
    }
}
pub fn make_move_only_2(__ret: Ptr<MoveOnly>) {
    MoveOnly::new(__ret.clone(), { 4 });
    return;
}
pub fn scale_move_only_3(__ret: Ptr<MoveOnly>, x: Value<MoveOnly>) {
    MoveOnly::new(__ret.clone(), {
        (({ MoveOnlyImpl::get(&x.as_pointer()) }) * 10)
    });
    return;
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut p1: FnPtr<fn(i32) -> i32> = (FnPtr::<fn(i32) -> i32>::new(S::pick_1));
    let mut p2: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::new(S::solo);
    assert!((({ p1.call(5,) }) == 6));
    assert!((({ p2.call(5,) }) == 8));
    assert!((({ S::pick_2(5_i64,) }) == 7));
    let h3: Value<Handler> = Rc::new(RefCell::new(<Handler>::uninit()));
    {
        let __d = h3.as_pointer();
        field!(__d, tag).write(3);
        field!(__d, cb).write((FnPtr::<fn(i32) -> i32>::new(S::pick_1)));
    };
    assert!(({ (({ { (*h3.borrow()).cb.clone() }.call(1,) }) == 2) }));
    let h1: Value<Handler> = Rc::new(RefCell::new(<Handler>::uninit()));
    {
        let __d = h1.as_pointer();
        field!(__d, tag).write(1);
        field!(__d, cb).write(FnPtr::<fn(i32) -> i32>::new(double_it_0));
    };
    let h2: Value<Handler> = Rc::new(RefCell::new(<Handler>::uninit()));
    {
        let __d = h2.as_pointer();
        field!(__d, tag).write(2);
        field!(__d, cb).write(FnPtr::<fn(i32) -> i32>::new(negate_1));
    };
    assert!(({ !(({ (*h1.borrow()).cb.clone() }).is_null()) }));
    assert!(({ (({ { (*h1.borrow()).cb.clone() }.call(5,) }) == 10) }));
    assert!(({ (({ { (*h2.borrow()).cb.clone() }.call(7,) }) == -7_i32) }));
    (*h1.borrow_mut()).cb = FnPtr::<fn(i32) -> i32>::new(negate_1);
    assert!(({ (({ { (*h1.borrow()).cb.clone() }.call(3,) }) == -3_i32) }));
    assert!(({ ({ { (*h1.borrow()).cb.clone() } } == { { (*h2.borrow()).cb.clone() } }) }));
    let mut mk: FnPtr<fn(Ptr<MoveOnly>)> = FnPtr::<fn(Ptr<MoveOnly>)>::new(make_move_only_2);
    let m: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
    ({ mk.call(m.as_pointer()) });
    assert!((({ MoveOnlyImpl::get(&m.as_pointer(),) }) == 4));
    let mut sc: FnPtr<fn(Ptr<MoveOnly>, Value<MoveOnly>)> =
        FnPtr::<fn(Ptr<MoveOnly>, Value<MoveOnly>)>::new(scale_move_only_3);
    let n: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
    ({
        sc.call(n.as_pointer(), {
            let __v: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
            MoveOnly::move_from(__v.as_pointer(), { m.as_pointer() });
            __v
        })
    });
    assert!((({ MoveOnlyImpl::get(&n.as_pointer(),) }) == 40));
    assert!((({ MoveOnlyImpl::get(&m.as_pointer(),) }) == 0));
    return 0;
}
pub trait MoveOnlyImpl {
    fn get(&self) -> i32;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.data_);
    }
}
pub fn __cpp2rust_init_globals() {}
