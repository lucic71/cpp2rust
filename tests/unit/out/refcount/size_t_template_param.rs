extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn array_ref_0(a: Ptr<u64>) -> u64 {
    elem!(a, 0).write({ (elem!(a, 0).read()).wrapping_add(1_u64) });
    return (elem!(a, (3_u64 as u64).wrapping_sub(1_u64)).read());
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[placement]
#[cpp_from(Ptr< u64  >  , new, value)]
#[byte_size(8)]
pub struct PtrCtor_unsigned_long_ {
    #[offset(0)]
    pub v: u64,
}
impl PtrCtor_unsigned_long_ {
    pub fn new(this: Ptr<PtrCtor_unsigned_long_>, mut p: Ptr<u64>) {
        field!(this, v).write((elem!(p, 1).read()));
    }
    pub fn copy_from(this: Ptr<PtrCtor_unsigned_long_>, _a0: Ptr<PtrCtor_unsigned_long_>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[placement]
#[cpp_from(u64, new, ptr)]
#[byte_size(8)]
pub struct RefCtor_unsigned_long_ {
    #[offset(0)]
    pub v: u64,
}
impl RefCtor_unsigned_long_ {
    pub fn new(this: Ptr<RefCtor_unsigned_long_>, x: Ptr<u64>) {
        field!(this, v).write((x.read()).wrapping_add(1_u64));
    }
    pub fn copy_from(this: Ptr<RefCtor_unsigned_long_>, _a0: Ptr<RefCtor_unsigned_long_>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
}
pub fn neg_ll_1() -> i64 {
    return -9000000_i64;
}
pub fn neg_char_2() -> i8 {
    return (b'\x80' as i8);
}
pub fn neg_int_3() -> i32 {
    return -1;
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let a1: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([1_usize, 2_usize, 3_usize])));
    assert!((({ array_ref_0((a1.decay()).reinterpret_cast::<u64>(),) }) == 3_u64));
    assert!(({ ((*a1.borrow())[(0) as usize] == 2_usize) }));
    let a2: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([4_usize, 5_usize])));
    let pc: Value<PtrCtor_unsigned_long_> =
        Rc::new(RefCell::new(<PtrCtor_unsigned_long_>::uninit()));
    PtrCtor_unsigned_long_::new(pc.as_pointer(), { (a2.decay()).reinterpret_cast::<u64>() });
    assert!(({ ({ (*pc.borrow()).v } == 5_u64) }));
    let v1: Value<usize> = Rc::new(RefCell::new(6_usize));
    let rc: Value<RefCtor_unsigned_long_> =
        Rc::new(RefCell::new(<RefCtor_unsigned_long_>::uninit()));
    RefCtor_unsigned_long_::new(rc.as_pointer(), {
        (v1.as_pointer()).reinterpret_cast::<u64>()
    });
    assert!(({ ({ (*rc.borrow()).v } == 7_u64) }));
    assert!((({ neg_ll_1() }) == (-9000000_i32 as i64)));
    assert!(((({ neg_char_2() }) as i32) == ((b'\x80' as i8) as i32)));
    assert!((({ neg_int_3() }) == -1_i32));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
