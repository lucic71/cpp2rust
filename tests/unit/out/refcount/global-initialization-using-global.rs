extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static first_0: Value<i32> = <Value<i32>>::default();
);
thread_local!(
    pub static second_1: Value<i32> =
        Rc::new(RefCell::new(((*first_0.with(Value::clone).borrow()) + 1)));
);
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!(((*first_0.with(Value::clone).borrow()) == 0));
    assert!(
        ((*second_1.with(Value::clone).borrow()) == ((*first_0.with(Value::clone).borrow()) + 1))
    );
    return 0;
}
