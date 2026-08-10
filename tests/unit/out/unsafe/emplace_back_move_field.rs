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
pub struct Item {
    pub value: Option<Box<i32>>,
}
#[repr(C)]
#[derive(Default)]
pub struct Holder {
    pub items: Vec<Item>,
    pub pending: Item,
}
pub unsafe fn store_0(mut h: *mut Holder) {
    (*h).items.push(std::mem::take(&mut (*h).pending));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut h: Holder = <Holder>::default();
    {
        let _a0: *mut i32 = (Box::leak(Box::new(7)) as *mut i32);
        h.pending.value = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    (unsafe { store_0((&raw mut h as *mut Holder)) });
    assert!(
        (h.pending
            .value
            .as_deref_mut()
            .map_or(::std::ptr::null_mut(), |v| v as *mut i32))
        .is_null()
    );
    assert!(
        ((*(*((h.items).first_mut().unwrap()))
            .value
            .as_deref_mut()
            .unwrap())
            == (7))
    );
    return 0;
}
