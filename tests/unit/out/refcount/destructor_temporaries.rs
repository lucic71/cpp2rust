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
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(4)]
    pub inner: S,
    #[offset(4)]
    pub w: i32,
}
impl Outer {
    pub fn copy_from(this: Ptr<Outer>, _a0: Ptr<Outer>) {
        S::copy_from(field_ptr!(this, inner), { field_ptr!(_a0, inner) });
        field!(this, w).write(_a0.with(|__s| __s.w));
    }
}
pub fn make_2(__ret: Ptr<S>, mut v: i32) {
    {
        let __d = __ret.clone();
        field!(__d, v).write(v);
    };
    return;
}
pub fn make_outer_3(__ret: Ptr<Outer>) {
    {
        let __d = __ret.clone();
        {
            let __d = field_ptr!(__d, inner);
            field!(__d, v).write(7);
        };
        field!(__d, w).write(8);
    };
    return;
}
pub fn use_4(s: Ptr<S>) -> i32 {
    assert!((destroyed_0.with(|rc| *rc.borrow()) == 0));
    return s.with(|__s| __s.v);
}
pub fn value_5(s: Ptr<S>) -> i32 {
    return s.with(|__s| __s.v);
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    Rc::try_unwrap({
        let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
        ({ make_2(__v.as_pointer(), 1) });
        __v
    })
    .ok()
    .unwrap()
    .into_inner();
    assert!((destroyed_0.with(|rc| *rc.borrow()) == 1) && (last_1.with(|rc| *rc.borrow()) == 1));
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    &(Rc::try_unwrap({
        let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
        ({ make_2(__v.as_pointer(), 2) });
        __v
    })
    .ok()
    .unwrap()
    .into_inner());
    assert!((destroyed_0.with(|rc| *rc.borrow()) == 1) && (last_1.with(|rc| *rc.borrow()) == 2));
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    let mut x: i32 = {
        Rc::try_unwrap({
            let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
            ({ make_2(__v.as_pointer(), 3) });
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
        .v
    };
    assert!(
        ((x == 3) && (destroyed_0.with(|rc| *rc.borrow()) == 1))
            && (last_1.with(|rc| *rc.borrow()) == 3)
    );
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    x = ({
        SImpl::get(
            &{
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({ make_2(__v.as_pointer(), 4) });
                __v
            }
            .as_pointer(),
        )
    });
    assert!(
        ((x == 4) && (destroyed_0.with(|rc| *rc.borrow()) == 1))
            && (last_1.with(|rc| *rc.borrow()) == 4)
    );
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    x = {
        Rc::try_unwrap({
            let __v: Value<Outer> = Rc::new(RefCell::new(<Outer>::uninit()));
            ({ make_outer_3(__v.as_pointer()) });
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
        .inner
        .v
    };
    assert!(
        ((x == 7) && (destroyed_0.with(|rc| *rc.borrow()) == 1))
            && (last_1.with(|rc| *rc.borrow()) == 7)
    );
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    x = { S { v: 5 }.v };
    assert!((x == 5) && (destroyed_0.with(|rc| *rc.borrow()) == 1));
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    x = ({
        let _s: Value<S> = {
            let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
            ({ make_2(__v.as_pointer(), 6) });
            __v
        };
        use_4(_s.as_pointer())
    });
    assert!(
        ((x == 6) && (destroyed_0.with(|rc| *rc.borrow()) == 1))
            && (last_1.with(|rc| *rc.borrow()) == 6)
    );
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    x = (({
        let _s: Value<S> = {
            let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
            {
                let __d = __v.as_pointer();
                field!(__d, v).write(1);
            };
            __v
        };
        value_5(_s.as_pointer())
    }) + ({
        let _s: Value<S> = {
            let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
            {
                let __d = __v.as_pointer();
                field!(__d, v).write(2);
            };
            __v
        };
        value_5(_s.as_pointer())
    }));
    assert!((x == 3) && (destroyed_0.with(|rc| *rc.borrow()) == 2));
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    let __rhs = {
        (if (x == 3) {
            Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({ make_2(__v.as_pointer(), 9) });
                __v
            })
            .ok()
            .unwrap()
            .into_inner()
        } else {
            S { v: 10 }
        })
        .v
    };
    x = __rhs;
    assert!(
        ((x == 9) && (destroyed_0.with(|rc| *rc.borrow()) == 1))
            && (last_1.with(|rc| *rc.borrow()) == 9)
    );
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    {
        let __tmp_0: Value<S> = {
            let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
            ({ make_2(__v.as_pointer(), 11) });
            __v
        };
        let r: Ptr<S> = __tmp_0.as_pointer();
        assert!((destroyed_0.with(|rc| *rc.borrow()) == 0) && (r.with(|__s| __s.v) == 11));
    }
    assert!((destroyed_0.with(|rc| *rc.borrow()) == 1) && (last_1.with(|rc| *rc.borrow()) == 11));
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    {
        let s: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
        {
            let __d = s.as_pointer();
            field!(__d, v).write(12);
        };
        let _dtor_s = std::mem::DropGuard::new(s.clone(), |__p| __p.as_pointer().destructor());
        (*s.borrow_mut()) = Rc::try_unwrap({
            let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
            ({ make_2(__v.as_pointer(), 13) });
            __v
        })
        .ok()
        .unwrap()
        .into_inner();
        assert!(
            ({
                ((destroyed_0.with(|rc| *rc.borrow()) == 1)
                    && (last_1.with(|rc| *rc.borrow()) == 13))
                    && ({ (*s.borrow()).v } == 13)
            })
        );
    }
    assert!((destroyed_0.with(|rc| *rc.borrow()) == 2) && (last_1.with(|rc| *rc.borrow()) == 13));
    return 0;
}
pub trait OuterImpl {
    fn destructor(&self);
}
impl OuterImpl for Ptr<Outer> {
    fn destructor(&self) {
        field_ptr!(self, inner).destructor();
    }
}
pub trait SImpl {
    fn destructor(&self);
    fn get(&self) -> i32;
}
impl SImpl for Ptr<S> {
    fn destructor(&self) {
        (*destroyed_0.with(Value::clone).borrow_mut()).postfix_inc();
        last_1.with(|rc| *rc.borrow_mut() = (*self).with(|__s| __s.v));
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.v);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = destroyed_0.with(|_| ());
    let _ = last_1.with(|_| ());
}
