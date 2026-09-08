extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static alive_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static copies_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Default)]
pub struct Buffer {
    pub data: Value<Ptr<i32>>,
    pub size: Value<i32>,
}
impl Buffer {
    pub fn Buffer(size: i32) -> Self {
        let size: Value<i32> = Rc::new(RefCell::new(size));
        let __this: Value<Buffer> = Rc::new(RefCell::new(Self {
            data: Rc::new(RefCell::new(Ptr::alloc_array(
                (0..((*size.borrow()) as usize))
                    .map(|_| <i32>::default())
                    .collect::<Box<[i32]>>(),
            ))),
            size: Rc::new(RefCell::new((*size.borrow()))),
        }));
        let this: Ptr<Buffer> = __this.as_pointer();
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*size.borrow())) {
            let __rhs = (*i.borrow());
            (*(*this.upgrade().deref()).data.borrow())
                .offset((*i.borrow()) as isize)
                .write(__rhs);
            (*i.borrow_mut()).prefix_inc();
        }
        (*alive_0.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn Buffer_pconstBuffer(o: Ptr<Buffer>) -> Self {
        let __this: Value<Buffer> = Rc::new(RefCell::new(Self {
            data: Rc::new(RefCell::new(Ptr::alloc_array(
                (0..((*(*o.upgrade().deref()).size.borrow()) as usize))
                    .map(|_| <i32>::default())
                    .collect::<Box<[i32]>>(),
            ))),
            size: Rc::new(RefCell::new((*(*o.upgrade().deref()).size.borrow()))),
        }));
        let this: Ptr<Buffer> = __this.as_pointer();
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*(*this.upgrade().deref()).size.borrow())) {
            let __rhs = ((*(*o.upgrade().deref()).data.borrow())
                .offset((*i.borrow()) as isize)
                .read());
            (*(*this.upgrade().deref()).data.borrow())
                .offset((*i.borrow()) as isize)
                .write(__rhs);
            (*i.borrow_mut()).prefix_inc();
        }
        (*alive_0.with(Value::clone).borrow_mut()).prefix_inc();
        (*copies_1.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Buffer {
    fn clone(&self) -> Self {
        let __this: Value<Buffer> = Rc::new(RefCell::new(Self {
            data: Rc::new(RefCell::new(Ptr::alloc_array(
                (0..((*(*o.upgrade().deref()).size.borrow()) as usize))
                    .map(|_| <i32>::default())
                    .collect::<Box<[i32]>>(),
            ))),
            size: Rc::new(RefCell::new((*(*o.upgrade().deref()).size.borrow()))),
        }));
        let this: Ptr<Buffer> = __this.as_pointer();
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*(*self).size.borrow())) {
            let __rhs = ((*(*o.upgrade().deref()).data.borrow())
                .offset((*i.borrow()) as isize)
                .read());
            (*(*self).data.borrow())
                .offset((*i.borrow()) as isize)
                .write(__rhs);
            (*i.borrow_mut()).prefix_inc();
        }
        (*alive_0.with(Value::clone).borrow_mut()).prefix_inc();
        (*copies_1.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Buffer {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.data.borrow()).to_bytes(&mut buf[0..8]);
        (*self.size.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data: Rc::new(RefCell::new(<Ptr<i32>>::from_bytes(&buf[0..8]))),
            size: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
        }
    }
}
pub fn sum_2(b: Buffer) -> i32 {
    let b: Value<Buffer> = Rc::new(RefCell::new(b));
    let s: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*(*b.borrow()).size.borrow())) {
        let __rhs = ((*(*b.borrow()).data.borrow())
            .offset((*i.borrow()) as isize)
            .read());
        (*s.borrow_mut()) += __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    return (*s.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        let a: Value<Buffer> = Rc::new(RefCell::new(Buffer::Buffer({ 4 })));
        let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
        let b: Value<Buffer> = Rc::new(RefCell::new((*a.borrow()).clone()));
        let _dtor_b = ScopedDestructor::new(&b, |__p| __p.destructor());
        assert!(
            ((*alive_0.with(Value::clone).borrow()) == 2)
                && ((*copies_1.with(Value::clone).borrow()) == 1)
        );
        (*(*b.borrow()).data.borrow())
            .offset((0) as isize)
            .write(100);
        assert!((((*(*a.borrow()).data.borrow()).offset((0) as isize).read()) == 0));
        let c: Value<Buffer> = Rc::new(RefCell::new(Buffer::Buffer({ 2 })));
        let _dtor_c = ScopedDestructor::new(&c, |__p| __p.destructor());
        ({ BufferImpl::operator_assign(&c.as_pointer(), a.as_pointer()) });
        assert!(
            ((*(*c.borrow()).size.borrow()) == 4)
                && (((*(*c.borrow()).data.borrow()).offset((3) as isize).read()) == 3)
        );
        assert!(
            ((*alive_0.with(Value::clone).borrow()) == 3)
                && ((*copies_1.with(Value::clone).borrow()) == 2)
        );
        ({
            let _o: Ptr<Buffer> = c.as_pointer();
            BufferImpl::operator_assign(&c.as_pointer(), _o)
        });
        assert!(((*copies_1.with(Value::clone).borrow()) == 2));
        assert!((({ sum_2((*a.borrow()).clone(),) }) == 6));
        assert!(
            ((*alive_0.with(Value::clone).borrow()) == 3)
                && ((*copies_1.with(Value::clone).borrow()) == 3)
        );
        let d: Value<Buffer> = Rc::new(RefCell::new((*a.borrow())));
        let _dtor_d = ScopedDestructor::new(&d, |__p| __p.destructor());
        assert!(
            ((*alive_0.with(Value::clone).borrow()) == 4)
                && ((*copies_1.with(Value::clone).borrow()) == 4)
        );
        assert!(
            (!((*(*a.borrow()).data.borrow()).is_null())) && ((*(*a.borrow()).size.borrow()) == 4)
        );
        ({ BufferImpl::operator_assign(&d.as_pointer(), b.as_pointer()) });
        assert!(((*copies_1.with(Value::clone).borrow()) == 5));
        assert!(
            (((*(*b.borrow()).data.borrow()).offset((0) as isize).read()) == 100)
                && (((*(*d.borrow()).data.borrow()).offset((0) as isize).read()) == 100)
        );
    }
    assert!(((*alive_0.with(Value::clone).borrow()) == 0));
    return 0;
}
pub trait BufferImpl {
    fn destructor(&self);
    fn operator_assign(&self, o: Ptr<Buffer>) -> Ptr<Buffer>;
}
impl BufferImpl for Ptr<Buffer> {
    fn destructor(&self) {
        (*(*(*self).upgrade().deref()).data.borrow()).delete_array();
        (*alive_0.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn operator_assign(&self, o: Ptr<Buffer>) -> Ptr<Buffer> {
        if ((*self) == (o)) {
            return (*self).clone();
        }
        (*(*(*self).upgrade().deref()).data.borrow()).delete_array();
        (*(*(*self).upgrade().deref()).data.borrow_mut()) = Ptr::alloc_array(
            (0..((*(*o.upgrade().deref()).size.borrow()) as usize))
                .map(|_| <i32>::default())
                .collect::<Box<[i32]>>(),
        );
        let __rhs = (*(*o.upgrade().deref()).size.borrow());
        (*(*(*self).upgrade().deref()).size.borrow_mut()) = __rhs;
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*(*(*self).upgrade().deref()).size.borrow())) {
            let __rhs = ((*(*o.upgrade().deref()).data.borrow())
                .offset((*i.borrow()) as isize)
                .read());
            (*(*(*self).upgrade().deref()).data.borrow())
                .offset((*i.borrow()) as isize)
                .write(__rhs);
            (*i.borrow_mut()).prefix_inc();
        }
        (*copies_1.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
}
