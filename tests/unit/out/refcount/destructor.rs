extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static global_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Default)]
pub struct S {}
impl Drop for S {
    fn drop(&mut self) {
        (*global_0.with(Value::clone).borrow_mut()).postfix_inc();
    }
}
impl Clone for S {
    fn clone(&self) -> Self {
        let mut this = Self {};
        this
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        let s: Value<S> = Rc::new(RefCell::new(S {}));
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 1));
    {
        let s: Value<S> = Rc::new(RefCell::new(S {}));
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 2));
    return 0;
}
