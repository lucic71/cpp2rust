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
#[derive(
    Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, CppFrom, CopyAssign, MoveAssign, Destructor,
)]
#[cpp_from(i32, new, value)]
#[byte_size(20)]
pub struct Buffer {
    #[offset(0)]
    #[byte_size(16)]
    #[array_len(4)]
    pub data: Box<[i32]>,
    #[offset(16)]
    pub size: i32,
}
impl Buffer {
    pub fn new(this: Ptr<Buffer>, mut size: i32) {
        field!(this, size).write(size);
        {
            let mut i: i32 = 0;
            'loop_: while (i < 4) {
                array_elem!(this, data, i).write({ if (i < size) { i } else { -1_i32 } });
                i.prefix_inc();
            }
        }
        (*alive_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
    pub fn copy_from(this: Ptr<Buffer>, o: Ptr<Buffer>) {
        field!(this, size).write(o.with(|__s| __s.size));
        {
            let mut i: i32 = 0;
            'loop_: while (i < 4) {
                array_elem!(this, data, i).write({ (array_elem!(o, data, i).read()) });
                i.prefix_inc();
            }
        }
        (*alive_0.with(Value::clone).borrow_mut()).prefix_inc();
        (*copies_1.with(Value::clone).borrow_mut()).prefix_inc();
    }
    pub fn move_from(this: Ptr<Buffer>, o: Ptr<Buffer>) {
        field!(this, size).write(o.with(|__s| __s.size));
        {
            let mut i: i32 = 0;
            'loop_: while (i < 4) {
                array_elem!(this, data, i).write({ (array_elem!(o, data, i).read()) });
                array_elem!(o, data, i).write(-1_i32);
                i.prefix_inc();
            }
        }
        field!(o, size).write(0);
        (*alive_0.with(Value::clone).borrow_mut()).prefix_inc();
        (*moves_2.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
impl Clone for Buffer {
    fn clone(&self) -> Self {
        let __src: Value<Buffer> = Rc::new(RefCell::new(Buffer {
            data: self.data.clone(),
            size: self.size.clone(),
        }));
        let __dst: Value<Buffer> = Rc::new(RefCell::new(<Buffer>::uninit()));
        Buffer::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
impl Default for Buffer {
    fn default() -> Self {
        {
            Buffer {
                data: (0..4).map(|_| 0_i32).collect::<Box<[i32]>>(),
                size: 0_i32,
            }
        }
    }
}
pub fn make_3(__ret: Ptr<Buffer>, mut size: i32) {
    let b: Value<Buffer> = Rc::new(RefCell::new(<Buffer>::uninit()));
    Buffer::new(b.as_pointer(), { size });
    let _dtor_b = std::mem::DropGuard::new(b.clone(), |__p| __p.as_pointer().destructor());
    Buffer::move_from(__ret.clone(), { b.as_pointer() });
    return;
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    {
        let a: Value<Buffer> = Rc::new(RefCell::new(<Buffer>::uninit()));
        Buffer::new(a.as_pointer(), { 4 });
        let _dtor_a = std::mem::DropGuard::new(a.clone(), |__p| __p.as_pointer().destructor());
        let b: Value<Buffer> = Rc::new(RefCell::new(<Buffer>::uninit()));
        Buffer::copy_from(b.as_pointer(), { a.as_pointer() });
        let _dtor_b = std::mem::DropGuard::new(b.clone(), |__p| __p.as_pointer().destructor());
        assert!(
            ((alive_0.with(|rc| *rc.borrow()) == 2) && (copies_1.with(|rc| *rc.borrow()) == 1))
                && (moves_2.with(|rc| *rc.borrow()) == 0)
        );
        array_elem!(b.as_pointer(), data, 0).write(100);
        assert!(((array_elem!(a.as_pointer(), data, 0).read()) == 0));
        let c: Value<Buffer> = Rc::new(RefCell::new(<Buffer>::uninit()));
        Buffer::move_from(c.as_pointer(), { a.as_pointer() });
        let _dtor_c = std::mem::DropGuard::new(c.clone(), |__p| __p.as_pointer().destructor());
        assert!((alive_0.with(|rc| *rc.borrow()) == 3) && (moves_2.with(|rc| *rc.borrow()) == 1));
        assert!(
            ({
                ({ (*a.borrow()).size } == 0)
                    && ((array_elem!(a.as_pointer(), data, 0).read()) == -1_i32)
            })
        );
        assert!(
            ({
                ({ (*c.borrow()).size } == 4)
                    && ((array_elem!(c.as_pointer(), data, 3).read()) == 3)
            })
        );
        let d: Value<Buffer> = Rc::new(RefCell::new(<Buffer>::uninit()));
        ({ make_3(d.as_pointer(), 2) });
        let _dtor_d = std::mem::DropGuard::new(d.clone(), |__p| __p.as_pointer().destructor());
        assert!(({ ({ (*d.borrow()).size } == 2) && (moves_2.with(|rc| *rc.borrow()) == 2) }));
        ({ BufferImpl::copy_assign(&d.as_pointer(), b.as_pointer()) });
        assert!(
            ({
                (({ (*d.borrow()).size } == 4)
                    && ((array_elem!(d.as_pointer(), data, 0).read()) == 100))
                    && (copies_1.with(|rc| *rc.borrow()) == 2)
            })
        );
        ({ BufferImpl::move_assign(&d.as_pointer(), c.as_pointer()) });
        assert!(
            ({
                (((array_elem!(d.as_pointer(), data, 0).read()) == 0)
                    && ({ (*c.borrow()).size } == 0))
                    && (moves_2.with(|rc| *rc.borrow()) == 3)
            })
        );
        ({
            let _o: Ptr<Buffer> = d.as_pointer();
            BufferImpl::move_assign(&d.as_pointer(), _o)
        });
        assert!(({ ({ (*d.borrow()).size } == 4) && (moves_2.with(|rc| *rc.borrow()) == 3) }));
    }
    assert!((alive_0.with(|rc| *rc.borrow()) == 0));
    return 0;
}
pub trait BufferImpl {
    fn destructor(&self);
    fn copy_assign(&self, o: Ptr<Buffer>) -> Ptr<Buffer>;
    fn move_assign(&self, o: Ptr<Buffer>) -> Ptr<Buffer>;
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
        {
            let mut i: i32 = 0;
            'loop_: while (i < 4) {
                array_elem!((*self), data, i).write({ (array_elem!(o, data, i).read()) });
                i.prefix_inc();
            }
        }
        (*copies_1.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
    fn move_assign(&self, o: Ptr<Buffer>) -> Ptr<Buffer> {
        if ((*self) == (o)) {
            return (*self).clone();
        }
        field!((*self), size).write({ o.with(|__s| __s.size) });
        {
            let mut i: i32 = 0;
            'loop_: while (i < 4) {
                array_elem!((*self), data, i).write({ (array_elem!(o, data, i).read()) });
                array_elem!(o, data, i).write(-1_i32);
                i.prefix_inc();
            }
        }
        field!(o, size).write(0);
        (*moves_2.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = alive_0.with(|_| ());
    let _ = copies_1.with(|_| ());
    let _ = moves_2.with(|_| ());
}
