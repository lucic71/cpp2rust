extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Test {
    pub x: i32,
}
impl Test {
    pub fn inc(&self) {
        self.x.postfix_inc();
    }
    pub fn dec(&self) {
        self.x.postfix_dec();
    }
    pub fn as_ptr(&self) -> Ptr<i32> {
        return (self.field_ptr(
            0,
            |__v: &Test| ::std::slice::from_ref(&__v.x),
            |__v: &mut Test| ::std::slice::from_mut(&mut __v.x),
        ));
    }
    pub fn update(&self, x: i32, y: i32) {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let y: Value<i32> = Rc::new(RefCell::new(y));
        self.x = ((*x.borrow()) + (*y.borrow()));
    }
}
impl Clone for Test {
    fn clone(&self) -> Self {
        let mut this = Self { x: self.x };
        this
    }
}
impl ByteRepr for Test {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
pub fn Update_0(t: Ptr<Test>) -> Ptr<Test> {
    let t: Value<Ptr<Test>> = Rc::new(RefCell::new(t));
    let x: Value<i32> = Rc::new(RefCell::new(1));
    let y: Value<i32> = Rc::new(RefCell::new(2));
    (*x.borrow_mut()).prefix_inc();
    ({ (*(*t.borrow()).upgrade().deref()).update((*x.borrow()), (*y.borrow())) });
    (*x.borrow_mut()) = (*(*t.borrow()).upgrade().deref()).x;
    (*y.borrow_mut()) = (*(*t.borrow()).upgrade().deref()).x;
    ({
        let _x: i32 = (*x.borrow());
        let _y: i32 = (*y.borrow());
        (*(*t.borrow()).upgrade().deref()).update(_x, _y)
    });
    return (*t.borrow()).clone();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let t1: Value<Test> = Rc::new(RefCell::new(Test { x: 100 }));
    let t2: Value<Ptr<Test>> = Rc::new(RefCell::new(({ Update_0((t1.as_pointer())) })));
    let t3: Value<Ptr<Test>> = Rc::new(RefCell::new(Ptr::<Test>::null()));
    (*t3.borrow_mut()) = (*t2.borrow()).clone();
    (*t3.borrow()).with_mut(|__v| __v.x = 15);
    {
        let _ptr = ({ (*(*t3.borrow()).upgrade().deref()).as_ptr() }).clone();
        _ptr.write((_ptr.read()) + 10)
    };
    return {
        let _lhs = {
            let _lhs = (*(*t3.borrow()).upgrade().deref()).x;
            _lhs + (*(*t2.borrow()).upgrade().deref()).x
        };
        _lhs + (*t1.borrow()).x
    };
}
