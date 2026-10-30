extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn median_0(mut w: (), mut n: (), mut nw: ()) -> () {
    let mx: () = if ((w) > (n)) { w } else { n };
    let mn: () = (((w) + (n)) - (mx));
    if ((nw) > (mx)) {
        return mn;
    } else if ((nw) < (mn)) {
        return mx;
    }
    return (((n) + (w)) - (nw));
}
pub unsafe fn assign_1(mut c: (), mut v: ()) -> () {
    let mut a: () = 1_();
    let mut b: () = 2_();
    (*if c {
        (&mut a as *mut ())
    } else {
        (&mut b as *mut ())
    }) = v;
    (*if c {
        (&mut a as *mut ())
    } else {
        (&mut b as *mut ())
    })
    .prefix_inc();
    return (((a) * (10_())) + (b));
}
pub unsafe fn lehmer_2(mut sigma: *const (), mut len: (), mut code: *mut ()) {
    let mut items: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (len)) {
        (*unimplemented!()) = i;
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (len)) {
        let mut it: () = unimplemented!();
        unimplemented!();
        (*code.offset((i) as isize)) = unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn sum_3(v: *mut ()) -> () {
    let mut s: () = 0_();
    let mut it: () = unimplemented!();
    'loop_: while !(unimplemented!()) {
        s += (*unimplemented!());
        unimplemented!();
    }
    return s;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut sigma: [(); 4] = [2_(), 0_(), 3_(), 1_()];
    let mut code: [(); 4] = [unimplemented!(); 4];
    (unsafe { lehmer_2((sigma.as_mut_ptr()).cast_const(), 4_(), code.as_mut_ptr()) });
    unimplemented!();
    let mut v: () = unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
