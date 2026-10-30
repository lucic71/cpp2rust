extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_0(mut p: Ptr<()>, mut n: ()) -> () {
    let total: Value<()> = Rc::new(RefCell::new(0_()));
    let mut i: () = 0_();
    'loop_: while (i < n) {
        (*total.borrow_mut()) += { (elem!(p, i).read()) };
        i.prefix_inc();
    }
    return (*total.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut array: Ptr<()> =
        Ptr::alloc_array((0..100_()).map(|_| unimplemented!()).collect::<Box<[()]>>());
    array.delete();
    let mut filled: Ptr<()> =
        Ptr::alloc_array((0..4_()).map(|_| unimplemented!()).collect::<Box<[()]>>());
    let mut i: () = 0_();
    'loop_: while (i < 4_()) {
        elem!(filled, i).write({ (i + 1_()) });
        i.prefix_inc();
    }
    if (({
        let _p: Ptr<()> = (filled).clone();
        sum_0(_p, 4_())
    }) != 10_())
    {
        return 1_();
    }
    filled.delete();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
