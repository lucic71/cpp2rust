extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct list {
    pub head: Value<Ptr<node>>,
    pub size: Value<i32>,
}
impl ByteRepr for list {}
#[derive(Default)]
pub struct node {
    pub value: Value<i32>,
    pub next: Value<Ptr<node>>,
}
impl ByteRepr for node {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n: Value<node> = Rc::new(RefCell::new(node {
        value: Rc::new(RefCell::new(42)),
        next: Rc::new(RefCell::new(Ptr::<node>::null())),
    }));
    let l: Value<list> = Rc::new(RefCell::new(list {
        head: Rc::new(RefCell::new((n.as_pointer()))),
        size: Rc::new(RefCell::new(1)),
    }));
    assert!(
        ((((*(*(*(*l.borrow()).head.borrow()).upgrade().deref())
            .value
            .borrow())
            == 42) as i32)
            != 0)
    );
    assert!(((((*(*l.borrow()).size.borrow()) == 1) as i32) != 0));
    return 0;
}
