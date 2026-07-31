extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct S {
    pub v: Vec<i32>,
    pub a: i32,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let mut this = Self {
            v: (self.v).clone(),
            a: self.a,
        };
        this
    }
}
impl ByteRepr for S {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(<S>::default()));
    (*s.borrow_mut()).v.push(1);
    'loop_: for mut e in (*s.borrow()).v.as_pointer() as Ptr<i32> {
        let e: Value<i32> = Rc::new(RefCell::new(e.read().clone()));
        (*s.borrow_mut()).a.postfix_inc();
    }
    return 0;
}
