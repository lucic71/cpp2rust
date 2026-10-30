extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Node {
    #[offset(0)]
    pub val: (),
    #[offset(8)]
    #[byte_size(8)]
    pub next: Ptr<Node>,
}
pub fn Find_0(mut head: Ptr<Node>, mut idx: ()) -> Ptr<Node> {
    let curr: Value<Ptr<Node>> = Rc::new(RefCell::new((head).clone()));
    let mut i: () = 0_();
    'loop_: while (i < idx) {
        (*curr.borrow_mut()) = { (*curr.borrow()).with(|__s| __s.next.clone()) };
        i.postfix_inc();
    }
    return (*curr.borrow()).clone();
}
pub fn Append_1(head: Ptr<Node>, new_node: Ptr<Node>) {
    let mut curr: Ptr<Node> = (head).clone();
    'loop_: while !((curr.with(|__s| __s.next.clone())).is_null()) {
        curr = { curr.with(|__s| __s.next.clone()) };
    }
    ({ NodeImpl::SetNext(&curr, (new_node).clone()) });
}
pub fn Delete_2(head: Ptr<Node>, mut val: ()) -> Ptr<Node> {
    let head: Value<Ptr<Node>> = Rc::new(RefCell::new(head));
    let mut curr: Ptr<Node> = (*head.borrow()).clone();
    let mut prev: Ptr<Node> = Ptr::<Node>::null();
    'loop_: while !((curr).is_null()) {
        if ({ curr.with(|__s| __s.val) } == { val }) {
            if !((prev).is_null()) {
                field!(prev, next).write({ curr.with(|__s| __s.next.clone()) });
                return (*head.borrow()).clone();
            } else {
                return curr.with(|__s| __s.next.clone());
            }
        }
        prev = (curr).clone();
        curr = { curr.with(|__s| __s.next.clone()) };
    }
    return (*head.borrow()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let n0: Value<Node> = Rc::new(RefCell::new(Node {
        val: 5_(),
        next: Ptr::<Node>::null(),
    }));
    let mut head: Ptr<Node> = (n0.as_pointer());
    let n1: Value<Node> = Rc::new(RefCell::new(Node {
        val: 4_(),
        next: Ptr::<Node>::null(),
    }));
    let n2: Value<Node> = Rc::new(RefCell::new(Node {
        val: 3_(),
        next: Ptr::<Node>::null(),
    }));
    let n3: Value<Node> = Rc::new(RefCell::new(Node {
        val: 2_(),
        next: Ptr::<Node>::null(),
    }));
    let n4: Value<Node> = Rc::new(RefCell::new(Node {
        val: 1_(),
        next: Ptr::<Node>::null(),
    }));
    let n5: Value<Node> = Rc::new(RefCell::new(Node {
        val: 0_(),
        next: Ptr::<Node>::null(),
    }));
    let n6: Value<Node> = Rc::new(RefCell::new(Node {
        val: -1_(),
        next: Ptr::<Node>::null(),
    }));
    let n7: Value<Node> = Rc::new(RefCell::new(Node {
        val: -2_(),
        next: Ptr::<Node>::null(),
    }));
    ({
        let _head: Ptr<Node> = (head).clone();
        let _new_node: Ptr<Node> = n1.as_pointer();
        Append_1(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (head).clone();
        let _new_node: Ptr<Node> = n2.as_pointer();
        Append_1(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (head).clone();
        let _new_node: Ptr<Node> = n3.as_pointer();
        Append_1(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (head).clone();
        let _new_node: Ptr<Node> = n4.as_pointer();
        Append_1(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (head).clone();
        let _new_node: Ptr<Node> = n5.as_pointer();
        Append_1(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (head).clone();
        let _new_node: Ptr<Node> = n6.as_pointer();
        Append_1(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (head).clone();
        let _new_node: Ptr<Node> = n7.as_pointer();
        Append_1(_head, _new_node)
    });
    let __rhs = ({ Delete_2((head).clone(), 5_()) });
    head = __rhs;
    let __rhs = ({ Delete_2((head).clone(), 0_()) });
    head = __rhs;
    let __rhs = ({ Delete_2((head).clone(), -2_()) });
    head = __rhs;
    unimplemented!();
    return 0_();
}
pub trait NodeImpl {
    fn SetNext(&self, next: Ptr<Node>);
}
impl NodeImpl for Ptr<Node> {
    fn SetNext(&self, mut next: Ptr<Node>) {
        field!((*self), next).write((next).clone());
    }
}
pub fn __cpp2rust_init_globals() {}
