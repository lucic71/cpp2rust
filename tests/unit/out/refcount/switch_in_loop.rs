extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn switch_in_loop_0(mut n: ()) -> () {
    let r: Value<()> = Rc::new(RefCell::new(0_()));
    let mut i: () = 0_();
    'loop_: while (i < n) {
        'switch: {
            match { (i % 3_()) } {
                __v if __v == 0_() => {
                    (*r.borrow_mut()) += 1_();
                    break 'switch;
                }
                __v if __v == 1_() => {
                    (*r.borrow_mut()) += 2_();
                    break 'switch;
                }
                _ => {
                    (*r.borrow_mut()) += 3_();
                    break 'switch;
                }
            }
        };
        (*r.borrow_mut()) += 10_();
        i.prefix_inc();
    }
    return (*r.borrow());
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
