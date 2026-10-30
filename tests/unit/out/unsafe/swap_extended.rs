extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn identity_0(mut x: ()) -> () {
    return x;
}
pub unsafe fn swap_by_ptr_1(mut a: *mut (), mut b: *mut ()) {
    let mut tmp: () = (*a);
    (*a) = (*b);
    (*b) = tmp;
}
pub unsafe fn swap_by_ref_2(a: *mut (), b: *mut ()) {
    let mut tmp: () = (*a);
    (*a) = (*b);
    (*b) = tmp;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x: () = 0_();
    unimplemented!();
    let mut a: () = 1_();
    let mut b: () = (unsafe { identity_0(a) });
    unimplemented!();
    let mut c: () = 2_();
    let mut p: *mut () = (&mut c as *mut ());
    unimplemented!();
    let mut d: () = 3_();
    let mut e: () = 4_();
    (unsafe { swap_by_ptr_1((&mut d as *mut ()), (&mut e as *mut ())) });
    let mut f: () = 4_();
    let mut g: () = 5_();
    (unsafe { swap_by_ref_2(&mut f, &mut g) });
    let mut h: *mut () = (Box::leak(Box::new(6_())) as *mut ());
    unimplemented!();
    {
        let __p = h;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    let mut i: *mut () = Box::leak(Box::new([7_(), 8_(), unimplemented!()])).as_mut_ptr();
    unimplemented!();
    {
        let __p = i;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
    (unsafe {
        swap_by_ptr_1(
            (Box::leak(Box::new(7_())) as *mut ()),
            (Box::leak(Box::new(8_())) as *mut ()),
        )
    });
    (unsafe {
        swap_by_ptr_1(
            (Box::leak(Box::new(7_())) as *mut ()).offset((0_()) as isize),
            (Box::leak(Box::new(8_())) as *mut ()).offset((0_()) as isize),
        )
    });
    (unsafe {
        swap_by_ref_2(
            &mut (*(Box::leak(Box::new(9_())) as *mut ())),
            &mut (*(Box::leak(Box::new(10_())) as *mut ())),
        )
    });
    (unsafe {
        swap_by_ref_2(
            &mut (*(Box::leak(Box::new(9_())) as *mut ()).offset((0_()) as isize)),
            &mut (*(Box::leak(Box::new(10_())) as *mut ()).offset((0_()) as isize)),
        )
    });
    let mut j: () = unimplemented!();
    let mut k: *mut () = unimplemented!();
    unimplemented!();
    let mut l: () = unimplemented!();
    let mut m: *mut () = unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
