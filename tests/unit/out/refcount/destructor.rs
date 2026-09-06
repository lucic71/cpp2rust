extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static global_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Default)]
pub struct S {}
pub trait SImpl {
    fn __dtor(&self);
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct Defaulted {
    pub s: Value<S>,
}
pub trait DefaultedImpl {
    fn __dtor(&self);
}
impl Clone for Defaulted {
    fn clone(&self) -> Self {
        let __this: Value<Defaulted> = Rc::new(RefCell::new(Self {
            s: Rc::new(RefCell::new((*self.s.borrow()).clone())),
        }));
        let this: Ptr<Defaulted> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Defaulted {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.s.borrow()).to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            s: Rc::new(RefCell::new(<S>::from_bytes(&buf[0..1]))),
        }
    }
}
#[derive(Default)]
pub struct Middle {
    pub s: Value<S>,
}
pub trait MiddleImpl {
    fn __dtor(&self);
}
impl Clone for Middle {
    fn clone(&self) -> Self {
        let __this: Value<Middle> = Rc::new(RefCell::new(Self {
            s: Rc::new(RefCell::new((*self.s.borrow()).clone())),
        }));
        let this: Ptr<Middle> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Middle {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.s.borrow()).to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            s: Rc::new(RefCell::new(<S>::from_bytes(&buf[0..1]))),
        }
    }
}
#[derive(Default)]
pub struct Outer {
    pub m: Value<Middle>,
}
pub trait OuterImpl {
    fn __dtor(&self);
}
impl Clone for Outer {
    fn clone(&self) -> Self {
        let __this: Value<Outer> = Rc::new(RefCell::new(Self {
            m: Rc::new(RefCell::new((*self.m.borrow()).clone())),
        }));
        let this: Ptr<Outer> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Outer {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.m.borrow()).to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            m: Rc::new(RefCell::new(<Middle>::from_bytes(&buf[0..1]))),
        }
    }
}
#[derive()]
pub struct ArrayMember {
    pub items: Value<Box<[S]>>,
}
pub trait ArrayMemberImpl {
    fn __dtor(&self);
}
impl Clone for ArrayMember {
    fn clone(&self) -> Self {
        let __this: Value<ArrayMember> = Rc::new(RefCell::new(Self {
            items: Rc::new(RefCell::new((*self.items.borrow()).clone())),
        }));
        let this: Ptr<ArrayMember> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for ArrayMember {
    fn default() -> Self {
        ArrayMember {
            items: Rc::new(RefCell::new(
                (0..3).map(|_| <S>::default()).collect::<Box<[S]>>(),
            )),
        }
    }
}
impl ByteRepr for ArrayMember {
    fn byte_size() -> usize {
        3
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.items.borrow()).to_bytes(&mut buf[0..3]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            items: Rc::new(RefCell::new(<Box<[S]>>::from_bytes(&buf[0..3]))),
        }
    }
}
#[derive(Default)]
pub struct EmptyBody {
    pub s: Value<S>,
}
pub trait EmptyBodyImpl {
    fn __dtor(&self);
}
impl Clone for EmptyBody {
    fn clone(&self) -> Self {
        let __this: Value<EmptyBody> = Rc::new(RefCell::new(Self {
            s: Rc::new(RefCell::new((*self.s.borrow()).clone())),
        }));
        let this: Ptr<EmptyBody> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for EmptyBody {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.s.borrow()).to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            s: Rc::new(RefCell::new(<S>::from_bytes(&buf[0..1]))),
        }
    }
}
#[derive(Default)]
pub struct Templated_char_ {
    pub v: Value<u8>,
}
pub trait Templated_char_Impl {
    fn __dtor(&self);
}
impl Clone for Templated_char_ {
    fn clone(&self) -> Self {
        let __this: Value<Templated_char_> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<Templated_char_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Templated_char_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<u8>::from_bytes(&buf[0..1]))),
        }
    }
}
#[derive(Default)]
pub struct Templated_int_ {
    pub v: Value<i32>,
}
pub trait Templated_int_Impl {
    fn __dtor(&self);
}
impl Clone for Templated_int_ {
    fn clone(&self) -> Self {
        let __this: Value<Templated_int_> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<Templated_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Templated_int_ {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Copied {
    pub v: Value<i32>,
}
pub trait CopiedImpl {
    fn __dtor(&self);
}
impl Clone for Copied {
    fn clone(&self) -> Self {
        let __this: Value<Copied> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<Copied> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Copied {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        let s: Value<S> = Rc::new(RefCell::new(S {}));
        let _dtor_s = ScopedDestructor::new(&s, |__p| __p.__dtor());
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 1));
    {
        let s: Value<S> = Rc::new(RefCell::new(S {}));
        let _dtor_s = ScopedDestructor::new(&s, |__p| __p.__dtor());
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 2));
    {
        let d: Value<Defaulted> = Rc::new(RefCell::new(Defaulted {
            s: Rc::new(RefCell::new(S {})),
        }));
        let _dtor_d = ScopedDestructor::new(&d, |__p| __p.__dtor());
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 3));
    {
        let o: Value<Outer> = Rc::new(RefCell::new(Outer {
            m: Rc::new(RefCell::new(Middle {
                s: Rc::new(RefCell::new(S {})),
            })),
        }));
        let _dtor_o = ScopedDestructor::new(&o, |__p| __p.__dtor());
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 4));
    {
        let am: Value<ArrayMember> = Rc::new(RefCell::new(ArrayMember {
            items: Rc::new(RefCell::new(Box::new([S {}, S {}, S {}]))),
        }));
        let _dtor_am = ScopedDestructor::new(&am, |__p| __p.__dtor());
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 7));
    {
        let e: Value<EmptyBody> = Rc::new(RefCell::new(EmptyBody {
            s: Rc::new(RefCell::new(S {})),
        }));
        let _dtor_e = ScopedDestructor::new(&e, |__p| __p.__dtor());
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 8));
    {
        let tc: Value<Templated_char_> = Rc::new(RefCell::new(Templated_char_ {
            v: Rc::new(RefCell::new(<u8>::default())),
        }));
        let _dtor_tc = ScopedDestructor::new(&tc, |__p| __p.__dtor());
        let ti: Value<Templated_int_> = Rc::new(RefCell::new(Templated_int_ {
            v: Rc::new(RefCell::new(<i32>::default())),
        }));
        let _dtor_ti = ScopedDestructor::new(&ti, |__p| __p.__dtor());
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 13));
    {
        let a: Value<Copied> = Rc::new(RefCell::new(Copied {
            v: Rc::new(RefCell::new(5)),
        }));
        let _dtor_a = ScopedDestructor::new(&a, |__p| __p.__dtor());
        let b: Value<Copied> = Rc::new(RefCell::new((*a.borrow()).clone()));
        let _dtor_b = ScopedDestructor::new(&b, |__p| __p.__dtor());
        assert!(((*(*b.borrow()).v.borrow()) == 5));
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 15));
    return 0;
}
impl ArrayMemberImpl for Ptr<ArrayMember> {
    fn __dtor(&self) {
        {
            let __p = (*self.upgrade().deref()).items.as_pointer();
            for __i in 0..__p.len() {
                SImpl::__dtor(&__p.offset(__i as isize));
            }
        }
    }
}
impl CopiedImpl for Ptr<Copied> {
    fn __dtor(&self) {
        (*global_0.with(Value::clone).borrow_mut()).postfix_inc();
    }
}
impl DefaultedImpl for Ptr<Defaulted> {
    fn __dtor(&self) {
        (*self.upgrade().deref()).s.as_pointer().__dtor();
    }
}
impl EmptyBodyImpl for Ptr<EmptyBody> {
    fn __dtor(&self) {
        (*self.upgrade().deref()).s.as_pointer().__dtor();
    }
}
impl MiddleImpl for Ptr<Middle> {
    fn __dtor(&self) {
        (*self.upgrade().deref()).s.as_pointer().__dtor();
    }
}
impl OuterImpl for Ptr<Outer> {
    fn __dtor(&self) {
        (*self.upgrade().deref()).m.as_pointer().__dtor();
    }
}
impl SImpl for Ptr<S> {
    fn __dtor(&self) {
        (*global_0.with(Value::clone).borrow_mut()).postfix_inc();
    }
}
impl Templated_char_Impl for Ptr<Templated_char_> {
    fn __dtor(&self) {
        {
            let rhs_0 = (((*global_0.with(Value::clone).borrow()) as usize)
                .wrapping_add((::std::mem::size_of::<u8>() as usize)))
                as i32;
            (*global_0.with(Value::clone).borrow_mut()) = rhs_0
        };
    }
}
impl Templated_int_Impl for Ptr<Templated_int_> {
    fn __dtor(&self) {
        {
            let rhs_0 = (((*global_0.with(Value::clone).borrow()) as usize)
                .wrapping_add((::std::mem::size_of::<i32>() as usize)))
                as i32;
            (*global_0.with(Value::clone).borrow_mut()) = rhs_0
        };
    }
}
