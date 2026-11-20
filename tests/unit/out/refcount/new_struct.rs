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
pub struct Pair {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl Pair {
    pub fn new_1(this: Ptr<Pair>) {}
    pub fn copy_from(this: Ptr<Pair>, _a0: Ptr<Pair>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
        field!(this, y).write(_a0.with(|__s| __s.y));
    }
    pub fn move_from(this: Ptr<Pair>, _a0: Ptr<Pair>) {
        field!(this, x).write({ (*_a0.upgrade().deref()).x });
        field!(this, y).write({ (*_a0.upgrade().deref()).y });
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Triple {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub p: Pair,
}
impl Triple {
    pub fn new_1(this: Ptr<Triple>) {
        Pair::new_1(field_ptr!(this, p));
    }
    pub fn copy_from(this: Ptr<Triple>, _a0: Ptr<Triple>) {
        field!(this, a).write(_a0.with(|__s| __s.a));
        field!(this, b).write(_a0.with(|__s| __s.b));
        Pair::copy_from(field_ptr!(this, p), { field_ptr!(_a0, p) });
    }
    pub fn move_from(this: Ptr<Triple>, _a0: Ptr<Triple>) {
        field!(this, a).write({ (*_a0.upgrade().deref()).a });
        field!(this, b).write({ (*_a0.upgrade().deref()).b });
        Pair::move_from(field_ptr!(this, p), { field_ptr!(_a0, p) });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut p: Ptr<Pair> = {
        let __p = Ptr::alloc(<Pair>::uninit());
        {
            let __d = __p.clone();
            field!(__d, x).write(1);
            field!(__d, y).write(2);
        };
        __p
    };
    let mut out: i32 = ({ p.with(|__s| __s.x) } + { p.with(|__s| __s.y) });
    p.delete();
    assert!((out == 3));
    let t: Value<Triple> = Rc::new(RefCell::new(<Triple>::uninit()));
    {
        let __d = t.as_pointer();
        field!(__d, a).write(1);
        ();
        {
            let __d = field_ptr!(__d, p);
            ();
            ();
        };
    };
    assert!(({ ({ (*t.borrow()).a } == 1) }));
    assert!(({ ({ (*t.borrow()).b } == 0) }));
    assert!(({ ({ (*t.borrow()).p.x } == 0) && ({ (*t.borrow()).p.y } == 0) }));
    let mut q: Ptr<Triple> = {
        let __p = Ptr::alloc(<Triple>::uninit());
        {
            let __d = __p.clone();
            field!(__d, a).write(2);
            field!(__d, b).write(3);
            {
                let __d = field_ptr!(__d, p);
                ();
                ();
            };
        };
        __p
    };
    assert!((q.with(|__s| __s.a) == 2));
    assert!((q.with(|__s| __s.b) == 3));
    assert!((q.with(|__s| __s.p.x) == 0) && (q.with(|__s| __s.p.y) == 0));
    q.delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
