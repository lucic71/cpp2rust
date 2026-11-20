extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Destructor, Default)]
#[placement]
#[byte_size(4)]
pub struct Tracker {
    #[offset(0)]
    pub tag: i32,
}
impl Tracker {
    pub fn new(this: Ptr<Tracker>, mut tag: i32) {
        field!(this, tag).write(tag);
        printf!(b"construct %d\n", tag);
    }
    pub fn copy_from(this: Ptr<Tracker>, _a0: Ptr<Tracker>) {
        field!(this, tag).write(_a0.with(|__s| __s.tag));
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Destructor, Default)]
#[placement]
#[byte_size(4)]
pub struct HoldsTracker {
    #[offset(0)]
    #[byte_size(4)]
    pub member: Tracker,
}
impl HoldsTracker {
    pub fn copy_from(this: Ptr<HoldsTracker>, _a0: Ptr<HoldsTracker>) {
        Tracker::copy_from(field_ptr!(this, member), { field_ptr!(_a0, member) });
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct TriviallyDestructible {
    #[offset(0)]
    pub value: i32,
}
thread_local!(
    pub static global_0: Value<Tracker> = {
        let __static = {
            let __v: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
            Tracker::new(__v.as_pointer(), { 1 });
            __v
        };
        libcc2rs::register_static_destructor(|| {
            global_0.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
pub fn never_called_1() {
    ({ TrackerImpl::touch(&never_constructed_2.with(|v| v.as_pointer())) });
}
pub fn test_3() {
    ({ TrackerImpl::touch(&global_0.with(|v| v.as_pointer())) });
}
thread_local!(
    static never_constructed_2: Value<Tracker> = {
        let __static = {
            let __v: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
            Tracker::new(__v.as_pointer(), { 100 });
            __v
        };
        libcc2rs::register_static_destructor(|| {
            never_constructed_2
                .with(Value::clone)
                .as_pointer()
                .destructor()
        });
        __static
    };
);
pub fn local_static_during_global_init_4() -> Ptr<Tracker> {
    return during_init_5.with(|v| v.as_pointer());
}
thread_local!(
    pub static before_6: Value<Tracker> = {
        let __static = {
            let __v: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
            Tracker::new(__v.as_pointer(), { 2 });
            __v
        };
        libcc2rs::register_static_destructor(|| {
            before_6.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
thread_local!(
    pub static after_7: Value<Tracker> = {
        let __static = {
            let __v: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
            Tracker::new(__v.as_pointer(), {
                (({ local_static_during_global_init_4() }).with(|__s| __s.tag) + 1)
            });
            __v
        };
        libcc2rs::register_static_destructor(|| {
            after_7.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
thread_local!(
    pub static holder_8: Value<HoldsTracker> = {
        let __static = {
            let __v: Value<HoldsTracker> = Rc::new(RefCell::new(<HoldsTracker>::uninit()));
            {
                let __d = __v.as_pointer();
                Tracker::new(field_ptr!(__d, member), { 5 });
            };
            __v
        };
        libcc2rs::register_static_destructor(|| {
            holder_8.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
thread_local!(
    pub static trivial_9: Value<TriviallyDestructible> =
        Rc::new(RefCell::new(TriviallyDestructible { value: 6 }));
);
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p3_reverse_completion_order_WithStaticMember {}
thread_local!(
    pub static member_10: Value<Tracker> = {
        let __static = {
            let __v: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
            Tracker::new(__v.as_pointer(), { 7 });
            __v
        };
        libcc2rs::register_static_destructor(|| {
            member_10.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
thread_local!(
    pub static capturing_11: Value<FnPtr<fn() -> i32>> = {
        let __static = Rc::new(RefCell::new(lambda!(
            {
                let captured: Value<Tracker> = {
                    let __v: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
                    Tracker::new(__v.as_pointer(), { 8 });
                    __v
                };
            },
            || -> i32 {
                return { (*captured.borrow()).tag };
            }
        )));
        libcc2rs::register_static_destructor(|| {
            capturing_11
                .with(Value::clone)
                .as_pointer()
                .with(|__f| __f.destroy())
        });
        __static
    };
);
pub fn run_local_static_12() {
    ({ TrackerImpl::touch(&during_main_13.with(|v| v.as_pointer())) });
}
pub fn test_14() {
    ({ run_local_static_12() });
    ({ run_local_static_12() });
    ({ TrackerImpl::touch(&({ local_static_during_global_init_4() })) });
    printf!(b"trivial %d\n", { trivial_9.with(|rc| *rc.borrow()).value });
    ({ TrackerImpl::touch(&member_10.with(|v| v.as_pointer())) });
    printf!(
        b"captured %d\n",
        ({ (*capturing_11.with(Value::clone).borrow()).call() })
    );
}
thread_local!(
    static during_init_5: Value<Tracker> = {
        let __static = {
            let __v: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
            Tracker::new(__v.as_pointer(), { 3 });
            __v
        };
        libcc2rs::register_static_destructor(|| {
            during_init_5.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
thread_local!(
    static during_main_13: Value<Tracker> = {
        let __static = {
            let __v: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
            Tracker::new(__v.as_pointer(), { 9 });
            __v
        };
        libcc2rs::register_static_destructor(|| {
            during_main_13.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_3() });
    ({ test_14() });
    printf!(b"return from main\n");
    return 0;
}
pub trait HoldsTrackerImpl {
    fn destructor(&self);
}
impl HoldsTrackerImpl for Ptr<HoldsTracker> {
    fn destructor(&self) {
        field_ptr!(self, member).destructor();
    }
}
pub trait TrackerImpl {
    fn destructor(&self);
    fn touch(&self);
}
impl TrackerImpl for Ptr<Tracker> {
    fn destructor(&self) {
        printf!(b"destroy %d\n", (*self).with(|__s| __s.tag));
    }
    fn touch(&self) {
        printf!(b"touch %d\n", (*self).with(|__s| __s.tag));
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = global_0.with(|_| ());
    let _ = before_6.with(|_| ());
    let _ = after_7.with(|_| ());
    let _ = holder_8.with(|_| ());
    let _ = trivial_9.with(|_| ());
    let _ = member_10.with(|_| ());
    let _ = capturing_11.with(|_| ());
}
