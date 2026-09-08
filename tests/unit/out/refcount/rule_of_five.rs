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
thread_local!(
    pub static moves_2: Value<i32> = Rc::new(RefCell::new(0));
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
    pub fn Buffer_pmutBuffer(o: Ptr<Buffer>) -> Self {
        let __this: Value<Buffer> = Rc::new(RefCell::new(Self {
            data: Rc::new(RefCell::new(
                (*(*o.upgrade().deref()).data.borrow()).clone(),
            )),
            size: Rc::new(RefCell::new((*(*o.upgrade().deref()).size.borrow()))),
        }));
        let this: Ptr<Buffer> = __this.as_pointer();
        (*(*o.upgrade().deref()).data.borrow_mut()) = Ptr::<i32>::null();
        (*(*o.upgrade().deref()).size.borrow_mut()) = 0;
        (*alive_0.with(Value::clone).borrow_mut()).prefix_inc();
        (*moves_2.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Buffer {
    fn clone(&self) -> Self {
        let __src: Value<Buffer> = Rc::new(RefCell::new(Buffer {
            data: self.data.clone(),
            size: self.size.clone(),
        }));
        Buffer::Buffer_pconstBuffer(__src.as_pointer())
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
pub fn make_3(size: i32) -> Buffer {
    let size: Value<i32> = Rc::new(RefCell::new(size));
    let b: Value<Buffer> = Rc::new(RefCell::new(Buffer::Buffer({ (*size.borrow()) })));
    let _dtor_b = ScopedDestructor::new(&b, |__p| __p.destructor());
    return Buffer::Buffer_pmutBuffer({ b.as_pointer() });
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        let a: Value<Buffer> = Rc::new(RefCell::new(Buffer::Buffer({ 4 })));
        let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
        let b: Value<Buffer> = Rc::new(RefCell::new(Buffer::Buffer_pconstBuffer({
            a.as_pointer()
        })));
        let _dtor_b = ScopedDestructor::new(&b, |__p| __p.destructor());
        assert!(
            (((*alive_0.with(Value::clone).borrow()) == 2)
                && ((*copies_1.with(Value::clone).borrow()) == 1))
                && ((*moves_2.with(Value::clone).borrow()) == 0)
        );
        (*(*b.borrow()).data.borrow())
            .offset((0) as isize)
            .write(100);
        assert!((((*(*a.borrow()).data.borrow()).offset((0) as isize).read()) == 0));
        let c: Value<Buffer> = Rc::new(RefCell::new(Buffer::Buffer_pmutBuffer({ a.as_pointer() })));
        let _dtor_c = ScopedDestructor::new(&c, |__p| __p.destructor());
        assert!(
            ((*alive_0.with(Value::clone).borrow()) == 3)
                && ((*moves_2.with(Value::clone).borrow()) == 1)
        );
        assert!(
            ((*(*a.borrow()).data.borrow()).is_null()) && ((*(*a.borrow()).size.borrow()) == 0)
        );
        assert!(
            ((*(*c.borrow()).size.borrow()) == 4)
                && (((*(*c.borrow()).data.borrow()).offset((3) as isize).read()) == 3)
        );
        let d: Value<Buffer> = Rc::new(RefCell::new(({ make_3(2) })));
        let _dtor_d = ScopedDestructor::new(&d, |__p| __p.destructor());
        assert!(
            ((*(*d.borrow()).size.borrow()) == 2) && ((*moves_2.with(Value::clone).borrow()) == 2)
        );
        ({ BufferImpl::operator_assign_pconstBuffer(&d.as_pointer(), b.as_pointer()) });
        assert!(
            (((*(*d.borrow()).size.borrow()) == 4)
                && (((*(*d.borrow()).data.borrow()).offset((0) as isize).read()) == 100))
                && ((*copies_1.with(Value::clone).borrow()) == 2)
        );
        ({ BufferImpl::operator_assign_pmutBuffer(&d.as_pointer(), c.as_pointer()) });
        assert!(
            ((((*(*d.borrow()).data.borrow()).offset((0) as isize).read()) == 0)
                && ((*(*c.borrow()).data.borrow()).is_null()))
                && ((*moves_2.with(Value::clone).borrow()) == 3)
        );
        ({
            let _o: Ptr<Buffer> = d.as_pointer();
            BufferImpl::operator_assign_pmutBuffer(&d.as_pointer(), _o)
        });
        assert!(
            ((*(*d.borrow()).size.borrow()) == 4) && ((*moves_2.with(Value::clone).borrow()) == 3)
        );
        let vec_: Value<Vec<Buffer>> = Rc::new(RefCell::new(Vec::new()));
        (*vec_.borrow_mut()).push(Buffer::Buffer({ 3 }));
        {
            let a0_clone = (*b.borrow()).clone();
            (*vec_.borrow_mut()).push(a0_clone)
        };
        assert!(
            ((*(*(vec_.as_pointer() as Ptr<Buffer>)
                .offset(0_usize)
                .upgrade()
                .deref())
            .size
            .borrow())
                == 3)
                && (((*(*(vec_.as_pointer() as Ptr<Buffer>)
                    .offset(1_usize)
                    .upgrade()
                    .deref())
                .data
                .borrow())
                .offset((0) as isize)
                .read())
                    == 100)
        );
    }
    assert!(((*alive_0.with(Value::clone).borrow()) == 0));
    return 0;
}
pub trait BufferImpl {
    fn destructor(&self);
    fn operator_assign_pconstBuffer(&self, o: Ptr<Buffer>) -> Ptr<Buffer>;
    fn operator_assign_pmutBuffer(&self, o: Ptr<Buffer>) -> Ptr<Buffer>;
}
impl BufferImpl for Ptr<Buffer> {
    fn destructor(&self) {
        (*(*(*self).upgrade().deref()).data.borrow()).delete_array();
        (*alive_0.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn operator_assign_pconstBuffer(&self, o: Ptr<Buffer>) -> Ptr<Buffer> {
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
    fn operator_assign_pmutBuffer(&self, o: Ptr<Buffer>) -> Ptr<Buffer> {
        if ((*self) == (o)) {
            return (*self).clone();
        }
        (*(*(*self).upgrade().deref()).data.borrow()).delete_array();
        let __rhs = (*(*o.upgrade().deref()).data.borrow()).clone();
        (*(*(*self).upgrade().deref()).data.borrow_mut()) = __rhs;
        let __rhs = (*(*o.upgrade().deref()).size.borrow());
        (*(*(*self).upgrade().deref()).size.borrow_mut()) = __rhs;
        (*(*o.upgrade().deref()).data.borrow_mut()) = Ptr::<i32>::null();
        (*(*o.upgrade().deref()).size.borrow_mut()) = 0;
        (*moves_2.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
}
