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
    pub v: u32,
}
impl S {
    pub fn new_1(this: Ptr<S>) {}
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
    pub fn move_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
    }
}
pub fn operator_bitnot_0(__ret: Ptr<S>, a: Ptr<S>) {
    {
        let __d = __ret.clone();
        field!(__d, v).write(!a.with(|__s| __s.v));
    };
    return;
}
pub fn operator_bitand_1(__ret: Ptr<S>, a: Ptr<S>, b: Ptr<S>) {
    {
        let __d = __ret.clone();
        field!(__d, v).write(({ a.with(|__s| __s.v) } & { b.with(|__s| __s.v) }));
    };
    return;
}
pub fn operator_bitor_2(__ret: Ptr<S>, a: Ptr<S>, b: Ptr<S>) {
    {
        let __d = __ret.clone();
        field!(__d, v).write(({ a.with(|__s| __s.v) } | { b.with(|__s| __s.v) }));
    };
    return;
}
pub fn operator_bitxor_3(__ret: Ptr<S>, a: Ptr<S>, b: Ptr<S>) {
    {
        let __d = __ret.clone();
        field!(__d, v).write(({ a.with(|__s| __s.v) } ^ { b.with(|__s| __s.v) }));
    };
    return;
}
pub fn operator_shl_4(__ret: Ptr<S>, a: Ptr<S>, mut n: i32) {
    {
        let __d = __ret.clone();
        field!(__d, v).write(({ a.with(|__s| __s.v) } << { n }));
    };
    return;
}
pub fn operator_shr_5(__ret: Ptr<S>, a: Ptr<S>, mut n: i32) {
    {
        let __d = __ret.clone();
        field!(__d, v).write(({ a.with(|__s| __s.v) } >> { n }));
    };
    return;
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = a.as_pointer();
        field!(__d, v).write(12_u32);
    };
    let b: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = b.as_pointer();
        field!(__d, v).write(10_u32);
    };
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({
                    let _a: Ptr<S> = a.as_pointer();
                    operator_bitnot_0(__v.as_pointer(), _a)
                });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == !12_u32)
    );
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({
                    let _a: Ptr<S> = a.as_pointer();
                    operator_bitand_1(__v.as_pointer(), _a, b.as_pointer())
                });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 8_u32)
    );
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({
                    let _a: Ptr<S> = a.as_pointer();
                    operator_bitor_2(__v.as_pointer(), _a, b.as_pointer())
                });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 14_u32)
    );
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({
                    let _a: Ptr<S> = a.as_pointer();
                    operator_bitxor_3(__v.as_pointer(), _a, b.as_pointer())
                });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 6_u32)
    );
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({
                    let _a: Ptr<S> = a.as_pointer();
                    operator_shl_4(__v.as_pointer(), _a, 2)
                });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 48_u32)
    );
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({
                    let _a: Ptr<S> = a.as_pointer();
                    operator_shr_5(__v.as_pointer(), _a, 2)
                });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 3_u32)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
