extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static alive_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static copies_1: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(20)]
pub struct Buffer {
    #[offset(0)]
    #[byte_size(16)]
    pub data: Box<[()]>,
    #[offset(16)]
    pub size: (),
}
impl Buffer {
    pub fn new(mut size: ()) -> Self {
        let __this: Value<Buffer> = Rc::new(RefCell::new(Self {
            data: (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            size: size,
        }));
        let this: Ptr<Buffer> = __this.as_pointer();
        let mut i: () = 0_();
        'loop_: while (i < 4_()) {
            array_elem!(this, data, i).write({ if (i < size) { i } else { -1_() } });
            i.prefix_inc();
        }
        (*alive_0.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(o: Ptr<Buffer>) -> Self {
        let __this: Value<Buffer> = Rc::new(RefCell::new(Self {
            data: (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            size: o.with(|__s| __s.size),
        }));
        let this: Ptr<Buffer> = __this.as_pointer();
        let mut i: () = 0_();
        'loop_: while (i < 4_()) {
            array_elem!(this, data, i).write({ (array_elem!(o, data, i).read()) });
            i.prefix_inc();
        }
        (*alive_0.with(Value::clone).borrow_mut()).prefix_inc();
        (*copies_1.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Buffer {
    fn clone(&self) -> Self {
        let __src: Value<Buffer> = Rc::new(RefCell::new(Buffer {
            data: self.data.clone(),
            size: self.size.clone(),
        }));
        Buffer::copy_from(__src.as_pointer())
    }
}
impl Default for Buffer {
    fn default() -> Self {
        {
            Buffer {
                data: (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                size: unimplemented!(),
            }
        }
    }
}
pub fn sum_2(b: Ptr<Buffer>) -> () {
    let s: Value<()> = Rc::new(RefCell::new(0_()));
    let mut i: () = 0_();
    'loop_: while ({ i } < { b.with(|__s| __s.size) }) {
        (*s.borrow_mut()) += (array_elem!(b, data, i).read());
        i.prefix_inc();
    }
    return (*s.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    {
        let a: Value<Buffer> = Rc::new(RefCell::new(Buffer::new({ 4_() })));
        let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
        let b: Value<Buffer> = Rc::new(RefCell::new(Buffer::copy_from({ a.as_pointer() })));
        let _dtor_b = ScopedDestructor::new(&b, |__p| __p.destructor());
        unimplemented!();
        array_elem!(b.as_pointer(), data, 0_()).write(100_());
        unimplemented!();
        let c: Value<Buffer> = Rc::new(RefCell::new(Buffer::new({ 2_() })));
        let _dtor_c = ScopedDestructor::new(&c, |__p| __p.destructor());
        ({ BufferImpl::copy_assign(&c.as_pointer(), a.as_pointer()) });
        unimplemented!();
        unimplemented!();
        ({
            let _o: Ptr<Buffer> = c.as_pointer();
            BufferImpl::copy_assign(&c.as_pointer(), _o)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let d: Value<Buffer> = Rc::new(RefCell::new(Buffer::copy_from({ a.as_pointer() })));
        let _dtor_d = ScopedDestructor::new(&d, |__p| __p.destructor());
        unimplemented!();
        unimplemented!();
        ({ BufferImpl::copy_assign(&d.as_pointer(), b.as_pointer()) });
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    return 0_();
}
pub trait BufferImpl {
    fn destructor(&self);
    fn copy_assign(&self, o: Ptr<Buffer>) -> Ptr<Buffer>;
}
impl BufferImpl for Ptr<Buffer> {
    fn destructor(&self) {
        (*alive_0.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn copy_assign(&self, o: Ptr<Buffer>) -> Ptr<Buffer> {
        if ((*self) == (o)) {
            return (*self).clone();
        }
        field!((*self), size).write({ o.with(|__s| __s.size) });
        let mut i: () = 0_();
        'loop_: while (i < 4_()) {
            array_elem!((*self), data, i).write({ (array_elem!(o, data, i).read()) });
            i.prefix_inc();
        }
        (*copies_1.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = alive_0.with(|_| ());
    let _ = copies_1.with(|_| ());
}
