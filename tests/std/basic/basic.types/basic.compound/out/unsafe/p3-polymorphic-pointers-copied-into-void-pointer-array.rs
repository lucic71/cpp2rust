extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut first_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut second_calls_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut third_calls_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe trait DeclaresVirtual {
    unsafe fn f(&mut self);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FirstOverrider {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SecondOverrider {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ThirdOverrider {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut first: FirstOverrider = <FirstOverrider>::default();
    let mut second: SecondOverrider = <SecondOverrider>::default();
    let mut third: ThirdOverrider = <ThirdOverrider>::default();
    let mut polymorphicPointers: [*mut dyn DeclaresVirtual; 3] = [
        (&mut first as *mut FirstOverrider),
        (&mut second as *mut SecondOverrider),
        (&mut third as *mut ThirdOverrider),
    ];
    let mut voidPointers: [*mut ::libc::c_void; 3] = [
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ];
    match ({ (voidPointers.as_mut_ptr() as *mut ::libc::c_void) }, {
        (polymorphicPointers.as_mut_ptr() as *const ::libc::c_void)
    }) {
        (mut __tmp_0, mut __tmp_1) => {
            if ::std::mem::size_of::<[*mut ::libc::c_void; 3]>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    __tmp_1,
                    __tmp_0,
                    ::std::mem::size_of::<[*mut ::libc::c_void; 3]>(),
                )
            }
            __tmp_0
        }
    };
    assert!(
        ((voidPointers[(0) as usize])
            == ((&mut first as *mut FirstOverrider) as *mut ::libc::c_void))
    );
    assert!(
        ((voidPointers[(1) as usize])
            == ((&mut second as *mut SecondOverrider) as *mut ::libc::c_void))
    );
    assert!(
        ((voidPointers[(2) as usize])
            == ((&mut third as *mut ThirdOverrider) as *mut ::libc::c_void))
    );
    (unsafe { (*(voidPointers[(0) as usize] as *mut dyn DeclaresVirtual)).f() });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_1)) == (0)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut third_calls_2)) == (0))
    );
    (unsafe { (*(voidPointers[(1) as usize] as *mut dyn DeclaresVirtual)).f() });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_1)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut third_calls_2)) == (0))
    );
    (unsafe { (*(voidPointers[(2) as usize] as *mut dyn DeclaresVirtual)).f() });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_1)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut third_calls_2)) == (1))
    );
    return 0;
}
unsafe impl DeclaresVirtual for FirstOverrider {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_0)).prefix_inc();
    }
}
unsafe impl DeclaresVirtual for SecondOverrider {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_1)).prefix_inc();
    }
}
unsafe impl DeclaresVirtual for ThirdOverrider {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut third_calls_2)).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const first_calls_0);
    std::cell::LazyCell::force(&*&raw const second_calls_1);
    std::cell::LazyCell::force(&*&raw const third_calls_2);
}
