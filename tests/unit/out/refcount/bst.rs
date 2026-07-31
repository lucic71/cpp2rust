extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
        _lhs < (*(*node.borrow()).upgrade().deref()).value
    }) && (!(((*(*node.borrow()).upgrade().deref()).left).is_null()))
    {
        return ({
            find_0(
                ((*(*node.borrow()).upgrade().deref()).left).clone(),
                (*value.borrow()),
            )
        });
    } else if ({
        let _lhs = (*value.borrow());
        _lhs > (*(*node.borrow()).upgrade().deref()).value
    }) && (!(((*(*node.borrow()).upgrade().deref()).right).is_null()))
    {
        return ({
            find_0(
                ((*(*node.borrow()).upgrade().deref()).right).clone(),
                (*value.borrow()),
            )
        });
    } else if {
        let _lhs = (*value.borrow());
        _lhs == (*(*node.borrow()).upgrade().deref()).value
    } {
        return (*node.borrow()).clone();
    }
    return Ptr::<node_t>::null();
}
pub fn insert_1(node: Ptr<node_t>, new_node: Ptr<node_t>) -> Ptr<node_t> {
    let node: Value<Ptr<node_t>> = Rc::new(RefCell::new(node));
    let new_node: Value<Ptr<node_t>> = Rc::new(RefCell::new(new_node));
    if (*node.borrow()).is_null() {
        return (*new_node.borrow()).clone();
    }
    if {
        let _lhs = (*(*new_node.borrow()).upgrade().deref()).value;
        _lhs < (*(*node.borrow()).upgrade().deref()).value
    } {
        let __rhs = ({
            insert_1(
                ((*(*node.borrow()).upgrade().deref()).left).clone(),
                (*new_node.borrow()).clone(),
            )
        });
        (*node.borrow()).with_mut(|__v| __v.left = __rhs);
    } else if {
        let _lhs = (*(*new_node.borrow()).upgrade().deref()).value;
        _lhs > (*(*node.borrow()).upgrade().deref()).value
    } {
        let __rhs = ({
            insert_1(
                ((*(*node.borrow()).upgrade().deref()).right).clone(),
                (*new_node.borrow()).clone(),
            )
        });
        (*node.borrow()).with_mut(|__v| __v.right = __rhs);
    }
    return (*node.borrow()).clone();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let tree: Value<Option<Value<node_t>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(node_t {
            left: Ptr::<node_t>::null(),
            right: Ptr::<node_t>::null(),
            value: 0,
        })))));
    let n1: Value<Option<Value<node_t>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(node_t {
            left: Ptr::<node_t>::null(),
            right: Ptr::<node_t>::null(),
            value: 1,
        })))));
    let n2: Value<Option<Value<node_t>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(node_t {
            left: Ptr::<node_t>::null(),
            right: Ptr::<node_t>::null(),
            value: 2,
        })))));
    let n3: Value<Option<Value<node_t>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(node_t {
            left: Ptr::<node_t>::null(),
            right: Ptr::<node_t>::null(),
            value: 3,
        })))));
    let n4: Value<Option<Value<node_t>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(node_t {
            left: Ptr::<node_t>::null(),
            right: Ptr::<node_t>::null(),
            value: 4,
        })))));
    let ptr1: Value<Ptr<node_t>> = Rc::new(RefCell::new(((*tree.borrow()).as_pointer())));
    let __rhs = ({ insert_1((*ptr1.borrow()).clone(), ((*n1.borrow()).as_pointer())) });
    (*ptr1.borrow_mut()) = __rhs;
    let __rhs = ({ insert_1((*ptr1.borrow()).clone(), ((*n2.borrow()).as_pointer())) });
    (*ptr1.borrow_mut()) = __rhs;
    let __rhs = ({ insert_1((*ptr1.borrow()).clone(), ((*n3.borrow()).as_pointer())) });
    (*ptr1.borrow_mut()) = __rhs;
    let __rhs = ({ insert_1((*ptr1.borrow()).clone(), ((*n4.borrow()).as_pointer())) });
    (*ptr1.borrow_mut()) = __rhs;
    return ((((((((*({ find_0((*ptr1.borrow()).clone(), 0) }).upgrade().deref()).value == 0)
        && ((*({ find_0((*ptr1.borrow()).clone(), 1) }).upgrade().deref()).value == 1))
        && ((*({ find_0((*ptr1.borrow()).clone(), 2) }).upgrade().deref()).value == 2))
        && ((*({ find_0((*ptr1.borrow()).clone(), 3) }).upgrade().deref()).value == 3))
        && ((*({ find_0((*ptr1.borrow()).clone(), 4) }).upgrade().deref()).value == 4))
        && (({ find_0((*ptr1.borrow()).clone(), 5) }).is_null())) as i32);
}
