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
pub struct Lt {
    #[offset(0)]
    pub v: i32,
}
impl Lt {
    pub fn new_1(this: Ptr<Lt>) {}
    pub fn copy_from(this: Ptr<Lt>, _a0: Ptr<Lt>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
    pub fn move_from(this: Ptr<Lt>, _a0: Ptr<Lt>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
    }
}
impl CppLess for Lt {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        LtImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Lt {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(LtImpl::operator_lt(__lhs, __rhs.clone())) && !(LtImpl::operator_lt(__rhs, __lhs.clone()))
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Eq {
    #[offset(0)]
    pub v: i32,
}
impl Eq {
    pub fn new_1(this: Ptr<Eq>) {}
    pub fn copy_from(this: Ptr<Eq>, _a0: Ptr<Eq>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
    pub fn move_from(this: Ptr<Eq>, _a0: Ptr<Eq>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
    }
}
impl CppEq for Eq {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        EqImpl::operator_eq(__lhs, __rhs.clone())
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Cmp {
    #[offset(0)]
    pub v: i32,
}
impl Cmp {
    pub fn new_1(this: Ptr<Cmp>) {}
    pub fn copy_from(this: Ptr<Cmp>, _a0: Ptr<Cmp>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
    pub fn move_from(this: Ptr<Cmp>, _a0: Ptr<Cmp>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
    }
}
impl CppLess for Cmp {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        CmpImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for Cmp {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        CmpImpl::operator_eq(__lhs, __rhs.clone())
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Free {
    #[offset(0)]
    pub v: i32,
}
impl Free {
    pub fn new_1(this: Ptr<Free>) {}
    pub fn copy_from(this: Ptr<Free>, _a0: Ptr<Free>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
    pub fn move_from(this: Ptr<Free>, _a0: Ptr<Free>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
    }
}
impl CppLess for Free {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_0(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for Free {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_lt_0(a: Ptr<Free>, b: Ptr<Free>) -> bool {
    return ({ a.with(|__s| __s.v) } < { b.with(|__s| __s.v) });
}
pub fn operator_eq_1(a: Ptr<Free>, b: Ptr<Free>) -> bool {
    return ({ a.with(|__s| __s.v) } == { b.with(|__s| __s.v) });
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Wrapped_int_ {
    #[offset(0)]
    pub v: i32,
}
impl Wrapped_int_ {
    pub fn new_1(this: Ptr<Wrapped_int_>) {}
    pub fn copy_from(this: Ptr<Wrapped_int_>, _a0: Ptr<Wrapped_int_>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
    pub fn move_from(this: Ptr<Wrapped_int_>, _a0: Ptr<Wrapped_int_>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let w: Value<Wrapped_int_> = Rc::new(RefCell::new(<Wrapped_int_>::uninit()));
    {
        let __d = w.as_pointer();
        field!(__d, v).write(2);
    };
    assert!(({ ({ (*w.borrow()).v } == 2) }));
    let lts: Value<Vec<Lt>> = Rc::new(RefCell::new(Vec::from([
        Rc::try_unwrap({
            let __v: Value<Lt> = Rc::new(RefCell::new(<Lt>::uninit()));
            {
                let __d = __v.as_pointer();
                field!(__d, v).write(3);
            };
            __v
        })
        .ok()
        .unwrap()
        .into_inner(),
        Rc::try_unwrap({
            let __v: Value<Lt> = Rc::new(RefCell::new(<Lt>::uninit()));
            {
                let __d = __v.as_pointer();
                field!(__d, v).write(1);
            };
            __v
        })
        .ok()
        .unwrap()
        .into_inner(),
        Rc::try_unwrap({
            let __v: Value<Lt> = Rc::new(RefCell::new(<Lt>::uninit()));
            {
                let __d = __v.as_pointer();
                field!(__d, v).write(2);
            };
            __v
        })
        .ok()
        .unwrap()
        .into_inner(),
    ])));
    {
        let mut __tmp_0 = { lts.decay().begin() };
        let __n = lts.decay().end().iter_distance(&__tmp_0);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| cpp_cmp(&__tmp_0.iter_at(__x), &__tmp_0.iter_at(__y)));
        reorder(__order, |__i| __tmp_0.iter_at(__i));
    };
    assert!(
        ({
            (({ { (*lts.borrow())[0_usize] }.v } == 1) && ({ { (*lts.borrow())[1_usize] }.v } == 2))
                && ({ { (*lts.borrow())[2_usize] }.v } == 3)
        })
    );
    let eqs: Value<Vec<Eq>> = Rc::new(RefCell::new(Vec::from([
        Rc::try_unwrap({
            let __v: Value<Eq> = Rc::new(RefCell::new(<Eq>::uninit()));
            {
                let __d = __v.as_pointer();
                field!(__d, v).write(1);
            };
            __v
        })
        .ok()
        .unwrap()
        .into_inner(),
        Rc::try_unwrap({
            let __v: Value<Eq> = Rc::new(RefCell::new(<Eq>::uninit()));
            {
                let __d = __v.as_pointer();
                field!(__d, v).write(2);
            };
            __v
        })
        .ok()
        .unwrap()
        .into_inner(),
        Rc::try_unwrap({
            let __v: Value<Eq> = Rc::new(RefCell::new(<Eq>::uninit()));
            {
                let __d = __v.as_pointer();
                field!(__d, v).write(3);
            };
            __v
        })
        .ok()
        .unwrap()
        .into_inner(),
    ])));
    let two: Value<Eq> = Rc::new(RefCell::new(<Eq>::uninit()));
    {
        let __d = two.as_pointer();
        field!(__d, v).write(2);
    };
    let nine: Value<Eq> = Rc::new(RefCell::new(<Eq>::uninit()));
    {
        let __d = nine.as_pointer();
        field!(__d, v).write(9);
    };
    assert!(
        ({
            let mut __tmp_1 = { eqs.decay().begin() };
            let mut __tmp_2 = { eqs.decay().end() };
            let mut __it = (__tmp_1).clone();
            while __it != __tmp_2 && !CppEq::equal(&__it.iter_deref(), &two.as_pointer()) {
                __it.iter_inc();
            }
            __it
        }
        .iter_deref()
        .with(|__s| __s.v)
            == 2)
    );
    assert!(
        {
            let mut __tmp_3 = { eqs.decay().begin() };
            let mut __tmp_4 = { eqs.decay().end() };
            let mut __it = (__tmp_3).clone();
            while __it != __tmp_4 && !CppEq::equal(&__it.iter_deref(), &nine.as_pointer()) {
                __it.iter_inc();
            }
            __it
        } == eqs.decay().end()
    );
    let cmps: Value<Vec<Cmp>> = Rc::new(RefCell::new(Vec::from([
        Rc::try_unwrap({
            let __v: Value<Cmp> = Rc::new(RefCell::new(<Cmp>::uninit()));
            {
                let __d = __v.as_pointer();
                field!(__d, v).write(3);
            };
            __v
        })
        .ok()
        .unwrap()
        .into_inner(),
        Rc::try_unwrap({
            let __v: Value<Cmp> = Rc::new(RefCell::new(<Cmp>::uninit()));
            {
                let __d = __v.as_pointer();
                field!(__d, v).write(1);
            };
            __v
        })
        .ok()
        .unwrap()
        .into_inner(),
        Rc::try_unwrap({
            let __v: Value<Cmp> = Rc::new(RefCell::new(<Cmp>::uninit()));
            {
                let __d = __v.as_pointer();
                field!(__d, v).write(2);
            };
            __v
        })
        .ok()
        .unwrap()
        .into_inner(),
    ])));
    {
        let mut __tmp_5 = { cmps.decay().begin() };
        let __n = cmps.decay().end().iter_distance(&__tmp_5);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| cpp_cmp(&__tmp_5.iter_at(__x), &__tmp_5.iter_at(__y)));
        reorder(__order, |__i| __tmp_5.iter_at(__i));
    };
    assert!(
        ({
            ({ { (*cmps.borrow())[0_usize] }.v } == 1) && ({ { (*cmps.borrow())[2_usize] }.v } == 3)
        })
    );
    let three: Value<Cmp> = Rc::new(RefCell::new(<Cmp>::uninit()));
    {
        let __d = three.as_pointer();
        field!(__d, v).write(3);
    };
    assert!(
        ({
            let mut __tmp_6 = { cmps.decay().begin() };
            let mut __tmp_7 = { cmps.decay().end() };
            let mut __it = (__tmp_6).clone();
            while __it != __tmp_7 && !CppEq::equal(&__it.iter_deref(), &three.as_pointer()) {
                __it.iter_inc();
            }
            __it
        }
        .iter_deref()
        .with(|__s| __s.v)
            == 3)
    );
    let frees: Value<Vec<Free>> = Rc::new(RefCell::new(Vec::from([
        Rc::try_unwrap({
            let __v: Value<Free> = Rc::new(RefCell::new(<Free>::uninit()));
            {
                let __d = __v.as_pointer();
                field!(__d, v).write(2);
            };
            __v
        })
        .ok()
        .unwrap()
        .into_inner(),
        Rc::try_unwrap({
            let __v: Value<Free> = Rc::new(RefCell::new(<Free>::uninit()));
            {
                let __d = __v.as_pointer();
                field!(__d, v).write(1);
            };
            __v
        })
        .ok()
        .unwrap()
        .into_inner(),
    ])));
    {
        let mut __tmp_8 = { frees.decay().begin() };
        let __n = frees.decay().end().iter_distance(&__tmp_8);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| cpp_cmp(&__tmp_8.iter_at(__x), &__tmp_8.iter_at(__y)));
        reorder(__order, |__i| __tmp_8.iter_at(__i));
    };
    assert!(({ ({ { (*frees.borrow())[0_usize] }.v } == 1) }));
    let ftwo: Value<Free> = Rc::new(RefCell::new(<Free>::uninit()));
    {
        let __d = ftwo.as_pointer();
        field!(__d, v).write(2);
    };
    assert!(
        ({
            let mut __tmp_9 = { frees.decay().begin() };
            let mut __tmp_10 = { frees.decay().end() };
            let mut __it = (__tmp_9).clone();
            while __it != __tmp_10 && !CppEq::equal(&__it.iter_deref(), &ftwo.as_pointer()) {
                __it.iter_inc();
            }
            __it
        }
        .iter_deref()
        .with(|__s| __s.v)
            == 2)
    );
    let m: Value<Map<Lt, (Value<Lt>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    match ({
        let __v: Value<Lt> = Rc::new(RefCell::new(<Lt>::uninit()));
        {
            let __d = __v.as_pointer();
            field!(__d, v).write(2);
        };
        __v
    }
    .as_pointer(),)
    {
        (__tmp_11,) => {
            ({ (*m.borrow_mut()).subscript_move(__tmp_11, |__k| cpp_move!(__k), || 0_i32) })
        }
    }
    .write(20);
    match ({
        let __v: Value<Lt> = Rc::new(RefCell::new(<Lt>::uninit()));
        {
            let __d = __v.as_pointer();
            field!(__d, v).write(1);
        };
        __v
    }
    .as_pointer(),)
    {
        (__tmp_12,) => {
            ({ (*m.borrow_mut()).subscript_move(__tmp_12, |__k| cpp_move!(__k), || 0_i32) })
        }
    }
    .write(10);
    assert!(
        ((*(*m
            .as_pointer()
            .with(|__m: &Map<Lt, (Value<Lt>, Value<i32>)>| __m.begin())
            .iter_deref()
            .upgrade()
            .deref())
        .1
        .borrow())
            == 10)
    );
    assert!(
        ((match ({
            let __v: Value<Lt> = Rc::new(RefCell::new(<Lt>::uninit()));
            {
                let __d = __v.as_pointer();
                field!(__d, v).write(2);
            };
            __v
        }
        .as_pointer(),)
        {
            (__tmp_13,) => {
                ({ (*m.borrow_mut()).subscript_move(__tmp_13, |__k| cpp_move!(__k), || 0_i32) })
            }
        }
        .read())
            == 20)
    );
    return 0;
}
pub trait CmpImpl {
    fn operator_cmp(&self, o: Ptr<Cmp>) -> std::cmp::Ordering;
    fn operator_eq(&self, o: Ptr<Cmp>) -> bool;
}
impl CmpImpl for Ptr<Cmp> {
    fn operator_cmp(&self, o: Ptr<Cmp>) -> std::cmp::Ordering {
        return std::cmp::Ord::cmp(&((*self).with(|__s| __s.v)), &(o.with(|__s| __s.v)));
    }
    fn operator_eq(&self, o: Ptr<Cmp>) -> bool {
        return ({ (*self).with(|__s| __s.v) } == { o.with(|__s| __s.v) });
    }
}
pub trait EqImpl {
    fn operator_eq(&self, o: Ptr<Eq>) -> bool;
}
impl EqImpl for Ptr<Eq> {
    fn operator_eq(&self, o: Ptr<Eq>) -> bool {
        return ({ (*self).with(|__s| __s.v) } == { o.with(|__s| __s.v) });
    }
}
pub trait LtImpl {
    fn operator_lt(&self, o: Ptr<Lt>) -> bool;
}
impl LtImpl for Ptr<Lt> {
    fn operator_lt(&self, o: Ptr<Lt>) -> bool {
        return ({ (*self).with(|__s| __s.v) } < { o.with(|__s| __s.v) });
    }
}
pub fn __cpp2rust_init_globals() {}
