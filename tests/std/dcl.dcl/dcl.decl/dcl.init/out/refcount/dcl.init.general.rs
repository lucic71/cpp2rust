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
pub struct p6_1_zero_initialized_pointer_to_member_HasTwoInts {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
impl p6_1_zero_initialized_pointer_to_member_HasTwoInts {
    pub fn new_1(this: Ptr<p6_1_zero_initialized_pointer_to_member_HasTwoInts>) {}
    pub fn copy_from(
        this: Ptr<p6_1_zero_initialized_pointer_to_member_HasTwoInts>,
        _a0: Ptr<p6_1_zero_initialized_pointer_to_member_HasTwoInts>,
    ) {
        field!(this, first).write(_a0.with(|__s| __s.first));
        field!(this, second).write(_a0.with(|__s| __s.second));
    }
    pub fn move_from(
        this: Ptr<p6_1_zero_initialized_pointer_to_member_HasTwoInts>,
        _a0: Ptr<p6_1_zero_initialized_pointer_to_member_HasTwoInts>,
    ) {
        field!(this, first).write({ (*_a0.upgrade().deref()).first });
        field!(this, second).write({ (*_a0.upgrade().deref()).second });
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct p6_1_zero_initialized_pointer_to_member_HoldsSelector {
    #[offset(0)]
    #[byte_size(8)]
    pub selector: isize,
}
impl p6_1_zero_initialized_pointer_to_member_HoldsSelector {
    pub fn new_1(this: Ptr<p6_1_zero_initialized_pointer_to_member_HoldsSelector>) {}
    pub fn copy_from(
        this: Ptr<p6_1_zero_initialized_pointer_to_member_HoldsSelector>,
        _a0: Ptr<p6_1_zero_initialized_pointer_to_member_HoldsSelector>,
    ) {
        field!(this, selector).write(_a0.with(|__s| __s.selector));
    }
    pub fn move_from(
        this: Ptr<p6_1_zero_initialized_pointer_to_member_HoldsSelector>,
        _a0: Ptr<p6_1_zero_initialized_pointer_to_member_HoldsSelector>,
    ) {
        field!(this, selector).write({ (*_a0.upgrade().deref()).selector });
    }
}
impl Default for p6_1_zero_initialized_pointer_to_member_HoldsSelector {
    fn default() -> Self {
        { p6_1_zero_initialized_pointer_to_member_HoldsSelector { selector: -1_isize } }
    }
}
thread_local!(
    pub static static_pm_0: Value<isize> = Rc::new(RefCell::new(-1_isize));
);
pub fn test_1() {
    assert!(((static_pm_0.with(|rc| *rc.borrow())) == -1_isize));
    assert!(
        (static_pm_0.with(|rc| *rc.borrow())
            != (offset_of_field!(p6_1_zero_initialized_pointer_to_member_HasTwoInts, first)
                as isize))
    );
    let mut value_initialized: isize = -1_isize;
    assert!(((value_initialized) == -1_isize));
    assert!(
        (value_initialized
            != (offset_of_field!(p6_1_zero_initialized_pointer_to_member_HasTwoInts, first)
                as isize))
    );
    let holder: Value<p6_1_zero_initialized_pointer_to_member_HoldsSelector> = Rc::new(
        RefCell::new(<p6_1_zero_initialized_pointer_to_member_HoldsSelector>::uninit()),
    );
    {
        let __d = holder.as_pointer();
        field!(__d, selector).write(-1_isize);
    };
    assert!(({ (({ (*holder.borrow()).selector }) == -1_isize) }));
    assert!(
        ({
            ({ (*holder.borrow()).selector }
                != (offset_of_field!(p6_1_zero_initialized_pointer_to_member_HasTwoInts, first)
                    as isize))
        })
    );
}
thread_local!(
    pub static constructed_2: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct p7_default_initialization_calls_default_constructor_Counted {
    #[offset(0)]
    pub id: i32,
}
impl p7_default_initialization_calls_default_constructor_Counted {
    pub fn new(this: Ptr<p7_default_initialization_calls_default_constructor_Counted>) {
        field!(this, id).write((*(*constructed_2.with(Value::clone).borrow_mut()).prefix_inc()));
    }
    pub fn copy_from(
        this: Ptr<p7_default_initialization_calls_default_constructor_Counted>,
        _a0: Ptr<p7_default_initialization_calls_default_constructor_Counted>,
    ) {
        field!(this, id).write(_a0.with(|__s| __s.id));
    }
    pub fn move_from(
        this: Ptr<p7_default_initialization_calls_default_constructor_Counted>,
        _a0: Ptr<p7_default_initialization_calls_default_constructor_Counted>,
    ) {
        field!(this, id).write({ (*_a0.upgrade().deref()).id });
    }
}
impl Default for p7_default_initialization_calls_default_constructor_Counted {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<p7_default_initialization_calls_default_constructor_Counted> =
                Rc::new(RefCell::new(
                    <p7_default_initialization_calls_default_constructor_Counted>::uninit(),
                ));
            p7_default_initialization_calls_default_constructor_Counted::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
pub fn test_3() {
    let one: Value<p7_default_initialization_calls_default_constructor_Counted> = Rc::new(
        RefCell::new(<p7_default_initialization_calls_default_constructor_Counted>::uninit()),
    );
    p7_default_initialization_calls_default_constructor_Counted::new(one.as_pointer());
    assert!(({ (constructed_2.with(|rc| *rc.borrow()) == 1) && ({ (*one.borrow()).id } == 1) }));
    let many: Value<Box<[p7_default_initialization_calls_default_constructor_Counted]>> =
        Rc::new(RefCell::new(
            (0..3)
                .map(|_| <p7_default_initialization_calls_default_constructor_Counted>::uninit())
                .collect::<Box<[_]>>(),
        ));
    {
        let __d = many.decay();
        for __i in 0..3_isize {
            p7_default_initialization_calls_default_constructor_Counted::new(__d.offset(__i));
        }
    };
    assert!((constructed_2.with(|rc| *rc.borrow()) == 4));
    assert!(
        ({
            (({ (*many.borrow())[(0) as usize].id } == 2)
                && ({ (*many.borrow())[(1) as usize].id } == 3))
                && ({ (*many.borrow())[(2) as usize].id } == 4)
        })
    );
}
thread_local!(
    pub static constructed_4: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct p9_1_1_value_initialization_calls_user_provided_constructor_Seven {
    #[offset(0)]
    pub value: i32,
}
impl p9_1_1_value_initialization_calls_user_provided_constructor_Seven {
    pub fn new(this: Ptr<p9_1_1_value_initialization_calls_user_provided_constructor_Seven>) {
        field!(this, value).write(7);
        (*constructed_4.with(Value::clone).borrow_mut()).prefix_inc();
    }
    pub fn copy_from(
        this: Ptr<p9_1_1_value_initialization_calls_user_provided_constructor_Seven>,
        _a0: Ptr<p9_1_1_value_initialization_calls_user_provided_constructor_Seven>,
    ) {
        field!(this, value).write(_a0.with(|__s| __s.value));
    }
    pub fn move_from(
        this: Ptr<p9_1_1_value_initialization_calls_user_provided_constructor_Seven>,
        _a0: Ptr<p9_1_1_value_initialization_calls_user_provided_constructor_Seven>,
    ) {
        field!(this, value).write({ (*_a0.upgrade().deref()).value });
    }
}
impl Default for p9_1_1_value_initialization_calls_user_provided_constructor_Seven {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<p9_1_1_value_initialization_calls_user_provided_constructor_Seven> =
                Rc::new(RefCell::new(
                    <p9_1_1_value_initialization_calls_user_provided_constructor_Seven>::uninit(),
                ));
            p9_1_1_value_initialization_calls_user_provided_constructor_Seven::new(
                __v.as_pointer(),
            );
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
pub fn test_5() {
    let braces: Value<p9_1_1_value_initialization_calls_user_provided_constructor_Seven> = Rc::new(
        RefCell::new(<p9_1_1_value_initialization_calls_user_provided_constructor_Seven>::uninit()),
    );
    p9_1_1_value_initialization_calls_user_provided_constructor_Seven::new(braces.as_pointer());
    assert!(
        ({ ({ (*braces.borrow()).value } == 7) && (constructed_4.with(|rc| *rc.borrow()) == 1) })
    );
    assert!(
        ({
            Rc::try_unwrap({
                let __v: Value<p9_1_1_value_initialization_calls_user_provided_constructor_Seven> =
                    Rc::new(RefCell::new(
                        <p9_1_1_value_initialization_calls_user_provided_constructor_Seven>::uninit(
                        ),
                    ));
                p9_1_1_value_initialization_calls_user_provided_constructor_Seven::new(
                    __v.as_pointer(),
                );
                __v
            })
            .ok()
            .unwrap()
            .into_inner()
            .value
        } == 7)
            && (constructed_4.with(|rc| *rc.borrow()) == 2)
    );
    let mut heap: Ptr<p9_1_1_value_initialization_calls_user_provided_constructor_Seven> = {
        let __p = Ptr::alloc(
            <p9_1_1_value_initialization_calls_user_provided_constructor_Seven>::uninit(),
        );
        p9_1_1_value_initialization_calls_user_provided_constructor_Seven::new(__p.clone());
        __p
    };
    assert!((heap.with(|__s| __s.value) == 7) && (constructed_4.with(|rc| *rc.borrow()) == 3));
    heap.delete();
}
thread_local!(
    pub static constructed_6: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct p9_1_2_value_initialization_zeroes_then_default_initializes_Pair {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
impl p9_1_2_value_initialization_zeroes_then_default_initializes_Pair {
    pub fn new_1(this: Ptr<p9_1_2_value_initialization_zeroes_then_default_initializes_Pair>) {}
    pub fn copy_from(
        this: Ptr<p9_1_2_value_initialization_zeroes_then_default_initializes_Pair>,
        _a0: Ptr<p9_1_2_value_initialization_zeroes_then_default_initializes_Pair>,
    ) {
        field!(this, first).write(_a0.with(|__s| __s.first));
        field!(this, second).write(_a0.with(|__s| __s.second));
    }
    pub fn move_from(
        this: Ptr<p9_1_2_value_initialization_zeroes_then_default_initializes_Pair>,
        _a0: Ptr<p9_1_2_value_initialization_zeroes_then_default_initializes_Pair>,
    ) {
        field!(this, first).write({ (*_a0.upgrade().deref()).first });
        field!(this, second).write({ (*_a0.upgrade().deref()).second });
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct p9_1_2_value_initialization_zeroes_then_default_initializes_Member {
    #[offset(0)]
    pub id: i32,
}
impl p9_1_2_value_initialization_zeroes_then_default_initializes_Member {
    pub fn new(this: Ptr<p9_1_2_value_initialization_zeroes_then_default_initializes_Member>) {
        field!(this, id).write((*(*constructed_6.with(Value::clone).borrow_mut()).prefix_inc()));
    }
    pub fn copy_from(
        this: Ptr<p9_1_2_value_initialization_zeroes_then_default_initializes_Member>,
        _a0: Ptr<p9_1_2_value_initialization_zeroes_then_default_initializes_Member>,
    ) {
        field!(this, id).write(_a0.with(|__s| __s.id));
    }
    pub fn move_from(
        this: Ptr<p9_1_2_value_initialization_zeroes_then_default_initializes_Member>,
        _a0: Ptr<p9_1_2_value_initialization_zeroes_then_default_initializes_Member>,
    ) {
        field!(this, id).write({ (*_a0.upgrade().deref()).id });
    }
}
impl Default for p9_1_2_value_initialization_zeroes_then_default_initializes_Member {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<p9_1_2_value_initialization_zeroes_then_default_initializes_Member> =
                Rc::new(RefCell::new(
                    <p9_1_2_value_initialization_zeroes_then_default_initializes_Member>::uninit(),
                ));
            p9_1_2_value_initialization_zeroes_then_default_initializes_Member::new(
                __v.as_pointer(),
            );
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct p9_1_2_value_initialization_zeroes_then_default_initializes_Holder {
    #[offset(0)]
    pub number: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub member: p9_1_2_value_initialization_zeroes_then_default_initializes_Member,
    #[offset(8)]
    #[byte_size(8)]
    pub pointer: Ptr<i32>,
}
impl p9_1_2_value_initialization_zeroes_then_default_initializes_Holder {
    pub fn new_1(this: Ptr<p9_1_2_value_initialization_zeroes_then_default_initializes_Holder>) {
        p9_1_2_value_initialization_zeroes_then_default_initializes_Member::new(field_ptr!(
            this, member
        ));
    }
    pub fn copy_from(
        this: Ptr<p9_1_2_value_initialization_zeroes_then_default_initializes_Holder>,
        _a0: Ptr<p9_1_2_value_initialization_zeroes_then_default_initializes_Holder>,
    ) {
        field!(this, number).write(_a0.with(|__s| __s.number));
        p9_1_2_value_initialization_zeroes_then_default_initializes_Member::copy_from(
            field_ptr!(this, member),
            { field_ptr!(_a0, member) },
        );
        field!(this, pointer).write(_a0.with(|__s| __s.pointer.clone()));
    }
    pub fn move_from(
        this: Ptr<p9_1_2_value_initialization_zeroes_then_default_initializes_Holder>,
        _a0: Ptr<p9_1_2_value_initialization_zeroes_then_default_initializes_Holder>,
    ) {
        field!(this, number).write({ (*_a0.upgrade().deref()).number });
        p9_1_2_value_initialization_zeroes_then_default_initializes_Member::move_from(
            field_ptr!(this, member),
            { field_ptr!(_a0, member) },
        );
        field!(this, pointer).write({ (*_a0.upgrade().deref()).pointer.clone() });
    }
}
pub fn test_7() {
    let braces: Value<p9_1_2_value_initialization_zeroes_then_default_initializes_Pair> = Rc::new(
        RefCell::new(<p9_1_2_value_initialization_zeroes_then_default_initializes_Pair>::uninit()),
    );
    {
        let __d = braces.as_pointer();
        ();
        ();
    };
    assert!(({ ({ (*braces.borrow()).first } == 0) && ({ (*braces.borrow()).second } == 0) }));
    assert!(
        ({
            Rc::try_unwrap({
                let __v: Value<p9_1_2_value_initialization_zeroes_then_default_initializes_Pair> =
                    Rc::new(RefCell::new(
                        <p9_1_2_value_initialization_zeroes_then_default_initializes_Pair>::uninit(
                        ),
                    ));
                {
                    let __d = __v.as_pointer();
                    ();
                    p9_1_2_value_initialization_zeroes_then_default_initializes_Pair::new_1(__d)
                };
                __v
            })
            .ok()
            .unwrap()
            .into_inner()
            .first
        } == 0)
            && ({
                Rc::try_unwrap({
                    let __v: Value<
                        p9_1_2_value_initialization_zeroes_then_default_initializes_Pair,
                    > = Rc::new(RefCell::new(
                        <p9_1_2_value_initialization_zeroes_then_default_initializes_Pair>::uninit(
                        ),
                    ));
                    {
                        let __d = __v.as_pointer();
                        ();
                        p9_1_2_value_initialization_zeroes_then_default_initializes_Pair::new_1(__d)
                    };
                    __v
                })
                .ok()
                .unwrap()
                .into_inner()
                .second
            } == 0)
    );
    let mut dirty: Ptr<p9_1_2_value_initialization_zeroes_then_default_initializes_Pair> = {
        let __p = Ptr::alloc(
            <p9_1_2_value_initialization_zeroes_then_default_initializes_Pair>::uninit(),
        );
        p9_1_2_value_initialization_zeroes_then_default_initializes_Pair::new_1(__p.clone());
        __p
    };
    field!(dirty, first).write(23130);
    field!(dirty, second).write(23130);
    dirty.delete();
    let mut heap: Ptr<p9_1_2_value_initialization_zeroes_then_default_initializes_Pair> = {
        let __p = Ptr::alloc(
            <p9_1_2_value_initialization_zeroes_then_default_initializes_Pair>::uninit(),
        );
        {
            let __d = __p.clone();
            ();
            p9_1_2_value_initialization_zeroes_then_default_initializes_Pair::new_1(__d)
        };
        __p
    };
    assert!((heap.with(|__s| __s.first) == 0) && (heap.with(|__s| __s.second) == 0));
    heap.delete();
    let holder: Value<p9_1_2_value_initialization_zeroes_then_default_initializes_Holder> =
        Rc::new(RefCell::new(
            <p9_1_2_value_initialization_zeroes_then_default_initializes_Holder>::uninit(),
        ));
    {
        let __d = holder.as_pointer();
        ();
        p9_1_2_value_initialization_zeroes_then_default_initializes_Member::new(field_ptr!(
            __d, member
        ));
        ();
    };
    assert!(
        ({
            ({ (*holder.borrow()).number } == 0)
                && (({ (*holder.borrow()).pointer.clone() }).is_null())
        })
    );
    assert!(
        ({
            (constructed_6.with(|rc| *rc.borrow()) == 1) && ({ (*holder.borrow()).member.id } == 1)
        })
    );
    let mut heap_holder: Ptr<p9_1_2_value_initialization_zeroes_then_default_initializes_Holder> = {
        let __p = Ptr::alloc(
            <p9_1_2_value_initialization_zeroes_then_default_initializes_Holder>::uninit(),
        );
        {
            let __d = __p.clone();
            ();
            p9_1_2_value_initialization_zeroes_then_default_initializes_Holder::new_1(__d)
        };
        __p
    };
    assert!(
        (heap_holder.with(|__s| __s.number) == 0)
            && ((heap_holder.with(|__s| __s.pointer.clone())).is_null())
    );
    assert!(
        (constructed_6.with(|rc| *rc.borrow()) == 2)
            && (heap_holder.with(|__s| __s.member.id) == 2)
    );
    heap_holder.delete();
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct p9_2_value_initialized_array_and_scalar_Pair {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
impl p9_2_value_initialized_array_and_scalar_Pair {
    pub fn new_1(this: Ptr<p9_2_value_initialized_array_and_scalar_Pair>) {}
    pub fn copy_from(
        this: Ptr<p9_2_value_initialized_array_and_scalar_Pair>,
        _a0: Ptr<p9_2_value_initialized_array_and_scalar_Pair>,
    ) {
        field!(this, first).write(_a0.with(|__s| __s.first));
        field!(this, second).write(_a0.with(|__s| __s.second));
    }
    pub fn move_from(
        this: Ptr<p9_2_value_initialized_array_and_scalar_Pair>,
        _a0: Ptr<p9_2_value_initialized_array_and_scalar_Pair>,
    ) {
        field!(this, first).write({ (*_a0.upgrade().deref()).first });
        field!(this, second).write({ (*_a0.upgrade().deref()).second });
    }
}
pub fn test_8() {
    let pairs: Value<Box<[p9_2_value_initialized_array_and_scalar_Pair]>> = Rc::new(RefCell::new(
        (0..2)
            .map(|_| <p9_2_value_initialized_array_and_scalar_Pair>::uninit())
            .collect::<Box<[_]>>(),
    ));
    {
        let __d = pairs.decay();
        {
            let __d = __d.offset(0);
            ();
            ();
        };
        {
            let __d = __d.offset(1);
            ();
            ();
        };
    };
    assert!(
        ({
            ({ (*pairs.borrow())[(0) as usize].first } == 0)
                && ({ (*pairs.borrow())[(1) as usize].second } == 0)
        })
    );
    let mut numbers: [i32; 3] = [0_i32; 3];
    assert!((numbers[(0) as usize] == 0) && (numbers[(2) as usize] == 0));
    let mut number: i32 = 0_i32;
    let mut pointer: Ptr<i32> = Ptr::<i32>::null();
    assert!((number == 0) && ((pointer).is_null()));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_3() });
    ({ test_5() });
    ({ test_7() });
    ({ test_8() });
    ({ test_1() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = static_pm_0.with(|_| ());
    let _ = constructed_2.with(|_| ());
    let _ = constructed_4.with(|_| ());
    let _ = constructed_6.with(|_| ());
}
