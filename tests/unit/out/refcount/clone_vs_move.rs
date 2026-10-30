extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Bar {
    #[offset(0)]
    pub w: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct Foo {
    #[offset(0)]
    pub x: (),
    #[offset(8)]
    #[byte_size(8)]
    pub y: Ptr<()>,
    #[offset(16)]
    #[byte_size(8)]
    pub z: Ptr<()>,
    #[offset(24)]
    #[byte_size(12)]
    pub a: Box<[()]>,
    #[offset(36)]
    #[byte_size(4)]
    pub bar: Bar,
}
impl Default for Foo {
    fn default() -> Self {
        {
            Foo {
                x: unimplemented!(),
                y: <Ptr<()>>::default(),
                z: Ptr::<()>::null(),
                a: (0..3).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                bar: <Bar>::default(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Refs {
    #[offset(0)]
    #[byte_size(8)]
    pub a: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub b: Ptr<()>,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let x1: Value<()> = Rc::new(RefCell::new(1_()));
    let mut x2: () = (*x1.borrow());
    x2.prefix_inc();
    unimplemented!();
    unimplemented!();
    let mut x3: () = 3_f64;
    let mut x4: () = x3;
    x4.prefix_inc();
    unimplemented!();
    unimplemented!();
    let reference: Ptr<()> = x1.as_pointer();
    let mut x5: () = (reference.read());
    x5.prefix_inc();
    unimplemented!();
    unimplemented!();
    let mut pointer: Ptr<()> = (x1.as_pointer());
    let mut x6: () = (pointer.read());
    x6.prefix_inc();
    unimplemented!();
    unimplemented!();
    let mut other_pointer: Ptr<()> = (pointer).clone();
    unimplemented!();
    other_pointer.with_mut(|__v| __v.prefix_inc());
    unimplemented!();
    let mut f1: Foo = Foo {
        x: 1_(),
        y: x1.as_pointer(),
        z: (x1.as_pointer()),
        a: Box::new([0_(), 1_(), 2_()]),
        bar: Bar { w: 10_() },
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut f2: Foo = (f1).clone();
    f2.x.prefix_inc();
    f2.y.with_mut(|__v| __v.prefix_inc());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    f2.z.with_mut(|__v| __v.prefix_inc());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    f2.a[(0_()) as usize].prefix_inc();
    f2.a[(1_()) as usize].prefix_inc();
    f2.a[(2_()) as usize].prefix_inc();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    f2.bar.w = 20_();
    unimplemented!();
    unimplemented!();
    let mut N: () = 5_();
    let mut v1: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < N) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut v2: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < N) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < N) {
        unimplemented!().with_mut(|__v| __v.prefix_inc());
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < N) {
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    let m1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < N) {
        unimplemented!();
        i.prefix_inc();
    }
    let m2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < N) {
        unimplemented!();
        unimplemented!();
        let mut j: () = 0_();
        'loop_: while (j < 10_()) {
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < N) {
        let mut j: () = 0_();
        'loop_: while (j < 10_()) {
            unimplemented!().with_mut(|__v| __v.postfix_inc());
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < N) {
        unimplemented!();
        unimplemented!();
        let mut j: () = 0_();
        'loop_: while (j < 10_()) {
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let map1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < N) {
        let __rhs = i;
        unimplemented!().write(__rhs);
        i.prefix_inc();
    }
    let map2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < N) {
        unimplemented!();
        unimplemented!().with_mut(|__v| __v.prefix_inc());
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < N) {
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    let pair1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let pair2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*pair2.borrow_mut()).first = { ({ (*pair2.borrow()).first } * 10_()) };
    (*pair2.borrow_mut()).second = { ({ (*pair2.borrow()).second } * 10_()) };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let pair3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let pair4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    (*pair4.borrow_mut()).second = 1_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    unimplemented!().write(('b' as ()));
    unimplemented!().write(('b' as ()));
    unimplemented!().write(('b' as ()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut b1: Bar = Bar { w: 1_() };
    let mut b2: Bar = Bar { w: 2_() };
    b2 = (b1).clone();
    b2.w.postfix_inc();
    unimplemented!();
    unimplemented!();
    let mut v4: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < N) {
        unimplemented!();
        unimplemented!().with_mut(|__v| __v.prefix_inc());
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < N) {
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    let ra: Value<()> = Rc::new(RefCell::new(1_()));
    let rb: Value<()> = Rc::new(RefCell::new(2_()));
    let mut r1: Refs = Refs {
        a: ra.as_pointer(),
        b: rb.as_pointer(),
    };
    let mut r2: Refs = (r1).clone();
    r2.a.write(10_());
    r2.b.with_mut(|__v| __v.prefix_inc());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
