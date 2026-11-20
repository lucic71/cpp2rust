extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
impl Pair {
    pub fn new_1(this: Ptr<Pair>) {}
    pub fn copy_from(this: Ptr<Pair>, _a0: Ptr<Pair>) {
        field!(this, first).write(_a0.with(|__s| __s.first));
        field!(this, second).write(_a0.with(|__s| __s.second));
    }
    pub fn move_from(this: Ptr<Pair>, _a0: Ptr<Pair>) {
        field!(this, first).write({ (*_a0.upgrade().deref()).first });
        field!(this, second).write({ (*_a0.upgrade().deref()).second });
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Route {
    #[offset(0)]
    #[byte_size(8)]
    pub path: Pair,
    #[offset(8)]
    pub cost: f64,
}
impl Route {
    pub fn new_1(this: Ptr<Route>) {
        Pair::new_1(field_ptr!(this, path));
    }
    pub fn copy_from(this: Ptr<Route>, _a0: Ptr<Route>) {
        Pair::copy_from(field_ptr!(this, path), { field_ptr!(_a0, path) });
        field!(this, cost).write(_a0.with(|__s| __s.cost));
    }
    pub fn move_from(this: Ptr<Route>, _a0: Ptr<Route>) {
        Pair::move_from(field_ptr!(this, path), { field_ptr!(_a0, path) });
        field!(this, cost).write({ (*_a0.upgrade().deref()).cost });
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Counter {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    pub calls: i32,
}
impl Counter {
    pub fn new_1(this: Ptr<Counter>) {}
    pub fn copy_from(this: Ptr<Counter>, _a0: Ptr<Counter>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
        field!(this, calls).write(_a0.with(|__s| __s.calls));
    }
    pub fn move_from(this: Ptr<Counter>, _a0: Ptr<Counter>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
        field!(this, calls).write({ (*_a0.upgrade().deref()).calls });
    }
}
impl CppEq for Counter {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        CounterImpl::operator_eq(__lhs, __rhs.clone())
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Tally {
    #[offset(0)]
    pub calls: i32,
}
impl Tally {
    pub fn new_1(this: Ptr<Tally>) {}
    pub fn copy_from(this: Ptr<Tally>, _a0: Ptr<Tally>) {
        field!(this, calls).write(_a0.with(|__s| __s.calls));
    }
    pub fn move_from(this: Ptr<Tally>, _a0: Ptr<Tally>) {
        field!(this, calls).write({ (*_a0.upgrade().deref()).calls });
    }
}
pub fn RandomRoute_0(route: Ptr<Route>) -> i32 {
    if ((route.with(|__s| __s.path.first) % 2) != 0) {
        return ({
            let _new_first: i32 = ({ PairImpl::SetSecond(&field_ptr!(route, path), 10) });
            PairImpl::SetFirst(&field_ptr!(route, path), _new_first)
        });
    } else {
        return ({
            let _new_second: i32 = ({ PairImpl::SetFirst(&field_ptr!(route, path), -10_i32) });
            PairImpl::SetSecond(&field_ptr!(route, path), _new_second)
        });
    }
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let route1: Value<Route> = Rc::new(RefCell::new(<Route>::uninit()));
    {
        let __d = route1.as_pointer();
        {
            let __d = field_ptr!(__d, path);
            field!(__d, first).write(0);
            field!(__d, second).write(1);
        };
        field!(__d, cost).write(5_f64);
    };
    let route2: Value<Route> = Rc::new(RefCell::new(<Route>::uninit()));
    {
        let __d = route2.as_pointer();
        {
            let __d = field_ptr!(__d, path);
            field!(__d, first).write(1);
            field!(__d, second).write(0);
        };
        field!(__d, cost).write(10_f64);
    };
    let mut old_cost: f64 = ({
        RouteImpl::SetCost(
            &route1.as_pointer(),
            ({ RouteImpl::SetCost(&route2.as_pointer(), 15_f64) }),
        )
    });
    assert!(
        ((((({ RandomRoute_0(route1.as_pointer(),) }) + ({ RandomRoute_0(route2.as_pointer(),) }))
            as f64)
            + old_cost)
            == 9_f64)
    );
    let c1: Value<Counter> = Rc::new(RefCell::new(<Counter>::uninit()));
    {
        let __d = c1.as_pointer();
        field!(__d, v).write(3);
        field!(__d, calls).write(0);
    };
    let c2: Value<Counter> = Rc::new(RefCell::new(<Counter>::uninit()));
    {
        let __d = c2.as_pointer();
        field!(__d, v).write(3);
        field!(__d, calls).write(0);
    };
    let mut pc: Ptr<Counter> = (c1.as_pointer());
    assert!((({ CounterImpl::Get(&c1.as_pointer(),) }) == 3));
    assert!((({ CounterImpl::Get(&c2.as_pointer(),) }) == 3));
    assert!((({ CounterImpl::Get(&pc,) }) == 3));
    assert!(({ CounterImpl::operator_eq(&c1.as_pointer(), c2.as_pointer(),) }));
    assert!(({ CounterImpl::operator_eq(&c2.as_pointer(), c1.as_pointer(),) }));
    assert!(({ ({ (*c1.borrow()).calls } == 3) }));
    assert!(({ ({ (*c2.borrow()).calls } == 2) }));
    let t: Value<Tally> = Rc::new(RefCell::new(<Tally>::uninit()));
    {
        let __d = t.as_pointer();
        field!(__d, calls).write(0);
    };
    let ct: Value<Tally> = Rc::new(RefCell::new(<Tally>::uninit()));
    {
        let __d = ct.as_pointer();
        field!(__d, calls).write(0);
    };
    let rt: Ptr<Tally> = t.as_pointer();
    assert!((({ TallyImpl::SameNameBetweenConstAndNonConst_1(&t.as_pointer(),) }) == 1));
    assert!((({ TallyImpl::SameNameBetweenConstAndNonConst_2(&ct.as_pointer(),) }) == 2));
    assert!((({ TallyImpl::SameNameBetweenConstAndNonConst_2(&rt,) }) == 2));
    assert!(({ ({ (*t.borrow()).calls } == 11) }));
    assert!(({ ({ (*ct.borrow()).calls } == 10) }));
    return 0;
}
pub trait CounterImpl {
    fn Get(&self) -> i32;
    fn operator_eq(&self, o: Ptr<Counter>) -> bool;
}
impl CounterImpl for Ptr<Counter> {
    fn Get(&self) -> i32 {
        field!((*self), calls).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).with(|__s| __s.v);
    }
    fn operator_eq(&self, o: Ptr<Counter>) -> bool {
        field!((*self), calls).with_mut(|__v| {
            __v.prefix_inc();
        });
        return ({ (*self).with(|__s| __s.v) } == { o.with(|__s| __s.v) });
    }
}
pub trait PairImpl {
    fn NOP(&self);
    fn GetFirst(&self) -> i32;
    fn GetSecond(&self) -> i32;
    fn Set(&self, field: Ptr<i32>, new_val: i32) -> i32;
    fn SetFirst(&self, new_first: i32) -> i32;
    fn SetSecond(&self, new_second: i32) -> i32;
}
impl PairImpl for Ptr<Pair> {
    fn NOP(&self) {}
    fn GetFirst(&self) -> i32 {
        return (*self).with(|__s| __s.first);
    }
    fn GetSecond(&self) -> i32 {
        return (*self).with(|__s| __s.second);
    }
    fn Set(&self, field: Ptr<i32>, mut new_val: i32) -> i32 {
        ({ PairImpl::NOP(self) });
        let mut old_val: i32 = (field.read());
        field.write({ new_val });
        return old_val;
    }
    fn SetFirst(&self, mut new_first: i32) -> i32 {
        return (({ PairImpl::GetFirst(self) })
            + ({
                let _field: Ptr<i32> = field_ptr!((*self), first);
                PairImpl::Set(self, _field, new_first)
            }));
    }
    fn SetSecond(&self, mut new_second: i32) -> i32 {
        return (({ PairImpl::GetSecond(self) })
            + ({
                let _field: Ptr<i32> = field_ptr!((*self), second);
                PairImpl::Set(self, _field, new_second)
            }));
    }
}
pub trait RouteImpl {
    fn SetCost(&self, new_cost: f64) -> f64;
}
impl RouteImpl for Ptr<Route> {
    fn SetCost(&self, mut new_cost: f64) -> f64 {
        let mut old_cost: f64 = (*self).with(|__s| __s.cost);
        field!((*self), cost).write(new_cost);
        return old_cost;
    }
}
pub trait TallyImpl {
    fn SameNameBetweenConstAndNonConst_1(&self) -> i32;
    fn SameNameBetweenConstAndNonConst_2(&self) -> i32;
}
impl TallyImpl for Ptr<Tally> {
    fn SameNameBetweenConstAndNonConst_1(&self) -> i32 {
        {
            field!((*self), calls).with_mut(|__v| *__v = *__v + 1)
        };
        return 1;
    }
    fn SameNameBetweenConstAndNonConst_2(&self) -> i32 {
        {
            field!((*self), calls).with_mut(|__v| *__v = *__v + 10)
        };
        return 2;
    }
}
pub fn __cpp2rust_init_globals() {}
