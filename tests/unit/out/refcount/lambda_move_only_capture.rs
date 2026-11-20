extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, Uninit, FnPtrArg, MoveCtor, CppFrom, Destructor, Default)]
#[placement]
#[cpp_from(i32, new, value)]
#[byte_size(8)]
pub struct Owner {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Ptr<i32>,
}
impl Owner {
    pub fn new(this: Ptr<Owner>, mut v: i32) {
        field!(this, p).write(Ptr::alloc(v));
    }
    pub fn move_from(this: Ptr<Owner>, o: Ptr<Owner>) {
        field!(this, p).write(o.with(|__s| __s.p.clone()));
        field!(o, p).write(Ptr::<i32>::null());
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Owner> = Rc::new(RefCell::new(<Owner>::uninit()));
    Owner::new(o.as_pointer(), { 5 });
    let _dtor_o = std::mem::DropGuard::new(o.clone(), |__p| __p.as_pointer().destructor());
    let f: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let h: Value<Owner> = {
                let __v: Value<Owner> = Rc::new(RefCell::new(<Owner>::uninit()));
                Owner::move_from(__v.as_pointer(), { o.as_pointer() });
                __v
            };
        },
        || -> i32 {
            return ({ (*h.borrow()).p.clone() }.read());
        }
    )));
    let _dtor_f =
        std::mem::DropGuard::new(f.clone(), |__p| __p.as_pointer().with(|__f| __f.destroy()));
    assert!(({ ({ (*o.borrow()).p.clone() }).is_null() }));
    assert!(({ (({ (*f.borrow()).call() }) == 5) }));
    let g: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*f.borrow()).move_from()));
    let _dtor_g =
        std::mem::DropGuard::new(g.clone(), |__p| __p.as_pointer().with(|__f| __f.destroy()));
    assert!(({ (({ (*g.borrow()).call() }) == 5) }));
    let total: Value<i32> = Rc::new(RefCell::new(0));
    let consume: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
        {
            let h: Value<Owner> = {
                let __v: Value<Owner> = Rc::new(RefCell::new(<Owner>::uninit()));
                Owner::new(__v.as_pointer(), { 7 });
                __v
            };
            let total: Ptr<i32> = total.as_pointer();
        },
        || {
            {
                let __rhs = { ({ (*h.borrow()).p.clone() }.read()) };
                total.with_mut(|__v| *__v = *__v + __rhs)
            };
            { (*h.borrow()).p.clone() }.write(0);
        }
    )));
    let _dtor_consume = std::mem::DropGuard::new(consume.clone(), |__p| {
        __p.as_pointer().with(|__f| __f.destroy())
    });
    ({ (*consume.borrow()).call() });
    ({ (*consume.borrow()).call() });
    assert!(({ ((*total.borrow()) == 7) }));
    let o2: Value<Owner> = Rc::new(RefCell::new(<Owner>::uninit()));
    Owner::new(o2.as_pointer(), { 9 });
    let _dtor_o2 = std::mem::DropGuard::new(o2.clone(), |__p| __p.as_pointer().destructor());
    let t: Value<FnPtr<fn() -> i32>> =
        Rc::new(RefCell::new(({ OwnerImpl::take(&o2.as_pointer()) })));
    let _dtor_t =
        std::mem::DropGuard::new(t.clone(), |__p| __p.as_pointer().with(|__f| __f.destroy()));
    assert!(({ ({ (*o2.borrow()).p.clone() }).is_null() }));
    assert!(({ (({ (*t.borrow()).call() }) == 9) }));
    return 0;
}
pub trait OwnerImpl {
    fn destructor(&self);
    fn take(&self) -> FnPtr<fn() -> i32>;
}
impl OwnerImpl for Ptr<Owner> {
    fn destructor(&self) {
        (*self).with(|__s| __s.p.clone()).delete();
    }
    fn take(&self) -> FnPtr<fn() -> i32> {
        return lambda!(
            {
                let self_: Value<Owner> = {
                    let __v: Value<Owner> = Rc::new(RefCell::new(<Owner>::uninit()));
                    Owner::move_from(__v.as_pointer(), { (*self).clone() });
                    __v
                };
            },
            || -> i32 {
                return ({ (*self_.borrow()).p.clone() }.read());
            }
        );
    }
}
pub fn __cpp2rust_init_globals() {}
