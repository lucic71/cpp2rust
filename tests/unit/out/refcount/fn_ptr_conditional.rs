extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn inc_0(mut x: ()) -> () {
    return (x + 1_());
}
pub fn dec_1(mut x: ()) -> () {
    return (x - 1_());
}
pub fn identity_2(x: ()) -> () {
    let x: Value<()> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn pick_3(mut mode: ()) -> FnPtr<fn(()) -> ()> {
    return if (mode > 0_()) {
        FnPtr::<fn(()) -> ()>::new(inc_0)
    } else {
        if (mode < 0_()) {
            FnPtr::<fn(()) -> ()>::new(dec_1)
        } else {
            FnPtr::<fn(()) -> ()>::new(identity_2)
        }
    };
}
pub fn apply_4(mut fn_: FnPtr<fn(()) -> ()>, mut x: ()) -> () {
    let mut actual: FnPtr<fn(()) -> ()> = if !(fn_).is_null() {
        (fn_).clone()
    } else {
        FnPtr::<fn(()) -> ()>::new(identity_2)
    };
    return ({ actual.call(x) });
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
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
