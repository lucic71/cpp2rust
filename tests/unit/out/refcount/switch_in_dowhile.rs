extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn switch_in_dowhile_0(mut n: ()) -> () {
    let r: Value<()> = Rc::new(RefCell::new(0_()));
    let mut i: () = 0_();
    let mut __do_while = true;
    'loop_: while __do_while || (i < n) {
        __do_while = false;
        'switch: {
            match { i } {
                __v if __v == 0_() => {
                    (*r.borrow_mut()) += 1_();
                    break 'switch;
                }
                __v if __v == 1_() => {
                    (*r.borrow_mut()) += 10_();
                    break 'switch;
                }
                _ => {
                    (*r.borrow_mut()) += 100_();
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
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
