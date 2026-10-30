extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn inc_0(mut p: *mut ()) {
    (*p) += 1_();
}
pub unsafe fn add_1(mut p: *mut (), mut n: ()) {
    (*p) += n;
}
pub unsafe fn twice_2(mut n: ()) -> () {
    return ((n) * (2_()));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Access_S_ {}
impl Access_S_ {
    pub unsafe fn get_1(&mut self, mut p: *mut S) -> () {
        return (*p).base;
    }
    pub unsafe fn get_2(&mut self, mut p: *const S) -> () {
        return (((*p).base) + (1_()));
    }
    pub unsafe fn ref_3(&mut self, r: *mut S) -> () {
        return (((*r).base) + (2_()));
    }
    pub unsafe fn ref_4(&mut self, r: *const S) -> () {
        return (((*r).base) + (3_()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub base: (),
}
impl S {
    pub unsafe fn plain_4(&self, mut x: ()) -> () {
        return ((self.base) + (x));
    }
    pub unsafe fn plain_5(&self, mut x: ()) -> () {
        return (((self.base) + (x)) + (1_()));
    }
    pub unsafe fn take_6(&self, x: *mut ()) -> () {
        return (((self.base) + (*x)) + (1_()));
    }
    pub unsafe fn take_7(&self, x: *mut ()) -> () {
        return (((self.base) + (*x)) + (2_()));
    }
    pub unsafe fn pick_8(&self, mut p: ()) -> () {
        return ((self.base) + (p.first));
    }
    pub unsafe fn pick_9(&self, mut p: ()) -> () {
        return ((self.base) + (p.second));
    }
    pub unsafe fn apply_10(&self, mut f: Option<unsafe fn(*mut ())>, mut x: ()) -> () {
        (unsafe { (f).unwrap()((&mut x as *mut ())) });
        return ((self.base) + (x));
    }
    pub unsafe fn apply_11(&self, mut f: Option<unsafe fn(*mut (), ())>, mut x: ()) -> () {
        (unsafe { (f).unwrap()((&mut x as *mut ()), 10_()) });
        return ((self.base) + (x));
    }
    pub unsafe fn apply_12(&self, mut f: Option<unsafe fn(()) -> ()>, mut x: ()) -> () {
        return ((self.base) + (unsafe { (f).unwrap()(x) }));
    }
    pub unsafe fn combine_13(
        &self,
        p: *const (),
        mut f: Option<unsafe fn(()) -> ()>,
        mut q: *const (),
        mut n: (),
    ) -> () {
        return ((((self.base) + ((*p).second)) + (unsafe { (f).unwrap()((*q)) })) + (n));
    }
    pub unsafe fn combine_14(
        &self,
        p: *const (),
        mut f: Option<unsafe fn(*mut (), ())>,
        mut q: *mut (),
        mut n: (),
    ) -> () {
        (unsafe {
            let _arg0: *mut () = q;
            let _arg1: () = n;
            (f).unwrap()(_arg0, _arg1)
        });
        return (((self.base) + ((*p).first)) + (*q));
    }
    pub unsafe fn width_1_char(&self, mut x: ()) -> () {
        return ((self.base) + ((x) * (::std::mem::size_of::<()>())));
    }
    pub unsafe fn width_1_int(&self, mut x: ()) -> () {
        return ((self.base) + ((x) * (::std::mem::size_of::<()>())));
    }
    pub unsafe fn scale_2_2(&self, mut x: ()) -> () {
        return ((self.base) + ((x) * (2_())));
    }
    pub unsafe fn scale_2_3(&self, mut x: ()) -> () {
        return ((self.base) + ((x) * (3_())));
    }
    pub unsafe fn count_3(&self, mut x: ()) -> () {
        return (((self.base) + (x)) + (0));
    }
    pub unsafe fn count_3_int_long(&self, mut x: ()) -> () {
        return (((self.base) + (x)) + (2));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Box_ {
    pub v: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: S = S { base: 100_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut y: () = 1_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let c: () = 3_();
    unimplemented!();
    let mut z: () = 1_();
    unimplemented!();
    unimplemented!();
    let mut a: Access_S_ = <Access_S_>::default();
    let mut cs: *const S = (&mut s as *mut S).cast_const();
    let cr: *const S = &s;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut b: Box_ = Box_ { v: 4_() };
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
