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
pub union anon_0 {
    pub i: i32,
    pub z: *mut libc::c_char,
    pub pBig: *mut i64,
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Rec {
    pub kind: i32,
    pub u: anon_0,
}
pub static mut zeroRec_1: Rec = unsafe {
    Rec {
        kind: 0,
        u: anon_0 {
            z: std::ptr::null_mut(),
        },
    }
};
#[repr(C)]
#[derive(Copy, Clone)]
pub union anon_2 {
    pub a: [libc::c_char; 8],
    pub align: i16,
}
impl Default for anon_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub static mut blob_3: anon_2 = unsafe {
    anon_2 {
        a: std::mem::transmute(*b"0123456\0"),
    }
};
#[repr(C)]
#[derive(Copy, Clone)]
pub union Num {
    pub i: i32,
    pub b: [u8; 4],
}
impl Default for Num {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub static mut num_4: Num = unsafe { Num { i: 16909060 } };
pub unsafe fn get_rec_5() -> *mut Rec {
    static mut dummy_6: Rec = unsafe {
        Rec {
            kind: 0_i32,
            u: anon_0 {
                z: std::ptr::null_mut(),
            },
        }
    };;
    return (&raw mut dummy_6 as *mut Rec);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((zeroRec_1.kind) == (0)) as i32) != 0));
    assert!(((((zeroRec_1.u.z).is_null()) as i32) != 0));
    let mut p: *mut Rec = (unsafe { get_rec_5() });
    assert!((((((*p).u.pBig).is_null()) as i32) != 0));
    (*p).u.i = 5;
    assert!((((((*p).u.i) == (5)) as i32) != 0));
    let mut r: Rec = <Rec>::default();
    r.kind = 3;
    r.u.i = 9;
    r = zeroRec_1;
    assert!(((((r.kind) == (0)) as i32) != 0));
    assert!(((((r.u.i) == (0)) as i32) != 0));
    assert!(((((blob_3.a[((0) as usize)] as i32) == ('0' as i32)) as i32) != 0));
    assert!(((((blob_3.a[((6) as usize)] as i32) == ('6' as i32)) as i32) != 0));
    assert!(((((blob_3.a[((7) as usize)] as i32) == (0)) as i32) != 0));
    assert!(((((num_4.i) == (16909060)) as i32) != 0));
    assert!(((((num_4.b[((0) as usize)] as i32) == (4)) as i32) != 0));
    assert!(((((num_4.b[((3) as usize)] as i32) == (1)) as i32) != 0));
    return 0;
}
