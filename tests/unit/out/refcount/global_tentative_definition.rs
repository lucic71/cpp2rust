extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive(Clone)]
pub struct ops_struct {
    pub first: FnPtr<fn(i32) -> i32>,
    pub second: FnPtr<fn(i32) -> i32>,
}
impl Default for ops_struct {
    fn default() -> Self {
        ops_struct {
            first: FnPtr::<fn(i32) -> i32>::null(),
            second: FnPtr::<fn(i32) -> i32>::null(),
        }
    }
}
impl ByteRepr for ops_struct {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.first.to_bytes(&mut buf[0..8]);
        self.second.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            first: <FnPtr<fn(i32) -> i32>>::from_bytes(&buf[0..8]),
            second: <FnPtr<fn(i32) -> i32>>::from_bytes(&buf[8..16]),
        }
    }
}
thread_local!();
thread_local!();
pub fn twice_2(v: i32) -> i32 {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    return ((*v.borrow()) * 2);
}
thread_local!(
    pub static table_0: Value<ops_struct> = Rc::new(RefCell::new(ops_struct {
        first: FnPtr::<fn(i32) -> i32>::null(),
        second: FnPtr::<fn(i32) -> i32>::new(twice_2),
    }));
);
thread_local!(
    pub static limits_1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([4, 5, 6])));
);
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!((((((*table_0.with(Value::clone).borrow()).first).is_null()) as i32) != 0));
    assert!((((!(((*table_0.with(Value::clone).borrow()).second).is_null())) as i32) != 0));
    assert!((((({ (*(*table_0.with(Value::clone).borrow()).second)(7) }) == 14) as i32) != 0));
    assert!(((((*limits_1.with(Value::clone).borrow())[(1) as usize] == 5) as i32) != 0));
    return 0;
}
