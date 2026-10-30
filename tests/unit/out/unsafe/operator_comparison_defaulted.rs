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
pub struct Eq {
    pub a: (),
    pub b: (),
}
impl Eq {
    pub unsafe fn operator_eq(&self, _a0: *const Eq) -> () {
        return (((*(self as *const Eq)).a) == ((*_a0).a))
            && (((*(self as *const Eq)).b) == ((*_a0).b));
    }
}
impl std::cmp::PartialEq for Eq {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Eq::operator_eq(self, other as *const Eq) }
    }
}
impl std::cmp::Eq for Eq {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Cmp {
    pub a: (),
    pub b: (),
}
impl Cmp {
    pub unsafe fn operator_cmp(&self, _a0: *const Cmp) -> () {
        {
            let mut cmp: () = std::cmp::Ord::cmp(&((*(self as *const Cmp)).a), &((*_a0).a));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        {
            let mut cmp: () = std::cmp::Ord::cmp(&((*(self as *const Cmp)).b), &((*_a0).b));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, _a0: *const Cmp) -> () {
        return (((*(self as *const Cmp)).a) == ((*_a0).a))
            && (((*(self as *const Cmp)).b) == ((*_a0).b));
    }
}
impl std::cmp::Ord for Cmp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { Cmp::operator_cmp(self, other as *const Cmp) }
    }
}
impl std::cmp::PartialOrd for Cmp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Cmp {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Cmp::operator_cmp(self, other as *const Cmp) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for Cmp {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Both {
    pub a: (),
}
impl Both {
    pub unsafe fn operator_eq(&self, _a0: *const Both) -> () {
        return (((*(self as *const Both)).a) == ((*_a0).a));
    }
    pub unsafe fn operator_cmp(&self, _a0: *const Both) -> () {
        {
            let mut cmp: () = std::cmp::Ord::cmp(&((*(self as *const Both)).a), &((*_a0).a));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
}
impl std::cmp::Ord for Both {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { Both::operator_cmp(self, other as *const Both) }
    }
}
impl std::cmp::PartialOrd for Both {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Both {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Both::operator_eq(self, other as *const Both) }
    }
}
impl std::cmp::Eq for Both {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OrdOnly {
    pub a: (),
}
impl OrdOnly {
    pub unsafe fn operator_cmp(&self, _a0: *const OrdOnly) -> () {
        {
            let mut cmp: () = std::cmp::Ord::cmp(&((*(self as *const OrdOnly)).a), &((*_a0).a));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
}
impl std::cmp::Ord for OrdOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { OrdOnly::operator_cmp(self, other as *const OrdOnly) }
    }
}
impl std::cmp::PartialOrd for OrdOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for OrdOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { OrdOnly::operator_cmp(self, other as *const OrdOnly) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for OrdOnly {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Inner {
    pub x: (),
}
impl Inner {
    pub unsafe fn operator_cmp(&self, _a0: *const Inner) -> () {
        {
            let mut cmp: () = std::cmp::Ord::cmp(&((*(self as *const Inner)).x), &((*_a0).x));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, _a0: *const Inner) -> () {
        return (((*(self as *const Inner)).x) == ((*_a0).x));
    }
}
impl std::cmp::Ord for Inner {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { Inner::operator_cmp(self, other as *const Inner) }
    }
}
impl std::cmp::PartialOrd for Inner {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Inner {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Inner::operator_cmp(self, other as *const Inner) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for Inner {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer {
    pub i: Inner,
    pub y: (),
}
impl Outer {
    pub unsafe fn operator_cmp(&self, _a0: *const Outer) -> () {
        {
            let mut cmp: () = (unsafe {
                let _arg0: *const Inner = &(*_a0).i;
                Inner::operator_cmp(&(*(self as *const Outer)).i, _arg0)
            });
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        {
            let mut cmp: () = std::cmp::Ord::cmp(&((*(self as *const Outer)).y), &((*_a0).y));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, _a0: *const Outer) -> () {
        return (unsafe {
            let _arg0: *const Inner = &(*_a0).i;
            Inner::operator_eq(&(*(self as *const Outer)).i, _arg0)
        }) && (((*(self as *const Outer)).y) == ((*_a0).y));
    }
}
impl std::cmp::Ord for Outer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { Outer::operator_cmp(self, other as *const Outer) }
    }
}
impl std::cmp::PartialOrd for Outer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Outer {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Outer::operator_cmp(self, other as *const Outer) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for Outer {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Secondary {
    pub a: (),
}
impl Secondary {
    pub unsafe fn operator_eq(&self, _a0: *const Secondary) -> () {
        return (((*(self as *const Secondary)).a) == ((*_a0).a));
    }
    pub unsafe fn operator_ne(&self, _a0: *const Secondary) -> () {
        return !(unsafe {
            let _arg0: *const Secondary = _a0;
            Secondary::operator_eq(&(*(self as *const Secondary)), _arg0)
        });
    }
    pub unsafe fn operator_cmp(&self, _a0: *const Secondary) -> () {
        {
            let mut cmp: () = std::cmp::Ord::cmp(&((*(self as *const Secondary)).a), &((*_a0).a));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    pub unsafe fn operator_lt(&self, _a0: *const Secondary) -> () {
        return unimplemented!();
    }
    pub unsafe fn operator_ge(&self, _a0: *const Secondary) -> () {
        return unimplemented!();
    }
}
impl std::cmp::Ord for Secondary {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { Secondary::operator_cmp(self, other as *const Secondary) }
    }
}
impl std::cmp::PartialOrd for Secondary {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Secondary {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Secondary::operator_eq(self, other as *const Secondary) }
    }
}
impl std::cmp::Eq for Secondary {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PtrMember {
    pub p: *mut (),
}
impl PtrMember {
    pub unsafe fn operator_cmp(&self, _a0: *const PtrMember) -> () {
        {
            let mut cmp: () = std::cmp::Ord::cmp(&((*(self as *const PtrMember)).p), &((*_a0).p));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, _a0: *const PtrMember) -> () {
        return (((*(self as *const PtrMember)).p) == ((*_a0).p));
    }
}
impl std::cmp::Ord for PtrMember {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { PtrMember::operator_cmp(self, other as *const PtrMember) }
    }
}
impl std::cmp::PartialOrd for PtrMember {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PtrMember {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            PtrMember::operator_cmp(self, other as *const PtrMember) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for PtrMember {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut e1: Eq = Eq { a: 1_(), b: 2_() };
    let mut e2: Eq = Eq { a: 1_(), b: 2_() };
    let mut e3: Eq = Eq { a: 1_(), b: 3_() };
    unimplemented!();
    unimplemented!();
    let mut c1: Cmp = Cmp { a: 1_(), b: 2_() };
    let mut c2: Cmp = Cmp { a: 1_(), b: 3_() };
    let mut c3: Cmp = Cmp { a: 2_(), b: 0_() };
    let mut c4: Cmp = Cmp { a: 1_(), b: 9_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut b1: Both = Both { a: 1_() };
    let mut b2: Both = Both { a: 2_() };
    unimplemented!();
    unimplemented!();
    let mut o1: OrdOnly = OrdOnly { a: 1_() };
    let mut o2: OrdOnly = OrdOnly { a: 2_() };
    unimplemented!();
    unimplemented!();
    let mut x1: Outer = Outer {
        i: Inner { x: 1_() },
        y: 9_(),
    };
    let mut x2: Outer = Outer {
        i: Inner { x: 2_() },
        y: 0_(),
    };
    let mut x3: Outer = Outer {
        i: Inner { x: 1_() },
        y: 9_(),
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s1: Secondary = Secondary { a: 1_() };
    let mut s2: Secondary = Secondary { a: 2_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut arr: [(); 2] = [0_(), 0_()];
    let mut p1: PtrMember = PtrMember {
        p: arr.as_mut_ptr(),
    };
    let mut p2: PtrMember = PtrMember {
        p: arr.as_mut_ptr().offset((1_()) as isize),
    };
    let mut p3: PtrMember = PtrMember {
        p: arr.as_mut_ptr(),
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
