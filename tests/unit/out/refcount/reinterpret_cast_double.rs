extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let d: Value<f64> = Rc::new(RefCell::new(1.0E+0));
    let bits: Value<Ptr<u64>> = Rc::new(RefCell::new((d.as_pointer()).reinterpret_cast::<u64>()));
    assert!((((*bits.borrow()).read()) == 4607182418800017408_u64));
    (*bits.borrow()).write(4614256656552045848_u64);
    assert!(((*d.borrow()) > 3.1400000000000001E+0) && ((*d.borrow()) < 3.1499999999999999E+0));
    return 0;
}
