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
pub struct S {
    #[offset(0)]
    pub v: i32,
}
impl S {
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
}
impl CppLess for S {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_0(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for S {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_eq_1(a: Ptr<S>, b: Ptr<S>) -> bool {
    return ({ a.with(|__s| __s.v) } == { b.with(|__s| __s.v) });
}
pub fn operator_ne_2(a: Ptr<S>, b: Ptr<S>) -> bool {
    return ({ a.with(|__s| __s.v) } != { b.with(|__s| __s.v) });
}
pub fn operator_lt_0(a: Ptr<S>, b: Ptr<S>) -> bool {
    return ({ a.with(|__s| __s.v) } < { b.with(|__s| __s.v) });
}
pub fn operator_gt_3(a: Ptr<S>, b: Ptr<S>) -> bool {
    return ({ a.with(|__s| __s.v) } > { b.with(|__s| __s.v) });
}
pub fn operator_le_4(a: Ptr<S>, b: Ptr<S>) -> bool {
    return ({ a.with(|__s| __s.v) } <= { b.with(|__s| __s.v) });
}
pub fn operator_ge_5(a: Ptr<S>, b: Ptr<S>) -> bool {
    return ({ a.with(|__s| __s.v) } >= { b.with(|__s| __s.v) });
}
pub fn operator_lt_6(a: Ptr<S>, mut b: i32) -> bool {
    return ({ a.with(|__s| __s.v) } < { b });
}
pub fn operator_lt_7(mut a: i32, b: Ptr<S>) -> bool {
    return ({ a } < { b.with(|__s| __s.v) });
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct V {
    #[offset(0)]
    pub v: i32,
}
impl V {
    pub fn copy_from(this: Ptr<V>, _a0: Ptr<V>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
}
impl CppLess for V {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_8(
            Rc::new(RefCell::new(__lhs.read())),
            Rc::new(RefCell::new(__rhs.read())),
        )
    }
}
impl CppEq for V {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_9(
            Rc::new(RefCell::new(__lhs.read())),
            Rc::new(RefCell::new(__rhs.read())),
        )
    }
}
pub fn operator_eq_9(a: Value<V>, b: Value<V>) -> bool {
    return ({ (*a.borrow()).v } == { (*b.borrow()).v });
}
pub fn operator_ne_10(a: Value<V>, b: Value<V>) -> bool {
    return ({ (*a.borrow()).v } != { (*b.borrow()).v });
}
pub fn operator_lt_8(a: Value<V>, b: Value<V>) -> bool {
    return ({ (*a.borrow()).v } < { (*b.borrow()).v });
}
pub fn operator_gt_11(a: Value<V>, b: Value<V>) -> bool {
    return ({ (*a.borrow()).v } > { (*b.borrow()).v });
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<V> = Rc::new(RefCell::new(<V>::uninit()));
    {
        let __d = x.as_pointer();
        field!(__d, v).write(1);
    };
    let y: Value<V> = Rc::new(RefCell::new(<V>::uninit()));
    {
        let __d = y.as_pointer();
        field!(__d, v).write(2);
    };
    let z: Value<V> = Rc::new(RefCell::new(<V>::uninit()));
    {
        let __d = z.as_pointer();
        field!(__d, v).write(1);
    };
    assert!(
        ({
            let _a: V = {
                let __v: Value<V> = Rc::new(RefCell::new(<V>::uninit()));
                V::copy_from(__v.as_pointer(), { x.as_pointer() });
                __v
            };
            operator_eq_9(_a, {
                let __v: Value<V> = Rc::new(RefCell::new(<V>::uninit()));
                V::copy_from(__v.as_pointer(), { z.as_pointer() });
                __v
            })
        })
    );
    assert!(
        ({
            let _a: V = {
                let __v: Value<V> = Rc::new(RefCell::new(<V>::uninit()));
                V::copy_from(__v.as_pointer(), { x.as_pointer() });
                __v
            };
            operator_ne_10(_a, {
                let __v: Value<V> = Rc::new(RefCell::new(<V>::uninit()));
                V::copy_from(__v.as_pointer(), { y.as_pointer() });
                __v
            })
        })
    );
    assert!(
        ({
            let _a: V = {
                let __v: Value<V> = Rc::new(RefCell::new(<V>::uninit()));
                V::copy_from(__v.as_pointer(), { x.as_pointer() });
                __v
            };
            operator_lt_8(_a, {
                let __v: Value<V> = Rc::new(RefCell::new(<V>::uninit()));
                V::copy_from(__v.as_pointer(), { y.as_pointer() });
                __v
            })
        })
    );
    assert!(
        ({
            let _a: V = {
                let __v: Value<V> = Rc::new(RefCell::new(<V>::uninit()));
                V::copy_from(__v.as_pointer(), { y.as_pointer() });
                __v
            };
            operator_gt_11(_a, {
                let __v: Value<V> = Rc::new(RefCell::new(<V>::uninit()));
                V::copy_from(__v.as_pointer(), { x.as_pointer() });
                __v
            })
        })
    );
    assert!(
        !({
            let _a: V = {
                let __v: Value<V> = Rc::new(RefCell::new(<V>::uninit()));
                V::copy_from(__v.as_pointer(), { y.as_pointer() });
                __v
            };
            operator_lt_8(_a, {
                let __v: Value<V> = Rc::new(RefCell::new(<V>::uninit()));
                V::copy_from(__v.as_pointer(), { x.as_pointer() });
                __v
            })
        })
    );
    let a: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = a.as_pointer();
        field!(__d, v).write(1);
    };
    let b: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = b.as_pointer();
        field!(__d, v).write(2);
    };
    let c: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = c.as_pointer();
        field!(__d, v).write(1);
    };
    assert!(
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_eq_1(_a, c.as_pointer())
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_ne_2(_a, b.as_pointer())
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_lt_0(_a, b.as_pointer())
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = b.as_pointer();
            operator_gt_3(_a, a.as_pointer())
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_le_4(_a, c.as_pointer())
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_ge_5(_a, c.as_pointer())
        })
    );
    assert!(
        !({
            let _a: Ptr<S> = b.as_pointer();
            operator_lt_0(_a, a.as_pointer())
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_lt_6(_a, 5)
        })
    );
    assert!(({ operator_lt_7(0, a.as_pointer(),) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
