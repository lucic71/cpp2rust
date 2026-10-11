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
pub struct Pair {
    pub first: i32,
    pub second: i32,
}
impl Pair {
    pub unsafe fn new(mut a: i32, mut b: i32) -> Self {
        let mut this = Self {
            first: a,
            second: ((b) * (2)),
        };
        this
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut pairs: Vec<Pair> = Vec::new();
    {
        let __init = Pair::new({ 1 }, { 2 });
        pairs.push(__init)
    };
    assert!(((pairs.len()) == (1_usize)));
    assert!(((pairs[(0_usize)].first) == (1)) && ((pairs[(0_usize)].second) == (4)));
    let mut nested: Vec<Vec<i32>> = Vec::new();
    {
        let __init = vec![5; (3 as usize) as usize];
        nested.push(__init)
    };
    assert!(((nested.len()) == (1_usize)));
    assert!(((nested[(0_usize)].len()) == (3_usize)) && ((nested[(0_usize)][(2_usize)]) == (5)));
    let mut queue: Vec<Pair> = Vec::new();
    {
        let __init = Pair::new({ 6 }, { 7 });
        queue.push(__init)
    };
    assert!(
        (((*((queue).first_mut().unwrap())).first) == (6))
            && (((*((queue).first_mut().unwrap())).second) == (14))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
