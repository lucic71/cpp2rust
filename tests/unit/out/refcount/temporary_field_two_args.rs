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
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(48)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(24)]
    pub values: Vec<i32>,
    #[offset(24)]
    #[byte_size(24)]
    pub inner: Inner,
}
impl Outer {
    pub fn copy_from(this: Ptr<Outer>, _a0: Ptr<Outer>) {
        field!(this, values).write((*_a0.upgrade().deref()).values.clone());
        Inner::copy_from(field_ptr!(this, inner), { field_ptr!(_a0, inner) });
    }
    pub fn move_from(this: Ptr<Outer>, _a0: Ptr<Outer>) {
        field!(this, values).write(field!(_a0, values).with_mut(std::mem::take));
        Inner::move_from(field_ptr!(this, inner), { field_ptr!(_a0, inner) });
    }
}
pub fn make_0(__ret: Ptr<Outer>) {
    {
        let __d = __ret.clone();
        field!(__d, values).write(Vec::from([1, 2, 3]));
        {
            let __d = field_ptr!(__d, inner);
            field!(__d, values).write(Vec::from([4, 5, 6]));
        };
    };
    return;
}
pub fn two_1(a: Ptr<Vec<i32>>, b: Ptr<Vec<i32>>) -> i32 {
    return ({ (index!(a, 0_usize).read()) } + { (index!(b, 2_usize).read()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    printf!(
        b"%d\n",
        ({
            let _a: Ptr<Vec<i32>> = (match {
                let __v: Value<Outer> = Rc::new(RefCell::new(<Outer>::uninit()));
                ({ make_0(__v.as_pointer()) });
                __v
            }
            .as_pointer()
            {
                __tmp_0 => field_ref!(__tmp_0, values).as_pointer(),
            });
            let _b: Ptr<Vec<i32>> = (match {
                let __v: Value<Outer> = Rc::new(RefCell::new(<Outer>::uninit()));
                ({ make_0(__v.as_pointer()) });
                __v
            }
            .as_pointer()
            {
                __tmp_1 => field_ref!(field_ptr!(__tmp_1, inner), values).as_pointer(),
            });
            two_1(_a, _b)
        })
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
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
