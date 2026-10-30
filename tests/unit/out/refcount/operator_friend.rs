extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(_a0: Ptr<Defaulted>, _a1: Ptr<Defaulted>) -> () {
    return ({ _a0.with(|__s| __s.a) } == { _a1.with(|__s| __s.a) })
        && ({ _a0.with(|__s| __s.b) } == { _a1.with(|__s| __s.b) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Defaulted {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
}
impl CppEq for Defaulted {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_cmp_1(_a0: Ptr<DefaultedOrd>, _a1: Ptr<DefaultedOrd>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.a)),
            &(_a1.with(|__s| __s.a)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
pub fn operator_eq_2(_a0: Ptr<DefaultedOrd>, _a1: Ptr<DefaultedOrd>) -> () {
    return ({ _a0.with(|__s| __s.a) } == { _a1.with(|__s| __s.a) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct DefaultedOrd {
    #[offset(0)]
    pub a: (),
}
impl CppLess for DefaultedOrd {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_1(__lhs.clone(), __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for DefaultedOrd {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_2(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_eq_3(x: Ptr<Inline>, y: Ptr<Inline>) -> () {
    return ({ x.with(|__s| __s.a) } == { y.with(|__s| __s.a) });
}
pub fn operator_lt_4(x: Ptr<Inline>, y: Ptr<Inline>) -> () {
    return ({ x.with(|__s| __s.a) } < { y.with(|__s| __s.a) });
}
pub fn operator_add_5(x: Ptr<Inline>, y: Ptr<Inline>) -> Inline {
    return Inline {
        a: ({ x.with(|__s| __s.a) } + { y.with(|__s| __s.a) }),
    };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Inline {
    #[offset(0)]
    pub a: (),
}
impl CppLess for Inline {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_4(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for Inline {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_3(__lhs.clone(), __rhs.clone())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct OutOfLine {
    #[offset(0)]
    pub a: (),
}
impl CppEq for OutOfLine {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_6(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_eq_6(x: Ptr<OutOfLine>, y: Ptr<OutOfLine>) -> () {
    return ({ x.with(|__s| __s.a) } == { y.with(|__s| __s.a) });
}
pub fn operator_ne_7(x: Ptr<OutOfLine>, y: Ptr<OutOfLine>) -> () {
    return !({
        let _x: Ptr<OutOfLine> = (x).clone();
        let _y: Ptr<OutOfLine> = (y).clone();
        operator_eq_6(_x, _y)
    });
}
pub fn operator_eq_8(x: Ptr<Tmpl_int_>, y: Ptr<Tmpl_int_>) -> () {
    return ({ x.with(|__s| __s.v) } == { y.with(|__s| __s.v) });
}
pub fn operator_lt_9(x: Ptr<Tmpl_int_>, y: Ptr<Tmpl_int_>) -> () {
    return ({ x.with(|__s| __s.v) } < { y.with(|__s| __s.v) });
}
pub fn operator_eq_10(x: Ptr<Tmpl_int_>, y: Ptr<Tmpl_long_>) -> () {
    return ({ x.with(|__s| __s.v) } == { y.with(|__s| __s.v) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Tmpl_int_ {
    #[offset(0)]
    pub v: (),
}
impl CppLess for Tmpl_int_ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_9(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for Tmpl_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_8(__lhs.clone(), __rhs.clone())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Tmpl_long_ {
    #[offset(0)]
    pub v: (),
}
pub fn operator_eq_11(_a0: Ptr<TmplDefaulted_int_>, _a1: Ptr<TmplDefaulted_int_>) -> () {
    return ({ _a0.with(|__s| __s.v) } == { _a1.with(|__s| __s.v) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct TmplDefaulted_int_ {
    #[offset(0)]
    pub v: (),
}
impl CppEq for TmplDefaulted_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_11(__lhs.clone(), __rhs.clone())
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let d1: Value<Defaulted> = Rc::new(RefCell::new(Defaulted { a: 1_(), b: 2_() }));
    let d2: Value<Defaulted> = Rc::new(RefCell::new(Defaulted { a: 1_(), b: 2_() }));
    let d3: Value<Defaulted> = Rc::new(RefCell::new(Defaulted { a: 1_(), b: 3_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let o1: Value<DefaultedOrd> = Rc::new(RefCell::new(DefaultedOrd { a: 1_() }));
    let o2: Value<DefaultedOrd> = Rc::new(RefCell::new(DefaultedOrd { a: 2_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let i1: Value<Inline> = Rc::new(RefCell::new(Inline { a: 1_() }));
    let i2: Value<Inline> = Rc::new(RefCell::new(Inline { a: 2_() }));
    let i3: Value<Inline> = Rc::new(RefCell::new(Inline { a: 1_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let f1: Value<OutOfLine> = Rc::new(RefCell::new(OutOfLine { a: 4_() }));
    let f2: Value<OutOfLine> = Rc::new(RefCell::new(OutOfLine { a: 4_() }));
    let f3: Value<OutOfLine> = Rc::new(RefCell::new(OutOfLine { a: 5_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let t1: Value<Tmpl_int_> = Rc::new(RefCell::new(Tmpl_int_ { v: 1_() }));
    let t2: Value<Tmpl_int_> = Rc::new(RefCell::new(Tmpl_int_ { v: 2_() }));
    let t3: Value<Tmpl_int_> = Rc::new(RefCell::new(Tmpl_int_ { v: 1_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let u1: Value<Tmpl_long_> = Rc::new(RefCell::new(Tmpl_long_ { v: 1_() }));
    let u2: Value<Tmpl_long_> = Rc::new(RefCell::new(Tmpl_long_ { v: 2_() }));
    unimplemented!();
    unimplemented!();
    let v1: Value<TmplDefaulted_int_> = Rc::new(RefCell::new(TmplDefaulted_int_ { v: 7_() }));
    let v2: Value<TmplDefaulted_int_> = Rc::new(RefCell::new(TmplDefaulted_int_ { v: 7_() }));
    let v3: Value<TmplDefaulted_int_> = Rc::new(RefCell::new(TmplDefaulted_int_ { v: 8_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
