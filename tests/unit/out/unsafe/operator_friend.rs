extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(_a0: *const Defaulted, _a1: *const Defaulted) -> () {
    return (((*_a0).a) == ((*_a1).a)) && (((*_a0).b) == ((*_a1).b));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Defaulted {
    pub a: (),
    pub b: (),
}
impl std::cmp::PartialEq for Defaulted {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const Defaulted, other as *const Defaulted) }
    }
}
impl std::cmp::Eq for Defaulted {}
pub unsafe fn operator_cmp_1(_a0: *const DefaultedOrd, _a1: *const DefaultedOrd) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).a), &((*_a1).a));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
pub unsafe fn operator_eq_2(_a0: *const DefaultedOrd, _a1: *const DefaultedOrd) -> () {
    return (((*_a0).a) == ((*_a1).a));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DefaultedOrd {
    pub a: (),
}
impl std::cmp::Ord for DefaultedOrd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_1(self as *const DefaultedOrd, other as *const DefaultedOrd) }
    }
}
impl std::cmp::PartialOrd for DefaultedOrd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for DefaultedOrd {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_2(self as *const DefaultedOrd, other as *const DefaultedOrd) }
    }
}
impl std::cmp::Eq for DefaultedOrd {}
pub unsafe fn operator_eq_3(x: *const Inline, y: *const Inline) -> () {
    return (((*x).a) == ((*y).a));
}
pub unsafe fn operator_lt_4(x: *const Inline, y: *const Inline) -> () {
    return (((*x).a) < ((*y).a));
}
pub unsafe fn operator_add_5(x: *const Inline, y: *const Inline) -> Inline {
    return Inline {
        a: (((*x).a) + ((*y).a)),
    };
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Inline {
    pub a: (),
}
impl std::cmp::Ord for Inline {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_4(self as *const Inline, other as *const Inline) {
                std::cmp::Ordering::Less
            } else if operator_lt_4(other as *const Inline, self as *const Inline) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Inline {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Inline {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_3(self as *const Inline, other as *const Inline) }
    }
}
impl std::cmp::Eq for Inline {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OutOfLine {
    pub a: (),
}
impl std::cmp::PartialEq for OutOfLine {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_6(self as *const OutOfLine, other as *const OutOfLine) }
    }
}
impl std::cmp::Eq for OutOfLine {}
pub unsafe fn operator_eq_6(x: *const OutOfLine, y: *const OutOfLine) -> () {
    return (((*x).a) == ((*y).a));
}
pub unsafe fn operator_ne_7(x: *const OutOfLine, y: *const OutOfLine) -> () {
    return !(unsafe {
        let _x: *const OutOfLine = x;
        let _y: *const OutOfLine = y;
        operator_eq_6(_x, _y)
    });
}
pub unsafe fn operator_eq_8(x: *const Tmpl_int_, y: *const Tmpl_int_) -> () {
    return (((*x).v) == ((*y).v));
}
pub unsafe fn operator_lt_9(x: *const Tmpl_int_, y: *const Tmpl_int_) -> () {
    return (((*x).v) < ((*y).v));
}
pub unsafe fn operator_eq_10(x: *const Tmpl_int_, y: *const Tmpl_long_) -> () {
    return (((*x).v) == ((*y).v));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Tmpl_int_ {
    pub v: (),
}
impl std::cmp::Ord for Tmpl_int_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_9(self as *const Tmpl_int_, other as *const Tmpl_int_) {
                std::cmp::Ordering::Less
            } else if operator_lt_9(other as *const Tmpl_int_, self as *const Tmpl_int_) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Tmpl_int_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Tmpl_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_8(self as *const Tmpl_int_, other as *const Tmpl_int_) }
    }
}
impl std::cmp::Eq for Tmpl_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Tmpl_long_ {
    pub v: (),
}
pub unsafe fn operator_eq_11(_a0: *const TmplDefaulted_int_, _a1: *const TmplDefaulted_int_) -> () {
    return (((*_a0).v) == ((*_a1).v));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TmplDefaulted_int_ {
    pub v: (),
}
impl std::cmp::PartialEq for TmplDefaulted_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_11(
                self as *const TmplDefaulted_int_,
                other as *const TmplDefaulted_int_,
            )
        }
    }
}
impl std::cmp::Eq for TmplDefaulted_int_ {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut d1: Defaulted = Defaulted { a: 1_(), b: 2_() };
    let mut d2: Defaulted = Defaulted { a: 1_(), b: 2_() };
    let mut d3: Defaulted = Defaulted { a: 1_(), b: 3_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut o1: DefaultedOrd = DefaultedOrd { a: 1_() };
    let mut o2: DefaultedOrd = DefaultedOrd { a: 2_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i1: Inline = Inline { a: 1_() };
    let mut i2: Inline = Inline { a: 2_() };
    let mut i3: Inline = Inline { a: 1_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut f1: OutOfLine = OutOfLine { a: 4_() };
    let mut f2: OutOfLine = OutOfLine { a: 4_() };
    let mut f3: OutOfLine = OutOfLine { a: 5_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut t1: Tmpl_int_ = Tmpl_int_ { v: 1_() };
    let mut t2: Tmpl_int_ = Tmpl_int_ { v: 2_() };
    let mut t3: Tmpl_int_ = Tmpl_int_ { v: 1_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut u1: Tmpl_long_ = Tmpl_long_ { v: 1_() };
    let mut u2: Tmpl_long_ = Tmpl_long_ { v: 2_() };
    unimplemented!();
    unimplemented!();
    let mut v1: TmplDefaulted_int_ = TmplDefaulted_int_ { v: 7_() };
    let mut v2: TmplDefaulted_int_ = TmplDefaulted_int_ { v: 7_() };
    let mut v3: TmplDefaulted_int_ = TmplDefaulted_int_ { v: 8_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
