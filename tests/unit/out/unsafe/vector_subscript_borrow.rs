extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut next_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn bump_1(mut v: *mut Vec<i32>, mut x: i32) -> i32 {
    (*unimplemented!()) += x;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut next_0)).postfix_inc();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<i32> = unimplemented!();
    let mut items: Vec<u32> = unimplemented!();
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (items.len())) {
        (*unimplemented!()) = (i).wrapping_mul(2_u32);
        i.prefix_inc();
    }
    unimplemented!() = (unimplemented!()).wrapping_add(5_u32);
    (*unimplemented!()).postfix_inc();
    assert!(
        (((*unimplemented!()) == (0_u32)) && ((*unimplemented!()) == (7_u32)))
            && ((*unimplemented!()) == (5_u32))
    );
    (*unimplemented!()) = 3;
    (*unimplemented!()) = ((*unimplemented!()) + (4));
    assert!(((*unimplemented!()) == (3)) && ((*unimplemented!()) == (7)));
    let mut p: *mut i32 = (&mut (*unimplemented!()) as *mut i32);
    (*unimplemented!()) = 9;
    assert!(((*unimplemented!()) == (9)));
    (*unimplemented!()) = 2;
    assert!(((*unimplemented!()) == (2)));
    assert!(
        ((unsafe {
            let _v: *mut Vec<i32> = (&mut v as *mut Vec<i32>);
            let _x: i32 = (*unimplemented!());
            bump_1(_v, _x)
        }) == (1))
            && ((*unimplemented!()) == (3))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const next_0);
}
