extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Eq {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
}
impl CppEq for Eq {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        EqImpl::operator_eq(__lhs, __rhs.clone())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Cmp {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
}
impl CppLess for Cmp {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        CmpImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for Cmp {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        CmpImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Equal
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Both {
    #[offset(0)]
    pub a: (),
}
impl CppLess for Both {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        BothImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for Both {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        BothImpl::operator_eq(__lhs, __rhs.clone())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct OrdOnly {
    #[offset(0)]
    pub a: (),
}
impl CppLess for OrdOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        OrdOnlyImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for OrdOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        OrdOnlyImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Equal
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Inner {
    #[offset(0)]
    pub x: (),
}
impl CppLess for Inner {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        InnerImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for Inner {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        InnerImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Equal
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(4)]
    pub i: Inner,
    #[offset(4)]
    pub y: (),
}
impl CppLess for Outer {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        OuterImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for Outer {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        OuterImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Equal
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Secondary {
    #[offset(0)]
    pub a: (),
}
impl CppLess for Secondary {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        SecondaryImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Secondary {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        SecondaryImpl::operator_eq(__lhs, __rhs.clone())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct PtrMember {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Ptr<()>,
}
impl CppLess for PtrMember {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        PtrMemberImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for PtrMember {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        PtrMemberImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Equal
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let e1: Value<Eq> = Rc::new(RefCell::new(Eq { a: 1_(), b: 2_() }));
    let e2: Value<Eq> = Rc::new(RefCell::new(Eq { a: 1_(), b: 2_() }));
    let e3: Value<Eq> = Rc::new(RefCell::new(Eq { a: 1_(), b: 3_() }));
    unimplemented!();
    unimplemented!();
    let c1: Value<Cmp> = Rc::new(RefCell::new(Cmp { a: 1_(), b: 2_() }));
    let c2: Value<Cmp> = Rc::new(RefCell::new(Cmp { a: 1_(), b: 3_() }));
    let c3: Value<Cmp> = Rc::new(RefCell::new(Cmp { a: 2_(), b: 0_() }));
    let c4: Value<Cmp> = Rc::new(RefCell::new(Cmp { a: 1_(), b: 9_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let b1: Value<Both> = Rc::new(RefCell::new(Both { a: 1_() }));
    let b2: Value<Both> = Rc::new(RefCell::new(Both { a: 2_() }));
    unimplemented!();
    unimplemented!();
    let o1: Value<OrdOnly> = Rc::new(RefCell::new(OrdOnly { a: 1_() }));
    let o2: Value<OrdOnly> = Rc::new(RefCell::new(OrdOnly { a: 2_() }));
    unimplemented!();
    unimplemented!();
    let x1: Value<Outer> = Rc::new(RefCell::new(Outer {
        i: Inner { x: 1_() },
        y: 9_(),
    }));
    let x2: Value<Outer> = Rc::new(RefCell::new(Outer {
        i: Inner { x: 2_() },
        y: 0_(),
    }));
    let x3: Value<Outer> = Rc::new(RefCell::new(Outer {
        i: Inner { x: 1_() },
        y: 9_(),
    }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let s1: Value<Secondary> = Rc::new(RefCell::new(Secondary { a: 1_() }));
    let s2: Value<Secondary> = Rc::new(RefCell::new(Secondary { a: 2_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 0_()])));
    let p1: Value<PtrMember> = Rc::new(RefCell::new(PtrMember {
        p: (arr.as_pointer() as Ptr<()>),
    }));
    let p2: Value<PtrMember> = Rc::new(RefCell::new(PtrMember {
        p: (arr.as_pointer() as Ptr<()>).offset((1_()) as isize),
    }));
    let p3: Value<PtrMember> = Rc::new(RefCell::new(PtrMember {
        p: (arr.as_pointer() as Ptr<()>),
    }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait BothImpl {
    fn operator_eq(&self, _a0: Ptr<Both>) -> ();
    fn operator_cmp(&self, _a0: Ptr<Both>) -> ();
}
impl BothImpl for Ptr<Both> {
    fn operator_eq(&self, _a0: Ptr<Both>) -> () {
        return ({ (*self).with(|__s| __s.a) } == { _a0.with(|__s| __s.a) });
    }
    fn operator_cmp(&self, _a0: Ptr<Both>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.a)),
                &(_a0.with(|__s| __s.a)),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
}
pub trait CmpImpl {
    fn operator_cmp(&self, _a0: Ptr<Cmp>) -> ();
    fn operator_eq(&self, _a0: Ptr<Cmp>) -> ();
}
impl CmpImpl for Ptr<Cmp> {
    fn operator_cmp(&self, _a0: Ptr<Cmp>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.a)),
                &(_a0.with(|__s| __s.a)),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.b)),
                &(_a0.with(|__s| __s.b)),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<Cmp>) -> () {
        return ({ (*self).with(|__s| __s.a) } == { _a0.with(|__s| __s.a) })
            && ({ (*self).with(|__s| __s.b) } == { _a0.with(|__s| __s.b) });
    }
}
pub trait EqImpl {
    fn operator_eq(&self, _a0: Ptr<Eq>) -> ();
}
impl EqImpl for Ptr<Eq> {
    fn operator_eq(&self, _a0: Ptr<Eq>) -> () {
        return ({ (*self).with(|__s| __s.a) } == { _a0.with(|__s| __s.a) })
            && ({ (*self).with(|__s| __s.b) } == { _a0.with(|__s| __s.b) });
    }
}
pub trait InnerImpl {
    fn operator_cmp(&self, _a0: Ptr<Inner>) -> ();
    fn operator_eq(&self, _a0: Ptr<Inner>) -> ();
}
impl InnerImpl for Ptr<Inner> {
    fn operator_cmp(&self, _a0: Ptr<Inner>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.x)),
                &(_a0.with(|__s| __s.x)),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<Inner>) -> () {
        return ({ (*self).with(|__s| __s.x) } == { _a0.with(|__s| __s.x) });
    }
}
pub trait OrdOnlyImpl {
    fn operator_cmp(&self, _a0: Ptr<OrdOnly>) -> ();
}
impl OrdOnlyImpl for Ptr<OrdOnly> {
    fn operator_cmp(&self, _a0: Ptr<OrdOnly>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.a)),
                &(_a0.with(|__s| __s.a)),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
}
pub trait OuterImpl {
    fn operator_cmp(&self, _a0: Ptr<Outer>) -> ();
    fn operator_eq(&self, _a0: Ptr<Outer>) -> ();
}
impl OuterImpl for Ptr<Outer> {
    fn operator_cmp(&self, _a0: Ptr<Outer>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(
                ({
                    let _arg0: Ptr<Inner> = field_ptr!(_a0, i);
                    InnerImpl::operator_cmp(&field_ptr!((*self), i), _arg0)
                }),
            ));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.y)),
                &(_a0.with(|__s| __s.y)),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<Outer>) -> () {
        return ({
            let _arg0: Ptr<Inner> = field_ptr!(_a0, i);
            InnerImpl::operator_eq(&field_ptr!((*self), i), _arg0)
        }) && ({ (*self).with(|__s| __s.y) } == { _a0.with(|__s| __s.y) });
    }
}
pub trait PtrMemberImpl {
    fn operator_cmp(&self, _a0: Ptr<PtrMember>) -> ();
    fn operator_eq(&self, _a0: Ptr<PtrMember>) -> ();
}
impl PtrMemberImpl for Ptr<PtrMember> {
    fn operator_cmp(&self, _a0: Ptr<PtrMember>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.p.clone())),
                &(_a0.with(|__s| __s.p.clone())),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<PtrMember>) -> () {
        return ({ (*self).with(|__s| __s.p.clone()) } == { _a0.with(|__s| __s.p.clone()) });
    }
}
pub trait SecondaryImpl {
    fn operator_eq(&self, _a0: Ptr<Secondary>) -> ();
    fn operator_ne(&self, _a0: Ptr<Secondary>) -> ();
    fn operator_cmp(&self, _a0: Ptr<Secondary>) -> ();
    fn operator_lt(&self, _a0: Ptr<Secondary>) -> ();
    fn operator_ge(&self, _a0: Ptr<Secondary>) -> ();
}
impl SecondaryImpl for Ptr<Secondary> {
    fn operator_eq(&self, _a0: Ptr<Secondary>) -> () {
        return ({ (*self).with(|__s| __s.a) } == { _a0.with(|__s| __s.a) });
    }
    fn operator_ne(&self, _a0: Ptr<Secondary>) -> () {
        return !({
            let _arg0: Ptr<Secondary> = (_a0).clone();
            SecondaryImpl::operator_eq(&(*self), _arg0)
        });
    }
    fn operator_cmp(&self, _a0: Ptr<Secondary>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.a)),
                &(_a0.with(|__s| __s.a)),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_lt(&self, _a0: Ptr<Secondary>) -> () {
        return unimplemented!();
    }
    fn operator_ge(&self, _a0: Ptr<Secondary>) -> () {
        return unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {}
