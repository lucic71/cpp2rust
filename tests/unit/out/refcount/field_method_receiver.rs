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
pub struct Counter {
    #[offset(0)]
    pub n: i32,
}
impl Counter {
    pub fn copy_from(this: Ptr<Counter>, _a0: Ptr<Counter>) {
        field!(this, n).write(_a0.with(|__s| __s.n));
    }
    pub fn move_from(this: Ptr<Counter>, _a0: Ptr<Counter>) {
        field!(this, n).write({ (*_a0.upgrade().deref()).n });
    }
    pub fn new_1(this: Ptr<Counter>) {}
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(40)]
pub struct S {
    #[offset(0)]
    pub tag: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub c: Counter,
    #[offset(8)]
    #[byte_size(8)]
    #[array_len(2)]
    pub arr: Box<[Counter]>,
    #[offset(16)]
    #[byte_size(24)]
    pub v: Vec<i32>,
}
impl S {
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, tag).write(_a0.with(|__s| __s.tag));
        Counter::copy_from(field_ptr!(this, c), { field_ptr!(_a0, c) });
        {
            let __d = array_field_ptr!(this, arr);
            for __i in 0..2_usize {
                Counter::copy_from(__d.offset(__i as isize), {
                    array_field_ptr!(_a0, arr).offset(__i)
                });
            }
        };
        field!(this, v).write((*_a0.upgrade().deref()).v.clone());
    }
    pub fn move_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, tag).write({ (*_a0.upgrade().deref()).tag });
        Counter::move_from(field_ptr!(this, c), { field_ptr!(_a0, c) });
        {
            let __d = array_field_ptr!(this, arr);
            for __i in 0..2_usize {
                Counter::move_from(__d.offset(__i as isize), {
                    array_field_ptr!(_a0, arr).offset(__i)
                });
            }
        };
        field!(this, v).write(field!(_a0, v).with_mut(std::mem::take));
    }
    pub fn new_1(this: Ptr<S>) {
        Counter::new_1(field_ptr!(this, c));
        {
            let __d = array_field_ptr!(this, arr);
            for __i in 0..2_isize {
                Counter::new_1(__d.offset(__i));
            }
        };
        field!(this, v).write(Vec::new());
    }
}
impl Default for S {
    fn default() -> Self {
        {
            S {
                tag: 0_i32,
                c: <Counter>::default(),
                arr: (0..2)
                    .map(|_| <Counter>::default())
                    .collect::<Box<[Counter]>>(),
                v: Default::default(),
            }
        }
    }
}
pub fn run_0(mut o: Ptr<S>) {
    ({ CounterImpl::add(&field_ptr!(o, c), 2) });
    assert!((({ CounterImpl::get(&field_ptr!(o, c),) }) == 2));
    ({ CounterImpl::add(&array_field_ptr!(o, arr).offset((1) as isize), 5) });
    assert!((({ CounterImpl::get(&array_field_ptr!(o, arr).offset((1) as isize),) }) == 5));
    ({ CounterImpl::add(&({ CounterImpl::self_(&field_ptr!(o, c)) }), 1) });
    assert!((({ CounterImpl::get(&field_ptr!(o, c),) }) == 3));
    assert!(({ ({ CounterImpl::self_(&field_ptr!(o, c),) }) } == { (field_ptr!(o, c)) }));
    ({
        let _other: Ptr<Counter> = (array_field_ptr!(o, arr).offset((1) as isize));
        CounterImpl::take(&array_field_ptr!(o, arr).offset((0) as isize), _other)
    });
    assert!(
        (({ CounterImpl::get(&array_field_ptr!(o, arr).offset((0) as isize),) }) == 5)
            && (({ CounterImpl::get(&array_field_ptr!(o, arr).offset((1) as isize),) }) == 0)
    );
    ({
        let _other: Ptr<Counter> = (field_ptr!(o, c));
        CounterImpl::take(&field_ptr!(o, c), _other)
    });
    assert!((({ CounterImpl::get(&field_ptr!(o, c),) }) == 0));
    ({ SImpl::bump(&o) });
    assert!((({ CounterImpl::get(&field_ptr!(o, c),) }) == 1));
    {
        let __a1 = ({ CounterImpl::get(&field_ptr!(o, c)) });
        field!(o, v).with_mut(|__v: &mut Vec<i32>| __v.push(__a1))
    };
    assert!(
        ({
            ((*o.upgrade().deref()).v.len() == 1_usize) && ((vec_elem!(o, v, 0_usize).read()) == 1)
        })
    );
    assert!((o.with(|__s| __s.tag) == 1));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let local: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = local.as_pointer();
        ();
        {
            let __d = field_ptr!(__d, c);
            ();
        };
        {
            let __d = array_field_ptr!(__d, arr);
            {
                let __d = __d.offset(0);
                ();
            };
            {
                let __d = __d.offset(1);
                ();
            };
        };
        field!(__d, v).write(Vec::new());
    };
    (*local.borrow_mut()).tag = 1;
    ({ run_0((local.as_pointer())) });
    let mut heap: Ptr<S> = {
        let __p = Ptr::alloc(<S>::uninit());
        {
            let __d = __p.clone();
            ();
            {
                let __d = field_ptr!(__d, c);
                ();
            };
            {
                let __d = array_field_ptr!(__d, arr);
                {
                    let __d = __d.offset(0);
                    ();
                };
                {
                    let __d = __d.offset(1);
                    ();
                };
            };
            field!(__d, v).write(Vec::new());
        };
        __p
    };
    field!(heap, tag).write(1);
    ({ run_0((heap).clone()) });
    heap.delete();
    return 0;
}
pub trait CounterImpl {
    fn get(&self) -> i32;
    fn add(&self, k: i32);
    fn self_(&self) -> Ptr<Counter>;
    fn take(&self, other: Ptr<Counter>);
}
impl CounterImpl for Ptr<Counter> {
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.n);
    }
    fn add(&self, mut k: i32) {
        {
            let __rhs = k;
            field!((*self), n).with_mut(|__v| *__v = *__v + __rhs)
        };
    }
    fn self_(&self) -> Ptr<Counter> {
        return (*self).clone();
    }
    fn take(&self, mut other: Ptr<Counter>) {
        {
            let __rhs = { other.with(|__s| __s.n) };
            field!((*self), n).with_mut(|__v| *__v = *__v + __rhs)
        };
        field!(other, n).write(0);
    }
}
pub trait SImpl {
    fn bump(&self);
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S>;
}
impl SImpl for Ptr<S> {
    fn bump(&self) {
        ({
            let _k: i32 = (*self).with(|__s| __s.tag);
            CounterImpl::add(&field_ptr!((*self), c), _k)
        });
    }
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S> {
        field!((*self), tag).write({ { (*_a0.upgrade().deref()).tag } });
        field!((*self), c).write({ { (*_a0.upgrade().deref()).c } });
        {
            let mut __tmp_0 = { (array_field_ptr!((*self), arr)).to_any() };
            __tmp_0.memcpy(&(array_field_ptr!(_a0, arr)).to_any(), 8_usize);
            (__tmp_0).clone()
        };
        field!((*self), v).with_mut(|__b0| {
            (*__b0) = field!(_a0, v).with_mut(std::mem::take);
        });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
