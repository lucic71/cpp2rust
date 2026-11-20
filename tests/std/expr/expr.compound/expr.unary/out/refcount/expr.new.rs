extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static constructed_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct p23_1_new_without_initializer_Counted {
    #[offset(0)]
    pub id: i32,
}
impl p23_1_new_without_initializer_Counted {
    pub fn new(this: Ptr<p23_1_new_without_initializer_Counted>) {
        field!(this, id).write((*(*constructed_0.with(Value::clone).borrow_mut()).prefix_inc()));
    }
    pub fn copy_from(
        this: Ptr<p23_1_new_without_initializer_Counted>,
        _a0: Ptr<p23_1_new_without_initializer_Counted>,
    ) {
        field!(this, id).write(_a0.with(|__s| __s.id));
    }
    pub fn move_from(
        this: Ptr<p23_1_new_without_initializer_Counted>,
        _a0: Ptr<p23_1_new_without_initializer_Counted>,
    ) {
        field!(this, id).write({ (*_a0.upgrade().deref()).id });
    }
}
impl Default for p23_1_new_without_initializer_Counted {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<p23_1_new_without_initializer_Counted> = Rc::new(RefCell::new(
                <p23_1_new_without_initializer_Counted>::uninit(),
            ));
            p23_1_new_without_initializer_Counted::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
pub fn test_1() {
    let mut one: Ptr<p23_1_new_without_initializer_Counted> = {
        let __p = Ptr::alloc(<p23_1_new_without_initializer_Counted>::uninit());
        p23_1_new_without_initializer_Counted::new(__p.clone());
        __p
    };
    assert!((constructed_0.with(|rc| *rc.borrow()) == 1) && (one.with(|__s| __s.id) == 1));
    one.delete();
    let mut many: Ptr<p23_1_new_without_initializer_Counted> = Ptr::alloc_array(
        (0..3_usize)
            .map(|_| <p23_1_new_without_initializer_Counted>::default())
            .collect::<Box<[p23_1_new_without_initializer_Counted]>>(),
    );
    assert!((constructed_0.with(|rc| *rc.borrow()) == 4));
    assert!(
        ((elem!(many, 0).with(|__s| __s.id) == 2) && (elem!(many, 1).with(|__s| __s.id) == 3))
            && (elem!(many, 2).with(|__s| __s.id) == 4)
    );
    many.delete();
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct p23_2_new_with_initializer_Pair {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
impl p23_2_new_with_initializer_Pair {
    pub fn new_1(this: Ptr<p23_2_new_with_initializer_Pair>) {}
    pub fn copy_from(
        this: Ptr<p23_2_new_with_initializer_Pair>,
        _a0: Ptr<p23_2_new_with_initializer_Pair>,
    ) {
        field!(this, first).write(_a0.with(|__s| __s.first));
        field!(this, second).write(_a0.with(|__s| __s.second));
    }
    pub fn move_from(
        this: Ptr<p23_2_new_with_initializer_Pair>,
        _a0: Ptr<p23_2_new_with_initializer_Pair>,
    ) {
        field!(this, first).write({ (*_a0.upgrade().deref()).first });
        field!(this, second).write({ (*_a0.upgrade().deref()).second });
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(16)]
pub struct p23_2_new_with_initializer_Tracker {
    #[offset(0)]
    #[byte_size(8)]
    pub self_: Ptr<p23_2_new_with_initializer_Tracker>,
    #[offset(8)]
    pub value: i32,
}
impl p23_2_new_with_initializer_Tracker {
    pub fn new(this: Ptr<p23_2_new_with_initializer_Tracker>, mut value: i32) {
        field!(this, self_).write((this).clone());
        field!(this, value).write(value);
    }
    pub fn copy_from(
        this: Ptr<p23_2_new_with_initializer_Tracker>,
        _a0: Ptr<p23_2_new_with_initializer_Tracker>,
    ) {
        field!(this, self_).write(_a0.with(|__s| __s.self_.clone()));
        field!(this, value).write(_a0.with(|__s| __s.value));
    }
    pub fn move_from(
        this: Ptr<p23_2_new_with_initializer_Tracker>,
        _a0: Ptr<p23_2_new_with_initializer_Tracker>,
    ) {
        field!(this, self_).write({ (*_a0.upgrade().deref()).self_.clone() });
        field!(this, value).write({ (*_a0.upgrade().deref()).value });
    }
}
pub fn test_2() {
    let mut dirty: Ptr<p23_2_new_with_initializer_Pair> = {
        let __p = Ptr::alloc(<p23_2_new_with_initializer_Pair>::uninit());
        p23_2_new_with_initializer_Pair::new_1(__p.clone());
        __p
    };
    field!(dirty, first).write(23130);
    field!(dirty, second).write(23130);
    dirty.delete();
    let mut zeroed: Ptr<p23_2_new_with_initializer_Pair> = {
        let __p = Ptr::alloc(<p23_2_new_with_initializer_Pair>::uninit());
        {
            let __d = __p.clone();
            ();
            p23_2_new_with_initializer_Pair::new_1(__d)
        };
        __p
    };
    assert!((zeroed.with(|__s| __s.first) == 0) && (zeroed.with(|__s| __s.second) == 0));
    zeroed.delete();
    let mut tracker: Ptr<p23_2_new_with_initializer_Tracker> = {
        let __p = Ptr::alloc(<p23_2_new_with_initializer_Tracker>::uninit());
        p23_2_new_with_initializer_Tracker::new(__p.clone(), { 4 });
        __p
    };
    assert!(
        ({ tracker.with(|__s| __s.self_.clone()) } == { (tracker).clone() })
            && (tracker.with(|__s| __s.value) == 4)
    );
    tracker.delete();
    let mut braces: Ptr<p23_2_new_with_initializer_Pair> = {
        let __p = Ptr::alloc(<p23_2_new_with_initializer_Pair>::uninit());
        {
            let __d = __p.clone();
            field!(__d, first).write(1);
            field!(__d, second).write(2);
        };
        __p
    };
    assert!((braces.with(|__s| __s.first) == 1) && (braces.with(|__s| __s.second) == 2));
    braces.delete();
    let mut number: Ptr<i32> = Ptr::alloc(7);
    assert!(((number.read()) == 7));
    number.delete();
    let mut zero: Ptr<i32> = Ptr::alloc(0_i32);
    assert!(((zero.read()) == 0));
    zero.delete();
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_1() });
    ({ test_2() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = constructed_0.with(|_| ());
}
