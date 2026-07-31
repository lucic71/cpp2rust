extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Pair {
    pub first: i32,
    pub second: i32,
}
impl Pair {
    pub fn NOP(&self) {}
    pub fn GetFirst(&self) -> i32 {
        return self.first;
    }
    pub fn GetSecond(&self) -> i32 {
        return self.second;
    }
    pub fn Set(&self, field: Ptr<i32>, new_val: i32) -> i32 {
        let new_val: Value<i32> = Rc::new(RefCell::new(new_val));
        ({ self.NOP() });
        let old_val: Value<i32> = Rc::new(RefCell::new((field.read())));
        let __rhs = (*new_val.borrow());
        field.write(__rhs);
        return (*old_val.borrow());
    }
    pub fn SetFirst(&self, new_first: i32) -> i32 {
        let new_first: Value<i32> = Rc::new(RefCell::new(new_first));
        return (({ self.GetFirst() })
            + ({
                let _field: Ptr<i32> = self.field_ptr(
                    0,
                    |__v: &Pair| ::std::slice::from_ref(&__v.first),
                    |__v: &mut Pair| ::std::slice::from_mut(&mut __v.first),
                );
                self.Set(_field, (*new_first.borrow()))
            }));
    }
    pub fn SetSecond(&self, new_second: i32) -> i32 {
        let new_second: Value<i32> = Rc::new(RefCell::new(new_second));
        return (({ self.GetSecond() })
            + ({
                let _field: Ptr<i32> = self.field_ptr(
                    4,
                    |__v: &Pair| ::std::slice::from_ref(&__v.second),
                    |__v: &mut Pair| ::std::slice::from_mut(&mut __v.second),
                );
                self.Set(_field, (*new_second.borrow()))
            }));
    }
}
impl Clone for Pair {
    fn clone(&self) -> Self {
        let mut this = Self {
            first: self.first,
            second: self.second,
        };
        this
    }
}
impl ByteRepr for Pair {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.first.to_bytes(&mut buf[0..4]);
        self.second.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            first: <i32>::from_bytes(&buf[0..4]),
            second: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
#[derive(Default)]
pub struct Route {
    pub path: Pair,
    pub cost: f64,
}
impl Route {
    pub fn SetCost(&self, new_cost: f64) -> f64 {
        let new_cost: Value<f64> = Rc::new(RefCell::new(new_cost));
        let old_cost: Value<f64> = Rc::new(RefCell::new(self.cost));
        self.cost = (*new_cost.borrow());
        return (*old_cost.borrow());
    }
}
impl Clone for Route {
    fn clone(&self) -> Self {
        let mut this = Self {
            path: (self.path).clone(),
            cost: self.cost,
        };
        this
    }
}
impl ByteRepr for Route {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.path.to_bytes(&mut buf[0..8]);
        self.cost.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            path: <Pair>::from_bytes(&buf[0..8]),
            cost: <f64>::from_bytes(&buf[8..16]),
        }
    }
}
pub fn RandomRoute_0(route: Ptr<Route>) -> i32 {
    if (((*route.upgrade().deref()).path.first % 2) != 0) {
        return ({
            let _new_first: i32 = ({ (*route.upgrade().deref()).path.SetSecond(10) });
            (*route.upgrade().deref()).path.SetFirst(_new_first)
        });
    } else {
        return ({
            let _new_second: i32 = ({ (*route.upgrade().deref()).path.SetFirst(-10_i32) });
            (*route.upgrade().deref()).path.SetSecond(_new_second)
        });
    }
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let route1: Value<Route> = Rc::new(RefCell::new(Route {
        path: Pair {
            first: 0,
            second: 1,
        },
        cost: 5_f64,
    }));
    let route2: Value<Route> = Rc::new(RefCell::new(Route {
        path: Pair {
            first: 1,
            second: 0,
        },
        cost: 10_f64,
    }));
    let old_cost: Value<f64> = Rc::new(RefCell::new(
        ({ (*route1.borrow()).SetCost(({ (*route2.borrow()).SetCost(15_f64) })) }),
    ));
    return ((((({ RandomRoute_0(route1.as_pointer()) }) + ({ RandomRoute_0(route2.as_pointer()) }))
        as f64)
        + (*old_cost.borrow())) as i32);
}
