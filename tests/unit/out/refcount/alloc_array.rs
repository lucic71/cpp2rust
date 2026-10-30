extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn All_0(arr: Ptr<Option<Value<Box<[i32]>>>>, mut N: i32, mut element: i32) {
    let mut all: Option<Value<Box<[i32]>>> = unimplemented!();
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        (all.as_pointer() as Ptr<i32>)
            .offset((i as usize) as isize)
            .write(element);
        i.prefix_inc();
    }
    arr.write(all.take());
}
pub fn Consume_1(mut arr: Option<Value<Box<[i32]>>>, mut N: i32) -> i32 {
    let sum: Value<i32> = Rc::new(RefCell::new(0));
    let mut i: i32 = -1_i32;
    'loop_: while (i.prefix_inc() < N) {
        (*sum.borrow_mut()) += ((arr.as_pointer() as Ptr<i32>)
            .offset((i as usize) as isize)
            .read());
    }
    return (*sum.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut N: i32 = 10;
    let arr: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(unimplemented!()));
    ({ All_0(arr.as_pointer(), N, 1) });
    assert!((({ Consume_1((*arr.borrow_mut()).take(), N,) }) == 10));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
