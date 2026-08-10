extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Default)]
pub struct Wrapper {
    pub single: Option<Box<i32>>,
    pub array: Option<Box<[u8]>>,
}
pub unsafe fn setup_0(mut w: *mut Wrapper, mut value: i32) {
    {
        let _a0: *mut i32 = (Box::leak(Box::new(value)) as *mut i32);
        (*w).single = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    (*w).array = Some(Box::from_raw(Box::leak(
        (0..(value as usize)).map(|_| 0_u8).collect::<Box<[u8]>>(),
    )));
}
pub unsafe fn clear_1(mut w: *mut Wrapper) {
    {
        let _a0: *mut i32 = std::ptr::null_mut();
        (*w).single = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    (*w).array = None;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut w: Wrapper = <Wrapper>::default();
    (unsafe { setup_0((&raw mut w as *mut Wrapper), 3) });
    w.array.as_mut().unwrap()[(0_usize)] = ((*w.single.as_deref_mut().unwrap()) as u8);
    assert!(((w.array.as_mut().unwrap()[(0_usize)] as i32) == (3)));
    (unsafe { clear_1((&raw mut w as *mut Wrapper)) });
    assert!(
        (w.single
            .as_deref_mut()
            .map_or(::std::ptr::null_mut(), |v| v as *mut i32))
        .is_null()
    );
    return 0;
}
