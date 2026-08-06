extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive(Default)]
pub struct node_t {
    pub left: Ptr<node_t>,
    pub right: Ptr<node_t>,
    pub value: i32,
}
impl Clone for node_t {
    fn clone(&self) -> Self {
        let mut this = Self {
            left: (self.left).clone(),
            right: (self.right).clone(),
            value: self.value,
        };
        this
    }
}
impl ByteRepr for node_t {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.left.to_bytes(&mut buf[0..8]);
        self.right.to_bytes(&mut buf[8..16]);
        self.value.to_bytes(&mut buf[16..20]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            left: <Ptr<node_t>>::from_bytes(&buf[0..8]),
            right: <Ptr<node_t>>::from_bytes(&buf[8..16]),
            value: <i32>::from_bytes(&buf[16..20]),
        }
    }
}
pub fn find_0(node: Ptr<node_t>, value: i32) -> Ptr<node_t> {
    let node: Value<Ptr<node_t>> = Rc::new(RefCell::new(node));
    let value: Value<i32> = Rc::new(RefCell::new(value));
    if ({
        let _lhs = (*value.borrow());
        _lhs < (*node.borrow()).with(|__v| (*__v).value)
    }) && (!(((*node.borrow()).with(|__v| (*__v).left.clone())).is_null()))
    {
        return ({
            find_0(
                ((*node.borrow()).with(|__v| (*__v).left.clone())).clone(),
                (*value.borrow()),
            )
        });
    } else if ({
        let _lhs = (*value.borrow());
        _lhs > (*node.borrow()).with(|__v| (*__v).value)
    }) && (!(((*node.borrow()).with(|__v| (*__v).right.clone())).is_null()))
    {
        return ({
            find_0(
                ((*node.borrow()).with(|__v| (*__v).right.clone())).clone(),
                (*value.borrow()),
            )
        });
    } else if {
        let _lhs = (*value.borrow());
        _lhs == (*node.borrow()).with(|__v| (*__v).value)
    } {
        return (*node.borrow()).clone();
    }
    return Ptr::<node_t>::null();
}
pub fn insert_1(node: Ptr<node_t>, value: i32) -> Ptr<node_t> {
    let node: Value<Ptr<node_t>> = Rc::new(RefCell::new(node));
    let value: Value<i32> = Rc::new(RefCell::new(value));
    if (*node.borrow()).is_null() {
        return Ptr::alloc(node_t {
            left: Ptr::<node_t>::null(),
            right: Ptr::<node_t>::null(),
            value: (*value.borrow()),
        });
    }
    if {
        let _lhs = (*value.borrow());
        _lhs < (*node.borrow()).with(|__v| (*__v).value)
    } {
        {
            let __rhs = ({
                insert_1(
                    ((*node.borrow()).with(|__v| (*__v).left.clone())).clone(),
                    (*value.borrow()),
                )
            });
            (*node.borrow()).with_mut(|__v| __v.left = __rhs)
        };
    } else if {
        let _lhs = (*value.borrow());
        _lhs > (*node.borrow()).with(|__v| (*__v).value)
    } {
        {
            let __rhs = ({
                insert_1(
                    ((*node.borrow()).with(|__v| (*__v).right.clone())).clone(),
                    (*value.borrow()),
                )
            });
            (*node.borrow()).with_mut(|__v| __v.right = __rhs)
        };
    }
    return (*node.borrow()).clone();
}
pub fn del_2(node: Ptr<node_t>) {
    let node: Value<Ptr<node_t>> = Rc::new(RefCell::new(node));
    if !(((*node.borrow()).with(|__v| (*__v).left.clone())).is_null()) {
        ({ del_2(((*node.borrow()).with(|__v| (*__v).left.clone())).clone()) });
    }
    if !(((*node.borrow()).with(|__v| (*__v).right.clone())).is_null()) {
        ({ del_2(((*node.borrow()).with(|__v| (*__v).right.clone())).clone()) });
    }
    (*node.borrow()).delete();
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let root: Value<Ptr<node_t>> = Rc::new(RefCell::new(Ptr::alloc(node_t {
        left: Ptr::<node_t>::null(),
        right: Ptr::<node_t>::null(),
        value: 0,
    })));
    {
        let __rhs = ({ insert_1((*root.borrow()).clone(), 1) });
        (*root.borrow_mut()) = __rhs
    };
    {
        let __rhs = ({ insert_1((*root.borrow()).clone(), 2) });
        (*root.borrow_mut()) = __rhs
    };
    {
        let __rhs = ({ insert_1((*root.borrow()).clone(), 3) });
        (*root.borrow_mut()) = __rhs
    };
    {
        let __rhs = ({ insert_1((*root.borrow()).clone(), 4) });
        (*root.borrow_mut()) = __rhs
    };
    let out: Value<bool> = Rc::new(RefCell::new(
        (((((({ find_0((*root.borrow()).clone(), 0) }).with(|__v| (*__v).value) == 0)
            && (({ find_0((*root.borrow()).clone(), 1) }).with(|__v| (*__v).value) == 1))
            && (({ find_0((*root.borrow()).clone(), 2) }).with(|__v| (*__v).value) == 2))
            && (({ find_0((*root.borrow()).clone(), 3) }).with(|__v| (*__v).value) == 3))
            && (({ find_0((*root.borrow()).clone(), 4) }).with(|__v| (*__v).value) == 4))
            && (({ find_0((*root.borrow()).clone(), 5) }).is_null()),
    ));
    ({ del_2((*root.borrow()).clone()) });
    return ((*out.borrow()) as i32);
}
