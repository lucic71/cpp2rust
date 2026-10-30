extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static default_constructions_0: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Key {
    #[offset(0)]
    pub order: (),
    #[offset(4)]
    pub identity: (),
}
impl CppLess for Key {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_1(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for Key {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_2(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_lt_1(a: Ptr<Key>, b: Ptr<Key>) -> () {
    return ({ a.with(|__s| __s.order) } < { b.with(|__s| __s.order) });
}
pub fn operator_eq_2(a: Ptr<Key>, b: Ptr<Key>) -> () {
    return ({ a.with(|__s| __s.identity) } == { b.with(|__s| __s.identity) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Mapped {
    #[offset(0)]
    pub value: (),
}
impl Mapped {
    pub fn new() -> Self {
        let __this: Mapped = Self { value: 0_() };
        (*default_constructions_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Default for Mapped {
    fn default() -> Self {
        { Mapped::new() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut ok: () = true;
    let distinct_a: Value<Key> = Rc::new(RefCell::new(Key {
        order: 1_(),
        identity: 7_(),
    }));
    let distinct_b: Value<Key> = Rc::new(RefCell::new(Key {
        order: 2_(),
        identity: 7_(),
    }));
    ok &= ({
        let _a: Ptr<Key> = distinct_a.as_pointer();
        operator_eq_2(_a, distinct_b.as_pointer())
    });
    let distinct: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let a: Ptr<Mapped> = unimplemented!();
    field!(a, value).write(11_());
    let a_it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut a_value: Ptr<Mapped> = (field_ptr!(a_it.as_pointer(), second));
    ok &= (default_constructions_0.with(|rc| *rc.borrow()) == 1_());
    let b: Ptr<Mapped> = unimplemented!();
    field!(b, value).write(22_());
    let b_it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ok &= (default_constructions_0.with(|rc| *rc.borrow()) == 2_());
    ok &= (unimplemented!() == 2_());
    ok &= ({ (a).clone() } != { (b).clone() });
    ok &= ((a.with(|__s| __s.value) == 11_()) && (b.with(|__s| __s.value) == 22_()));
    ok &= (unimplemented!());
    ok &= (unimplemented!());
    ok &= (!(unimplemented!()));
    ok &= ({ (field_ptr!(a_it.as_pointer(), second)) } == { (a_value).clone() });
    ok &= ({ ((*a_it.borrow()).read()).second.value } == 11_());
    ok &= ({
        let _a: Ptr<Key> = distinct_a.as_pointer();
        operator_eq_2(_a, distinct_b.as_pointer())
    });
    let mut distinct_count: () = 0_();
    'loop_: for entry in IterRange::new(unimplemented!(), unimplemented!()) {
        let entry: Ptr<()> = unimplemented!();
        if (distinct_count == 0_()) {
            ok &= ((entry.with(|__s| __s.first.order) == 1_())
                && (entry.with(|__s| __s.second.value) == 11_()));
        } else if (distinct_count == 1_()) {
            ok &= ((entry.with(|__s| __s.first.order) == 2_())
                && (entry.with(|__s| __s.second.value) == 22_()));
        }
        distinct_count.prefix_inc();
    }
    ok &= (distinct_count == 2_());
    let equivalent_a: Value<Key> = Rc::new(RefCell::new(Key {
        order: 3_(),
        identity: 8_(),
    }));
    let equivalent_b: Value<Key> = Rc::new(RefCell::new(Key {
        order: 3_(),
        identity: 9_(),
    }));
    ok &= !({
        let _a: Ptr<Key> = equivalent_a.as_pointer();
        operator_eq_2(_a, equivalent_b.as_pointer())
    });
    let equivalent: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut before_equivalent: () = default_constructions_0.with(|rc| *rc.borrow());
    let first: Ptr<Mapped> = unimplemented!();
    field!(first, value).write(31_());
    let first_it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut first_value: Ptr<Mapped> = (field_ptr!(first_it.as_pointer(), second));
    ok &= (default_constructions_0.with(|rc| *rc.borrow()) == (before_equivalent + 1_()));
    let second: Ptr<Mapped> = unimplemented!();
    let mut reused: () =
        (default_constructions_0.with(|rc| *rc.borrow()) == (before_equivalent + 1_()));
    ok &= reused;
    ok &= (unimplemented!() == 1_());
    if reused {
        ok &= (!(unimplemented!()));
        ok &= ({ (first).clone() } == { (second).clone() });
        ok &= (unimplemented!());
        ok &= (unimplemented!());
        ok &= ({ (field_ptr!(first_it.as_pointer(), second)) } == { (first_value).clone() });
        ok &= (({ ((*first_it.borrow()).read()).second.value } == 31_())
            && (second.with(|__s| __s.value) == 31_()));
        let mut equivalent_count: () = 0_();
        'loop_: for entry in IterRange::new(unimplemented!(), unimplemented!()) {
            let entry: Ptr<()> = unimplemented!();
            ok &= ((entry.with(|__s| __s.first.order) == 3_())
                && (entry.with(|__s| __s.second.value) == 31_()));
            equivalent_count.prefix_inc();
        }
        ok &= (equivalent_count == 1_());
    }
    ok &= !({
        let _a: Ptr<Key> = equivalent_a.as_pointer();
        operator_eq_2(_a, equivalent_b.as_pointer())
    });
    let mut later_key: Key = Key {
        order: 4_(),
        identity: 10_(),
    };
    let later: Ptr<Mapped> = unimplemented!();
    field!(later, value).write(41_());
    ok &= (default_constructions_0.with(|rc| *rc.borrow()) == (before_equivalent + 2_()));
    ok &= (unimplemented!() == 2_());
    if reused {
        ok &= (unimplemented!());
        ok &= ({ (field_ptr!(first_it.as_pointer(), second)) } == { (first_value).clone() });
        ok &= ({ ((*first_it.borrow()).read()).second.value } == 31_());
    }
    let mut matching_a: Key = Key {
        order: 5_(),
        identity: 5_(),
    };
    let mut matching_b: Key = Key {
        order: 5_(),
        identity: 5_(),
    };
    let mut matching_c: Key = Key {
        order: 6_(),
        identity: 6_(),
    };
    let mut before_matching: () = default_constructions_0.with(|rc| *rc.borrow());
    let matching: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let matched: Ptr<Mapped> = unimplemented!();
    field!(matched, value).write(51_());
    let matched_it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut matched_value: Ptr<Mapped> = (field_ptr!(matched_it.as_pointer(), second));
    ok &= (default_constructions_0.with(|rc| *rc.borrow()) == (before_matching + 1_()));
    let matched_again: Ptr<Mapped> = unimplemented!();
    ok &= (default_constructions_0.with(|rc| *rc.borrow()) == (before_matching + 1_()));
    ok &= (unimplemented!() == 1_());
    ok &= ({ (matched).clone() } == { (matched_again).clone() });
    ok &= (unimplemented!());
    ok &= ({ (field_ptr!(matched_it.as_pointer(), second)) } == { (matched_value).clone() });
    ok &= ({ ((*matched_it.borrow()).read()).second.value } == 51_());
    let other: Ptr<Mapped> = unimplemented!();
    field!(other, value).write(61_());
    ok &= (default_constructions_0.with(|rc| *rc.borrow()) == (before_matching + 2_()));
    ok &= (unimplemented!() == 2_());
    ok &= (unimplemented!());
    ok &= ({ (field_ptr!(matched_it.as_pointer(), second)) } == { (matched_value).clone() });
    ok &= ({ ((*matched_it.borrow()).read()).second.value } == 51_());
    return if ok { 0_() } else { 1_() };
}
pub fn __cpp2rust_init_globals() {
    let _ = default_constructions_0.with(|_| ());
}
