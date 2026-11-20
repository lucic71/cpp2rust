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
pub fn operator_comma_0(__ret: Ptr<S>, a: Ptr<S>, b: Ptr<S>) {
    {
        let __d = __ret.clone();
        field!(__d, v).write(({ (a.with(|__s| __s.v) * 10) } + { b.with(|__s| __s.v) }));
    };
    return;
}
pub fn operator_literal__k_1(mut v: u64) -> i64 {
    return (((v).wrapping_mul(1000_u64)) as i64);
}
pub fn operator_literal__half_2(mut v: f64) -> f64 {
    return (v / 2_f64);
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = s.as_pointer();
        field!(__d, v).write(3);
    };
    let t: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = t.as_pointer();
        field!(__d, v).write(4);
    };
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({
                    let _a: Ptr<S> = s.as_pointer();
                    operator_comma_0(__v.as_pointer(), _a, t.as_pointer())
                });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 34)
    );
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({
                    let _a: Value<S> = {
                        let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                        ({
                            let _a: Ptr<S> = s.as_pointer();
                            operator_comma_0(__v.as_pointer(), _a, t.as_pointer())
                        });
                        __v
                    };
                    let _b: Ptr<S> = s.as_pointer();
                    operator_comma_0(__v.as_pointer(), _a.as_pointer(), _b)
                });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 343)
    );
    assert!((({ operator_literal__k_1(2_u64,) }) == 2000_i64));
    assert!((({ operator_literal__half_2(3_f64,) }) == 1.5_f64));
    assert!((({ operator_literal__k_1(4_u64,) }) == 4000_i64));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
