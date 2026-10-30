extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn identity_0(x: ()) -> () {
    let x: Value<()> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn swap_by_ptr_1(mut a: Ptr<()>, mut b: Ptr<()>) {
    let mut tmp: () = (a.read());
    a.write({ (b.read()) });
    b.write({ tmp });
}
pub fn swap_by_ref_2(a: Ptr<()>, b: Ptr<()>) {
    let mut tmp: () = (a.read());
    a.write({ (b.read()) });
    b.write({ tmp });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut x: () = 0_();
    unimplemented!();
    let mut a: () = 1_();
    let mut b: () = ({ identity_0(a) });
    unimplemented!();
    let c: Value<()> = Rc::new(RefCell::new(2_()));
    let mut p: Ptr<()> = (c.as_pointer());
    unimplemented!();
    let d: Value<()> = Rc::new(RefCell::new(3_()));
    let e: Value<()> = Rc::new(RefCell::new(4_()));
    ({ swap_by_ptr_1((d.as_pointer()), (e.as_pointer())) });
    let f: Value<()> = Rc::new(RefCell::new(4_()));
    let g: Value<()> = Rc::new(RefCell::new(5_()));
    ({ swap_by_ref_2(f.as_pointer(), g.as_pointer()) });
    let mut h: Ptr<()> = Ptr::alloc(6_());
    unimplemented!();
    h.delete();
    let mut i: Ptr<()> = Ptr::alloc_array(Box::new([7_(), 8_(), unimplemented!()]));
    unimplemented!();
    i.delete();
    ({ swap_by_ptr_1(Ptr::alloc(7_()), Ptr::alloc(8_())) });
    ({
        swap_by_ptr_1(
            Ptr::alloc(7_()).offset((0_()) as isize),
            Ptr::alloc(8_()).offset((0_()) as isize),
        )
    });
    ({ swap_by_ref_2(Ptr::alloc(9_()), Ptr::alloc(10_())) });
    ({
        swap_by_ref_2(
            (Ptr::alloc(9_())).offset((0_()) as isize),
            (Ptr::alloc(10_())).offset((0_()) as isize),
        )
    });
    let mut j: () = unimplemented!();
    let mut k: Ptr<()> = unimplemented!();
    unimplemented!();
    let mut l: () = unimplemented!();
    let mut m: Ptr<()> = unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
