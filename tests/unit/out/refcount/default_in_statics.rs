extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Inner {
    #[offset(0)]
    pub v: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub name: Ptr<i8>,
}
impl Inner {
    pub fn new_1(this: Ptr<Inner>) {}
    pub fn copy_from(this: Ptr<Inner>, _a0: Ptr<Inner>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
        field!(this, name).write(_a0.with(|__s| __s.name.clone()));
    }
    pub fn move_from(this: Ptr<Inner>, _a0: Ptr<Inner>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
        field!(this, name).write({ (*_a0.upgrade().deref()).name.clone() });
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(88)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(8)]
    pub p1: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub p2: Ptr<i32>,
    #[offset(16)]
    #[byte_size(24)]
    #[array_len(3)]
    pub arr: Box<[Ptr<i32>]>,
    #[offset(40)]
    #[byte_size(8)]
    pub cp: Ptr<i8>,
    #[offset(48)]
    #[byte_size(8)]
    pub pp: Ptr<Ptr<i32>>,
    #[offset(56)]
    #[byte_size(16)]
    pub inner: Inner,
    #[offset(72)]
    pub x: i32,
    #[offset(80)]
    #[byte_size(8)]
    pub fn_: FnPtr<fn(i32) -> i32>,
}
impl Outer {
    pub fn new_1(this: Ptr<Outer>) {
        Inner::new_1(field_ptr!(this, inner));
    }
    pub fn copy_from(this: Ptr<Outer>, _a0: Ptr<Outer>) {
        field!(this, p1).write(_a0.with(|__s| __s.p1.clone()));
        field!(this, p2).write(_a0.with(|__s| __s.p2.clone()));
        field!(this, arr).write(Box::new(std::array::from_fn::<_, 3, _>(|__i: usize| {
            ((*_a0.upgrade().deref()).arr[(__i) as usize]).clone()
        })));
        field!(this, cp).write(_a0.with(|__s| __s.cp.clone()));
        field!(this, pp).write(_a0.with(|__s| __s.pp.clone()));
        Inner::copy_from(field_ptr!(this, inner), { field_ptr!(_a0, inner) });
        field!(this, x).write(_a0.with(|__s| __s.x));
        field!(this, fn_).write(_a0.with(|__s| __s.fn_.clone()));
    }
    pub fn move_from(this: Ptr<Outer>, _a0: Ptr<Outer>) {
        field!(this, p1).write({ (*_a0.upgrade().deref()).p1.clone() });
        field!(this, p2).write({ (*_a0.upgrade().deref()).p2.clone() });
        field!(this, arr).write(Box::new(std::array::from_fn::<_, 3, _>(|__i: usize| {
            ((*_a0.upgrade().deref()).arr[(__i) as usize]).clone()
        })));
        field!(this, cp).write({ (*_a0.upgrade().deref()).cp.clone() });
        field!(this, pp).write({ (*_a0.upgrade().deref()).pp.clone() });
        Inner::move_from(field_ptr!(this, inner), { field_ptr!(_a0, inner) });
        field!(this, x).write({ (*_a0.upgrade().deref()).x });
        field!(this, fn_).write({ (*_a0.upgrade().deref()).fn_.clone() });
    }
}
impl Default for Outer {
    fn default() -> Self {
        {
            Outer {
                p1: Ptr::<i32>::null(),
                p2: Ptr::<i32>::null(),
                arr: (0..3)
                    .map(|_| Ptr::<i32>::null())
                    .collect::<Box<[Ptr<i32>]>>(),
                cp: Ptr::<i8>::null(),
                pp: Ptr::<Ptr<i32>>::null(),
                inner: <Inner>::default(),
                x: 0_i32,
                fn_: FnPtr::<fn(i32) -> i32>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(40)]
pub struct Foo {
    #[offset(0)]
    #[byte_size(8)]
    pub s1: Ptr<i8>,
    #[offset(8)]
    #[byte_size(8)]
    pub s2: Ptr<i8>,
    #[offset(16)]
    #[byte_size(8)]
    pub fn1: FnPtr<fn(i32) -> i32>,
    #[offset(24)]
    #[byte_size(8)]
    pub fn2: FnPtr<fn(i32) -> i32>,
    #[offset(32)]
    pub n: i32,
}
impl Foo {
    pub fn new_1(this: Ptr<Foo>) {}
    pub fn copy_from(this: Ptr<Foo>, _a0: Ptr<Foo>) {
        field!(this, s1).write(_a0.with(|__s| __s.s1.clone()));
        field!(this, s2).write(_a0.with(|__s| __s.s2.clone()));
        field!(this, fn1).write(_a0.with(|__s| __s.fn1.clone()));
        field!(this, fn2).write(_a0.with(|__s| __s.fn2.clone()));
        field!(this, n).write(_a0.with(|__s| __s.n));
    }
    pub fn move_from(this: Ptr<Foo>, _a0: Ptr<Foo>) {
        field!(this, s1).write({ (*_a0.upgrade().deref()).s1.clone() });
        field!(this, s2).write({ (*_a0.upgrade().deref()).s2.clone() });
        field!(this, fn1).write({ (*_a0.upgrade().deref()).fn1.clone() });
        field!(this, fn2).write({ (*_a0.upgrade().deref()).fn2.clone() });
        field!(this, n).write({ (*_a0.upgrade().deref()).n });
    }
}
thread_local!(
    pub static static_fn_0: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::null()));
);
thread_local!(
    pub static static_outer_1: Value<Outer> = {
        let __v: Value<Outer> = Rc::new(RefCell::new(<Outer>::uninit()));
        Outer::new_1(__v.as_pointer());
        __v
    };
);
thread_local!(
    pub static static_inner_array_2: Value<Box<[Inner]>> = {
        let __v: Value<Box<[Inner]>> = Rc::new(RefCell::new(
            (0..2).map(|_| <Inner>::uninit()).collect::<Box<[_]>>(),
        ));
        {
            let __d = __v.decay();
            for __i in 0..2_isize {
                Inner::new_1(__d.offset(__i));
            }
        };
        __v
    };
);
thread_local!(
    pub static static_foo_3: Value<Foo> = {
        let __v: Value<Foo> = Rc::new(RefCell::new(<Foo>::uninit()));
        {
            let __d = __v.as_pointer();
            field!(__d, s1).write(Ptr::<i8>::from_string_literal(b"hello"));
            field!(__d, s2).write(Ptr::<i8>::null());
            field!(__d, fn1).write(FnPtr::<fn(i32) -> i32>::null());
            field!(__d, fn2).write(FnPtr::<fn(i32) -> i32>::null());
            field!(__d, n).write(42);
        };
        __v
    };
);
thread_local!(
    pub static static_foo_array_4: Value<Box<[Foo]>> = {
        let __v: Value<Box<[Foo]>> = Rc::new(RefCell::new(
            (0..2).map(|_| <Foo>::uninit()).collect::<Box<[_]>>(),
        ));
        {
            let __d = __v.decay();
            {
                let __d = __d.offset(0);
                field!(__d, s1).write(Ptr::<i8>::from_string_literal(b"first"));
                field!(__d, s2).write(Ptr::<i8>::null());
                field!(__d, fn1).write(FnPtr::<fn(i32) -> i32>::null());
                field!(__d, fn2).write(FnPtr::<fn(i32) -> i32>::null());
                field!(__d, n).write(1);
            };
            {
                let __d = __d.offset(1);
                field!(__d, s1).write(Ptr::<i8>::from_string_literal(b"second"));
                field!(__d, s2).write(Ptr::<i8>::null());
                field!(__d, fn1).write(FnPtr::<fn(i32) -> i32>::null());
                field!(__d, fn2).write(FnPtr::<fn(i32) -> i32>::null());
                field!(__d, n).write(2);
            };
        };
        __v
    };
);
pub fn check_local_static_5() {
    assert!(({ ({ (*local_outer_6.with(Value::clone).borrow()).p1.clone() }).is_null() }));
    assert!(({ ({ (*local_outer_6.with(Value::clone).borrow()).fn_.clone() }).is_null() }));
    assert!(({ (*local_fn_7.with(Value::clone).borrow()).is_null() }));
    assert!(({ (*local_p_8.with(Value::clone).borrow()).is_null() }));
}
thread_local!(
    static local_outer_6: Value<Outer> = {
        let __v: Value<Outer> = Rc::new(RefCell::new(<Outer>::uninit()));
        Outer::new_1(__v.as_pointer());
        __v
    };
);
thread_local!(
    static local_fn_7: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::null()));
);
thread_local!(
    static local_p_8: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
);
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    assert!(({ (*static_fn_0.with(Value::clone).borrow()).is_null() }));
    assert!(({ ({ (*static_outer_1.with(Value::clone).borrow()).p1.clone() }).is_null() }));
    assert!(({ ({ (*static_outer_1.with(Value::clone).borrow()).p2.clone() }).is_null() }));
    assert!(({ ({ (*static_outer_1.with(Value::clone).borrow()).cp.clone() }).is_null() }));
    assert!(({ ({ (*static_outer_1.with(Value::clone).borrow()).pp.clone() }).is_null() }));
    assert!(({ ({ (*static_outer_1.with(Value::clone).borrow()).fn_.clone() }).is_null() }));
    {
        let mut i: i32 = 0;
        'loop_: while (i < 3) {
            assert!(
                (array_elem!(static_outer_1.with(|v| v.as_pointer()), arr, i).read()).is_null()
            );
            i.prefix_inc();
        }
    }
    assert!(
        ({
            ({
                (*static_outer_1.with(Value::clone).borrow())
                    .inner
                    .name
                    .clone()
            })
            .is_null()
        })
    );
    {
        let mut i: i32 = 0;
        'loop_: while (i < 2) {
            assert!(
                ({
                    ({
                        (*static_inner_array_2.with(Value::clone).borrow())[(i) as usize]
                            .name
                            .clone()
                    })
                    .is_null()
                })
            );
            i.prefix_inc();
        }
    }
    assert!(({ ({ (*static_foo_3.with(Value::clone).borrow()).s2.clone() }).is_null() }));
    assert!(({ ({ (*static_foo_3.with(Value::clone).borrow()).fn1.clone() }).is_null() }));
    assert!(({ ({ (*static_foo_3.with(Value::clone).borrow()).fn2.clone() }).is_null() }));
    assert!(({ ({ (*static_foo_3.with(Value::clone).borrow()).n } == 42) }));
    {
        let mut i: i32 = 0;
        'loop_: while (i < 2) {
            assert!(
                ({
                    ({
                        (*static_foo_array_4.with(Value::clone).borrow())[(i) as usize]
                            .s2
                            .clone()
                    })
                    .is_null()
                })
            );
            assert!(
                ({
                    ({
                        (*static_foo_array_4.with(Value::clone).borrow())[(i) as usize]
                            .fn1
                            .clone()
                    })
                    .is_null()
                })
            );
            assert!(
                ({
                    ({
                        (*static_foo_array_4.with(Value::clone).borrow())[(i) as usize]
                            .fn2
                            .clone()
                    })
                    .is_null()
                })
            );
            i.prefix_inc();
        }
    }
    ({ check_local_static_5() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = static_fn_0.with(|_| ());
    let _ = static_outer_1.with(|_| ());
    let _ = static_inner_array_2.with(|_| ());
    let _ = static_foo_3.with(|_| ());
    let _ = static_foo_array_4.with(|_| ());
}
