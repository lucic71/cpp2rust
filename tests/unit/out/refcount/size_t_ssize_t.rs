extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn add_sizes_0(mut a: (), mut b: ()) -> () {
    return (a).wrapping_add(b);
}
pub fn take_ulong_1(x: ()) -> () {
    let x: Value<()> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn sub_signed_2(mut a: (), mut b: ()) -> () {
    return (a - b);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut n: () = (::std::mem::size_of::<()>() as ()).wrapping_add(4_());
    unimplemented!();
    let mut ul: () = 10_();
    let mut sz: () = 20_();
    let mut mixed: () = (sz).wrapping_add(ul);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut chain: () = (((sz).wrapping_add(ul)).wrapping_add(5_()))
        .wrapping_add((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    let mut acc: () = 100_();
    acc = { (acc).wrapping_add((::std::mem::size_of::<()>() as ())) };
    acc = { (acc).wrapping_mul(2_()) };
    acc = { (acc).wrapping_sub(ul) };
    unimplemented!();
    sz = { (sz).wrapping_add(1_()) };
    unimplemented!();
    let mut fr: () = ({ add_sizes_0((::std::mem::size_of::<()>() as ()).wrapping_add(sz), ul) });
    unimplemented!();
    let mut fr2: () = ({ take_ulong_1(sz) });
    unimplemented!();
    let mut lo: () = (unimplemented!().read());
    let mut hi: () = (unimplemented!().read());
    unimplemented!();
    unimplemented!();
    let mut bound: () = (unimplemented!().read());
    unimplemented!();
    let mut data: [(); 8] = [unimplemented!(); 8];
    let mut count: () =
        (::std::mem::size_of::<[(); 8]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut i: () = 0_();
    'loop_: while (i < count) {
        data[(i) as usize] = { ((i).wrapping_mul(2_())) };
        i.postfix_inc();
    }
    let mut total: () = 0_();
    let mut i: () = 0_();
    'loop_: while (i < count) {
        total = { (total).wrapping_add(data[(i) as usize]) };
        i.postfix_inc();
    }
    unimplemented!();
    let mut cond: () = if (sz > ul) {
        (sz).wrapping_add((::std::mem::size_of::<()>() as ()))
    } else {
        ul
    };
    unimplemented!();
    let mut arr: [(); 4] = [0_(), 1_(), 2_(), 3_()];
    let mut idx: () = if (::std::mem::size_of::<()>() > 2_()) {
        2_()
    } else {
        0_()
    };
    unimplemented!();
    let mut s1: () = 5_();
    let mut s2: () = 12_();
    let mut sd: () = ({ sub_signed_2(s1, s2) });
    unimplemented!();
    unimplemented!();
    let mut l: () = 3_();
    let mut sm: () = (s2 + l);
    unimplemented!();
    unimplemented!();
    let mut smin: () = (unimplemented!().read());
    let mut smax: () = (unimplemented!().read());
    unimplemented!();
    unimplemented!();
    let mut delta: () = (sz - ul);
    unimplemented!();
    let mut a64: () = 100_();
    let mut b: () = 30_();
    a64 -= b;
    unimplemented!();
    a64 += b;
    unimplemented!();
    let mut c: () = -20_();
    a64 -= c;
    unimplemented!();
    unimplemented!();
    let mut mx: () = 5_();
    let mut mins: [(); 4] = [
        0_(),
        (unimplemented!().read()),
        (unimplemented!().read()),
        mx,
    ];
    unimplemented!();
    unimplemented!();
    let pr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let sizes: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_()])));
    let mut sizes_end: Ptr<()> = unimplemented!();
    unimplemented!();
    unimplemented!();
    let set: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!().write(42_());
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
