extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct S {
    pub v: (),
    pub n: [(); 2],
}
impl S {
    pub unsafe fn new(mut x: ()) -> Self {
        let mut this = Self {
            v: unimplemented!(),
            n: [x, ((x) + (1_()))],
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut S) -> Self {
        let mut this = Self {
            v: unimplemented!(),
            n: std::array::from_fn::<_, 2, _>(|__i: usize| (*_a0).n[(__i) as usize]),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut S) -> *mut S {
        unimplemented!();
        unimplemented!();
        return &mut (*(self as *mut S));
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct T {
    pub v: (),
}
impl T {
    pub unsafe fn size(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn move_from(_a0: *mut T) -> Self {
        let mut this = Self {
            v: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut T) -> *mut T {
        unimplemented!();
        return &mut (*(self as *mut T));
    }
}
pub unsafe fn sum_0(s: *const S) -> () {
    return (((unimplemented!()) + ((*s).n[(0_()) as usize])) + ((*s).n[(1_()) as usize]));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: S = S::new({ 2_() });
    unimplemented!();
    unimplemented!();
    let mut t: T = T {
        v: unimplemented!(),
    };
    unimplemented!();
    return 0_();
}
pub unsafe fn shuffle_1(mut x: ()) -> () {
    let mut a: S = S::new({ x });
    let mut b: S = S::move_from({ &mut a });
    unimplemented!();
    let mut c: S = S::new({ 1_() });
    (unsafe { S::move_assign(&mut c, &mut b) });
    unimplemented!();
    let mut t: T = T {
        v: unimplemented!(),
    };
    let mut u: T = T::move_from({ &mut t });
    let mut w: T = <T>::default();
    (unsafe { T::move_assign(&mut w, &mut u) });
    unimplemented!();
    return (unsafe { sum_0(&c) });
}
pub unsafe fn __cpp2rust_init_globals() {}
