extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C, align(4))]
#[derive(Copy, Clone, Default)]
#[bitfields(__bits_0 { b: u32 @ 0..3 unsigned, w: u32 @ 3..15 unsigned, s: i32 @ 15..18 signed })]
pub struct bits {
    pub __bits_0: [u8; 3],
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut nine: i32 = 9;
    let mut big: i32 = 4660;
    let mut seven: i32 = 7;
    let mut v: bits = <bits>::default();
    v.set_b(0_u32);
    v.set_w(0_u32);
    v.set_s(0);
    v.set_b((nine as u32));
    assert!(((((v.b() as i32) == (1)) as i32) != 0));
    v.set_b(7_u32);
    {
        let __bf_old = v.b();
        v.set_b((((__bf_old as i32) + ((1) as i32)) as u32));
        __bf_old
    };
    assert!(((((v.b() as i32) == (0)) as i32) != 0));
    v.set_b(0_u32);
    {
        let __bf_old = v.b();
        v.set_b((((__bf_old as i32) - ((1) as i32)) as u32));
        __bf_old
    };
    assert!(((((v.b() as i32) == (7)) as i32) != 0));
    v.set_w((big as u32));
    assert!(((((v.w() as i32) == (564)) as i32) != 0));
    v.set_s(seven);
    assert!(((((v.s()) == (-1_i32)) as i32) != 0));
    v.set_s(3);
    {
        let __bf_old = v.s();
        v.set_s((((__bf_old as i32) + ((1) as i32)) as i32));
        __bf_old
    };
    assert!(((((v.s()) == (-4_i32)) as i32) != 0));
    v.set_s(-4_i32);
    {
        let __bf_old = v.s();
        v.set_s((((__bf_old as i32) - ((1) as i32)) as i32));
        __bf_old
    };
    assert!(((((v.s()) == (3)) as i32) != 0));
    return 0;
}
