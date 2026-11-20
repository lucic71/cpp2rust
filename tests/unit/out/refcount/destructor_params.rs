extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static destroyed_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static last_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: i32,
}
impl S {
    pub fn new_1(this: Ptr<S>) {}
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
}
pub fn by_value_2(s: Value<S>) -> i32 {
    assert!((destroyed_0.with(|rc| *rc.borrow()) == 0));
    return { (*s.borrow()).v };
}
pub fn with_default_3(s: Value<S>) -> i32 {
    return { (*s.borrow()).v };
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Holder {
    #[offset(0)]
    pub v: i32,
}
impl Holder {
    pub fn new(this: Ptr<Holder>, s: Value<S>) {
        field!(this, v).write({ (*s.borrow()).v });
    }
    pub fn copy_from(this: Ptr<Holder>, _a0: Ptr<Holder>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
    pub fn move_from(this: Ptr<Holder>, _a0: Ptr<Holder>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    {
        let s: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
        {
            let __d = s.as_pointer();
            field!(__d, v).write(1);
        };
        let _dtor_s = std::mem::DropGuard::new(s.clone(), |__p| __p.as_pointer().destructor());
        assert!(
            (({
                by_value_2({
                    let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                    S::copy_from(__v.as_pointer(), { s.as_pointer() });
                    __v
                })
            }) == 1)
        );
        assert!(
            (destroyed_0.with(|rc| *rc.borrow()) == 1) && (last_1.with(|rc| *rc.borrow()) == 1)
        );
    }
    assert!((destroyed_0.with(|rc| *rc.borrow()) == 2));
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    assert!(
        (({
            by_value_2({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                {
                    let __d = __v.as_pointer();
                    field!(__d, v).write(2);
                };
                __v
            })
        }) == 2)
    );
    assert!((destroyed_0.with(|rc| *rc.borrow()) == 1) && (last_1.with(|rc| *rc.borrow()) == 2));
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    assert!(
        (({
            with_default_3({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                {
                    let __d = __v.as_pointer();
                    field!(__d, v).write(5);
                };
                __v
            })
        }) == 5)
    );
    assert!((destroyed_0.with(|rc| *rc.borrow()) == 1) && (last_1.with(|rc| *rc.borrow()) == 5));
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    {
        let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::uninit()));
        Holder::new(h.as_pointer(), {
            {
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                {
                    let __d = __v.as_pointer();
                    field!(__d, v).write(3);
                };
                __v
            }
        });
        assert!(
            (destroyed_0.with(|rc| *rc.borrow()) == 1) && (last_1.with(|rc| *rc.borrow()) == 3)
        );
        assert!(
            (({
                HolderImpl::add(&h.as_pointer(), {
                    let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                    {
                        let __d = __v.as_pointer();
                        field!(__d, v).write(4);
                    };
                    __v
                })
            }) == 7)
        );
        assert!(
            (destroyed_0.with(|rc| *rc.borrow()) == 2) && (last_1.with(|rc| *rc.borrow()) == 4)
        );
    }
    assert!((destroyed_0.with(|rc| *rc.borrow()) == 2));
    return 0;
}
pub trait HolderImpl {
    fn add(&self, s: Value<S>) -> i32;
}
impl HolderImpl for Ptr<Holder> {
    fn add(&self, s: Value<S>) -> i32 {
        return ((*self).with(|__s| __s.v) + { (*s.borrow()).v });
    }
}
pub trait SImpl {
    fn destructor(&self);
}
impl SImpl for Ptr<S> {
    fn destructor(&self) {
        (*destroyed_0.with(Value::clone).borrow_mut()).postfix_inc();
        last_1.with(|rc| *rc.borrow_mut() = (*self).with(|__s| __s.v));
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = destroyed_0.with(|_| ());
    let _ = last_1.with(|_| ());
}
