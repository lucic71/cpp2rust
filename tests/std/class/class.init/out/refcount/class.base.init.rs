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
pub struct Logged {
    #[offset(0)]
    pub id: i32,
}
impl Logged {
    pub fn new(this: Ptr<Logged>, mut id: i32) {
        field!(this, id).write(id);
        printf!(b"member %d\n", id);
    }
    pub fn copy_from(this: Ptr<Logged>, _a0: Ptr<Logged>) {
        field!(this, id).write(_a0.with(|__s| __s.id));
    }
    pub fn move_from(this: Ptr<Logged>, _a0: Ptr<Logged>) {
        field!(this, id).write({ (*_a0.upgrade().deref()).id });
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct p6_delegating_constructor_runs_target_first_Delegating {
    #[offset(0)]
    pub value: i32,
}
impl p6_delegating_constructor_runs_target_first_Delegating {
    pub fn new_1(
        this: Ptr<p6_delegating_constructor_runs_target_first_Delegating>,
        mut value: i32,
    ) {
        field!(this, value).write(value);
        printf!(b"target %d\n", value);
    }
    pub fn new_2(this: Ptr<p6_delegating_constructor_runs_target_first_Delegating>) {
        p6_delegating_constructor_runs_target_first_Delegating::new_1(this.clone(), { 42 });
        printf!(b"delegating %d\n", this.with(|__s| __s.value));
        {
            field!(this, value).with_mut(|__v| *__v = *__v + 1)
        };
    }
    pub fn copy_from(
        this: Ptr<p6_delegating_constructor_runs_target_first_Delegating>,
        _a0: Ptr<p6_delegating_constructor_runs_target_first_Delegating>,
    ) {
        field!(this, value).write(_a0.with(|__s| __s.value));
    }
    pub fn move_from(
        this: Ptr<p6_delegating_constructor_runs_target_first_Delegating>,
        _a0: Ptr<p6_delegating_constructor_runs_target_first_Delegating>,
    ) {
        field!(this, value).write({ (*_a0.upgrade().deref()).value });
    }
}
impl Default for p6_delegating_constructor_runs_target_first_Delegating {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<p6_delegating_constructor_runs_target_first_Delegating> = Rc::new(
                RefCell::new(<p6_delegating_constructor_runs_target_first_Delegating>::uninit()),
            );
            p6_delegating_constructor_runs_target_first_Delegating::new_2(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
pub fn test_0() {
    let object: Value<p6_delegating_constructor_runs_target_first_Delegating> = Rc::new(
        RefCell::new(<p6_delegating_constructor_runs_target_first_Delegating>::uninit()),
    );
    p6_delegating_constructor_runs_target_first_Delegating::new_2(object.as_pointer());
    assert!(({ ({ (*object.borrow()).value } == 43) }));
}
thread_local!(
    pub static evaluated_1: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn next_2() -> i32 {
    return (*(*evaluated_1.with(Value::clone).borrow_mut()).prefix_inc());
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new_1, value)]
#[byte_size(8)]
pub struct p9_1_default_member_initializer_WithDefaults {
    #[offset(0)]
    pub fixed: i32,
    #[offset(4)]
    pub counted: i32,
}
impl p9_1_default_member_initializer_WithDefaults {
    pub fn new_2(this: Ptr<p9_1_default_member_initializer_WithDefaults>) {
        field!(this, fixed).write(5);
        field!(this, counted).write(({ next_2() }));
    }
    pub fn new_1(this: Ptr<p9_1_default_member_initializer_WithDefaults>, mut counted: i32) {
        field!(this, fixed).write(5);
        field!(this, counted).write(counted);
    }
    pub fn copy_from(
        this: Ptr<p9_1_default_member_initializer_WithDefaults>,
        _a0: Ptr<p9_1_default_member_initializer_WithDefaults>,
    ) {
        field!(this, fixed).write(_a0.with(|__s| __s.fixed));
        field!(this, counted).write(_a0.with(|__s| __s.counted));
    }
    pub fn move_from(
        this: Ptr<p9_1_default_member_initializer_WithDefaults>,
        _a0: Ptr<p9_1_default_member_initializer_WithDefaults>,
    ) {
        field!(this, fixed).write({ (*_a0.upgrade().deref()).fixed });
        field!(this, counted).write({ (*_a0.upgrade().deref()).counted });
    }
}
impl Default for p9_1_default_member_initializer_WithDefaults {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<p9_1_default_member_initializer_WithDefaults> = Rc::new(RefCell::new(
                <p9_1_default_member_initializer_WithDefaults>::uninit(),
            ));
            p9_1_default_member_initializer_WithDefaults::new_2(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
pub fn test_3() {
    let defaults: Value<p9_1_default_member_initializer_WithDefaults> = Rc::new(RefCell::new(
        <p9_1_default_member_initializer_WithDefaults>::uninit(),
    ));
    p9_1_default_member_initializer_WithDefaults::new_2(defaults.as_pointer());
    assert!(
        ({
            (({ (*defaults.borrow()).fixed } == 5) && ({ (*defaults.borrow()).counted } == 1))
                && (evaluated_1.with(|rc| *rc.borrow()) == 1)
        })
    );
    let given: Value<p9_1_default_member_initializer_WithDefaults> = Rc::new(RefCell::new(
        <p9_1_default_member_initializer_WithDefaults>::uninit(),
    ));
    p9_1_default_member_initializer_WithDefaults::new_1(given.as_pointer(), { 9 });
    assert!(
        ({
            (({ (*given.borrow()).fixed } == 5) && ({ (*given.borrow()).counted } == 9))
                && (evaluated_1.with(|rc| *rc.borrow()) == 1)
        })
    );
}
thread_local!(
    pub static constructed_4: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct p9_3_member_is_default_initialized_Member {
    #[offset(0)]
    pub id: i32,
}
impl p9_3_member_is_default_initialized_Member {
    pub fn new(this: Ptr<p9_3_member_is_default_initialized_Member>) {
        field!(this, id).write((*(*constructed_4.with(Value::clone).borrow_mut()).prefix_inc()));
    }
    pub fn copy_from(
        this: Ptr<p9_3_member_is_default_initialized_Member>,
        _a0: Ptr<p9_3_member_is_default_initialized_Member>,
    ) {
        field!(this, id).write(_a0.with(|__s| __s.id));
    }
    pub fn move_from(
        this: Ptr<p9_3_member_is_default_initialized_Member>,
        _a0: Ptr<p9_3_member_is_default_initialized_Member>,
    ) {
        field!(this, id).write({ (*_a0.upgrade().deref()).id });
    }
}
impl Default for p9_3_member_is_default_initialized_Member {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<p9_3_member_is_default_initialized_Member> = Rc::new(RefCell::new(
                <p9_3_member_is_default_initialized_Member>::uninit(),
            ));
            p9_3_member_is_default_initialized_Member::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct p9_3_member_is_default_initialized_Holder {
    #[offset(0)]
    #[byte_size(4)]
    pub first: p9_3_member_is_default_initialized_Member,
    #[offset(4)]
    #[byte_size(4)]
    pub second: p9_3_member_is_default_initialized_Member,
    #[offset(8)]
    pub number: i32,
}
impl p9_3_member_is_default_initialized_Holder {
    pub fn new(this: Ptr<p9_3_member_is_default_initialized_Holder>) {
        p9_3_member_is_default_initialized_Member::new(field_ptr!(this, first));
        p9_3_member_is_default_initialized_Member::new(field_ptr!(this, second));
        field!(this, number).write(3);
    }
    pub fn copy_from(
        this: Ptr<p9_3_member_is_default_initialized_Holder>,
        _a0: Ptr<p9_3_member_is_default_initialized_Holder>,
    ) {
        p9_3_member_is_default_initialized_Member::copy_from(field_ptr!(this, first), {
            field_ptr!(_a0, first)
        });
        p9_3_member_is_default_initialized_Member::copy_from(field_ptr!(this, second), {
            field_ptr!(_a0, second)
        });
        field!(this, number).write(_a0.with(|__s| __s.number));
    }
    pub fn move_from(
        this: Ptr<p9_3_member_is_default_initialized_Holder>,
        _a0: Ptr<p9_3_member_is_default_initialized_Holder>,
    ) {
        p9_3_member_is_default_initialized_Member::move_from(field_ptr!(this, first), {
            field_ptr!(_a0, first)
        });
        p9_3_member_is_default_initialized_Member::move_from(field_ptr!(this, second), {
            field_ptr!(_a0, second)
        });
        field!(this, number).write({ (*_a0.upgrade().deref()).number });
    }
}
impl Default for p9_3_member_is_default_initialized_Holder {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<p9_3_member_is_default_initialized_Holder> = Rc::new(RefCell::new(
                <p9_3_member_is_default_initialized_Holder>::uninit(),
            ));
            p9_3_member_is_default_initialized_Holder::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
pub fn test_5() {
    let holder: Value<p9_3_member_is_default_initialized_Holder> = Rc::new(RefCell::new(
        <p9_3_member_is_default_initialized_Holder>::uninit(),
    ));
    p9_3_member_is_default_initialized_Holder::new(holder.as_pointer());
    assert!(
        ({
            ((constructed_4.with(|rc| *rc.borrow()) == 2) && ({ (*holder.borrow()).first.id } == 1))
                && ({ (*holder.borrow()).second.id } == 2)
        })
    );
    assert!(({ ({ (*holder.borrow()).number } == 3) }));
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct p13_members_initialized_in_declaration_order_Ordered {
    #[offset(0)]
    #[byte_size(4)]
    pub first: Logged,
    #[offset(4)]
    #[byte_size(4)]
    pub second: Logged,
    #[offset(8)]
    pub sum: i32,
}
impl p13_members_initialized_in_declaration_order_Ordered {
    pub fn new(this: Ptr<p13_members_initialized_in_declaration_order_Ordered>) {
        Logged::new(field_ptr!(this, first), { 1 });
        Logged::new(field_ptr!(this, second), { 2 });
        field!(this, sum).write((this.with(|__s| __s.first.id) + this.with(|__s| __s.second.id)));
        printf!(b"body %d\n", this.with(|__s| __s.sum));
    }
    pub fn copy_from(
        this: Ptr<p13_members_initialized_in_declaration_order_Ordered>,
        _a0: Ptr<p13_members_initialized_in_declaration_order_Ordered>,
    ) {
        Logged::copy_from(field_ptr!(this, first), { field_ptr!(_a0, first) });
        Logged::copy_from(field_ptr!(this, second), { field_ptr!(_a0, second) });
        field!(this, sum).write(_a0.with(|__s| __s.sum));
    }
    pub fn move_from(
        this: Ptr<p13_members_initialized_in_declaration_order_Ordered>,
        _a0: Ptr<p13_members_initialized_in_declaration_order_Ordered>,
    ) {
        Logged::move_from(field_ptr!(this, first), { field_ptr!(_a0, first) });
        Logged::move_from(field_ptr!(this, second), { field_ptr!(_a0, second) });
        field!(this, sum).write({ (*_a0.upgrade().deref()).sum });
    }
}
impl Default for p13_members_initialized_in_declaration_order_Ordered {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<p13_members_initialized_in_declaration_order_Ordered> = Rc::new(
                RefCell::new(<p13_members_initialized_in_declaration_order_Ordered>::uninit()),
            );
            p13_members_initialized_in_declaration_order_Ordered::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
pub fn test_6() {
    let ordered: Value<p13_members_initialized_in_declaration_order_Ordered> = Rc::new(
        RefCell::new(<p13_members_initialized_in_declaration_order_Ordered>::uninit()),
    );
    p13_members_initialized_in_declaration_order_Ordered::new(ordered.as_pointer());
    assert!(({ ({ (*ordered.borrow()).sum } == 3) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    ({ test_3() });
    ({ test_5() });
    ({ test_6() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = evaluated_1.with(|_| ());
    let _ = constructed_4.with(|_| ());
}
