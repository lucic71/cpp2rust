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
pub struct item {
    pub flags: u8,
}
pub unsafe fn merge_0(mut a: *mut item, mut n: i32) -> u8 {
    let mut all: u8 = 0_u8;
    let mut i: i32 = 0_i32;
    i = ((n) - (1));
    'loop_: while (((i) > (0)) as i32) != 0 {
        all = (((all as i32)
            | (({
                (*a.offset(((i) as isize))).flags = (*a.offset((((i) - (1)) as isize))).flags;
                (*a.offset(((i) as isize))).flags
            }) as i32)) as u8);
        i.postfix_dec();
    }
    return all;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: [item; 3] = [<item>::default(); 3];
    a[((0) as usize)].flags = 1_u8;
    a[((1) as usize)].flags = 2_u8;
    a[((2) as usize)].flags = 4_u8;
    assert!((((((unsafe { merge_0(a.as_mut_ptr(), 3) }) as i32) == (3)) as i32) != 0));
    assert!(((((a[((0) as usize)].flags as i32) == (1)) as i32) != 0));
    assert!(((((a[((1) as usize)].flags as i32) == (1)) as i32) != 0));
    assert!(((((a[((2) as usize)].flags as i32) == (2)) as i32) != 0));
    let mut x: i32 = 0;
    let mut y: i32 = 5;
    let mut z: i32 = 0;
    z += {
        x = y;
        x
    };
    assert!(((((z) == (5)) as i32) != 0));
    assert!(((((x) == (5)) as i32) != 0));
    let mut c: u8 = 1_u8;
    let mut v: i32 = (({
        c = (((c as i32) << 3) as u8);
        c
    }) as i32);
    assert!(((((v) == (8)) as i32) != 0));
    assert!(((((c as i32) == (8)) as i32) != 0));
    let mut steps: i32 = 0;
    c = 1_u8;
    let mut __do_while = true;
    'loop_: while __do_while
        || (((((({
            c = (((c as i32) << 1) as u8);
            c
        }) as i32)
            & (64))
            != (64)) as i32)
            != 0)
    {
        __do_while = false;
        steps.postfix_inc();
    }
    assert!(((((steps) == (6)) as i32) != 0));
    assert!(((((c as i32) == (64)) as i32) != 0));
    return 0;
}
