extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct X {
    #[offset(0)]
    pub v: i32,
}
impl X {
    pub fn copy_from(this: Ptr<X>, _a0: Ptr<X>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
    pub fn move_from(this: Ptr<X>, _a0: Ptr<X>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
    }
}
impl CppLess for X {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        ((operator_lt_0(
            Rc::new(RefCell::new(__lhs.read())),
            Rc::new(RefCell::new(__rhs.read())),
        )) != 0)
    }
}
impl CppEq for X {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        ((operator_eq_1(
            Rc::new(RefCell::new(__lhs.read())),
            Rc::new(RefCell::new(__rhs.read())),
        )) != 0)
    }
}
pub fn operator_eq_1(a: Value<X>, b: Value<X>) -> i32 {
    return if ({ (*a.borrow()).v } == { (*b.borrow()).v }) {
        2
    } else {
        0
    };
}
pub fn operator_ne_2(a: Value<X>, b: Value<X>) -> i32 {
    return if ({ (*a.borrow()).v } != { (*b.borrow()).v }) {
        3
    } else {
        0
    };
}
pub fn operator_lt_0(a: Value<X>, b: Value<X>) -> i32 {
    return if ({ (*a.borrow()).v } < { (*b.borrow()).v }) {
        4
    } else {
        0
    };
}
pub fn operator_gt_3(a: Value<X>, b: Value<X>) -> i32 {
    return if ({ (*a.borrow()).v } > { (*b.borrow()).v }) {
        5
    } else {
        0
    };
}
pub fn operator_le_4(a: Value<X>, b: Value<X>) -> i32 {
    return if ({ (*a.borrow()).v } <= { (*b.borrow()).v }) {
        6
    } else {
        0
    };
}
pub fn operator_ge_5(a: Value<X>, b: Value<X>) -> i32 {
    return if ({ (*a.borrow()).v } >= { (*b.borrow()).v }) {
        7
    } else {
        0
    };
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Result {
    #[offset(0)]
    pub r: i32,
}
impl Result {
    pub fn copy_from(this: Ptr<Result>, _a0: Ptr<Result>) {
        field!(this, r).write(_a0.with(|__s| __s.r));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Custom {
    #[offset(0)]
    pub v: i32,
}
impl Custom {
    pub fn copy_from(this: Ptr<Custom>, _a0: Ptr<Custom>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Mixed {
    #[offset(0)]
    pub v: i32,
}
impl Mixed {
    pub fn copy_from(this: Ptr<Mixed>, _a0: Ptr<Mixed>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
    pub fn move_from(this: Ptr<Mixed>, _a0: Ptr<Mixed>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
    }
}
impl CppLess for Mixed {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        ((MixedImpl::operator_lt(__lhs, __rhs.clone())) != 0)
    }
}
impl CppEq for Mixed {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        ((MixedImpl::operator_eq(__lhs, __rhs.clone())) != 0)
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(bool, to_bool)]
#[byte_size(1)]
pub struct MyBool {
    #[offset(0)]
    pub value: bool,
}
impl MyBool {
    pub fn new(this: Ptr<MyBool>, mut v: bool) {
        field!(this, value).write(v);
    }
    pub fn copy_from(this: Ptr<MyBool>, _a0: Ptr<MyBool>) {
        field!(this, value).write(_a0.with(|__s| __s.value));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Boolish {
    #[offset(0)]
    pub v: i32,
}
impl Boolish {
    pub fn copy_from(this: Ptr<Boolish>, _a0: Ptr<Boolish>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
    pub fn move_from(this: Ptr<Boolish>, _a0: Ptr<Boolish>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
    }
}
impl CppLess for Boolish {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        {
            let __cmp: Value<MyBool> = Rc::new(RefCell::new(<MyBool>::uninit()));
            operator_lt_6(
                __cmp.as_pointer(),
                Rc::new(RefCell::new(__lhs.read())),
                Rc::new(RefCell::new(__rhs.read())),
            );
            MyBoolImpl::to_bool(&__cmp.as_pointer())
        }
    }
}
impl CppEq for Boolish {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        {
            let __cmp: Value<MyBool> = Rc::new(RefCell::new(<MyBool>::uninit()));
            operator_eq_7(
                __cmp.as_pointer(),
                Rc::new(RefCell::new(__lhs.read())),
                Rc::new(RefCell::new(__rhs.read())),
            );
            MyBoolImpl::to_bool(&__cmp.as_pointer())
        }
    }
}
pub fn operator_eq_7(__ret: Ptr<MyBool>, a: Value<Boolish>, b: Value<Boolish>) {
    MyBool::new(__ret.clone(), {
        ({ (*a.borrow()).v } == { (*b.borrow()).v })
    });
    return;
}
pub fn operator_lt_6(__ret: Ptr<MyBool>, a: Value<Boolish>, b: Value<Boolish>) {
    MyBool::new(__ret.clone(), {
        ({ (*a.borrow()).v } < { (*b.borrow()).v })
    });
    return;
}
#[derive(Record, ByteRepr, Uninit, FnPtrArg, CppFrom, Default)]
#[cpp_into(bool, to_bool)]
#[byte_size(1)]
pub struct RefBool {
    #[offset(0)]
    pub value: bool,
}
impl RefBool {
    pub fn new(this: Ptr<RefBool>, mut v: bool) {
        field!(this, value).write(v);
    }
}
thread_local!(
    pub static ref_true_8: Value<RefBool> = {
        let __v: Value<RefBool> = Rc::new(RefCell::new(<RefBool>::uninit()));
        RefBool::new(__v.as_pointer(), { true });
        __v
    };
);
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct RefInt {
    #[offset(0)]
    pub v: i32,
}
impl RefInt {
    pub fn copy_from(this: Ptr<RefInt>, _a0: Ptr<RefInt>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
}
impl CppEq for RefInt {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        {
            let __cmp = RefIntImpl::operator_eq(__lhs, __rhs.clone());
            RefBoolImpl::to_bool(&__cmp)
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<X> = Rc::new(RefCell::new(<X>::uninit()));
    {
        let __d = a.as_pointer();
        field!(__d, v).write(1);
    };
    let b: Value<X> = Rc::new(RefCell::new(<X>::uninit()));
    {
        let __d = b.as_pointer();
        field!(__d, v).write(2);
    };
    let c: Value<X> = Rc::new(RefCell::new(<X>::uninit()));
    {
        let __d = c.as_pointer();
        field!(__d, v).write(1);
    };
    assert!(
        (({
            let _a: X = {
                let __v: Value<X> = Rc::new(RefCell::new(<X>::uninit()));
                X::copy_from(__v.as_pointer(), { a.as_pointer() });
                __v
            };
            operator_eq_1(_a, {
                let __v: Value<X> = Rc::new(RefCell::new(<X>::uninit()));
                X::copy_from(__v.as_pointer(), { c.as_pointer() });
                __v
            })
        }) == 2)
    );
    assert!(
        (({
            let _a: X = {
                let __v: Value<X> = Rc::new(RefCell::new(<X>::uninit()));
                X::copy_from(__v.as_pointer(), { a.as_pointer() });
                __v
            };
            operator_eq_1(_a, {
                let __v: Value<X> = Rc::new(RefCell::new(<X>::uninit()));
                X::copy_from(__v.as_pointer(), { b.as_pointer() });
                __v
            })
        }) == 0)
    );
    assert!(
        (({
            let _a: X = {
                let __v: Value<X> = Rc::new(RefCell::new(<X>::uninit()));
                X::copy_from(__v.as_pointer(), { a.as_pointer() });
                __v
            };
            operator_ne_2(_a, {
                let __v: Value<X> = Rc::new(RefCell::new(<X>::uninit()));
                X::copy_from(__v.as_pointer(), { b.as_pointer() });
                __v
            })
        }) == 3)
    );
    assert!(
        (({
            let _a: X = {
                let __v: Value<X> = Rc::new(RefCell::new(<X>::uninit()));
                X::copy_from(__v.as_pointer(), { a.as_pointer() });
                __v
            };
            operator_lt_0(_a, {
                let __v: Value<X> = Rc::new(RefCell::new(<X>::uninit()));
                X::copy_from(__v.as_pointer(), { b.as_pointer() });
                __v
            })
        }) == 4)
    );
    assert!(
        (({
            let _a: X = {
                let __v: Value<X> = Rc::new(RefCell::new(<X>::uninit()));
                X::copy_from(__v.as_pointer(), { b.as_pointer() });
                __v
            };
            operator_gt_3(_a, {
                let __v: Value<X> = Rc::new(RefCell::new(<X>::uninit()));
                X::copy_from(__v.as_pointer(), { a.as_pointer() });
                __v
            })
        }) == 5)
    );
    assert!(
        (({
            let _a: X = {
                let __v: Value<X> = Rc::new(RefCell::new(<X>::uninit()));
                X::copy_from(__v.as_pointer(), { a.as_pointer() });
                __v
            };
            operator_le_4(_a, {
                let __v: Value<X> = Rc::new(RefCell::new(<X>::uninit()));
                X::copy_from(__v.as_pointer(), { c.as_pointer() });
                __v
            })
        }) == 6)
    );
    assert!(
        (({
            let _a: X = {
                let __v: Value<X> = Rc::new(RefCell::new(<X>::uninit()));
                X::copy_from(__v.as_pointer(), { a.as_pointer() });
                __v
            };
            operator_ge_5(_a, {
                let __v: Value<X> = Rc::new(RefCell::new(<X>::uninit()));
                X::copy_from(__v.as_pointer(), { c.as_pointer() });
                __v
            })
        }) == 7)
    );
    let xs: Value<Box<[X]>> = Rc::new(RefCell::new(
        (0..3).map(|_| <X>::uninit()).collect::<Box<[_]>>(),
    ));
    {
        let __d = xs.decay();
        {
            let __d = __d.offset(0);
            field!(__d, v).write(3);
        };
        {
            let __d = __d.offset(1);
            field!(__d, v).write(1);
        };
        {
            let __d = __d.offset(2);
            field!(__d, v).write(2);
        };
    };
    {
        let mut __tmp_0 = { xs.decay() };
        let __n = xs.decay().offset((3) as isize).iter_distance(&__tmp_0);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| cpp_cmp(&__tmp_0.iter_at(__x), &__tmp_0.iter_at(__y)));
        reorder(__order, |__i| __tmp_0.iter_at(__i));
    };
    assert!(
        ({
            (({ (*xs.borrow())[(0) as usize].v } == 1) && ({ (*xs.borrow())[(1) as usize].v } == 2))
                && ({ (*xs.borrow())[(2) as usize].v } == 3)
        })
    );
    let p: Value<Custom> = Rc::new(RefCell::new(<Custom>::uninit()));
    {
        let __d = p.as_pointer();
        field!(__d, v).write(5);
    };
    let q: Value<Custom> = Rc::new(RefCell::new(<Custom>::uninit()));
    {
        let __d = q.as_pointer();
        field!(__d, v).write(2);
    };
    let r: Value<Result> = Rc::new(RefCell::new(<Result>::uninit()));
    ({ CustomImpl::operator_cmp(&p.as_pointer(), r.as_pointer(), q.as_pointer()) });
    assert!(({ ({ (*r.borrow()).r } == 3) }));
    let m: Value<Mixed> = Rc::new(RefCell::new(<Mixed>::uninit()));
    {
        let __d = m.as_pointer();
        field!(__d, v).write(4);
    };
    let n: Value<Mixed> = Rc::new(RefCell::new(<Mixed>::uninit()));
    {
        let __d = n.as_pointer();
        field!(__d, v).write(4);
    };
    assert!((({ MixedImpl::operator_eq(&m.as_pointer(), n.as_pointer(),) }) == 9));
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<Result> = Rc::new(RefCell::new(<Result>::uninit()));
                ({ MixedImpl::operator_cmp(&m.as_pointer(), __v.as_pointer(), n.as_pointer()) });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .r
        } == 0)
    );
    let ms: Value<Box<[Mixed]>> = Rc::new(RefCell::new(
        (0..3).map(|_| <Mixed>::uninit()).collect::<Box<[_]>>(),
    ));
    {
        let __d = ms.decay();
        {
            let __d = __d.offset(0);
            field!(__d, v).write(6);
        };
        {
            let __d = __d.offset(1);
            field!(__d, v).write(4);
        };
        {
            let __d = __d.offset(2);
            field!(__d, v).write(5);
        };
    };
    {
        let mut __tmp_1 = { ms.decay() };
        let __n = ms.decay().offset((3) as isize).iter_distance(&__tmp_1);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| cpp_cmp(&__tmp_1.iter_at(__x), &__tmp_1.iter_at(__y)));
        reorder(__order, |__i| __tmp_1.iter_at(__i));
    };
    assert!(
        ({
            (({ (*ms.borrow())[(0) as usize].v } == 4) && ({ (*ms.borrow())[(1) as usize].v } == 5))
                && ({ (*ms.borrow())[(2) as usize].v } == 6)
        })
    );
    assert!(
        (({
            let _o: Ptr<Mixed> = ms.decay().offset(1);
            MixedImpl::operator_lt(&ms.decay().offset(0), _o)
        }) == 8)
    );
    let b1: Value<Boolish> = Rc::new(RefCell::new(<Boolish>::uninit()));
    {
        let __d = b1.as_pointer();
        field!(__d, v).write(1);
    };
    let b2: Value<Boolish> = Rc::new(RefCell::new(<Boolish>::uninit()));
    {
        let __d = b2.as_pointer();
        field!(__d, v).write(2);
    };
    assert!(
        ({
            MyBoolImpl::to_bool(
                &{
                    let __v: Value<MyBool> = Rc::new(RefCell::new(<MyBool>::uninit()));
                    ({
                        let _a: Boolish = {
                            let __v: Value<Boolish> = Rc::new(RefCell::new(<Boolish>::uninit()));
                            Boolish::copy_from(__v.as_pointer(), { b1.as_pointer() });
                            __v
                        };
                        operator_eq_7(__v.as_pointer(), _a, {
                            let __v: Value<Boolish> = Rc::new(RefCell::new(<Boolish>::uninit()));
                            {
                                let __d = __v.as_pointer();
                                field!(__d, v).write(1);
                            };
                            __v
                        })
                    });
                    __v
                }
                .as_pointer(),
            )
        })
    );
    assert!(
        !({
            MyBoolImpl::to_bool(
                &{
                    let __v: Value<MyBool> = Rc::new(RefCell::new(<MyBool>::uninit()));
                    ({
                        let _a: Boolish = {
                            let __v: Value<Boolish> = Rc::new(RefCell::new(<Boolish>::uninit()));
                            Boolish::copy_from(__v.as_pointer(), { b2.as_pointer() });
                            __v
                        };
                        operator_lt_6(__v.as_pointer(), _a, {
                            let __v: Value<Boolish> = Rc::new(RefCell::new(<Boolish>::uninit()));
                            Boolish::copy_from(__v.as_pointer(), { b1.as_pointer() });
                            __v
                        })
                    });
                    __v
                }
                .as_pointer(),
            )
        })
    );
    let lt: Value<MyBool> = Rc::new(RefCell::new(<MyBool>::uninit()));
    ({
        let _a: Boolish = {
            let __v: Value<Boolish> = Rc::new(RefCell::new(<Boolish>::uninit()));
            Boolish::copy_from(__v.as_pointer(), { b1.as_pointer() });
            __v
        };
        operator_lt_6(lt.as_pointer(), _a, {
            let __v: Value<Boolish> = Rc::new(RefCell::new(<Boolish>::uninit()));
            Boolish::copy_from(__v.as_pointer(), { b2.as_pointer() });
            __v
        })
    });
    assert!(({ { (*lt.borrow()).value } }));
    let bs: Value<Box<[Boolish]>> = Rc::new(RefCell::new(
        (0..3).map(|_| <Boolish>::uninit()).collect::<Box<[_]>>(),
    ));
    {
        let __d = bs.decay();
        {
            let __d = __d.offset(0);
            field!(__d, v).write(3);
        };
        {
            let __d = __d.offset(1);
            field!(__d, v).write(1);
        };
        {
            let __d = __d.offset(2);
            field!(__d, v).write(2);
        };
    };
    {
        let mut __tmp_2 = { bs.decay() };
        let __n = bs.decay().offset((3) as isize).iter_distance(&__tmp_2);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| cpp_cmp(&__tmp_2.iter_at(__x), &__tmp_2.iter_at(__y)));
        reorder(__order, |__i| __tmp_2.iter_at(__i));
    };
    assert!(
        ({
            (({ (*bs.borrow())[(0) as usize].v } == 1) && ({ (*bs.borrow())[(1) as usize].v } == 2))
                && ({ (*bs.borrow())[(2) as usize].v } == 3)
        })
    );
    let ri1: Value<RefInt> = Rc::new(RefCell::new(<RefInt>::uninit()));
    {
        let __d = ri1.as_pointer();
        field!(__d, v).write(1);
    };
    let ri2: Value<RefInt> = Rc::new(RefCell::new(<RefInt>::uninit()));
    {
        let __d = ri2.as_pointer();
        field!(__d, v).write(2);
    };
    assert!(
        ({
            RefBoolImpl::to_bool(
                &({ RefIntImpl::operator_eq(&ri1.as_pointer(), ri2.as_pointer()) }),
            )
        })
    );
    let ris: Value<Box<[RefInt]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <RefInt>::uninit()).collect::<Box<[_]>>(),
    ));
    {
        let __d = ris.decay();
        {
            let __d = __d.offset(0);
            field!(__d, v).write(1);
        };
        {
            let __d = __d.offset(1);
            field!(__d, v).write(2);
        };
    };
    assert!(
        ({
            let mut __tmp_3 = { ris.decay().offset((2) as isize) };
            let mut __it = ris.decay();
            while __it != __tmp_3 && !CppEq::equal(&__it.iter_deref(), &ri2.as_pointer()) {
                __it.iter_inc();
            }
            __it
        } == ris.decay())
    );
    return 0;
}
pub trait CustomImpl {
    fn operator_cmp(&self, __ret: Ptr<Result>, o: Ptr<Custom>);
}
impl CustomImpl for Ptr<Custom> {
    fn operator_cmp(&self, __ret: Ptr<Result>, o: Ptr<Custom>) {
        {
            let __d = __ret.clone();
            field!(__d, r).write(({ (*self).with(|__s| __s.v) } - { o.with(|__s| __s.v) }));
        };
        return;
    }
}
pub trait MixedImpl {
    fn operator_cmp(&self, __ret: Ptr<Result>, o: Ptr<Mixed>);
    fn operator_lt(&self, o: Ptr<Mixed>) -> i32;
    fn operator_eq(&self, o: Ptr<Mixed>) -> i32;
}
impl MixedImpl for Ptr<Mixed> {
    fn operator_cmp(&self, __ret: Ptr<Result>, o: Ptr<Mixed>) {
        {
            let __d = __ret.clone();
            field!(__d, r).write(({ (*self).with(|__s| __s.v) } - { o.with(|__s| __s.v) }));
        };
        return;
    }
    fn operator_lt(&self, o: Ptr<Mixed>) -> i32 {
        return if ({ (*self).with(|__s| __s.v) } < { o.with(|__s| __s.v) }) {
            8
        } else {
            0
        };
    }
    fn operator_eq(&self, o: Ptr<Mixed>) -> i32 {
        return if ({ (*self).with(|__s| __s.v) } == { o.with(|__s| __s.v) }) {
            9
        } else {
            0
        };
    }
}
pub trait MyBoolImpl {
    fn to_bool(&self) -> bool;
}
impl MyBoolImpl for Ptr<MyBool> {
    fn to_bool(&self) -> bool {
        return (*self).with(|__s| __s.value);
    }
}
pub trait RefBoolImpl {
    fn to_bool(&self) -> bool;
}
impl RefBoolImpl for Ptr<RefBool> {
    fn to_bool(&self) -> bool {
        return (*self).with(|__s| __s.value);
    }
}
pub trait RefIntImpl {
    fn operator_eq(&self, _a0: Ptr<RefInt>) -> Ptr<RefBool>;
}
impl RefIntImpl for Ptr<RefInt> {
    fn operator_eq(&self, _a0: Ptr<RefInt>) -> Ptr<RefBool> {
        return ref_true_8.with(|v| v.as_pointer());
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = ref_true_8.with(|_| ());
}
