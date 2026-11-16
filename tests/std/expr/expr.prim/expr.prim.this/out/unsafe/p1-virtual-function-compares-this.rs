extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe trait DeclaresVirtual {
    unsafe fn isSameObject(&mut self, other: *mut dyn DeclaresVirtual) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ComparesThis {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: ComparesThis = <ComparesThis>::default();
    let mut b: ComparesThis = <ComparesThis>::default();
    let mut baseOfA: *mut dyn DeclaresVirtual = (&mut a as *mut ComparesThis);
    let mut baseOfB: *mut dyn DeclaresVirtual = (&mut b as *mut ComparesThis);
    assert!(
        (unsafe {
            let _other: *mut dyn DeclaresVirtual = baseOfA;
            (*baseOfA).isSameObject(_other)
        })
    );
    assert!(!(unsafe { (*baseOfA).isSameObject(baseOfB,) }));
    assert!((unsafe { (*baseOfB).isSameObject((&mut b as *mut ComparesThis),) }));
    assert!(!(unsafe { (*baseOfB).isSameObject((&mut a as *mut ComparesThis),) }));
    return 0;
}
unsafe impl DeclaresVirtual for ComparesThis {
    unsafe fn isSameObject(&mut self, other: *mut dyn DeclaresVirtual) -> bool {
        return ((self as *mut ComparesThis) == (other));
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
