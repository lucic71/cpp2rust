extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut x: ()) {
    x = (x).wrapping_add(1_());
}
pub unsafe fn bar_1(x: *mut ()) {
    (*x) = (*x).wrapping_add(1_());
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut m: () = unimplemented!();
    (*unimplemented!()) = 1_();
    (*unimplemented!()) = 2_();
    (*unimplemented!()) = 3_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = 4_();
    (*unimplemented!()) = x;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe { foo_0((*unimplemented!())) });
    unimplemented!();
    (unsafe { bar_1(&mut (*unimplemented!())) });
    unimplemented!();
    (*unimplemented!()) = (*unimplemented!()).wrapping_add((*unimplemented!()));
    unimplemented!();
    let mut end: () = unimplemented!();
    let mut it: () = unimplemented!();
    let mut const_it: () = unimplemented!();
    let mut x1: () = if unimplemented!() { 0_() } else { (*it).second };
    unimplemented!();
    let mut x2: () = if unimplemented!() {
        0_()
    } else {
        (*const_it).second
    };
    unimplemented!();
    let mut x3: () = if unimplemented!() { 0_() } else { (*it).second };
    unimplemented!();
    let mut x4: () = if unimplemented!() {
        0_()
    } else {
        (*const_it).second
    };
    unimplemented!();
    (*unimplemented!()) = 5_();
    let mut it4: () = unimplemented!();
    let mut p: *mut () = (&mut (*it4).second as *mut ());
    let mut x5: () = (*p);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*p).prefix_inc();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut const_it4: () = unimplemented!();
    unimplemented!();
    let r: *mut () = &mut m;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*unimplemented!()) = 7_();
    (*unimplemented!()) = 8_();
    (*unimplemented!()) = 9_();
    unimplemented!();
    unimplemented!();
    let mut key9: () = 9_();
    unimplemented!();
    unimplemented!();
    let mut cit: () = unimplemented!();
    let mut next: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut other_map: () = unimplemented!();
    unimplemented!();
    let mut key0: () = unimplemented!();
    let mut value: () = 2_();
    (*unimplemented!()) = value;
    value = (*unimplemented!());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut k: () = 0_();
    unimplemented!();
    k.prefix_inc();
    unimplemented!();
    k.prefix_inc();
    unimplemented!();
    let mut m2: () = unimplemented!();
    unimplemented!();
    let mut indexes: () = unimplemented!();
    let mut i: () = 60_();
    'loop_: while ((i) > (30_())) {
        unimplemented!();
        i.prefix_dec();
    }
    let mut i: () = 100_();
    'loop_: while ((i) > (60_())) {
        unimplemented!();
        i.prefix_dec();
    }
    let mut i: () = 30_();
    'loop_: while ((i) > (0_())) {
        unimplemented!();
        i.prefix_dec();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        (*unimplemented!()) = ((i).wrapping_rem(2_()) != 0);
        i.prefix_inc();
    }
    unimplemented!();
    let mut last: () = -1_();
    'loop_: for pair in IterRange::new(unimplemented!(), unimplemented!()) {
        let pair: *mut () = &mut (*unimplemented!());
        unimplemented!();
        unimplemented!();
        last = (*pair).first;
    }
    k = 0_();
    let value_0: *const () = unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
