extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DeclaresVirtual {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HoldsPolymorphicPointer {
    pub pointer: *mut DeclaresVirtual,
    pub after: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HoldsVoidPointer {
    pub pointer: *mut ::libc::c_void,
    pub after: i32,
}
pub unsafe fn bytesBetween_0(mut from: *mut ::libc::c_void, mut to: *mut ::libc::c_void) -> i64 {
    return ((((to as *mut libc::c_char) as usize - (from as *mut libc::c_char) as usize)
        / ::std::mem::size_of::<libc::c_char>()) as i64 as i64);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut polymorphicPointers: [*mut DeclaresVirtual; 3] = [
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ];
    let mut voidPointers: [*mut ::libc::c_void; 3] = [
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ];
    let mut polymorphicStride: i64 = (unsafe {
        let _from: *mut ::libc::c_void = ((&mut polymorphicPointers[(0) as usize]
            as *mut *mut DeclaresVirtual)
            as *mut ::libc::c_void);
        let _to: *mut ::libc::c_void = ((&mut polymorphicPointers[(1) as usize]
            as *mut *mut DeclaresVirtual)
            as *mut ::libc::c_void);
        bytesBetween_0(_from, _to)
    });
    let mut voidStride: i64 = (unsafe {
        let _from: *mut ::libc::c_void =
            ((&mut voidPointers[(0) as usize] as *mut *mut ::libc::c_void) as *mut ::libc::c_void);
        let _to: *mut ::libc::c_void =
            ((&mut voidPointers[(1) as usize] as *mut *mut ::libc::c_void) as *mut ::libc::c_void);
        bytesBetween_0(_from, _to)
    });
    assert!(((polymorphicStride) == (voidStride)));
    assert!(
        ((unsafe {
            let _from: *mut ::libc::c_void = ((&mut polymorphicPointers[(0) as usize]
                as *mut *mut DeclaresVirtual)
                as *mut ::libc::c_void);
            let _to: *mut ::libc::c_void = ((&mut polymorphicPointers[(2) as usize]
                as *mut *mut DeclaresVirtual)
                as *mut ::libc::c_void);
            bytesBetween_0(_from, _to)
        }) == ((2_i64) * (voidStride)))
    );
    let mut holdsPolymorphic: HoldsPolymorphicPointer = <HoldsPolymorphicPointer>::default();
    let mut holdsVoid: HoldsVoidPointer = <HoldsVoidPointer>::default();
    let mut polymorphicOffset: i64 = (unsafe {
        let _from: *mut ::libc::c_void =
            ((&mut holdsPolymorphic as *mut HoldsPolymorphicPointer) as *mut ::libc::c_void);
        let _to: *mut ::libc::c_void =
            ((&mut holdsPolymorphic.after as *mut i32) as *mut ::libc::c_void);
        bytesBetween_0(_from, _to)
    });
    let mut voidOffset: i64 = (unsafe {
        let _from: *mut ::libc::c_void =
            ((&mut holdsVoid as *mut HoldsVoidPointer) as *mut ::libc::c_void);
        let _to: *mut ::libc::c_void = ((&mut holdsVoid.after as *mut i32) as *mut ::libc::c_void);
        bytesBetween_0(_from, _to)
    });
    assert!(((polymorphicOffset) == (voidOffset)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
