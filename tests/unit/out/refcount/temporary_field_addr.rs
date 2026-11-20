extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct Inner {
    #[offset(0)]
    #[byte_size(24)]
    pub values: Vec<i32>,
}
impl Inner {
    pub fn copy_from(this: Ptr<Inner>, _a0: Ptr<Inner>) {
        field!(this, values).write((*_a0.upgrade().deref()).values.clone());
    }
    pub fn move_from(this: Ptr<Inner>, _a0: Ptr<Inner>) {
        field!(this, values).write(field!(_a0, values).with_mut(std::mem::take));
    }
    pub fn new_1(this: Ptr<Inner>) {
        field!(this, values).write(Vec::new());
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(56)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(24)]
    pub values: Vec<i32>,
    #[offset(24)]
    #[byte_size(24)]
    pub inner: Inner,
    #[offset(48)]
    #[byte_size(8)]
    #[array_len(8)]
    pub name: Box<[i8]>,
}
impl Outer {
    pub fn copy_from(this: Ptr<Outer>, _a0: Ptr<Outer>) {
        field!(this, values).write((*_a0.upgrade().deref()).values.clone());
        Inner::copy_from(field_ptr!(this, inner), { field_ptr!(_a0, inner) });
        field!(this, name).write(Box::new(std::array::from_fn::<_, 8, _>(|__i: usize| {
            (*_a0.upgrade().deref()).name[(__i) as usize]
        })));
    }
    pub fn move_from(this: Ptr<Outer>, _a0: Ptr<Outer>) {
        field!(this, values).write(field!(_a0, values).with_mut(std::mem::take));
        Inner::move_from(field_ptr!(this, inner), { field_ptr!(_a0, inner) });
        field!(this, name).write(Box::new(std::array::from_fn::<_, 8, _>(|__i: usize| {
            (*_a0.upgrade().deref()).name[(__i) as usize]
        })));
    }
    pub fn new_1(this: Ptr<Outer>) {
        field!(this, values).write(Vec::new());
        Inner::new_1(field_ptr!(this, inner));
    }
}
impl Default for Outer {
    fn default() -> Self {
        {
            Outer {
                values: Default::default(),
                inner: <Inner>::default(),
                name: (0..8).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
pub fn make_0(__ret: Ptr<Outer>, calls: Ptr<i32>) {
    calls.with_mut(|__v| {
        __v.prefix_inc();
    });
    {
        let __d = __ret.clone();
        field!(__d, values).write(Vec::from([1, 2, 3]));
        {
            let __d = field_ptr!(__d, inner);
            field!(__d, values).write(Vec::from([4, 5, 6]));
        };
        field!(__d, name).write(i8::array_from_literal(b"abc\0\0\0\0\0"));
    };
    return;
}
pub fn sum_1(values: Ptr<Vec<i32>>) -> i32 {
    let mut total: i32 = 0;
    'loop_: for value in IterRange::new(values.decay().begin(), values.decay().end()) {
        let mut value: i32 = (value.iter_deref().read());
        total += value;
    }
    return total;
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let calls: Value<i32> = Rc::new(RefCell::new(0));
    let mut member: i32 = ({
        sum_1(
            (match {
                let __v: Value<Outer> = Rc::new(RefCell::new(<Outer>::uninit()));
                ({ make_0(__v.as_pointer(), calls.as_pointer()) });
                __v
            }
            .as_pointer()
            {
                __tmp_1 => field_ref!(__tmp_1, values).as_pointer(),
            }),
        )
    });
    let mut nested: i32 = ({
        sum_1(
            (match {
                let __v: Value<Outer> = Rc::new(RefCell::new(<Outer>::uninit()));
                ({ make_0(__v.as_pointer(), calls.as_pointer()) });
                __v
            }
            .as_pointer()
            {
                __tmp_2 => field_ref!(field_ptr!(__tmp_2, inner), values).as_pointer(),
            }),
        )
    });
    let mut element: i32 = (match (
        (match {
            let __v: Value<Outer> = Rc::new(RefCell::new(<Outer>::uninit()));
            ({ make_0(__v.as_pointer(), calls.as_pointer()) });
            __v
        }
        .as_pointer()
        {
            __tmp_4 => field_ref!(__tmp_4, values).as_pointer(),
        }),
        { (((*calls.borrow()) - 2) as usize) },
    ) {
        (__tmp_3, mut __tmp_5) => {
            if __tmp_5 >= (*__tmp_3.upgrade().deref()).len() {
                panic!("out of bounds access")
            }
            __tmp_3.decay().offset((__tmp_5) as isize)
        }
    }
    .read());
    let mut length: usize = (match {
        let __v: Value<Outer> = Rc::new(RefCell::new(<Outer>::uninit()));
        ({ make_0(__v.as_pointer(), calls.as_pointer()) });
        __v
    }
    .as_pointer()
    {
        __tmp_6 => array_field_ptr!(__tmp_6, name),
    })
    .to_c_string_iterator()
    .count();
    let mut aggregate: i32 = ({
        sum_1(
            (match {
                let __v: Value<Outer> = Rc::new(RefCell::new(<Outer>::uninit()));
                {
                    let __d = __v.as_pointer();
                    field!(__d, values).write(Vec::from([7]));
                    {
                        let __d = field_ptr!(__d, inner);
                        field!(__d, values).write(Vec::from([8, 9]));
                    };
                    field!(__d, name).write(i8::array_from_literal(b"x\0\0\0\0\0\0\0"));
                };
                __v
            }
            .as_pointer()
            {
                __tmp_7 => field_ref!(field_ptr!(__tmp_7, inner), values).as_pointer(),
            }),
        )
    });
    printf!(
        b"%d %d %d %zu %d %d\n",
        member,
        nested,
        element,
        length,
        aggregate,
        (*calls.borrow())
    );
    return 0;
}
pub trait InnerImpl {
    fn move_assign(&self, _a0: Ptr<Inner>) -> Ptr<Inner>;
}
impl InnerImpl for Ptr<Inner> {
    fn move_assign(&self, _a0: Ptr<Inner>) -> Ptr<Inner> {
        field!((*self), values).with_mut(|__b0| {
            (*__b0) = field!(_a0, values).with_mut(std::mem::take);
        });
        return (*self).clone();
    }
}
pub trait OuterImpl {
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer>;
}
impl OuterImpl for Ptr<Outer> {
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer> {
        field!((*self), values).with_mut(|__b0| {
            (*__b0) = field!(_a0, values).with_mut(std::mem::take);
        });
        ({
            let _arg0: Ptr<Inner> = field_ptr!(_a0, inner);
            InnerImpl::move_assign(&field_ptr!((*self), inner), _arg0)
        });
        {
            let mut __tmp_0 = { (array_field_ptr!((*self), name)).to_any() };
            __tmp_0.memcpy(&(array_field_ptr!(_a0, name)).to_any(), 8_usize);
            (__tmp_0).clone()
        };
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
