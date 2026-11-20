extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Celsius {
    #[offset(0)]
    pub degrees: i32,
}
impl Celsius {
    pub fn new(this: Ptr<Celsius>, mut d: i32) {
        field!(this, degrees).write(d);
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(i32, to_i32)]
#[byte_size(4)]
pub struct Meters {
    #[offset(0)]
    pub value: i32,
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Celsius, new, ptr)]
#[byte_size(4)]
pub struct Label {
    #[offset(0)]
    pub id: i32,
}
impl Label {
    pub fn new(this: Ptr<Label>, c: Ptr<Celsius>) {
        field!(this, id).write(c.with(|__s| __s.degrees));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Celsius, new_1, ptr)]
#[cpp_from_move(Celsius, new_2, ptr)]
#[byte_size(8)]
pub struct Owner {
    #[offset(0)]
    pub taken: i32,
    #[offset(4)]
    pub copied: i32,
}
impl Owner {
    pub fn new_1(this: Ptr<Owner>, c: Ptr<Celsius>) {
        field!(this, taken).write(0);
        field!(this, copied).write(c.with(|__s| __s.degrees));
    }
    pub fn new_2(this: Ptr<Owner>, c: Ptr<Celsius>) {
        field!(this, taken).write(c.with(|__s| __s.degrees));
        field!(this, copied).write(0);
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Celsius, new, storage)]
#[byte_size(4)]
pub struct Holder {
    #[offset(0)]
    pub value: i32,
}
impl Holder {
    pub fn new(this: Ptr<Holder>, c: Value<Celsius>) {
        field!(this, value).write({ (*c.borrow()).degrees });
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Fallback {
    #[offset(0)]
    pub value: i32,
}
impl Fallback {
    pub fn new(this: Ptr<Fallback>, mut v: i32) {
        field!(this, value).write(v);
    }
}
impl Default for Fallback {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<Fallback> = Rc::new(RefCell::new(<Fallback>::uninit()));
            Fallback::new(__v.as_pointer(), { -1_i32 });
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Counter {
    #[offset(0)]
    pub hits: i32,
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Counter, new, mut_ptr)]
#[byte_size(4)]
pub struct Observer {
    #[offset(0)]
    pub seen: i32,
}
impl Observer {
    pub fn new(this: Ptr<Observer>, c: Ptr<Counter>) {
        field!(this, seen).write(c.with(|__s| __s.hits));
        field!(c, hits).with_mut(|__v| __v.postfix_inc());
    }
}
impl Tag {
    pub fn new(this: Ptr<Tag>, u: Ptr<Celsius>) {
        field!(this, code).write((u.with(|__s| __s.degrees) + 100));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Celsius, new, ptr)]
#[byte_size(4)]
pub struct Tag {
    #[offset(0)]
    pub code: i32,
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Ptr< i8  >  , new, value)]
#[byte_size(4)]
pub struct Name {
    #[offset(0)]
    pub len: i32,
}
impl Name {
    pub fn new(this: Ptr<Name>, mut s: Ptr<i8>) {
        field!(this, len).write(0);
        'loop_: while ((elem!(s, this.with(|__s| __s.len)).read()) != 0) {
            field!(this, len).with_mut(|__v| {
                __v.prefix_inc();
            });
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let widened: Value<(Value<i64>, Value<u32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new((-1_i32 as i64))),
        Rc::new(RefCell::new(2_u32)),
    )));
    assert!(({ ((*(*widened.borrow()).0.borrow()) == -1_i64) }));
    assert!(({ ((*(*widened.borrow()).1.borrow()) == 2_u32) }));
    let from_ints: Value<(Value<Celsius>, Value<Celsius>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(
            Rc::try_unwrap({
                let __v: Value<Celsius> = Rc::new(RefCell::new(<Celsius>::uninit()));
                Celsius::new(__v.as_pointer(), { 1 });
                __v
            })
            .ok()
            .unwrap()
            .into_inner(),
        )),
        Rc::new(RefCell::new(
            Rc::try_unwrap({
                let __v: Value<Celsius> = Rc::new(RefCell::new(<Celsius>::uninit()));
                Celsius::new(__v.as_pointer(), { 2 });
                __v
            })
            .ok()
            .unwrap()
            .into_inner(),
        )),
    )));
    assert!(({ ({ (*(*from_ints.borrow()).0.borrow()).degrees } == 1) }));
    assert!(({ ({ (*(*from_ints.borrow()).1.borrow()).degrees } == 2) }));
    let m: Value<Meters> = Rc::new(RefCell::new(Meters { value: 7 }));
    let from_operators: Value<(Value<i32>, Value<i64>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(({ MetersImpl::to_i32(&m.as_pointer()) }))),
        Rc::new(RefCell::new(
            (({ MetersImpl::to_i32(&m.as_pointer()) }) as i64),
        )),
    )));
    assert!(({ ((*(*from_operators.borrow()).0.borrow()) == 7) }));
    assert!(({ ((*(*from_operators.borrow()).1.borrow()) == 7_i64) }));
    let from_moved_pair: Value<(Value<i64>, Value<f64>)> = Rc::new(RefCell::new(
        match (Rc::new(RefCell::new((
            Rc::new(RefCell::new(3)),
            Rc::new(RefCell::new(4)),
        )))
        .as_pointer(),)
        {
            (__tmp_0,) => (
                Rc::new(RefCell::new(
                    (((*__tmp_0.upgrade().deref()).0.as_pointer().read()) as i64),
                )),
                Rc::new(RefCell::new(
                    (((*__tmp_0.upgrade().deref()).1.as_pointer().read()) as f64),
                )),
            ),
        },
    ));
    assert!(({ ((*(*from_moved_pair.borrow()).0.borrow()) == 3_i64) }));
    assert!(({ ((*(*from_moved_pair.borrow()).1.borrow()) == 4_f64) }));
    let ints: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(5)),
        Rc::new(RefCell::new(6)),
    )));
    let from_copied_pair: Value<(Value<Celsius>, Value<i64>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(<Celsius as CppFrom<i32>>::cpp_from(
            (*ints.borrow()).0.as_pointer(),
        ))),
        Rc::new(RefCell::new(
            (((*ints.borrow()).1.as_pointer().read()) as i64),
        )),
    )));
    assert!(({ ({ (*(*from_copied_pair.borrow()).0.borrow()).degrees } == 5) }));
    assert!(({ ((*(*from_copied_pair.borrow()).1.borrow()) == 6_i64) }));
    let meters: Value<(Value<Meters>, Value<Meters>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new((Meters { value: 8 }))),
        Rc::new(RefCell::new((Meters { value: 9 }))),
    )));
    let from_operator_pair: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(<i32 as CppFrom<Meters>>::cpp_from(
            (*meters.borrow()).0.as_pointer(),
        ))),
        Rc::new(RefCell::new(<i32 as CppFrom<Meters>>::cpp_from(
            (*meters.borrow()).1.as_pointer(),
        ))),
    )));
    assert!(({ ((*(*from_operator_pair.borrow()).0.borrow()) == 8) }));
    assert!(({ ((*(*from_operator_pair.borrow()).1.borrow()) == 9) }));
    let celsius: Value<(Value<Celsius>, Value<Celsius>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(
            Rc::try_unwrap({
                let __v: Value<Celsius> = Rc::new(RefCell::new(<Celsius>::uninit()));
                Celsius::new(__v.as_pointer(), { 10 });
                __v
            })
            .ok()
            .unwrap()
            .into_inner(),
        )),
        Rc::new(RefCell::new(
            Rc::try_unwrap({
                let __v: Value<Celsius> = Rc::new(RefCell::new(<Celsius>::uninit()));
                Celsius::new(__v.as_pointer(), { 11 });
                __v
            })
            .ok()
            .unwrap()
            .into_inner(),
        )),
    )));
    let from_record_pair: Value<(Value<Label>, Value<Owner>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(<Label as CppFrom<Celsius>>::cpp_from(
            (*celsius.borrow()).0.as_pointer(),
        ))),
        Rc::new(RefCell::new(<Owner as CppFrom<Celsius>>::cpp_from(
            (*celsius.borrow()).1.as_pointer(),
        ))),
    )));
    assert!(({ ({ (*(*from_record_pair.borrow()).0.borrow()).id } == 10) }));
    assert!(({ ({ (*(*from_record_pair.borrow()).1.borrow()).copied } == 11) }));
    assert!(({ ({ (*(*from_record_pair.borrow()).1.borrow()).taken } == 0) }));
    let from_moved_record_pair: Value<(Value<Label>, Value<Owner>)> = Rc::new(RefCell::new(
        match (Rc::new(RefCell::new((
            Rc::new(RefCell::new(
                Rc::try_unwrap({
                    let __v: Value<Celsius> = Rc::new(RefCell::new(<Celsius>::uninit()));
                    Celsius::new(__v.as_pointer(), { 12 });
                    __v
                })
                .ok()
                .unwrap()
                .into_inner(),
            )),
            Rc::new(RefCell::new(
                Rc::try_unwrap({
                    let __v: Value<Celsius> = Rc::new(RefCell::new(<Celsius>::uninit()));
                    Celsius::new(__v.as_pointer(), { 13 });
                    __v
                })
                .ok()
                .unwrap()
                .into_inner(),
            )),
        )))
        .as_pointer(),)
        {
            (__tmp_1,) => (
                Rc::new(RefCell::new(<Label as CppFrom<Celsius>>::cpp_from_move(
                    (*__tmp_1.upgrade().deref()).0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<Owner as CppFrom<Celsius>>::cpp_from_move(
                    (*__tmp_1.upgrade().deref()).1.as_pointer(),
                ))),
            ),
        },
    ));
    assert!(({ ({ (*(*from_moved_record_pair.borrow()).0.borrow()).id } == 12) }));
    assert!(({ ({ (*(*from_moved_record_pair.borrow()).1.borrow()).taken } == 13) }));
    assert!(({ ({ (*(*from_moved_record_pair.borrow()).1.borrow()).copied } == 0) }));
    let from_by_value_pair: Value<(Value<Holder>, Value<Holder>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(<Holder as CppFrom<Celsius>>::cpp_from(
            (*celsius.borrow()).0.as_pointer(),
        ))),
        Rc::new(RefCell::new(<Holder as CppFrom<Celsius>>::cpp_from(
            (*celsius.borrow()).1.as_pointer(),
        ))),
    )));
    assert!(({ ({ (*(*from_by_value_pair.borrow()).0.borrow()).value } == 10) }));
    assert!(({ ({ (*(*from_by_value_pair.borrow()).1.borrow()).value } == 11) }));
    let from_moved_by_value_pair: Value<(Value<Holder>, Value<Holder>)> = Rc::new(RefCell::new(
        match (Rc::new(RefCell::new((
            Rc::new(RefCell::new(
                Rc::try_unwrap({
                    let __v: Value<Celsius> = Rc::new(RefCell::new(<Celsius>::uninit()));
                    Celsius::new(__v.as_pointer(), { 14 });
                    __v
                })
                .ok()
                .unwrap()
                .into_inner(),
            )),
            Rc::new(RefCell::new(
                Rc::try_unwrap({
                    let __v: Value<Celsius> = Rc::new(RefCell::new(<Celsius>::uninit()));
                    Celsius::new(__v.as_pointer(), { 15 });
                    __v
                })
                .ok()
                .unwrap()
                .into_inner(),
            )),
        )))
        .as_pointer(),)
        {
            (__tmp_2,) => (
                Rc::new(RefCell::new(<Holder as CppFrom<Celsius>>::cpp_from_move(
                    (*__tmp_2.upgrade().deref()).0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<Holder as CppFrom<Celsius>>::cpp_from_move(
                    (*__tmp_2.upgrade().deref()).1.as_pointer(),
                ))),
            ),
        },
    ));
    assert!(({ ({ (*(*from_moved_by_value_pair.borrow()).0.borrow()).value } == 14) }));
    assert!(({ ({ (*(*from_moved_by_value_pair.borrow()).1.borrow()).value } == 15) }));
    let defaulted: Value<Fallback> = Rc::new(RefCell::new(<Fallback>::uninit()));
    Fallback::new(defaulted.as_pointer(), { -1_i32 });
    assert!(({ ({ (*defaulted.borrow()).value } == -1_i32) }));
    let from_defaulted_ctor: Value<(Value<Fallback>, Value<i64>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(<Fallback as CppFrom<i32>>::cpp_from(
            (*ints.borrow()).0.as_pointer(),
        ))),
        Rc::new(RefCell::new(
            (((*ints.borrow()).1.as_pointer().read()) as i64),
        )),
    )));
    assert!(({ ({ (*(*from_defaulted_ctor.borrow()).0.borrow()).value } == 5) }));
    assert!(({ ((*(*from_defaulted_ctor.borrow()).1.borrow()) == 6_i64) }));
    let counter: Value<Counter> = Rc::new(RefCell::new(Counter { hits: 3 }));
    let forwarded_mutable_ref: Value<(Value<Observer>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(
            Rc::try_unwrap({
                let __v: Value<Observer> = Rc::new(RefCell::new(<Observer>::uninit()));
                Observer::new(__v.as_pointer(), { counter.as_pointer() });
                __v
            })
            .ok()
            .unwrap()
            .into_inner(),
        )),
        Rc::new(RefCell::new(0)),
    )));
    assert!(({ ({ (*(*forwarded_mutable_ref.borrow()).0.borrow()).seen } == 3) }));
    assert!(({ ({ (*counter.borrow()).hits } == 4) }));
    let from_template_ctor: Value<(Value<Tag>, Value<Tag>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(<Tag as CppFrom<Celsius>>::cpp_from(
            (*celsius.borrow()).0.as_pointer(),
        ))),
        Rc::new(RefCell::new(<Tag as CppFrom<Celsius>>::cpp_from(
            (*celsius.borrow()).1.as_pointer(),
        ))),
    )));
    assert!(({ ({ (*(*from_template_ctor.borrow()).0.borrow()).code } == 110) }));
    assert!(({ ({ (*(*from_template_ctor.borrow()).1.borrow()).code } == 111) }));
    let lvalue_source: Value<Celsius> = Rc::new(RefCell::new(<Celsius>::uninit()));
    Celsius::new(lvalue_source.as_pointer(), { 16 });
    let moved_source: Value<Celsius> = Rc::new(RefCell::new(<Celsius>::uninit()));
    Celsius::new(moved_source.as_pointer(), { 17 });
    let forwarded_records: Value<(Value<Label>, Value<Owner>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(
            Rc::try_unwrap({
                let __v: Value<Label> = Rc::new(RefCell::new(<Label>::uninit()));
                Label::new(__v.as_pointer(), { lvalue_source.as_pointer() });
                __v
            })
            .ok()
            .unwrap()
            .into_inner(),
        )),
        Rc::new(RefCell::new(
            Rc::try_unwrap({
                let __v: Value<Owner> = Rc::new(RefCell::new(<Owner>::uninit()));
                Owner::new_2(__v.as_pointer(), { moved_source.as_pointer() });
                __v
            })
            .ok()
            .unwrap()
            .into_inner(),
        )),
    )));
    assert!(({ ({ (*(*forwarded_records.borrow()).0.borrow()).id } == 16) }));
    assert!(({ ({ (*(*forwarded_records.borrow()).1.borrow()).taken } == 17) }));
    assert!(({ ({ (*(*forwarded_records.borrow()).1.borrow()).copied } == 0) }));
    let from_string_literals: Value<(Value<Name>, Value<Name>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(
            Rc::try_unwrap({
                let __v: Value<Name> = Rc::new(RefCell::new(<Name>::uninit()));
                Name::new(__v.as_pointer(), { Ptr::<i8>::from_string_literal(b"ab") });
                __v
            })
            .ok()
            .unwrap()
            .into_inner(),
        )),
        Rc::new(RefCell::new(
            Rc::try_unwrap({
                let __v: Value<Name> = Rc::new(RefCell::new(<Name>::uninit()));
                Name::new(__v.as_pointer(), { Ptr::<i8>::from_string_literal(b"cde") });
                __v
            })
            .ok()
            .unwrap()
            .into_inner(),
        )),
    )));
    assert!(({ ({ (*(*from_string_literals.borrow()).0.borrow()).len } == 2) }));
    assert!(({ ({ (*(*from_string_literals.borrow()).1.borrow()).len } == 3) }));
    return 0;
}
pub trait MetersImpl {
    fn to_i32(&self) -> i32;
}
impl MetersImpl for Ptr<Meters> {
    fn to_i32(&self) -> i32 {
        return (*self).with(|__s| __s.value);
    }
}
pub fn __cpp2rust_init_globals() {}
