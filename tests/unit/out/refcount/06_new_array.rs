extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn take_0(p: Ptr<Ptr<()>>) -> () {
    elem!((p.read()), 0_()).write(9_());
    let c: Value<()> = Rc::new(RefCell::new((elem!((p.read()), 0_()).read())));
    (p.read()).delete();
    return (*c.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut e: Ptr<()> =
        Ptr::alloc_array((0..2_()).map(|_| unimplemented!()).collect::<Box<[()]>>());
    elem!(e, 0_()).write(6_());
    elem!(e, 1_()).write(7_());
    e.delete();
    let mut c: () = ({
        let _p: Value<Ptr<()>> = Rc::new(RefCell::new(Ptr::alloc_array(
            (0..4_()).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        )));
        take_0(_p.as_pointer())
    });
    &(c);
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
