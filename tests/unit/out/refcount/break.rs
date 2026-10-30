extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn for_test_0(mut n: ()) -> () {
    let x: Value<()> = Rc::new(RefCell::new(0_()));
    let mut i: () = 0_();
    let mut j: () = 0_();
    'loop_: while (i < n) {
        (*x.borrow_mut()) += 1_();
        if ((*x.borrow()) == 100_()) {
            break;
        }
        let mut k: () = 0_();
        let mut w: () = 0_();
        'loop_: while (w < j) {
            break;
            {
                {
                    w += 1_();
                    k += 1_()
                };
                i += k
            };
        }
        (*x.borrow_mut()) = { ((*x.borrow()) + 1_()) };
        {
            j = i;
            i += 1_()
        };
    }
    return (*x.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
