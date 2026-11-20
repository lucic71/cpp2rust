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
pub struct p2_static_storage_is_zero_initialized_Pair {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
impl p2_static_storage_is_zero_initialized_Pair {
    pub fn new_1(this: Ptr<p2_static_storage_is_zero_initialized_Pair>) {}
    pub fn copy_from(
        this: Ptr<p2_static_storage_is_zero_initialized_Pair>,
        _a0: Ptr<p2_static_storage_is_zero_initialized_Pair>,
    ) {
        field!(this, first).write(_a0.with(|__s| __s.first));
        field!(this, second).write(_a0.with(|__s| __s.second));
    }
    pub fn move_from(
        this: Ptr<p2_static_storage_is_zero_initialized_Pair>,
        _a0: Ptr<p2_static_storage_is_zero_initialized_Pair>,
    ) {
        field!(this, first).write({ (*_a0.upgrade().deref()).first });
        field!(this, second).write({ (*_a0.upgrade().deref()).second });
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct p2_static_storage_is_zero_initialized_Partial {
    #[offset(0)]
    pub set: i32,
    #[offset(4)]
    pub unset: i32,
}
impl p2_static_storage_is_zero_initialized_Partial {
    pub fn new(this: Ptr<p2_static_storage_is_zero_initialized_Partial>) {
        field!(this, set).write(4);
    }
    pub fn copy_from(
        this: Ptr<p2_static_storage_is_zero_initialized_Partial>,
        _a0: Ptr<p2_static_storage_is_zero_initialized_Partial>,
    ) {
        field!(this, set).write(_a0.with(|__s| __s.set));
        field!(this, unset).write(_a0.with(|__s| __s.unset));
    }
    pub fn move_from(
        this: Ptr<p2_static_storage_is_zero_initialized_Partial>,
        _a0: Ptr<p2_static_storage_is_zero_initialized_Partial>,
    ) {
        field!(this, set).write({ (*_a0.upgrade().deref()).set });
        field!(this, unset).write({ (*_a0.upgrade().deref()).unset });
    }
}
impl Default for p2_static_storage_is_zero_initialized_Partial {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<p2_static_storage_is_zero_initialized_Partial> = Rc::new(RefCell::new(
                <p2_static_storage_is_zero_initialized_Partial>::uninit(),
            ));
            p2_static_storage_is_zero_initialized_Partial::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
thread_local!(
    pub static global_pair_0: Value<p2_static_storage_is_zero_initialized_Pair> = {
        let __v: Value<p2_static_storage_is_zero_initialized_Pair> = Rc::new(RefCell::new(
            <p2_static_storage_is_zero_initialized_Pair>::uninit(),
        ));
        p2_static_storage_is_zero_initialized_Pair::new_1(__v.as_pointer());
        __v
    };
);
thread_local!(
    pub static global_numbers_1: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..3).map(|_| 0_i32).collect::<Box<[i32]>>()));
);
thread_local!(
    pub static global_pointer_2: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
);
thread_local!(
    pub static global_partial_3: Value<p2_static_storage_is_zero_initialized_Partial> = {
        let __v: Value<p2_static_storage_is_zero_initialized_Partial> = Rc::new(RefCell::new(
            <p2_static_storage_is_zero_initialized_Partial>::uninit(),
        ));
        p2_static_storage_is_zero_initialized_Partial::new(__v.as_pointer());
        __v
    };
);
pub fn read_static_local_4() -> i32 {
    return (({ local_pair_5.with(|rc| *rc.borrow()).first } + {
        local_pair_5.with(|rc| *rc.borrow()).second
    }) + { local_partial_6.with(|rc| *rc.borrow()).unset });
}
pub fn test_7() {
    assert!(
        ({ global_pair_0.with(|rc| *rc.borrow()).first } == 0)
            && ({ global_pair_0.with(|rc| *rc.borrow()).second } == 0)
    );
    assert!(
        (({
            let __idx0 = (0) as usize;
            global_numbers_1.with(|rc| rc.borrow()[__idx0])
        }) == 0)
            && (({
                let __idx0 = (2) as usize;
                global_numbers_1.with(|rc| rc.borrow()[__idx0])
            }) == 0)
    );
    assert!(({ (*global_pointer_2.with(Value::clone).borrow()).is_null() }));
    assert!(
        ({ global_partial_3.with(|rc| *rc.borrow()).set } == 4)
            && ({ global_partial_3.with(|rc| *rc.borrow()).unset } == 0)
    );
    assert!((({ read_static_local_4() }) == 0));
}
thread_local!(
    static local_pair_5: Value<p2_static_storage_is_zero_initialized_Pair> = {
        let __v: Value<p2_static_storage_is_zero_initialized_Pair> = Rc::new(RefCell::new(
            <p2_static_storage_is_zero_initialized_Pair>::uninit(),
        ));
        p2_static_storage_is_zero_initialized_Pair::new_1(__v.as_pointer());
        __v
    };
);
thread_local!(
    static local_partial_6: Value<p2_static_storage_is_zero_initialized_Partial> = {
        let __v: Value<p2_static_storage_is_zero_initialized_Partial> = Rc::new(RefCell::new(
            <p2_static_storage_is_zero_initialized_Partial>::uninit(),
        ));
        p2_static_storage_is_zero_initialized_Partial::new(__v.as_pointer());
        __v
    };
);
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_7() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = global_pair_0.with(|_| ());
    let _ = global_numbers_1.with(|_| ());
    let _ = global_pointer_2.with(|_| ());
    let _ = global_partial_3.with(|_| ());
}
