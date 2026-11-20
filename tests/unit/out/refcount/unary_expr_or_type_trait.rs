extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct S {
    #[offset(0)]
    pub c: i8,
    #[offset(8)]
    pub x: i64,
}
impl S {
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, c).write(_a0.with(|__s| __s.c));
        field!(this, x).write(_a0.with(|__s| __s.x));
    }
}
pub fn pack_size_0() -> u64 {
    return ((0 as usize).wrapping_add((0 as usize)) as u64);
}
pub fn pack_size_1(mut args_0: i32, mut args_1: f64) -> u64 {
    return ((2 as usize).wrapping_add((2 as usize)) as u64);
}
pub fn may_throw_2() {}
pub fn no_throw_3() {}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut arr: [i64; 4] = [0_i64; 4];
    let s: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = s.as_pointer();
        field!(__d, c).write(0_i8);
        field!(__d, x).write(0_i64);
    };
    assert!((::std::mem::size_of::<i32>() == 4_usize));
    assert!((::std::mem::size_of::<[i64; 4]>() == 32_usize));
    assert!((16usize == 16_usize));
    assert!((::std::mem::align_of::<i32>() == 4_usize));
    assert!((16usize == 16_usize));
    assert!((::std::mem::align_of::<[i64; 4]>() == 8_usize));
    assert!((16usize == 16_usize));
    assert!((({ pack_size_0() }) == 0_u64));
    assert!((({ pack_size_1(1, 2_f64,) }) == 4_u64));
    assert!(true);
    assert!(!(false));
    let mut may: bool = false;
    assert!(!(may));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
