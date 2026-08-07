extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shape_a {
    pub code: u16,
    pub pad: [libc::c_char; 14],
}
impl Default for shape_a {
    fn default() -> Self {
        shape_a {
            code: 0_u16,
            pad: [(0 as libc::c_char); 14],
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shape_b {
    pub code: u16,
    pub lo: u16,
    pub hi: u32,
    pub fill: [libc::c_char; 8],
}
impl Default for shape_b {
    fn default() -> Self {
        shape_b {
            code: 0_u16,
            lo: 0_u16,
            hi: 0_u32,
            fill: [(0 as libc::c_char); 8],
        }
    }
}
#[repr(C, align(4))]
#[derive(Copy, Clone, Default)]
pub struct shape_c {
    pub code: u16,
    pub __bits_0: [u8; 2],
}
impl shape_c {
    #[inline]
    pub const fn f1(&self) -> u32 {
        ((((self.__bits_0[0] as u64) >> 0) & 0x1) << 0) as u32
    }
    #[inline]
    pub const fn set_f1(&mut self, v: u32) {
        assert!(v <= 1, "bitfield f1: value does not fit in 1 bits");
        let __v = v as u64;
        self.__bits_0[0] = (self.__bits_0[0] & !0x01u8) | ((((__v >> 0) as u8) << 0) & 0x01u8);
    }
    #[inline]
    pub const fn with_f1(mut self, v: u32) -> Self {
        self.set_f1(v);
        self
    }
    #[inline]
    pub const fn f2(&self) -> u32 {
        ((((self.__bits_0[0] as u64) >> 1) & 0x7) << 0) as u32
    }
    #[inline]
    pub const fn set_f2(&mut self, v: u32) {
        assert!(v <= 7, "bitfield f2: value does not fit in 3 bits");
        let __v = v as u64;
        self.__bits_0[0] = (self.__bits_0[0] & !0x0eu8) | ((((__v >> 0) as u8) << 1) & 0x0eu8);
    }
    #[inline]
    pub const fn with_f2(mut self, v: u32) -> Self {
        self.set_f2(v);
        self
    }
    #[inline]
    pub const fn f3(&self) -> u32 {
        ((((self.__bits_0[0] as u64) >> 4) & 0xf) << 0
            | (((self.__bits_0[1] as u64) >> 0) & 0xff) << 4) as u32
    }
    #[inline]
    pub const fn set_f3(&mut self, v: u32) {
        assert!(v <= 4095, "bitfield f3: value does not fit in 12 bits");
        let __v = v as u64;
        self.__bits_0[0] = (self.__bits_0[0] & !0xf0u8) | ((((__v >> 0) as u8) << 4) & 0xf0u8);
        self.__bits_0[1] = (self.__bits_0[1] & !0xffu8) | ((((__v >> 4) as u8) << 0) & 0xffu8);
    }
    #[inline]
    pub const fn with_f3(mut self, v: u32) -> Self {
        self.set_f3(v);
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union anon_0 {
    pub a: shape_a,
    pub b: shape_b,
    pub c: shape_c,
    pub raw_: [libc::c_char; 256],
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Container {
    pub view: anon_0,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: Container = <Container>::default();
    {
        let byte_0 =
            (((&raw mut c as *mut Container) as *mut Container) as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<Container>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (((&raw mut c as *mut Container) as *mut Container) as *mut ::libc::c_void)
    };
    assert!(((((c.view.a.code as i32) == (0)) as i32) != 0));
    assert!(((((c.view.b.lo as i32) == (0)) as i32) != 0));
    assert!(((((c.view.raw_[((0) as usize)] as i32) == (0)) as i32) != 0));
    assert!(((((c.view.raw_[((255) as usize)] as i32) == (0)) as i32) != 0));
    let mut src: [u8; 16] = [
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8,
    ];
    src[((0) as usize)] = 2_u8;
    src[((2) as usize)] = 80_u8;
    src[((3) as usize)] = 0_u8;
    src[((4) as usize)] = 127_u8;
    src[((5) as usize)] = 0_u8;
    src[((6) as usize)] = 0_u8;
    src[((7) as usize)] = 1_u8;
    let mut len: usize = 16_usize;
    assert!(((((len) <= (::std::mem::size_of::<[libc::c_char; 256]>())) as i32) != 0));
    {
        if len != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((src.as_mut_ptr() as *const u8) as *const ::libc::c_void),
                (((&raw mut c.view.raw_ as *mut [libc::c_char; 256]) as *mut [libc::c_char; 256])
                    as *mut ::libc::c_void),
                len as usize,
            )
        }
        (((&raw mut c.view.raw_ as *mut [libc::c_char; 256]) as *mut [libc::c_char; 256])
            as *mut ::libc::c_void)
    };
    assert!(((((c.view.b.code as i32) == (2)) as i32) != 0));
    assert!(
        (((((*((&raw mut c.view.b.lo as *mut u16) as *mut u8).offset(((0) as isize))) as i32)
            == (80)) as i32)
            != 0)
    );
    {
        let byte_0 =
            (((&raw mut c as *mut Container) as *mut Container) as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<Container>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (((&raw mut c as *mut Container) as *mut Container) as *mut ::libc::c_void)
    };
    assert!(((((c.view.b.code as i32) == (0)) as i32) != 0));
    assert!(((((::std::mem::size_of::<shape_c>()) == (4_usize)) as i32) != 0));
    assert!(
        ((((((((((c.view.c.f1() as i32) == (0)) as i32) != 0)
            && ((((c.view.c.f2() as i32) == (0)) as i32) != 0)) as i32)
            != 0)
            && ((((c.view.c.f3() as i32) == (0)) as i32) != 0)) as i32)
            != 0)
    );
    c.view.c.code = 2_u16;
    c.view.c.set_f1(1_u32);
    c.view.c.set_f2(5_u32);
    c.view.c.set_f3(2748_u32);
    assert!(
        (((((*((&raw mut c.view.raw_ as *mut [libc::c_char; 256]) as *mut u8)
            .offset(((2) as isize))) as i32)
            == (203)) as i32)
            != 0)
    );
    assert!(
        (((((*((&raw mut c.view.raw_ as *mut [libc::c_char; 256]) as *mut u8)
            .offset(((3) as isize))) as i32)
            == (171)) as i32)
            != 0)
    );
    {
        let byte_0 = (((&raw mut c.view.raw_[((2) as usize)] as *mut libc::c_char)
            as *mut libc::c_char) as *mut ::libc::c_void) as *mut u8;
        for offset in 0..2_usize {
            *byte_0.offset(offset as isize) = 255 as u8;
        }
        (((&raw mut c.view.raw_[((2) as usize)] as *mut libc::c_char) as *mut libc::c_char)
            as *mut ::libc::c_void)
    };
    assert!(
        ((((((((((c.view.c.f1() as i32) == (1)) as i32) != 0)
            && ((((c.view.c.f2() as i32) == (7)) as i32) != 0)) as i32)
            != 0)
            && ((((c.view.c.f3() as i32) == (4095)) as i32) != 0)) as i32)
            != 0)
    );
    assert!(((((c.view.c.code as i32) == (2)) as i32) != 0));
    {
        let byte_0 =
            (((&raw mut c as *mut Container) as *mut Container) as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<Container>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (((&raw mut c as *mut Container) as *mut Container) as *mut ::libc::c_void)
    };
    assert!(
        ((((((((((c.view.c.f1() as i32) == (0)) as i32) != 0)
            && ((((c.view.c.f2() as i32) == (0)) as i32) != 0)) as i32)
            != 0)
            && ((((c.view.c.f3() as i32) == (0)) as i32) != 0)) as i32)
            != 0)
    );
    assert!(((((c.view.c.code as i32) == (0)) as i32) != 0));
    return 0;
}
