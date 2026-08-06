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
pub struct handle {
    pub value: i32,
}
pub unsafe fn configure_0(mut h: *mut handle, mut op: i32, __args: &[VaArg]) -> i32 {
    let mut ap: VaList = VaList::default();
    let mut rc: i32 = 0;
    ap = VaList::new(__args);
    let mut onoff: i32 = ap.arg::<i32>();
    let mut pOut: *mut i32 = ap.arg::<*mut i32>();
    (*h).value = onoff;
    if !(pOut).is_null() {
        (*pOut) = onoff;
        rc = 1;
    };
    return rc;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut h: handle = handle { value: 0 };
    assert!(
        ((((unsafe { configure_0((&raw mut h as *mut handle), 7, &[(1).into(), (0).into(),]) })
            == (0)) as i32)
            != 0)
    );
    assert!(((((h.value) == (1)) as i32) != 0));
    let mut out: i32 = -1_i32;
    assert!(
        ((((unsafe {
            configure_0(
                (&raw mut h as *mut handle),
                7,
                &[(5).into(), (&raw mut out as *mut i32).into()],
            )
        }) == (1)) as i32)
            != 0)
    );
    assert!(((((out) == (5)) as i32) != 0));
    assert!(((((h.value) == (5)) as i32) != 0));
    return 0;
}
