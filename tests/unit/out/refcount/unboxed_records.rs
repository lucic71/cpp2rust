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
pub struct Inner {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
impl Inner {
    pub fn new_1(this: Ptr<Inner>) {}
    pub fn copy_from(this: Ptr<Inner>, _a0: Ptr<Inner>) {
        field!(this, a).write(_a0.with(|__s| __s.a));
        field!(this, b).write(_a0.with(|__s| __s.b));
    }
    pub fn move_from(this: Ptr<Inner>, _a0: Ptr<Inner>) {
        field!(this, a).write({ (*_a0.upgrade().deref()).a });
        field!(this, b).write({ (*_a0.upgrade().deref()).b });
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(48)]
pub struct Cookie {
    #[offset(0)]
    pub x: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub data: Ptr<i32>,
    #[offset(16)]
    #[byte_size(8)]
    pub in_: Ptr<i32>,
    #[offset(24)]
    #[byte_size(8)]
    pub inner: Inner,
    #[offset(32)]
    #[byte_size(16)]
    #[array_len(4)]
    pub arr: Box<[i32]>,
}
impl Cookie {
    pub fn new_1(this: Ptr<Cookie>) {
        Inner::new_1(field_ptr!(this, inner));
    }
    pub fn copy_from(this: Ptr<Cookie>, _a0: Ptr<Cookie>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
        field!(this, data).write(_a0.with(|__s| __s.data.clone()));
        field!(this, in_).write(_a0.with(|__s| __s.in_.clone()));
        Inner::copy_from(field_ptr!(this, inner), { field_ptr!(_a0, inner) });
        field!(this, arr).write(Box::new(std::array::from_fn::<_, 4, _>(|__i: usize| {
            (*_a0.upgrade().deref()).arr[(__i) as usize]
        })));
    }
    pub fn move_from(this: Ptr<Cookie>, _a0: Ptr<Cookie>) {
        field!(this, x).write({ (*_a0.upgrade().deref()).x });
        field!(this, data).write({ (*_a0.upgrade().deref()).data.clone() });
        field!(this, in_).write({ (*_a0.upgrade().deref()).in_.clone() });
        Inner::move_from(field_ptr!(this, inner), { field_ptr!(_a0, inner) });
        field!(this, arr).write(Box::new(std::array::from_fn::<_, 4, _>(|__i: usize| {
            (*_a0.upgrade().deref()).arr[(__i) as usize]
        })));
    }
}
impl Default for Cookie {
    fn default() -> Self {
        {
            Cookie {
                x: 0_i32,
                data: Ptr::<i32>::null(),
                in_: Ptr::<i32>::null(),
                inner: <Inner>::default(),
                arr: (0..4).map(|_| 0_i32).collect::<Box<[i32]>>(),
            }
        }
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Counter {
    #[offset(0)]
    pub n: i32,
}
impl Counter {
    pub fn new_1(this: Ptr<Counter>) {
        field!(this, n).write(0);
    }
    pub fn copy_from(this: Ptr<Counter>, _a0: Ptr<Counter>) {
        field!(this, n).write(_a0.with(|__s| __s.n));
    }
    pub fn move_from(this: Ptr<Counter>, _a0: Ptr<Counter>) {
        field!(this, n).write({ (*_a0.upgrade().deref()).n });
    }
}
impl Default for Counter {
    fn default() -> Self {
        { Counter { n: 0 } }
    }
}
pub fn set_0(mut p: Ptr<i32>, mut v: i32) {
    p.write({ v });
}
pub fn get_1(c: Ptr<Cookie>) -> i32 {
    return c.with(|__s| __s.x);
}
pub fn first_2(mut p: Ptr<i32>) -> i32 {
    return (elem!(p, 0).read());
}
pub fn consume_3(cookie: Ptr<Cookie>) -> i32 {
    let c: Value<Cookie> = Rc::new(RefCell::new(<Cookie>::uninit()));
    Cookie::copy_from(c.as_pointer(), { (cookie).clone() });
    let mut sum: i32 = 0;
    {
        let mut i: i32 = 0;
        'loop_: while (i < 4) {
            elem!({ (*c.borrow()).data.clone() }, i).write({
                ({ (elem!({ (*c.borrow()).in_.clone() }, i).read()) } * { { (*c.borrow()).x } })
            });
            sum += { (elem!({ (*c.borrow()).data.clone() }, i).read()) };
            i.prefix_inc();
        }
    }
    (*c.borrow_mut()).x = sum;
    (*c.borrow_mut()).inner.a += { { (*c.borrow()).x } };
    array_elem!(c.as_pointer(), arr, 1).write({ { (*c.borrow()).inner.a } });
    let __rhs = ({ (array_elem!(c.as_pointer(), arr, 1).read()) } + {
        ({ first_2({ (*c.borrow()).in_.clone() }) })
    });
    { (*c.borrow()).data.clone() }.write(__rhs);
    ({
        let _p: Ptr<i32> = { (*c.borrow()).data.clone() }.offset((1) as isize);
        let _v: i32 = ({ first_2({ (*c.borrow()).data.clone() }) });
        set_0(_p, _v)
    });
    return ({ (*c.borrow()).x } + { (*c.borrow()).inner.b });
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let data: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 0])));
    let in_: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4])));
    let cookie: Value<Cookie> = Rc::new(RefCell::new(<Cookie>::uninit()));
    Cookie::new_1(cookie.as_pointer());
    (*cookie.borrow_mut()).x = 2;
    (*cookie.borrow_mut()).data = data.decay();
    (*cookie.borrow_mut()).in_ = in_.decay();
    (*cookie.borrow_mut()).inner.a = 1;
    (*cookie.borrow_mut()).inner.b = 3;
    assert!((({ consume_3(cookie.as_pointer(),) }) == 23));
    assert!(
        ({
            (((*data.borrow())[(0) as usize] == 22) && ((*data.borrow())[(1) as usize] == 22))
                && ((*data.borrow())[(3) as usize] == 8)
        })
    );
    assert!((({ get_1(cookie.as_pointer(),) }) == 2));
    let local: Value<Inner> = Rc::new(RefCell::new(<Inner>::uninit()));
    Inner::new_1(local.as_pointer());
    (*local.borrow_mut()).a = 4;
    (*local.borrow_mut()).b = { ({ (*local.borrow()).a } * 2) };
    (*local.borrow_mut()).a.postfix_inc();
    assert!(({ (({ (*local.borrow()).a } + { (*local.borrow()).b }) == 13) }));
    let field_addr: Value<Inner> = Rc::new(RefCell::new(<Inner>::uninit()));
    {
        let __d = field_addr.as_pointer();
        field!(__d, a).write(0);
        field!(__d, b).write(0);
    };
    ({ set_0((field_ptr!(field_addr.as_pointer(), b)), 7) });
    assert!(({ ({ (*field_addr.borrow()).b } == 7) }));
    let counter: Value<Counter> = Rc::new(RefCell::new(<Counter>::uninit()));
    Counter::new_1(counter.as_pointer());
    ({ CounterImpl::inc(&counter.as_pointer()) });
    assert!(({ ({ (*counter.borrow()).n } == 1) }));
    return 0;
}
pub trait CounterImpl {
    fn inc(&self);
}
impl CounterImpl for Ptr<Counter> {
    fn inc(&self) {
        field!((*self), n).with_mut(|__v| {
            __v.prefix_inc();
        });
    }
}
pub fn __cpp2rust_init_globals() {}
