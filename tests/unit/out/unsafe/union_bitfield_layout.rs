extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct packed_flags {
    pub a: u32,
    pub b: u32,
    pub wide: u32,
    pub sgn: i32,
    pub tail: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union view {
    pub f: packed_flags,
    pub raw_: [u8; 8],
}
impl Default for view {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: view = <view>::default();
    {
        let byte_0 = (((&mut v as *mut view) as *mut view) as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<view>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (((&mut v as *mut view) as *mut view) as *mut ::libc::c_void)
    };
    v.f.a = 1_u32;
    v.f.b = 5_u32;
    v.f.wide = 703710_u32;
    v.f.sgn = -3_i32;
    v.f.tail = 287454020_u32;
    assert!(((((v.raw_[((0) as usize)] as i32) == (235)) as i32) != 0));
    assert!(((((v.raw_[((1) as usize)] as i32) == (205)) as i32) != 0));
    assert!(((((v.raw_[((2) as usize)] as i32) == (171)) as i32) != 0));
    assert!(((((v.raw_[((3) as usize)] as i32) == (13)) as i32) != 0));
    assert!(((((v.raw_[((4) as usize)] as i32) == (68)) as i32) != 0));
    assert!(((((v.raw_[((5) as usize)] as i32) == (51)) as i32) != 0));
    assert!(((((v.raw_[((6) as usize)] as i32) == (34)) as i32) != 0));
    assert!(((((v.raw_[((7) as usize)] as i32) == (17)) as i32) != 0));
    v.f.b = 2_u32;
    assert!(((((v.raw_[((0) as usize)] as i32) == (229)) as i32) != 0));
    assert!(((((v.f.a as i32) == (1)) as i32) != 0));
    assert!(((((v.f.wide as i32) == (703710)) as i32) != 0));
    assert!(((((v.f.sgn) == (-3_i32)) as i32) != 0));
    assert!(((((v.f.tail) == (287454020_u32)) as i32) != 0));
    {
        let byte_0 = (((&mut v as *mut view) as *mut view) as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<view>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (((&mut v as *mut view) as *mut view) as *mut ::libc::c_void)
    };
    v.raw_[((0) as usize)] = 60_u8;
    v.raw_[((1) as usize)] = 18_u8;
    v.raw_[((2) as usize)] = 0_u8;
    v.raw_[((3) as usize)] = 15_u8;
    assert!(((((v.f.a as i32) == (0)) as i32) != 0));
    assert!(((((v.f.b as i32) == (6)) as i32) != 0));
    assert!(((((v.f.wide as i32) == (291)) as i32) != 0));
    assert!(((((v.f.sgn) == (-1_i32)) as i32) != 0));
    assert!(((((v.f.tail) == (0_u32)) as i32) != 0));
    return 0;
}
