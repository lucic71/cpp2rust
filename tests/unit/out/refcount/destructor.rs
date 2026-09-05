extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static total_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive()]
pub struct Inner {
    pub target: Value<Ptr<i32>>,
}
pub trait InnerImpl {
    fn __dtor(&self);
}
impl Clone for Inner {
    fn clone(&self) -> Self {
        let __this: Value<Inner> = Rc::new(RefCell::new(Self {
            target: Rc::new(RefCell::new((*self.target.borrow()).clone())),
        }));
        let this: Ptr<Inner> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Inner {
    fn default() -> Self {
        Inner {
            target: Rc::new(RefCell::new(Ptr::<i32>::null())),
        }
    }
}
impl ByteRepr for Inner {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.target.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            target: Rc::new(RefCell::new(<Ptr<i32>>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive(Default)]
pub struct Outer {
    pub inner: Value<Inner>,
}
pub trait OuterImpl {
    fn __dtor(&self);
}
impl Clone for Outer {
    fn clone(&self) -> Self {
        let __this: Value<Outer> = Rc::new(RefCell::new(Self {
            inner: Rc::new(RefCell::new((*self.inner.borrow()).clone())),
        }));
        let this: Ptr<Outer> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Outer {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.inner.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            inner: Rc::new(RefCell::new(<Inner>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive()]
pub struct OutOfLine {
    pub step: Value<i32>,
}
pub trait OutOfLineImpl {
    fn __dtor(&self);
}
impl Clone for OutOfLine {
    fn clone(&self) -> Self {
        let __this: Value<OutOfLine> = Rc::new(RefCell::new(Self {
            step: Rc::new(RefCell::new((*self.step.borrow()))),
        }));
        let this: Ptr<OutOfLine> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for OutOfLine {
    fn default() -> Self {
        OutOfLine {
            step: Rc::new(RefCell::new(4)),
        }
    }
}
impl ByteRepr for OutOfLine {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.step.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            step: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let value: Value<i32> = Rc::new(RefCell::new(40));
    {
        let o: Value<Outer> = Rc::new(RefCell::new(<Outer>::default()));
        let _dtor_o = ScopedDestructor::new(&o, |__p| __p.__dtor());
        (*(*(*o.borrow()).inner.borrow()).target.borrow_mut()) = (value.as_pointer());
    }
    assert!(((*total_0.with(Value::clone).borrow()) == 40));
    {
        let t: Value<OutOfLine> = Rc::new(RefCell::new(<OutOfLine>::default()));
        let _dtor_t = ScopedDestructor::new(&t, |__p| __p.__dtor());
    }
    assert!(((*total_0.with(Value::clone).borrow()) == 44));
    return 0;
}
impl InnerImpl for Ptr<Inner> {
    fn __dtor(&self) {
        if !((*(*self.upgrade().deref()).target.borrow()).is_null()) {
            let __rhs = ((*(*self.upgrade().deref()).target.borrow()).read());
            (*total_0.with(Value::clone).borrow_mut()) += __rhs;
            (*(*self.upgrade().deref()).target.borrow_mut()) = Ptr::<i32>::null();
        }
    }
}
impl OutOfLineImpl for Ptr<OutOfLine> {
    fn __dtor(&self) {
        (*total_0.with(Value::clone).borrow_mut()) += (*(*self.upgrade().deref()).step.borrow());
    }
}
impl OuterImpl for Ptr<Outer> {
    fn __dtor(&self) {
        (*self.upgrade().deref()).inner.as_pointer().__dtor();
    }
}
