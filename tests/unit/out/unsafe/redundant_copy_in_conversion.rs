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
    let mut m: BTreeMap<i32, Box<i32>> = BTreeMap::new();
    (*m.entry(0).or_default().as_mut()) = 1;
    let mut end: UnsafeMapIterator<i32, i32> =
        UnsafeMapIterator::end(&m as *const BTreeMap<i32, Box<i32>>);
    let mut const_it: UnsafeMapIterator<i32, i32> =
        UnsafeMapIterator::find_key(&m as *const BTreeMap<i32, Box<i32>>, &0);
    return if const_it == end { 0 } else { 1 };
}
