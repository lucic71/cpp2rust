extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn nested_0(mut a: (), mut b: ()) -> () {
    let r: Value<()> = Rc::new(RefCell::new(0_()));
    'switch: {
        match { a } {
            __v if __v == 1_() => {
                'switch: {
                    match { b } {
                        __v if __v == 10_() => {
                            (*r.borrow_mut()) = 11_();
                            break 'switch;
                        }
                        __v if __v == 20_() => {
                            (*r.borrow_mut()) = 12_();
                            break 'switch;
                        }
                        _ => {
                            (*r.borrow_mut()) = 13_();
                            break 'switch;
                        }
                    }
                };
                (*r.borrow_mut()) += 1_();
                break 'switch;
            }
            __v if __v == 2_() => {
                (*r.borrow_mut()) = 2_();
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
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
