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
    assert!((((((*empty_buf.borrow())[(0) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    assert!((((((*empty_buf.borrow())[(255) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    let prefix_buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(
        *b"%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )));
    assert!((((((*prefix_buf.borrow())[(0) as usize] as i32) == ('%' as i32)) as i32) != 0));
    assert!((((((*prefix_buf.borrow())[(1) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    assert!((((((*prefix_buf.borrow())[(31) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    let short_buf: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::from(*b"hi\0\0\0\0\0\0\0\0\0\0\0\0\0\0")));
    assert!((((((*short_buf.borrow())[(0) as usize] as i32) == ('h' as i32)) as i32) != 0));
    assert!((((((*short_buf.borrow())[(1) as usize] as i32) == ('i' as i32)) as i32) != 0));
    assert!((((((*short_buf.borrow())[(2) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    assert!((((((*short_buf.borrow())[(15) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    let exact_buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(*b"hello\0")));
    assert!((((((*exact_buf.borrow())[(0) as usize] as i32) == ('h' as i32)) as i32) != 0));
    assert!((((((*exact_buf.borrow())[(4) as usize] as i32) == ('o' as i32)) as i32) != 0));
    assert!((((((*exact_buf.borrow())[(5) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    assert!((((6usize == 6_usize) as i32) != 0));
    assert!(((((6usize as usize).wrapping_sub(1_usize) == 5_usize) as i32) != 0));
    assert!((((1usize == 1_usize) as i32) != 0));
    assert!(((((16usize as usize).wrapping_sub(1_usize) == 15_usize) as i32) != 0));
    let bytes: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..4).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    (*bytes.borrow_mut())[(0) as usize] = (226u8 as u8);
    (*bytes.borrow_mut())[(1) as usize] = (144u8 as u8);
    (*bytes.borrow_mut())[(2) as usize] = ((128 + 1) as u8);
    (*bytes.borrow_mut())[(3) as usize] = 0_u8;
    assert!((((((*bytes.borrow())[(0) as usize] as i32) == ((226u8 as u8) as i32)) as i32) != 0));
    assert!(((((((*bytes.borrow())[(0) as usize] as u8) as i32) == 226) as i32) != 0));
    assert!(((((((*bytes.borrow())[(1) as usize] as u8) as i32) == 144) as i32) != 0));
    assert!(((((((*bytes.borrow())[(2) as usize] as u8) as i32) == 129) as i32) != 0));
    let wide: Value<i16> = Rc::new(RefCell::new((65535u16 as i16)));
    assert!((((((*wide.borrow()) as i32) == -1_i32) as i32) != 0));
    let narrow: Value<u8> = Rc::new(RefCell::new((44u8 as u8)));
    assert!((((((*narrow.borrow()) as i32) == 44) as i32) != 0));
    return 0;
}
