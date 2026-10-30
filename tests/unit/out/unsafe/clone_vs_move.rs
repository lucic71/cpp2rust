extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Bar {
    pub w: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Foo {
    pub x: (),
    pub y: *mut (),
    pub z: *mut (),
    pub a: [(); 3],
    pub bar: Bar,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Refs {
    pub a: *mut (),
    pub b: *mut (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x1: () = 1_();
    let mut x2: () = x1;
    x2.prefix_inc();
    unimplemented!();
    unimplemented!();
    let mut x3: () = 3_f64;
    let mut x4: () = x3;
    x4.prefix_inc();
    unimplemented!();
    unimplemented!();
    let reference: *mut () = &mut x1;
    let mut x5: () = (*reference);
    x5.prefix_inc();
    unimplemented!();
    unimplemented!();
    let mut pointer: *mut () = (&mut x1 as *mut ());
    let mut x6: () = (*pointer);
    x6.prefix_inc();
    unimplemented!();
    unimplemented!();
    let mut other_pointer: *mut () = pointer;
    unimplemented!();
    (*other_pointer).prefix_inc();
    unimplemented!();
    let mut f1: Foo = Foo {
        x: 1_(),
        y: &mut x1,
        z: (&mut x1 as *mut ()),
        a: [0_(), 1_(), 2_()],
        bar: Bar { w: 10_() },
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut f2: Foo = f1;
    f2.x.prefix_inc();
    (*f2.y).prefix_inc();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*f2.z).prefix_inc();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    f2.a[(0_()) as usize].prefix_inc();
    f2.a[(1_()) as usize].prefix_inc();
    f2.a[(2_()) as usize].prefix_inc();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    f2.bar.w = 20_();
    unimplemented!();
    unimplemented!();
    let mut N: () = 5_();
    let mut v1: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut v2: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        (*unimplemented!()).prefix_inc();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    let mut m1: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut m2: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        unimplemented!();
        let mut j: () = 0_();
        'loop_: while ((j) < (10_())) {
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        let mut j: () = 0_();
        'loop_: while ((j) < (10_())) {
            (*unimplemented!()).postfix_inc();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        unimplemented!();
        let mut j: () = 0_();
        'loop_: while ((j) < (10_())) {
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut map1: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        (*unimplemented!()) = i;
        i.prefix_inc();
    }
    let mut map2: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        (*unimplemented!()).prefix_inc();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    let mut pair1: () = unimplemented!();
    let mut pair2: () = unimplemented!();
    pair2.first = ((pair2.first) * (10_()));
    pair2.second = ((pair2.second) * (10_()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut pair3: () = unimplemented!();
    let mut pair4: () = unimplemented!();
    unimplemented!();
    pair4.second = 1_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    (*unimplemented!()) = ('b' as ());
    (*unimplemented!()) = ('b' as ());
    (*unimplemented!()) = ('b' as ());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut b1: Bar = Bar { w: 1_() };
    let mut b2: Bar = Bar { w: 2_() };
    b2 = b1;
    b2.w.postfix_inc();
    unimplemented!();
    unimplemented!();
    let mut v4: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        (*unimplemented!()).prefix_inc();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    let mut ra: () = 1_();
    let mut rb: () = 2_();
    let mut r1: Refs = Refs {
        a: &mut ra,
        b: &mut rb,
    };
    let mut r2: Refs = r1;
    (*r2.a) = 10_();
    (*r2.b).prefix_inc();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
