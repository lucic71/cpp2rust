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
pub struct Inner {
    #[offset(0)]
    pub value: i32,
}
impl Inner {
    pub fn copy_from(this: Ptr<Inner>, _a0: Ptr<Inner>) {
        field!(this, value).write(_a0.with(|__s| __s.value));
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Ptr<Inner>,
}
impl Outer {
    pub fn copy_from(this: Ptr<Outer>, _a0: Ptr<Outer>) {
        field!(this, p).write(_a0.with(|__s| __s.p.clone()));
    }
}
thread_local!(
    pub static alpha_0: Value<Inner> = {
        let __v: Value<Inner> = Rc::new(RefCell::new(<Inner>::uninit()));
        {
            let __d = __v.as_pointer();
            field!(__d, value).write(1);
        };
        __v
    };
);
thread_local!(
    pub static beta_1: Value<Inner> = {
        let __v: Value<Inner> = Rc::new(RefCell::new(<Inner>::uninit()));
        {
            let __d = __v.as_pointer();
            field!(__d, value).write(2);
        };
        __v
    };
);
thread_local!(
    pub static shared_2: Value<Inner> = {
        let __v: Value<Inner> = Rc::new(RefCell::new(<Inner>::uninit()));
        {
            let __d = __v.as_pointer();
            field!(__d, value).write(42);
        };
        __v
    };
);
thread_local!(
    pub static items_3: Value<Box<[Ptr<Inner>]>> = Rc::new(RefCell::new(Box::new([
        (alpha_0.with(|v| v.as_pointer())),
        (beta_1.with(|v| v.as_pointer())),
    ])));
);
thread_local!(
    pub static obj_4: Value<Outer> = {
        let __v: Value<Outer> = Rc::new(RefCell::new(<Outer>::uninit()));
        {
            let __d = __v.as_pointer();
            field!(__d, p).write((shared_2.with(|v| v.as_pointer())));
        };
        __v
    };
);
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (({
            let __idx0 = (0) as usize;
            items_3.with(|rc| rc.borrow()[__idx0].clone())
        })
        .with(|__s| __s.value)
            == 1)
    );
    assert!(
        (({
            let __idx0 = (1) as usize;
            items_3.with(|rc| rc.borrow()[__idx0].clone())
        })
        .with(|__s| __s.value)
            == 2)
    );
    assert!(({ ({ (*obj_4.with(Value::clone).borrow()).p.clone() }.with(|__s| __s.value) == 42) }));
    assert!(
        (({
            let __idx0 = (0) as usize;
            cache_5.with(|rc| rc.borrow()[__idx0].clone())
        })
        .with(|__s| __s.value)
            == 1)
    );
    assert!(
        (({
            let __idx0 = (1) as usize;
            cache_5.with(|rc| rc.borrow()[__idx0].clone())
        })
        .with(|__s| __s.value)
            == 2)
    );
    return 0;
}
thread_local!(
    static cache_5: Value<Box<[Ptr<Inner>]>> = Rc::new(RefCell::new(Box::new([
        (alpha_0.with(|v| v.as_pointer())),
        (beta_1.with(|v| v.as_pointer())),
    ])));
);
pub fn __cpp2rust_init_globals() {
    let _ = alpha_0.with(|_| ());
    let _ = beta_1.with(|_| ());
    let _ = shared_2.with(|_| ());
    let _ = items_3.with(|_| ());
    let _ = obj_4.with(|_| ());
}
