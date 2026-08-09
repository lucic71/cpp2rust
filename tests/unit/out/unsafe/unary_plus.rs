extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: i32 = 5;
    let mut d: f64 = 5.0E-1;
    let mut b: i32 = -3_i32;
    assert!(((((a) == (5)) as i32) != 0));
    assert!(((((b) == (-3_i32)) as i32) != 0));
    assert!((((((a) + (b)) == (2)) as i32) != 0));
    assert!((((((d) * (4.0E+0)) == (2.0E+0)) as i32) != 0));
    assert!(((((if (((a) > (0)) as i32) != 0 { 1 } else { -1_i32 }) == (1)) as i32) != 0));
    return 0;
}
