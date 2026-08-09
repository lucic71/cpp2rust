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
#[bitfields(__bits_0 { a: u32 @ 0..1 unsigned, b: u32 @ 1..4 unsigned }, __bits_1 { c: u32 @ 0..1 unsigned })]
pub struct flags {
    pub tag: u8,
    pub __bits_0: [u8; 1],
    pub x: i32,
    pub __bits_1: [u8; 1],
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct outer {
    pub lead: libc::c_char,
    pub f: flags,
}
#[repr(C, align(4))]
#[derive(Copy, Clone, Default)]
#[bitfields(__bits_0 { s: i32 @ 0..3 signed, u: u32 @ 3..8 unsigned, wide: u32 @ 8..20 unsigned })]
pub struct mixed_sign {
    pub __bits_0: [u8; 3],
}
#[repr(C, align(8))]
#[derive(Copy, Clone)]
#[bitfields(__bits_0 { flag: u32 @ 0..1 unsigned, kind: u32 @ 1..4 unsigned })]
pub struct with_fn_ptr {
    pub fn_: Option<unsafe fn()>,
    pub __bits_0: [u8; 1],
    pub n: i32,
}
impl Default for with_fn_ptr {
    fn default() -> Self {
        with_fn_ptr {
            fn_: None,
            n: 0_i32,
            __bits_0: [0u8; 1],
        }
    }
}
pub static mut g_0: flags = unsafe {
    flags {
        tag: 2_u8,
        x: 7,
        __bits_0: [0u8; 1],
        __bits_1: [0u8; 1],
    }
    .with_a(1_u32)
    .with_b(5_u32)
    .with_c(0_u32)
};
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((::std::mem::size_of::<flags>()) == (12_usize)) as i32) != 0));
    assert!(((((::std::mem::offset_of!(flags, tag)) == (0_usize)) as i32) != 0));
    assert!(((((::std::mem::offset_of!(flags, x)) == (4_usize)) as i32) != 0));
    assert!(((((::std::mem::offset_of!(outer, f)) == (4_usize)) as i32) != 0));
    assert!(((((::std::mem::size_of::<mixed_sign>()) == (4_usize)) as i32) != 0));
    assert!(
        ((((((((((((((((g_0.tag as i32) == (2)) as i32) != 0)
            && ((((g_0.a() as i32) == (1)) as i32) != 0)) as i32)
            != 0)
            && ((((g_0.b() as i32) == (5)) as i32) != 0)) as i32)
            != 0)
            && ((((g_0.x) == (7)) as i32) != 0)) as i32)
            != 0)
            && ((((g_0.c() as i32) == (0)) as i32) != 0)) as i32)
            != 0)
    );
    let mut f: flags = <flags>::default();
    {
        let byte_0 = (((&raw mut f as *mut flags) as *mut flags) as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<flags>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (((&raw mut f as *mut flags) as *mut flags) as *mut ::libc::c_void)
    };
    assert!(
        ((((((((((((((((f.tag as i32) == (0)) as i32) != 0)
            && ((((f.a() as i32) == (0)) as i32) != 0)) as i32)
            != 0)
            && ((((f.b() as i32) == (0)) as i32) != 0)) as i32)
            != 0)
            && ((((f.x) == (0)) as i32) != 0)) as i32)
            != 0)
            && ((((f.c() as i32) == (0)) as i32) != 0)) as i32)
            != 0)
    );
    f.set_a(1_u32);
    assert!(
        ((((((((((((((((f.a() as i32) == (1)) as i32) != 0)
            && ((((f.b() as i32) == (0)) as i32) != 0)) as i32)
            != 0)
            && ((((f.c() as i32) == (0)) as i32) != 0)) as i32)
            != 0)
            && ((((f.tag as i32) == (0)) as i32) != 0)) as i32)
            != 0)
            && ((((f.x) == (0)) as i32) != 0)) as i32)
            != 0)
    );
    f.set_b(5_u32);
    assert!(
        (((((((f.b() as i32) == (5)) as i32) != 0) && ((((f.a() as i32) == (1)) as i32) != 0))
            as i32)
            != 0)
    );
    f.tag = 255_u8;
    assert!(
        ((((((((((f.tag as i32) == (255)) as i32) != 0) && ((((f.a() as i32) == (1)) as i32) != 0))
            as i32)
            != 0)
            && ((((f.b() as i32) == (5)) as i32) != 0)) as i32)
            != 0)
    );
    {
        let __bf_old = f.b();
        f.set_b((((__bf_old as i32) + ((1) as i32)) as u32));
        __bf_old
    };
    assert!(
        (((((((f.b() as i32) == (6)) as i32) != 0) && ((((f.a() as i32) == (1)) as i32) != 0))
            as i32)
            != 0)
    );
    f.set_b((((f.b() as i32) + 1) as u32));
    assert!(
        (((((((f.b() as i32) == (7)) as i32) != 0) && ((((f.a() as i32) == (1)) as i32) != 0))
            as i32)
            != 0)
    );
    f.set_c(1_u32);
    assert!(
        ((((((((((f.c() as i32) == (1)) as i32) != 0) && ((((f.a() as i32) == (1)) as i32) != 0))
            as i32)
            != 0)
            && ((((f.b() as i32) == (7)) as i32) != 0)) as i32)
            != 0)
    );
    f.x = -3_i32;
    assert!(
        ((((((((((((((((f.x) == (-3_i32)) as i32) != 0) && ((((f.a() as i32) == (1)) as i32) != 0))
            as i32)
            != 0)
            && ((((f.b() as i32) == (7)) as i32) != 0)) as i32)
            != 0)
            && ((((f.c() as i32) == (1)) as i32) != 0)) as i32)
            != 0)
            && ((((f.tag as i32) == (255)) as i32) != 0)) as i32)
            != 0)
    );
    let mut px: *mut i32 = (&raw mut f.x as *mut i32);
    (*px) = 42;
    assert!(
        ((((((((((f.x) == (42)) as i32) != 0) && ((((f.b() as i32) == (7)) as i32) != 0)) as i32)
            != 0)
            && ((((f.c() as i32) == (1)) as i32) != 0)) as i32)
            != 0)
    );
    let mut raw_: [u8; 12] = [0_u8; 12];
    {
        let byte_0 = (((&raw mut f as *mut flags) as *mut flags) as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<flags>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (((&raw mut f as *mut flags) as *mut flags) as *mut ::libc::c_void)
    };
    f.set_b(7_u32);
    {
        if ::std::mem::size_of::<flags>() != 0 {
            ::std::ptr::copy_nonoverlapping(
                (((&raw mut f as *mut flags) as *const flags) as *const ::libc::c_void),
                ((raw_.as_mut_ptr() as *mut u8) as *mut ::libc::c_void),
                ::std::mem::size_of::<flags>() as usize,
            )
        }
        ((raw_.as_mut_ptr() as *mut u8) as *mut ::libc::c_void)
    };
    assert!(((((raw_[((0) as usize)] as i32) == (0)) as i32) != 0));
    assert!(((((raw_[((1) as usize)] as i32) == (14)) as i32) != 0));
    let mut copy: flags = f;
    assert!(
        ((((((((((copy.b() as i32) == (7)) as i32) != 0)
            && ((((copy.a() as i32) == (0)) as i32) != 0)) as i32)
            != 0)
            && ((((copy.tag as i32) == (0)) as i32) != 0)) as i32)
            != 0)
    );
    let mut dup: flags = <flags>::default();
    {
        if ::std::mem::size_of::<flags>() != 0 {
            ::std::ptr::copy_nonoverlapping(
                (((&raw mut f as *mut flags) as *const flags) as *const ::libc::c_void),
                (((&raw mut dup as *mut flags) as *mut flags) as *mut ::libc::c_void),
                ::std::mem::size_of::<flags>() as usize,
            )
        }
        (((&raw mut dup as *mut flags) as *mut flags) as *mut ::libc::c_void)
    };
    assert!(
        ((((((((((dup.b() as i32) == (7)) as i32) != 0)
            && ((((dup.a() as i32) == (0)) as i32) != 0)) as i32)
            != 0)
            && ((((dup.tag as i32) == (0)) as i32) != 0)) as i32)
            != 0)
    );
    let mut m: mixed_sign = <mixed_sign>::default();
    {
        let byte_0 = (((&raw mut m as *mut mixed_sign) as *mut mixed_sign) as *mut ::libc::c_void)
            as *mut u8;
        for offset in 0..::std::mem::size_of::<mixed_sign>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (((&raw mut m as *mut mixed_sign) as *mut mixed_sign) as *mut ::libc::c_void)
    };
    m.set_s(-4_i32);
    assert!(((((m.s()) == (-4_i32)) as i32) != 0));
    m.set_s(3);
    assert!(((((m.s()) == (3)) as i32) != 0));
    m.set_u(31_u32);
    assert!(
        (((((((m.u() as i32) == (31)) as i32) != 0) && ((((m.s()) == (3)) as i32) != 0)) as i32)
            != 0)
    );
    m.set_wide(2748_u32);
    assert!(
        ((((((((((m.wide() as i32) == (2748)) as i32) != 0)
            && ((((m.u() as i32) == (31)) as i32) != 0)) as i32)
            != 0)
            && ((((m.s()) == (3)) as i32) != 0)) as i32)
            != 0)
    );
    m.set_s(-1_i32);
    assert!(
        ((((((((((m.s()) == (-1_i32)) as i32) != 0) && ((((m.u() as i32) == (31)) as i32) != 0))
            as i32)
            != 0)
            && ((((m.wide() as i32) == (2748)) as i32) != 0)) as i32)
            != 0)
    );
    m.set_s(1);
    m.set_s((((m.s() as i32) - ((3) as i32)) as i32));
    assert!(
        (((((((m.s()) == (-2_i32)) as i32) != 0) && ((((m.u() as i32) == (31)) as i32) != 0))
            as i32)
            != 0)
    );
    f.set_a(1_u32);
    f.set_b(5_u32);
    assert!(((((!(f.a() != 0) as i32) == (0)) as i32) != 0));
    assert!(((((!(f.c() != 0) as i32) == (1)) as i32) != 0));
    assert!(((((!(f.b() as i32)) == (!5)) as i32) != 0));
    assert!(((((-(f.b() as i32)) == (-5_i32)) as i32) != 0));
    assert!(((((f.b() as i32) == (5)) as i32) != 0));
    if f.b() != 0 {
        assert!(((((f.b() as i32) == (5)) as i32) != 0));
    }
    let mut step: u8 = 2_u8;
    f.set_b(1_u32);
    f.set_b((((f.b() as i32) + (step as i32)) as u32));
    assert!(((((f.b() as i32) == (3)) as i32) != 0));
    f.set_b((((f.b() as i32) << 1) as u32));
    assert!(((((f.b() as i32) == (6)) as i32) != 0));
    f.set_b((((f.b() as i32) & ((!1_u32) as i32)) as u32));
    assert!(((((f.b() as i32) == (6)) as i32) != 0));
    f.set_b((((f.b() as i32) - (g_0.tag as i32)) as u32));
    assert!(
        (((((((f.b() as i32) == (4)) as i32) != 0) && ((((f.a() as i32) == (1)) as i32) != 0))
            as i32)
            != 0)
    );
    let mut t: i32 = (({
        f.set_b(3_u32);
        f.b()
    }) as i32);
    assert!(
        (((((((t) == (3)) as i32) != 0) && ((((f.b() as i32) == (3)) as i32) != 0)) as i32) != 0)
    );
    let mut u: i32 = ({
        let __bf_old = f.b();
        f.set_b((((__bf_old as i32) + ((1) as i32)) as u32));
        __bf_old
    } as i32);
    assert!(
        (((((((u) == (3)) as i32) != 0) && ((((f.b() as i32) == (4)) as i32) != 0)) as i32) != 0)
    );
    let mut v: i32 = ({
        let __bf_new = (((f.b() as i32) + ((1) as i32)) as u32);
        f.set_b(__bf_new);
        __bf_new
    } as i32);
    assert!(
        (((((((v) == (5)) as i32) != 0) && ((((f.b() as i32) == (5)) as i32) != 0)) as i32) != 0)
    );
    let mut w: with_fn_ptr = <with_fn_ptr>::default();
    {
        let byte_0 = (((&raw mut w as *mut with_fn_ptr) as *mut with_fn_ptr) as *mut ::libc::c_void)
            as *mut u8;
        for offset in 0..::std::mem::size_of::<with_fn_ptr>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (((&raw mut w as *mut with_fn_ptr) as *mut with_fn_ptr) as *mut ::libc::c_void)
    };
    assert!(
        (((((((((((((w.fn_).is_none()) as i32) != 0) && ((((w.flag() as i32) == (0)) as i32) != 0))
            as i32)
            != 0)
            && ((((w.kind() as i32) == (0)) as i32) != 0)) as i32)
            != 0)
            && ((((w.n) == (0)) as i32) != 0)) as i32)
            != 0)
    );
    w.set_flag(1_u32);
    w.set_kind(5_u32);
    w.n = -7_i32;
    assert!(
        ((((((((((w.flag() as i32) == (1)) as i32) != 0)
            && ((((w.kind() as i32) == (5)) as i32) != 0)) as i32)
            != 0)
            && ((((w.n) == (-7_i32)) as i32) != 0)) as i32)
            != 0)
    );
    assert!(
        ((((::std::mem::offset_of!(with_fn_ptr, n))
            == ((::std::mem::size_of::<*mut ::libc::c_void>() as usize).wrapping_add(4_usize)))
            as i32)
            != 0)
    );
    return 0;
}
