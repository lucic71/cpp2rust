extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestAllocator_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestAllocator_double_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestAllocator_unsigned_long_ {}
pub fn copy_0(mut copy_vector: ()) {}
pub fn fn_1(v: Ptr<()>, v3: ()) {
    let v3: Value<()> = Rc::new(RefCell::new(v3));
    unimplemented!();
    let mut x: () = unimplemented!();
    let mut v4: Ptr<()> = (v3.as_pointer());
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = (unimplemented!().read());
    unimplemented!().write(1_());
    unimplemented!().write(7_());
    unimplemented!().write(13_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
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
    let mut s2: () = unimplemented!();
    let mut v3: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 100_()) {
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
    let v7: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut v8: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({ fn_1(v7.as_pointer(), unimplemented!()) });
    let src: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    let mut v9: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut v10: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut v11: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let src1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    let mut v12: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let buf: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([10_(), 20_(), 30_(), 40_(), 50_()])));
    let mut start: Ptr<()> = (buf.as_pointer() as Ptr<()>);
    let mut len: () = 5_();
    let mut v13: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait TestAllocator_double_Impl {
    fn allocate(&self, n: ()) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, _a1: ());
}
impl TestAllocator_double_Impl for Ptr<TestAllocator_double_> {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        return Ptr::alloc_array((0..n).map(|_| unimplemented!()).collect::<Box<[()]>>());
    }
    fn deallocate(&self, mut p: Ptr<()>, mut _a1: ()) {
        p.delete();
    }
}
pub trait TestAllocator_int_Impl {
    fn allocate(&self, n: ()) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, _a1: ());
}
impl TestAllocator_int_Impl for Ptr<TestAllocator_int_> {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        return Ptr::alloc_array((0..n).map(|_| unimplemented!()).collect::<Box<[()]>>());
    }
    fn deallocate(&self, mut p: Ptr<()>, mut _a1: ()) {
        p.delete();
    }
}
pub trait TestAllocator_unsigned_long_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut _a1: ()) {
        unimplemented!()
    }
}
impl TestAllocator_unsigned_long_Impl for Ptr<TestAllocator_unsigned_long_> {}
pub fn __cpp2rust_init_globals() {}
