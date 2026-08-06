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
pub unsafe fn get_rec_2() -> *mut Rec {
    static mut dummy_3: Rec = unsafe {
        Rec {
            kind: 0_i32,
            u: anon_0 {
                z: std::ptr::null_mut(),
            },
        }
    };;
    return (&raw mut dummy_3 as *mut Rec);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((zeroRec_1.kind) == (0)) as i32) != 0));
    assert!(((((zeroRec_1.u.z).is_null()) as i32) != 0));
    let mut p: *mut Rec = (unsafe { get_rec_2() });
    assert!((((((*p).u.pBig).is_null()) as i32) != 0));
    (*p).u.i = 5;
    assert!((((((*p).u.i) == (5)) as i32) != 0));
    let mut r: Rec = <Rec>::default();
    r.kind = 3;
    r.u.i = 9;
    r = zeroRec_1;
    assert!(((((r.kind) == (0)) as i32) != 0));
    assert!(((((r.u.i) == (0)) as i32) != 0));
    return 0;
}
