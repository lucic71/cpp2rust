extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, MoveAssign)]
#[cpp_from(i32, new, value)]
#[byte_size(32)]
pub struct S {
    #[offset(0)]
    #[byte_size(24)]
    pub v: (),
    #[offset(24)]
    #[byte_size(8)]
    pub n: Box<[()]>,
}
impl S {
    pub fn new(mut x: ()) -> Self {
        Self {
            v: unimplemented!(),
            n: Box::new([x, (x + 1_())]),
        }
    }
    pub fn move_from(_a0: Ptr<S>) -> Self {
        Self {
            v: unimplemented!(),
            n: Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
                (*_a0.upgrade().deref()).n[(__i) as usize]
            })),
        }
    }
}
impl Default for S {
    fn default() -> Self {
        {
            S {
                v: unimplemented!(),
                n: (0..2).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct T {
    #[offset(0)]
    #[byte_size(24)]
    pub v: (),
}
impl T {
    pub fn move_from(_a0: Ptr<T>) -> Self {
        Self {
            v: unimplemented!(),
        }
    }
}
pub fn sum_0(s: Ptr<S>) -> () {
    return ({ ({ unimplemented!() } + { (array_elem!(s, n, 0_()).read()) }) } + {
        (array_elem!(s, n, 1_()).read())
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<S> = Rc::new(RefCell::new(S::new({ 2_() })));
    unimplemented!();
    unimplemented!();
    let t: Value<T> = Rc::new(RefCell::new(T {
        v: unimplemented!(),
    }));
    unimplemented!();
    return 0_();
}
pub fn shuffle_1(mut x: ()) -> () {
    let a: Value<S> = Rc::new(RefCell::new(S::new({ x })));
    let b: Value<S> = Rc::new(RefCell::new(S::move_from({ a.as_pointer() })));
    unimplemented!();
    let c: Value<S> = Rc::new(RefCell::new(S::new({ 1_() })));
    ({ SImpl::move_assign(&c.as_pointer(), b.as_pointer()) });
    unimplemented!();
    let t: Value<T> = Rc::new(RefCell::new(T {
        v: unimplemented!(),
    }));
    let u: Value<T> = Rc::new(RefCell::new(T::move_from({ t.as_pointer() })));
    let w: Value<T> = Rc::new(RefCell::new(<T>::default()));
    ({ TImpl::move_assign(&w.as_pointer(), u.as_pointer()) });
    unimplemented!();
    return ({ sum_0(c.as_pointer()) });
}
pub trait SImpl {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S>;
}
impl SImpl for Ptr<S> {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S> {
        unimplemented!();
        unimplemented!();
        return (*self).clone();
    }
}
pub trait TImpl {
    fn size(&self) -> ();
    fn move_assign(&self, _a0: Ptr<T>) -> Ptr<T>;
}
impl TImpl for Ptr<T> {
    fn size(&self) -> () {
        return unimplemented!();
    }
    fn move_assign(&self, _a0: Ptr<T>) -> Ptr<T> {
        unimplemented!();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
