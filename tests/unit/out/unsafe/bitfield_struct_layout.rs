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
pub struct flags {
    pub tag: u8,
    pub __bits_0: [u8; 1],
    pub x: i32,
    pub __bits_1: [u8; 1],
}
impl flags {
    #[inline]
    pub const fn a(&self) -> u32 {
        ((((self.__bits_0[0] as u64) >> 0) & 0x1) << 0) as u32
    }
    #[inline]
    pub const fn set_a(&mut self, v: u32) {
        assert!(v <= 1, "bitfield a: value does not fit in 1 bits");
        let __v = v as u64;
        self.__bits_0[0] = (self.__bits_0[0] & !0x01u8) | ((((__v >> 0) as u8) << 0) & 0x01u8);
    }
    #[inline]
    pub const fn with_a(mut self, v: u32) -> Self {
        self.set_a(v);
        self
    }
    #[inline]
    pub const fn b(&self) -> u32 {
        ((((self.__bits_0[0] as u64) >> 1) & 0x7) << 0) as u32
    }
    #[inline]
    pub const fn set_b(&mut self, v: u32) {
        assert!(v <= 7, "bitfield b: value does not fit in 3 bits");
        let __v = v as u64;
        self.__bits_0[0] = (self.__bits_0[0] & !0x0eu8) | ((((__v >> 0) as u8) << 1) & 0x0eu8);
    }
    #[inline]
    pub const fn with_b(mut self, v: u32) -> Self {
        self.set_b(v);
        self
    }
    #[inline]
    pub const fn c(&self) -> u32 {
        ((((self.__bits_1[0] as u64) >> 0) & 0x1) << 0) as u32
    }
    #[inline]
    pub const fn set_c(&mut self, v: u32) {
        assert!(v <= 1, "bitfield c: value does not fit in 1 bits");
        let __v = v as u64;
        self.__bits_1[0] = (self.__bits_1[0] & !0x01u8) | ((((__v >> 0) as u8) << 0) & 0x01u8);
    }
    #[inline]
    pub const fn with_c(mut self, v: u32) -> Self {
        self.set_c(v);
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct outer {
    pub lead: libc::c_char,
    pub f: flags,
}
#[repr(C, align(4))]
#[derive(Copy, Clone, Default)]
pub struct mixed_sign {
    pub __bits_0: [u8; 3],
}
impl mixed_sign {
    #[inline]
    pub const fn s(&self) -> i32 {
        (((((((self.__bits_0[0] as u64) >> 0) & 0x7) << 0) << 61) as i64) >> 61) as i32
    }
    #[inline]
    pub const fn set_s(&mut self, v: i32) {
        assert!(v >= -4 && v <= 3, "bitfield s: value out of range");
        let __v = v as u64;
        self.__bits_0[0] = (self.__bits_0[0] & !0x07u8) | ((((__v >> 0) as u8) << 0) & 0x07u8);
    }
    #[inline]
    pub const fn with_s(mut self, v: i32) -> Self {
        self.set_s(v);
        self
    }
    #[inline]
    pub const fn u(&self) -> u32 {
        ((((self.__bits_0[0] as u64) >> 3) & 0x1f) << 0) as u32
    }
    #[inline]
    pub const fn set_u(&mut self, v: u32) {
        assert!(v <= 31, "bitfield u: value does not fit in 5 bits");
        let __v = v as u64;
        self.__bits_0[0] = (self.__bits_0[0] & !0xf8u8) | ((((__v >> 0) as u8) << 3) & 0xf8u8);
    }
    #[inline]
    pub const fn with_u(mut self, v: u32) -> Self {
        self.set_u(v);
        self
    }
    #[inline]
    pub const fn wide(&self) -> u32 {
        ((((self.__bits_0[1] as u64) >> 0) & 0xff) << 0
            | (((self.__bits_0[2] as u64) >> 0) & 0xf) << 8) as u32
    }
    #[inline]
    pub const fn set_wide(&mut self, v: u32) {
        assert!(v <= 4095, "bitfield wide: value does not fit in 12 bits");
        let __v = v as u64;
        self.__bits_0[1] = (self.__bits_0[1] & !0xffu8) | ((((__v >> 0) as u8) << 0) & 0xffu8);
        self.__bits_0[2] = (self.__bits_0[2] & !0x0fu8) | ((((__v >> 8) as u8) << 0) & 0x0fu8);
    }
    #[inline]
    pub const fn with_wide(mut self, v: u32) -> Self {
        self.set_wide(v);
        self
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
        let __bf_v = (__bf_old + 1);
        f.set_b(__bf_v)
    };
    assert!(
        (((((((f.b() as i32) == (6)) as i32) != 0) && ((((f.a() as i32) == (1)) as i32) != 0))
            as i32)
            != 0)
    );
    {
        let __bf_old = f.b();
        let __bf_v = (__bf_old + (1));
        f.set_b(__bf_v)
    };
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
    return 0;
}
