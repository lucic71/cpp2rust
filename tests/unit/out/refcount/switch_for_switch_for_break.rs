extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn for_switch_for_break_0(mut n: ()) -> () {
    let r: Value<()> = Rc::new(RefCell::new(0_()));
    let mut i: () = 0_();
    'loop_: while (i < n) {
        'switch: {
            match { i } {
                __v if __v == 1_() => {
                    let mut j: () = 0_();
                    'loop_: while (j < 10_()) {
                        if (j == 2_()) {
                            break;
                        }
                        (*r.borrow_mut()) += 1_();
                        j.prefix_inc();
                    }
                    (*r.borrow_mut()) += 100_();
                    break 'switch;
                }
                _ => {
                    (*r.borrow_mut()) += 10_();
                    break 'switch;
                }
            }
        };
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
