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
pub struct TestAllocator_int_ {}
impl TestAllocator_int_ {
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        return Box::leak((0..n).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut _a1: ()) {
        {
            let __p = p;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<()>(),
                )))
            }
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestAllocator_double_ {}
impl TestAllocator_double_ {
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        return Box::leak((0..n).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut _a1: ()) {
        {
            let __p = p;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<()>(),
                )))
            }
        };
    }
}
pub unsafe fn copy_0(mut copy_vector: ()) {}
pub unsafe fn fn_1(v: *mut (), mut v3: ()) {
    unimplemented!();
    let mut x: () = unimplemented!();
    let mut v4: *mut () = (&mut v3 as *mut ());
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = (*unimplemented!());
    (*unimplemented!()) = 1_();
    (*unimplemented!()) = 7_();
    (*unimplemented!()) = 13_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut v1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*unimplemented!()) = 40_();
    (*unimplemented!()) = 50_();
    unimplemented!();
    unimplemented!();
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe { copy_0(unimplemented!()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s2: () = unimplemented!();
    let mut v3: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (100_())) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut v6: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (s2)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut p1: *const () = (unimplemented!()).cast_const();
    unimplemented!();
    let mut p2: *mut () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*p2) = 99_f64;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p2.prefix_inc();
    (*p2) = 98_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let ref0: *mut () = unimplemented!();
    (*ref0) = 5_f64;
    unimplemented!();
    let mut x0: () = (*unimplemented!());
    unimplemented!();
    x0 = 6_f64;
    unimplemented!();
    let mut idx: () = 0_();
    unimplemented!();
    unimplemented!();
    let ref1: *mut () = unimplemented!();
    (*ref1) += 1.5_f64;
    unimplemented!();
    let mut x1: () = (*unimplemented!());
    unimplemented!();
    x1 -= 1.5_f64;
    unimplemented!();
    let mut v7: () = unimplemented!();
    let mut v8: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe { fn_1(&mut v7, unimplemented!()) });
    let mut src: [(); 3] = [1_(), 2_(), 3_()];
    let mut v9: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut v10: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut v11: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let src1: [(); 3] = [1_(), 2_(), 3_()];
    let mut v12: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut buf: [(); 5] = [10_(), 20_(), 30_(), 40_(), 50_()];
    let mut start: *const () = (buf.as_mut_ptr()).cast_const();
    let mut len: () = 5_();
    let mut v13: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
