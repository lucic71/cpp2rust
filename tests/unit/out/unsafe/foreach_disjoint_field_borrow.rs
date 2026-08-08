extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, Default)]
pub struct S {
    pub parts: Vec<Vec<i32>>,
    pub a: i32,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = <S>::default();
    s.a = 0;
    s.parts
        .resize_with(3_usize as usize, || <Vec<i32>>::default());
    {
        let __a0 = 2_usize as usize;
        s.parts[(2_usize)].resize_with(__a0, || <i32>::default())
    };
    let mut points: i32 = 0;
    let mut p: *mut S = (&raw mut s as *mut S);
    'loop_: for part in 0..((*p).parts.len()) {
        let mut part = (*p).parts.as_ptr().add(part);
        points = (((points as usize).wrapping_add((*part).len())) as i32);
        s.a.postfix_inc();
    }
    assert!(((s.a) == (3)));
    assert!(((points) == (2)));
    return 0;
}
