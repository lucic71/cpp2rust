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
    let mut empty_buf: [libc::c_char; 256] = [0 as libc::c_char; 256];
    assert!(((((empty_buf[((0) as usize)] as i32) == ('\0' as i32)) as i32) != 0));
    assert!(((((empty_buf[((255) as usize)] as i32) == ('\0' as i32)) as i32) != 0));
    let mut prefix_buf: [libc::c_char; 32] =
        std::mem::transmute(*b"%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    assert!(((((prefix_buf[((0) as usize)] as i32) == ('%' as i32)) as i32) != 0));
    assert!(((((prefix_buf[((1) as usize)] as i32) == ('\0' as i32)) as i32) != 0));
    assert!(((((prefix_buf[((31) as usize)] as i32) == ('\0' as i32)) as i32) != 0));
    let mut short_buf: [libc::c_char; 16] = std::mem::transmute(*b"hi\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    assert!(((((short_buf[((0) as usize)] as i32) == ('h' as i32)) as i32) != 0));
    assert!(((((short_buf[((1) as usize)] as i32) == ('i' as i32)) as i32) != 0));
    assert!(((((short_buf[((2) as usize)] as i32) == ('\0' as i32)) as i32) != 0));
    assert!(((((short_buf[((15) as usize)] as i32) == ('\0' as i32)) as i32) != 0));
    let mut exact_buf: [libc::c_char; 6] = std::mem::transmute(*b"hello\0");
    assert!(((((exact_buf[((0) as usize)] as i32) == ('h' as i32)) as i32) != 0));
    assert!(((((exact_buf[((4) as usize)] as i32) == ('o' as i32)) as i32) != 0));
    assert!(((((exact_buf[((5) as usize)] as i32) == ('\0' as i32)) as i32) != 0));
    assert!(((((::std::mem::size_of::<[libc::c_char; 6]>()) == (6_usize)) as i32) != 0));
    assert!(
        (((((::std::mem::size_of::<[libc::c_char; 6]>() as usize).wrapping_sub(1_usize))
            == (5_usize)) as i32)
            != 0)
    );
    assert!(((((::std::mem::size_of::<[libc::c_char; 1]>()) == (1_usize)) as i32) != 0));
    assert!(
        (((((::std::mem::size_of::<[libc::c_char; 16]>() as usize).wrapping_sub(1_usize))
            == (15_usize)) as i32)
            != 0)
    );
    let mut bytes: [libc::c_char; 4] = [(0 as libc::c_char); 4];
    bytes[((0) as usize)] = (226u8 as libc::c_char);
    bytes[((1) as usize)] = (144u8 as libc::c_char);
    bytes[((2) as usize)] = (((128) + (1)) as libc::c_char);
    bytes[((3) as usize)] = (0 as libc::c_char);
    assert!(((((bytes[((0) as usize)] as i32) == ((226u8 as libc::c_char) as i32)) as i32) != 0));
    assert!((((((bytes[((0) as usize)] as u8) as i32) == (226)) as i32) != 0));
    assert!((((((bytes[((1) as usize)] as u8) as i32) == (144)) as i32) != 0));
    assert!((((((bytes[((2) as usize)] as u8) as i32) == (129)) as i32) != 0));
    assert!(
        (((((*c"Z".as_ptr().cast_mut().offset(((0) as isize))) as i32) == ('Z' as i32)) as i32)
            != 0)
    );
    assert!((((((*c"Z".as_ptr().cast_mut().offset(((1) as isize))) as i32) == (0)) as i32) != 0));
    assert!((((((*c"ab".as_ptr().cast_mut().offset(((2) as isize))) as i32) == (0)) as i32) != 0));
    assert!(
        (((((*c"ab".as_ptr().cast_mut().offset(((1) as isize))) as i32) == ('b' as i32)) as i32)
            != 0)
    );
    let mut i: i32 = 1;
    assert!((((((*c"Z".as_ptr().cast_mut().offset(((i) as isize))) as i32) == (0)) as i32) != 0));
    let mut p: *const libc::c_char = (c"Z".as_ptr().cast_mut()).cast_const();
    assert!((((((*p.offset(((1) as isize))) as i32) == (0)) as i32) != 0));
    let mut wide: i16 = (65535u16 as i16);
    assert!(((((wide as i32) == (-1_i32)) as i32) != 0));
    let mut narrow: u8 = (44u8 as u8);
    assert!(((((narrow as i32) == (44)) as i32) != 0));
    return 0;
}
