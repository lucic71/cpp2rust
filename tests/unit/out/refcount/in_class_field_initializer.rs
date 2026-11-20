extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Inner {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl Inner {
    pub fn new_1(this: Ptr<Inner>) {
        field!(this, x).write(3);
        field!(this, y).write(4);
    }
    pub fn copy_from(this: Ptr<Inner>, _a0: Ptr<Inner>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
        field!(this, y).write(_a0.with(|__s| __s.y));
    }
    pub fn move_from(this: Ptr<Inner>, _a0: Ptr<Inner>) {
        field!(this, x).write({ (*_a0.upgrade().deref()).x });
        field!(this, y).write({ (*_a0.upgrade().deref()).y });
    }
}
impl Default for Inner {
    fn default() -> Self {
        { Inner { x: 3, y: 4 } }
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct S {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i8,
    #[offset(8)]
    #[byte_size(8)]
    pub c: Inner,
    #[offset(16)]
    #[byte_size(8)]
    pub d: Inner,
}
impl S {
    pub fn new_1(this: Ptr<S>) {
        field!(this, a).write(1);
        field!(this, b).write(2_i8);
        {
            let __d = field_ptr!(this, c);
            field!(__d, x).write(3);
            field!(__d, y).write(4);
        };
        Inner::new_1(field_ptr!(this, d));
    }
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, a).write(_a0.with(|__s| __s.a));
        field!(this, b).write(_a0.with(|__s| __s.b));
        Inner::copy_from(field_ptr!(this, c), { field_ptr!(_a0, c) });
        Inner::copy_from(field_ptr!(this, d), { field_ptr!(_a0, d) });
    }
    pub fn move_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, a).write({ (*_a0.upgrade().deref()).a });
        field!(this, b).write({ (*_a0.upgrade().deref()).b });
        Inner::move_from(field_ptr!(this, c), { field_ptr!(_a0, c) });
        Inner::move_from(field_ptr!(this, d), { field_ptr!(_a0, d) });
    }
}
impl Default for S {
    fn default() -> Self {
        {
            S {
                a: 1,
                b: 2_i8,
                c: Rc::try_unwrap({
                    let __v: Value<Inner> = Rc::new(RefCell::new(<Inner>::uninit()));
                    {
                        let __d = __v.as_pointer();
                        field!(__d, x).write(3);
                        field!(__d, y).write(4);
                    };
                    __v
                })
                .ok()
                .unwrap()
                .into_inner(),
                d: <Inner>::default(),
            }
        }
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Boxed_int_ {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    pub tag: i32,
}
impl Boxed_int_ {
    pub fn new(this: Ptr<Boxed_int_>, mut x: i32, mut t: i32) {
        field!(this, v).write(x);
        field!(this, tag).write(t);
    }
    pub fn copy_from(this: Ptr<Boxed_int_>, _a0: Ptr<Boxed_int_>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
        field!(this, tag).write(_a0.with(|__s| __s.tag));
    }
    pub fn move_from(this: Ptr<Boxed_int_>, _a0: Ptr<Boxed_int_>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
        field!(this, tag).write({ (*_a0.upgrade().deref()).tag });
    }
}
impl Default for Boxed_int_ {
    fn default() -> Self {
        { Boxed_int_ { v: 0_i32, tag: 7 } }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    S::new_1(s.as_pointer());
    assert!(({ ({ (*s.borrow()).a } == 1) }));
    assert!(({ (({ (*s.borrow()).b } as i32) == 2) }));
    assert!(({ ({ (*s.borrow()).c.x } == 3) }));
    assert!(({ ({ (*s.borrow()).c.y } == 4) }));
    assert!(({ ({ (*s.borrow()).d.x } == 3) }));
    assert!(({ ({ (*s.borrow()).d.y } == 4) }));
    let boxed: Value<Boxed_int_> = Rc::new(RefCell::new(<Boxed_int_>::uninit()));
    Boxed_int_::new(boxed.as_pointer(), { 5 }, { 9 });
    assert!(({ ({ (*boxed.borrow()).v } == 5) }));
    assert!(({ ({ (*boxed.borrow()).tag } == 9) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
