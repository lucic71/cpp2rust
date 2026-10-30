extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static next_0: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn bump_1(mut v: Ptr<()>, mut x: ()) -> () {
    {
        let __rhs = x;
        unimplemented!().with_mut(|__v| *__v = *__v + __rhs)
    };
    return (*next_0.with(Value::clone).borrow_mut()).postfix_inc();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut items: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let __rhs = (i).wrapping_mul(2_());
        unimplemented!().write(__rhs);
        i.prefix_inc();
    }
    {
        let rhs_0 = (unimplemented!().read()).wrapping_add(5_());
        unimplemented!().write(rhs_0)
    };
    unimplemented!().with_mut(|__v| __v.postfix_inc());
    unimplemented!();
    unimplemented!().write(3_());
    let __rhs = ((unimplemented!().read()) + 4_());
    unimplemented!().write(__rhs);
    unimplemented!();
    let mut p: Ptr<()> = (unimplemented!());
    unimplemented!().write(9_());
    unimplemented!();
    unimplemented!().write(2_());
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = next_0.with(|_| ());
}
