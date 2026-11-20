extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, Uninit, FnPtrArg, MoveCtor, CppFrom, MoveAssign)]
#[cpp_from(i32, new, value)]
#[byte_size(32)]
pub struct S {
    #[offset(0)]
    #[byte_size(24)]
    pub v: Vec<i32>,
    #[offset(24)]
    #[byte_size(8)]
    #[array_len(2)]
    pub n: Box<[i32]>,
}
impl S {
    pub fn new(this: Ptr<S>, mut x: i32) {
        field!(this, v).write(vec![x; (x as usize)]);
        field!(this, n).write(Box::new([x, (x + 1)]));
    }
    pub fn move_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, v).write(field!(_a0, v).with_mut(std::mem::take));
        field!(this, n).write(Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
            (*_a0.upgrade().deref()).n[(__i) as usize]
        })));
    }
}
impl Default for S {
    fn default() -> Self {
        {
            S {
                v: Default::default(),
                n: (0..2).map(|_| 0_i32).collect::<Box<[i32]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct T {
    #[offset(0)]
    #[byte_size(24)]
    pub v: Vec<i32>,
}
impl T {
    pub fn copy_from(this: Ptr<T>, _a0: Ptr<T>) {
        field!(this, v).write((*_a0.upgrade().deref()).v.clone());
    }
    pub fn move_from(this: Ptr<T>, _a0: Ptr<T>) {
        field!(this, v).write(field!(_a0, v).with_mut(std::mem::take));
    }
}
pub fn sum_0(s: Ptr<S>) -> i32 {
    return ({
        ({ (((*s.upgrade().deref()).v.len()) as i32) } + { (array_elem!(s, n, 0).read()) })
    } + { (array_elem!(s, n, 1).read()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    S::new(s.as_pointer(), { 2 });
    assert!((({ sum_0(s.as_pointer(),) }) == 7));
    assert!((({ shuffle_1(3,) }) == 10));
    let t: Value<T> = Rc::new(RefCell::new(<T>::uninit()));
    {
        let __d = t.as_pointer();
        field!(__d, v).write(Vec::from([1, 2]));
    };
    assert!((({ TImpl::size(&t.as_pointer(),) }) == 2));
    return 0;
}
pub fn shuffle_1(mut x: i32) -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    S::new(a.as_pointer(), { x });
    let b: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    S::move_from(b.as_pointer(), { a.as_pointer() });
    assert!(({ (*a.borrow()).v.is_empty() }));
    let c: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    S::new(c.as_pointer(), { 1 });
    ({ SImpl::move_assign(&c.as_pointer(), b.as_pointer()) });
    assert!(({ (*b.borrow()).v.is_empty() }));
    let t: Value<T> = Rc::new(RefCell::new(<T>::uninit()));
    {
        let __d = t.as_pointer();
        field!(__d, v).write(Vec::from([1]));
    };
    let u: Value<T> = Rc::new(RefCell::new(<T>::uninit()));
    T::move_from(u.as_pointer(), { t.as_pointer() });
    let w: Value<T> = Rc::new(RefCell::new(<T>::uninit()));
    T::new_1(w.as_pointer());
    ({ TImpl::move_assign(&w.as_pointer(), u.as_pointer()) });
    assert!(
        ((({ TImpl::size(&t.as_pointer(),) }) == 0) && (({ TImpl::size(&u.as_pointer(),) }) == 0))
            && (({ TImpl::size(&w.as_pointer(),) }) == 1)
    );
    return ({ sum_0(c.as_pointer()) });
}
pub trait SImpl {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S>;
}
impl SImpl for Ptr<S> {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S> {
        field!((*self), v).with_mut(|__b0| {
            (*__b0) = field!(_a0, v).with_mut(std::mem::take);
        });
        {
            let mut __tmp_0 = { (array_field_ptr!((*self), n)).to_any() };
            __tmp_0.memcpy(&(array_field_ptr!(_a0, n)).to_any(), 8_usize);
            (__tmp_0).clone()
        };
        return (*self).clone();
    }
}
pub trait TImpl {
    fn size(&self) -> i32;
    fn move_assign(&self, _a0: Ptr<T>) -> Ptr<T>;
}
impl TImpl for Ptr<T> {
    fn size(&self) -> i32 {
        return (((*(*self).upgrade().deref()).v.len()) as i32);
    }
    fn move_assign(&self, _a0: Ptr<T>) -> Ptr<T> {
        field!((*self), v).with_mut(|__b0| {
            (*__b0) = field!(_a0, v).with_mut(std::mem::take);
        });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
