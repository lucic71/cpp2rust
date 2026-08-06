extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let empty_buf: Value<Box<[u8]>> = Rc::new(RefCell::new(vec![0u8; 256].into_boxed_slice()));
    assert!((((*empty_buf.borrow())[(0) as usize] as i32) == (('\0' as u8) as i32)));
    assert!((((*empty_buf.borrow())[(255) as usize] as i32) == (('\0' as u8) as i32)));
    let prefix_buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(
        *b"%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )));
    assert!((((*prefix_buf.borrow())[(0) as usize] as i32) == (('%' as u8) as i32)));
    assert!((((*prefix_buf.borrow())[(1) as usize] as i32) == (('\0' as u8) as i32)));
    assert!((((*prefix_buf.borrow())[(31) as usize] as i32) == (('\0' as u8) as i32)));
    let short_buf: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::from(*b"hi\0\0\0\0\0\0\0\0\0\0\0\0\0\0")));
    assert!((((*short_buf.borrow())[(0) as usize] as i32) == (('h' as u8) as i32)));
    assert!((((*short_buf.borrow())[(1) as usize] as i32) == (('i' as u8) as i32)));
    assert!((((*short_buf.borrow())[(2) as usize] as i32) == (('\0' as u8) as i32)));
    assert!((((*short_buf.borrow())[(15) as usize] as i32) == (('\0' as u8) as i32)));
    let exact_buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(*b"hello\0")));
    assert!((((*exact_buf.borrow())[(0) as usize] as i32) == (('h' as u8) as i32)));
    assert!((((*exact_buf.borrow())[(4) as usize] as i32) == (('o' as u8) as i32)));
    assert!((((*exact_buf.borrow())[(5) as usize] as i32) == (('\0' as u8) as i32)));
    assert!((6usize == 6_usize));
    assert!(((6usize as usize).wrapping_sub(1_usize) == 5_usize));
    assert!((1usize == 1_usize));
    assert!(((16usize as usize).wrapping_sub(1_usize) == 15_usize));
    return 0;
}
