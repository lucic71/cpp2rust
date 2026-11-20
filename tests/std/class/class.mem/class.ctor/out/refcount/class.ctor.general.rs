extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, CppFrom)]
#[placement]
#[cpp_from(i32, new_1, value)]
#[byte_size(16)]
pub struct Tracker {
    #[offset(0)]
    #[byte_size(8)]
    pub self_: Ptr<Tracker>,
    #[offset(8)]
    pub value: i32,
}
impl Tracker {
    pub fn new_2(this: Ptr<Tracker>) {
        field!(this, self_).write((this).clone());
        field!(this, value).write(-1_i32);
    }
    pub fn new_1(this: Ptr<Tracker>, mut value: i32) {
        field!(this, self_).write((this).clone());
        field!(this, value).write(value);
    }
    pub fn new_3(this: Ptr<Tracker>, mut a: i32, mut b: i32) {
        Tracker::new_1(this.clone(), { (a + b) });
    }
    pub fn copy_from(this: Ptr<Tracker>, other: Ptr<Tracker>) {
        field!(this, self_).write((this).clone());
        field!(this, value).write(other.with(|__s| __s.value));
    }
    pub fn move_from(this: Ptr<Tracker>, other: Ptr<Tracker>) {
        field!(this, self_).write((this).clone());
        field!(this, value).write(other.with(|__s| __s.value));
        field!(other, value).write(-2_i32);
    }
}
impl Clone for Tracker {
    fn clone(&self) -> Self {
        let __src: Value<Tracker> = Rc::new(RefCell::new(Tracker {
            self_: self.self_.clone(),
            value: self.value.clone(),
        }));
        let __dst: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
        Tracker::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
impl Default for Tracker {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
            Tracker::new_2(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
pub fn test_0() {
    let a: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
    Tracker::new_1(a.as_pointer(), { 1 });
    assert!(
        ({
            ({ { (*a.borrow()).self_.clone() } } == { (a.as_pointer()) })
                && ({ (*a.borrow()).value } == 1)
        })
    );
}
pub fn test_1() {
    let a: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
    Tracker::new_2(a.as_pointer());
    assert!(
        ({
            ({ { (*a.borrow()).self_.clone() } } == { (a.as_pointer()) })
                && ({ (*a.borrow()).value } == -1_i32)
        })
    );
    let elements: Value<Box<[Tracker]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <Tracker>::uninit()).collect::<Box<[_]>>(),
    ));
    {
        let __d = elements.decay();
        for __i in 0..2_isize {
            Tracker::new_2(__d.offset(__i));
        }
    };
    assert!(
        ({
            ({ { (*elements.borrow())[(1) as usize].self_.clone() } } == {
                (elements.decay().offset(1))
            }) && ({ (*elements.borrow())[(1) as usize].value } == -1_i32)
        })
    );
}
pub fn test_2() {
    let a: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
    Tracker::new_3(a.as_pointer(), { 1 }, { 2 });
    assert!(
        ({
            ({ { (*a.borrow()).self_.clone() } } == { (a.as_pointer()) })
                && ({ (*a.borrow()).value } == 3)
        })
    );
}
pub fn test_3() {
    let a: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
    Tracker::new_1(a.as_pointer(), { 1 });
    let b: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
    Tracker::copy_from(b.as_pointer(), { a.as_pointer() });
    assert!(
        ({
            ((({ { (*b.borrow()).self_.clone() } } == { (b.as_pointer()) })
                && ({ (*b.borrow()).value } == 1))
                && ({ { (*a.borrow()).self_.clone() } } == { (a.as_pointer()) }))
                && ({ (*a.borrow()).value } == 1)
        })
    );
}
pub fn test_4() {
    let a: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
    Tracker::new_1(a.as_pointer(), { 1 });
    let b: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
    Tracker::move_from(b.as_pointer(), { a.as_pointer() });
    assert!(
        ({
            ((({ { (*b.borrow()).self_.clone() } } == { (b.as_pointer()) })
                && ({ (*b.borrow()).value } == 1))
                && ({ { (*a.borrow()).self_.clone() } } == { (a.as_pointer()) }))
                && ({ (*a.borrow()).value } == -2_i32)
        })
    );
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, Default)]
#[placement]
#[byte_size(32)]
pub struct implicit_constructors_Pair {
    #[offset(0)]
    #[byte_size(16)]
    pub first: Tracker,
    #[offset(16)]
    #[byte_size(16)]
    pub second: Tracker,
}
impl implicit_constructors_Pair {
    pub fn copy_from(this: Ptr<implicit_constructors_Pair>, _a0: Ptr<implicit_constructors_Pair>) {
        Tracker::copy_from(field_ptr!(this, first), { field_ptr!(_a0, first) });
        Tracker::copy_from(field_ptr!(this, second), { field_ptr!(_a0, second) });
    }
    pub fn move_from(this: Ptr<implicit_constructors_Pair>, _a0: Ptr<implicit_constructors_Pair>) {
        Tracker::move_from(field_ptr!(this, first), { field_ptr!(_a0, first) });
        Tracker::move_from(field_ptr!(this, second), { field_ptr!(_a0, second) });
    }
    pub fn new_1(this: Ptr<implicit_constructors_Pair>) {
        Tracker::new_2(field_ptr!(this, first));
        Tracker::new_2(field_ptr!(this, second));
    }
}
pub fn test_5() {
    let a: Value<implicit_constructors_Pair> =
        Rc::new(RefCell::new(<implicit_constructors_Pair>::uninit()));
    implicit_constructors_Pair::new_1(a.as_pointer());
    assert!(
        ({
            ({ { (*a.borrow()).first.self_.clone() } } == { (field_ptr!(a.as_pointer(), first)) })
                && ({ { (*a.borrow()).second.self_.clone() } } == {
                    (field_ptr!(a.as_pointer(), second))
                })
        })
    );
    (*a.borrow_mut()).second.value = 5;
    let b: Value<implicit_constructors_Pair> =
        Rc::new(RefCell::new(<implicit_constructors_Pair>::uninit()));
    implicit_constructors_Pair::copy_from(b.as_pointer(), { a.as_pointer() });
    assert!(
        ({
            ({ { (*b.borrow()).first.self_.clone() } } == { (field_ptr!(b.as_pointer(), first)) })
                && ({ { (*b.borrow()).second.self_.clone() } } == {
                    (field_ptr!(b.as_pointer(), second))
                })
        })
    );
    assert!(({ ({ (*b.borrow()).second.value } == 5) }));
    let c: Value<implicit_constructors_Pair> =
        Rc::new(RefCell::new(<implicit_constructors_Pair>::uninit()));
    implicit_constructors_Pair::move_from(c.as_pointer(), { a.as_pointer() });
    assert!(
        ({
            ({ { (*c.borrow()).second.self_.clone() } } == { (field_ptr!(c.as_pointer(), second)) })
                && ({ (*c.borrow()).second.value } == 5)
        })
    );
    assert!(({ ({ (*a.borrow()).second.value } == -2_i32) }));
}
pub fn test_6() {
    let mut p: Ptr<Tracker> = {
        let __p = Ptr::alloc(<Tracker>::uninit());
        Tracker::new_1(__p.clone(), { 4 });
        __p
    };
    assert!(
        ({ p.with(|__s| __s.self_.clone()) } == { (p).clone() }) && (p.with(|__s| __s.value) == 4)
    );
    p.delete();
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, CppFrom, Default)]
#[placement]
#[cpp_from(i32, new, value)]
#[byte_size(24)]
pub struct member_Holder {
    #[offset(0)]
    pub before: i32,
    #[offset(8)]
    #[byte_size(16)]
    pub tracker: Tracker,
}
impl member_Holder {
    pub fn new(this: Ptr<member_Holder>, mut value: i32) {
        field!(this, before).write(value);
        Tracker::new_1(field_ptr!(this, tracker), {
            (this.with(|__s| __s.before) + 1)
        });
    }
    pub fn copy_from(this: Ptr<member_Holder>, _a0: Ptr<member_Holder>) {
        field!(this, before).write(_a0.with(|__s| __s.before));
        Tracker::copy_from(field_ptr!(this, tracker), { field_ptr!(_a0, tracker) });
    }
    pub fn move_from(this: Ptr<member_Holder>, _a0: Ptr<member_Holder>) {
        field!(this, before).write({ (*_a0.upgrade().deref()).before });
        Tracker::move_from(field_ptr!(this, tracker), { field_ptr!(_a0, tracker) });
    }
}
pub fn test_7() {
    let h: Value<member_Holder> = Rc::new(RefCell::new(<member_Holder>::uninit()));
    member_Holder::new(h.as_pointer(), { 5 });
    assert!(
        ({
            ({ { (*h.borrow()).tracker.self_.clone() } } == {
                (field_ptr!(h.as_pointer(), tracker))
            }) && ({ (*h.borrow()).tracker.value } == 6)
        })
    );
}
pub fn test_8() {
    let elements: Value<Box<[Tracker]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <Tracker>::uninit()).collect::<Box<[_]>>(),
    ));
    {
        let __d = elements.decay();
        Tracker::new_1(__d.offset(0), { 6 });
        Tracker::new_1(__d.offset(1), { 7 });
    };
    assert!(
        ({
            ({ { (*elements.borrow())[(0) as usize].self_.clone() } } == {
                (elements.decay().offset(0))
            }) && ({ (*elements.borrow())[(0) as usize].value } == 6)
        })
    );
    assert!(
        ({
            ({ { (*elements.borrow())[(1) as usize].self_.clone() } } == {
                (elements.decay().offset(1))
            }) && ({ (*elements.borrow())[(1) as usize].value } == 7)
        })
    );
}
pub fn is_self_9(tracker: Value<Tracker>) -> bool {
    return ({ { (*tracker.borrow()).self_.clone() } } == { (tracker.as_pointer()) });
}
pub fn test_10() {
    assert!(
        ({
            is_self_9({
                let __v: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
                Tracker::new_1(__v.as_pointer(), { 8 });
                __v
            })
        })
    );
}
pub fn make_11(__ret: Ptr<Tracker>, mut value: i32) {
    Tracker::new_1(__ret.clone(), { value });
    return;
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, Default)]
#[placement]
#[byte_size(16)]
pub struct returned_object_Aggregate {
    #[offset(0)]
    #[byte_size(16)]
    pub tracker: Tracker,
}
impl returned_object_Aggregate {
    pub fn copy_from(this: Ptr<returned_object_Aggregate>, _a0: Ptr<returned_object_Aggregate>) {
        Tracker::copy_from(field_ptr!(this, tracker), { field_ptr!(_a0, tracker) });
    }
    pub fn move_from(this: Ptr<returned_object_Aggregate>, _a0: Ptr<returned_object_Aggregate>) {
        Tracker::move_from(field_ptr!(this, tracker), { field_ptr!(_a0, tracker) });
    }
}
pub fn test_12() {
    let r: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
    ({ make_11(r.as_pointer(), 9) });
    assert!(
        ({
            ({ { (*r.borrow()).self_.clone() } } == { (r.as_pointer()) })
                && ({ (*r.borrow()).value } == 9)
        })
    );
    let aggregate: Value<returned_object_Aggregate> =
        Rc::new(RefCell::new(<returned_object_Aggregate>::uninit()));
    {
        let __d = aggregate.as_pointer();
        ({ make_11(field_ptr!(__d, tracker), 10) });
    };
    assert!(
        ({
            ({ { (*aggregate.borrow()).tracker.self_.clone() } } == {
                (field_ptr!(aggregate.as_pointer(), tracker))
            })
        })
    );
}
pub fn test_13() {
    assert!(
        ({
            TrackerImpl::is_self(
                &{
                    let __v: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
                    Tracker::new_1(__v.as_pointer(), { 11 });
                    __v
                }
                .as_pointer(),
            )
        })
    );
    let __tmp_0: Value<Tracker> = {
        let __v: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
        Tracker::new_1(__v.as_pointer(), { 12 });
        __v
    };
    let bound: Ptr<Tracker> = __tmp_0.as_pointer();
    assert!(
        ({ bound.with(|__s| __s.self_.clone()) } == { (bound).clone() })
            && (bound.with(|__s| __s.value) == 12)
    );
}
pub fn test_14() {
    {
        let mut i: i32 = 0;
        'loop_: while (i < 2) {
            let chosen: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
            {
                let __d = chosen.as_pointer();
                if (i == 0) {
                    Tracker::new_1(__d.clone(), { 13 })
                } else {
                    Tracker::new_1(__d.clone(), { 14 })
                }
            };
            assert!(
                ({
                    ({ { (*chosen.borrow()).self_.clone() } } == { (chosen.as_pointer()) })
                        && ({ (*chosen.borrow()).value } == (13 + i))
                })
            );
            i.prefix_inc();
        }
    }
}
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, CopyAssign, Destructor, Default)]
#[placement]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct lifetime_Logged {
    #[offset(0)]
    pub id: i32,
}
impl lifetime_Logged {
    pub fn new(this: Ptr<lifetime_Logged>, mut id: i32) {
        field!(this, id).write(id);
        printf!(b"ctor %d\n", id);
    }
    pub fn copy_from(this: Ptr<lifetime_Logged>, other: Ptr<lifetime_Logged>) {
        field!(this, id).write((other.with(|__s| __s.id) + 100));
        printf!(b"copy %d\n", this.with(|__s| __s.id));
    }
}
impl Clone for lifetime_Logged {
    fn clone(&self) -> Self {
        let __src: Value<lifetime_Logged> = Rc::new(RefCell::new(lifetime_Logged {
            id: self.id.clone(),
        }));
        let __dst: Value<lifetime_Logged> = Rc::new(RefCell::new(<lifetime_Logged>::uninit()));
        lifetime_Logged::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
pub fn make_15(__ret: Ptr<lifetime_Logged>, mut id: i32) {
    lifetime_Logged::new(__ret.clone(), { id });
    return;
}
pub fn test_16() {
    let local: Value<lifetime_Logged> = Rc::new(RefCell::new(<lifetime_Logged>::uninit()));
    lifetime_Logged::new(local.as_pointer(), { 1 });
    let _dtor_local = std::mem::DropGuard::new(local.clone(), |__p| __p.as_pointer().destructor());
    let copy: Value<lifetime_Logged> = Rc::new(RefCell::new(<lifetime_Logged>::uninit()));
    lifetime_Logged::copy_from(copy.as_pointer(), { local.as_pointer() });
    let _dtor_copy = std::mem::DropGuard::new(copy.clone(), |__p| __p.as_pointer().destructor());
    let returned: Value<lifetime_Logged> = Rc::new(RefCell::new(<lifetime_Logged>::uninit()));
    ({ make_15(returned.as_pointer(), 3) });
    let _dtor_returned =
        std::mem::DropGuard::new(returned.clone(), |__p| __p.as_pointer().destructor());
    printf!(b"end\n");
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_16() });
    ({ test_0() });
    ({ test_1() });
    ({ test_2() });
    ({ test_3() });
    ({ test_4() });
    ({ test_5() });
    ({ test_6() });
    ({ test_7() });
    ({ test_8() });
    ({ test_10() });
    ({ test_12() });
    ({ test_13() });
    ({ test_14() });
    return 0;
}
pub trait TrackerImpl {
    fn is_self(&self) -> bool;
}
impl TrackerImpl for Ptr<Tracker> {
    fn is_self(&self) -> bool {
        return ((*self).with(|__s| __s.self_.clone()) == (*self));
    }
}
pub trait lifetime_LoggedImpl {
    fn destructor(&self);
    fn copy_assign(&self, _a0: Ptr<lifetime_Logged>) -> Ptr<lifetime_Logged>;
}
impl lifetime_LoggedImpl for Ptr<lifetime_Logged> {
    fn destructor(&self) {
        printf!(b"dtor %d\n", (*self).with(|__s| __s.id));
    }
    fn copy_assign(&self, _a0: Ptr<lifetime_Logged>) -> Ptr<lifetime_Logged> {
        field!((*self), id).write({ _a0.with(|__s| __s.id) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
