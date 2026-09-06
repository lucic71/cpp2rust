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
impl Drop for S {
    fn drop(&mut self) {
        (*global_0.with(Value::clone).borrow_mut()).postfix_inc();
    }
}
impl Clone for S {
    fn clone(&self) -> Self {
        let mut this = Self {};
        this
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
impl Clone for Defaulted {
    fn clone(&self) -> Self {
        let mut this = Self {
            s: Rc::new(RefCell::new((*self.s.borrow()).clone())),
        };
        this
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
impl Clone for Middle {
    fn clone(&self) -> Self {
        let mut this = Self {
            s: Rc::new(RefCell::new((*self.s.borrow()).clone())),
        };
        this
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
impl Clone for Outer {
    fn clone(&self) -> Self {
        let mut this = Self {
            m: Rc::new(RefCell::new((*self.m.borrow()).clone())),
        };
        this
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
impl Clone for ArrayMember {
    fn clone(&self) -> Self {
        let mut this = Self {
            items: Rc::new(RefCell::new((*self.items.borrow()).clone())),
        };
        this
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
impl Clone for EmptyBody {
    fn clone(&self) -> Self {
        let mut this = Self {
            s: Rc::new(RefCell::new((*self.s.borrow()).clone())),
        };
        this
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
impl Drop for Templated_char_ {
    fn drop(&mut self) {
        {
            let rhs_0 = (((*global_0.with(Value::clone).borrow()) as usize)
                .wrapping_add((::std::mem::size_of::<u8>() as usize)))
                as i32;
            (*global_0.with(Value::clone).borrow_mut()) = rhs_0
        };
    }
}
impl Clone for Templated_char_ {
    fn clone(&self) -> Self {
        let mut this = Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        };
        this
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
impl Drop for Templated_int_ {
    fn drop(&mut self) {
        {
            let rhs_0 = (((*global_0.with(Value::clone).borrow()) as usize)
                .wrapping_add((::std::mem::size_of::<i32>() as usize)))
                as i32;
            (*global_0.with(Value::clone).borrow_mut()) = rhs_0
        };
    }
}
impl Clone for Templated_int_ {
    fn clone(&self) -> Self {
        let mut this = Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        };
        this
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
impl Drop for Copied {
    fn drop(&mut self) {
        (*global_0.with(Value::clone).borrow_mut()).postfix_inc();
    }
}
impl Clone for Copied {
    fn clone(&self) -> Self {
        let mut this = Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        };
        this
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
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 1));
    {
        let s: Value<S> = Rc::new(RefCell::new(S {}));
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 2));
    {
        let d: Value<Defaulted> = Rc::new(RefCell::new(Defaulted {
            s: Rc::new(RefCell::new(S {})),
        }));
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 3));
    {
        let o: Value<Outer> = Rc::new(RefCell::new(Outer {
            m: Rc::new(RefCell::new(Middle {
                s: Rc::new(RefCell::new(S {})),
            })),
        }));
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 4));
    {
        let am: Value<ArrayMember> = Rc::new(RefCell::new(ArrayMember {
            items: Rc::new(RefCell::new(Box::new([S {}, S {}, S {}]))),
        }));
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 7));
    {
        let e: Value<EmptyBody> = Rc::new(RefCell::new(EmptyBody {
            s: Rc::new(RefCell::new(S {})),
        }));
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 8));
    {
        let tc: Value<Templated_char_> = Rc::new(RefCell::new(Templated_char_ {
            v: Rc::new(RefCell::new(<u8>::default())),
        }));
        let ti: Value<Templated_int_> = Rc::new(RefCell::new(Templated_int_ {
            v: Rc::new(RefCell::new(<i32>::default())),
        }));
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 13));
    {
        let a: Value<Copied> = Rc::new(RefCell::new(Copied {
            v: Rc::new(RefCell::new(5)),
        }));
        let b: Value<Copied> = Rc::new(RefCell::new((*a.borrow()).clone()));
        assert!(((*(*b.borrow()).v.borrow()) == 5));
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 15));
    return 0;
}
