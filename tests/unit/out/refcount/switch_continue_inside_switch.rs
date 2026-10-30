extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn continue_inside_switch_0(mut n: ()) -> () {
    let r: Value<()> = Rc::new(RefCell::new(0_()));
    let mut i: () = 0_();
    'loop_: while (i < n) {
        'switch: {
            match { i } {
                __v if __v == 0_() || __v == 2_() || __v == 4_() => {
                    i.prefix_inc();
                    continue 'loop_;
                }
                _ => {
                    (*r.borrow_mut()) += i;
                    break 'switch;
                }
            }
        };
        (*r.borrow_mut()) += 1000_();
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
