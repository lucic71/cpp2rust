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
pub struct S {
    #[offset(0)]
    pub x: (),
}
pub fn copy_0(mut copy_vector: ()) {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut v1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!().write(40_());
    unimplemented!().write(50_());
    unimplemented!();
    unimplemented!();
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({ copy_0(unimplemented!()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = 7_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut src: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 4_()) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s2: () = unimplemented!();
    let mut v3: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 100_()) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut v4: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut v5: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut v6: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < s2) {
        unimplemented!();
        i.prefix_inc();
    }
    let v7: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 200_()) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut p1: Ptr<()> = unimplemented!();
    unimplemented!();
    let mut p2: Ptr<()> = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p2.write(99_f64);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p2.prefix_inc();
    p2.write(98_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let ref0: Ptr<()> = unimplemented!();
    ref0.write(5_f64);
    unimplemented!();
    let mut x0: () = (unimplemented!().read());
    unimplemented!();
    x0 = 6_f64;
    unimplemented!();
    let mut idx: () = 0_();
    unimplemented!();
    unimplemented!();
    let ref1: Ptr<()> = unimplemented!();
    {
        ref1.with_mut(|__v| *__v = *__v + 1.5_f64)
    };
    unimplemented!();
    let mut x1: () = (unimplemented!().read());
    unimplemented!();
    x1 -= 1.5_f64;
    unimplemented!();
    unimplemented!();
    let a: Value<S> = Rc::new(RefCell::new(S { x: 1_() }));
    let pa: Value<Ptr<S>> = Rc::new(RefCell::new((a.as_pointer())));
    let mut v8: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let ppa: Value<Ptr<Ptr<S>>> = Rc::new(RefCell::new((pa.as_pointer())));
    let mut v9: () = unimplemented!();
    unimplemented!();
    field!((((unimplemented!().read()).read()).read()), x).write(2_());
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
