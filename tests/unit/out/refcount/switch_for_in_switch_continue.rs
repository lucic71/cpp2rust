extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn for_in_switch_continue_0(mut n: ()) -> () {
    let r: Value<()> = Rc::new(RefCell::new(0_()));
    'switch: {
        match { n } {
            __v if __v == 0_() => {
                let mut i: () = 0_();
                'loop_: while (i < 5_()) {
                    if ((i % 2_()) == 0_()) {
                        i.prefix_inc();
                        continue 'loop_;
                    }
                    (*r.borrow_mut()) += i;
                    i.prefix_inc();
                }
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = -1_();
                break 'switch;
            }
        }
    };
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
