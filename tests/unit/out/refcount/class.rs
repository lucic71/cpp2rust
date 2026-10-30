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
pub struct Pair {
    #[offset(0)]
    pub first: (),
    #[offset(4)]
    pub second: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Route {
    #[offset(0)]
    #[byte_size(8)]
    pub path: Pair,
    #[offset(8)]
    pub cost: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Counter {
    #[offset(0)]
    pub v: (),
    #[offset(4)]
    pub calls: (),
}
impl CppEq for Counter {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        CounterImpl::operator_eq(__lhs, __rhs.clone())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Tally {
    #[offset(0)]
    pub calls: (),
}
pub fn RandomRoute_0(route: Ptr<Route>) -> () {
    if ((route.with(|__s| __s.path.first) % 2_()) != 0) {
        return ({
            let _new_first: () = ({ PairImpl::SetSecond(&field_ptr!(route, path), 10_()) });
            PairImpl::SetFirst(&field_ptr!(route, path), _new_first)
        });
    } else {
        return ({
            let _new_second: () = ({ PairImpl::SetFirst(&field_ptr!(route, path), -10_()) });
            PairImpl::SetSecond(&field_ptr!(route, path), _new_second)
        });
    }
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let route1: Value<Route> = Rc::new(RefCell::new(Route {
        path: Pair {
            first: 0_(),
            second: 1_(),
        },
        cost: 5_(),
    }));
    let route2: Value<Route> = Rc::new(RefCell::new(Route {
        path: Pair {
            first: 1_(),
            second: 0_(),
        },
        cost: 10_(),
    }));
    let mut old_cost: () = ({
        RouteImpl::SetCost(
            &route1.as_pointer(),
            ({ RouteImpl::SetCost(&route2.as_pointer(), 15_()) }),
        )
    });
    unimplemented!();
    let c1: Value<Counter> = Rc::new(RefCell::new(Counter {
        v: 3_(),
        calls: 0_(),
    }));
    let c2: Value<Counter> = Rc::new(RefCell::new(Counter {
        v: 3_(),
        calls: 0_(),
    }));
    let mut pc: Ptr<Counter> = (c1.as_pointer());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let t: Value<Tally> = Rc::new(RefCell::new(Tally { calls: 0_() }));
    let ct: Value<Tally> = Rc::new(RefCell::new(Tally { calls: 0_() }));
    let rt: Ptr<Tally> = t.as_pointer();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait CounterImpl {
    fn Get(&self) -> ();
    fn operator_eq(&self, o: Ptr<Counter>) -> ();
}
impl CounterImpl for Ptr<Counter> {
    fn Get(&self) -> () {
        field!((*self), calls).with_mut(|__v| __v.prefix_inc());
        return (*self).with(|__s| __s.v);
    }
    fn operator_eq(&self, o: Ptr<Counter>) -> () {
        field!((*self), calls).with_mut(|__v| __v.prefix_inc());
        return ({ (*self).with(|__s| __s.v) } == { o.with(|__s| __s.v) });
    }
}
pub trait PairImpl {
    fn NOP(&self);
    fn GetFirst(&self) -> ();
    fn GetSecond(&self) -> ();
    fn Set(&self, field: Ptr<()>, new_val: ()) -> ();
    fn SetFirst(&self, new_first: ()) -> ();
    fn SetSecond(&self, new_second: ()) -> ();
}
impl PairImpl for Ptr<Pair> {
    fn NOP(&self) {}
    fn GetFirst(&self) -> () {
        return (*self).with(|__s| __s.first);
    }
    fn GetSecond(&self) -> () {
        return (*self).with(|__s| __s.second);
    }
    fn Set(&self, field: Ptr<()>, mut new_val: ()) -> () {
        ({ PairImpl::NOP(self) });
        let old_val: Value<()> = Rc::new(RefCell::new((field.read())));
        field.write({ new_val });
        return (*old_val.borrow());
    }
    fn SetFirst(&self, mut new_first: ()) -> () {
        return (({ PairImpl::GetFirst(self) })
            + ({
                let _field: Ptr<()> = field_ptr!((*self), first);
                PairImpl::Set(self, _field, new_first)
            }));
    }
    fn SetSecond(&self, mut new_second: ()) -> () {
        return (({ PairImpl::GetSecond(self) })
            + ({
                let _field: Ptr<()> = field_ptr!((*self), second);
                PairImpl::Set(self, _field, new_second)
            }));
    }
}
pub trait RouteImpl {
    fn SetCost(&self, new_cost: ()) -> ();
}
impl RouteImpl for Ptr<Route> {
    fn SetCost(&self, mut new_cost: ()) -> () {
        let old_cost: Value<()> = Rc::new(RefCell::new((*self).with(|__s| __s.cost)));
        field!((*self), cost).write(new_cost);
        return (*old_cost.borrow());
    }
}
pub trait TallyImpl {
    fn SameNameBetweenConstAndNonConst_1(&self) -> ();
    fn SameNameBetweenConstAndNonConst_2(&self) -> ();
}
impl TallyImpl for Ptr<Tally> {
    fn SameNameBetweenConstAndNonConst_1(&self) -> () {
        {
            field!((*self), calls).with_mut(|__v| *__v = *__v + 1_())
        };
        return 1_();
    }
    fn SameNameBetweenConstAndNonConst_2(&self) -> () {
        {
            field!((*self), calls).with_mut(|__v| *__v = *__v + 10_())
        };
        return 2_();
    }
}
pub fn __cpp2rust_init_globals() {}
