extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct Node {
    #[offset(0)]
    pub val: (),
    #[offset(8)]
    #[byte_size(8)]
    pub next: Ptr<Node>,
    #[offset(16)]
    #[byte_size(8)]
    pub prev: Ptr<Node>,
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
pub fn FindBack_1(mut tail: Ptr<Node>, mut idx: ()) -> Ptr<Node> {
    let curr: Value<Ptr<Node>> = Rc::new(RefCell::new((tail).clone()));
    let mut i: () = 0_();
    'loop_: while (i < idx) {
        (*curr.borrow_mut()) = { (*curr.borrow()).with(|__s| __s.prev.clone()) };
        i.postfix_inc();
    }
    return (*curr.borrow()).clone();
}
pub fn Append_2(head: Ptr<Node>, new_node: Ptr<Node>) {
    let mut curr: Ptr<Node> = (head).clone();
    'loop_: while !((curr.with(|__s| __s.next.clone())).is_null()) {
        curr = { curr.with(|__s| __s.next.clone()) };
    }
    ({ NodeImpl::SetNext(&curr, (new_node).clone()) });
    ({
        let _p: Ptr<Node> = (curr).clone();
        NodeImpl::SetPrev(&new_node, _p)
    });
}
pub fn Delete_3(head: Ptr<Node>, mut val: ()) -> Ptr<Node> {
    let head: Value<Ptr<Node>> = Rc::new(RefCell::new(head));
    let mut curr: Ptr<Node> = (*head.borrow()).clone();
    'loop_: while !((curr).is_null()) {
        if ({ curr.with(|__s| __s.val) } == { val }) {
            let mut prev: Ptr<Node> = curr.with(|__s| __s.prev.clone());
            let next: Value<Ptr<Node>> = Rc::new(RefCell::new(curr.with(|__s| __s.next.clone())));
            if !((prev).is_null()) {
                field!(prev, next).write((*next.borrow()).clone());
            }
            if !((*next.borrow()).is_null()) {
                field!((*next.borrow()), prev).write((prev).clone());
            }
            if !((prev).is_null()) {
                return (*head.borrow()).clone();
            } else {
                return (*next.borrow()).clone();
            }
        }
        curr = { curr.with(|__s| __s.next.clone()) };
    }
    return (*head.borrow()).clone();
}
pub fn Tail_4(mut head: Ptr<Node>) -> Ptr<Node> {
    let curr: Value<Ptr<Node>> = Rc::new(RefCell::new((head).clone()));
    'loop_: while !(((*curr.borrow()).with(|__s| __s.next.clone())).is_null()) {
        (*curr.borrow_mut()) = { (*curr.borrow()).with(|__s| __s.next.clone()) };
    }
    return (*curr.borrow()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let n0: Value<Node> = Rc::new(RefCell::new(Node {
        val: 5_(),
        next: Ptr::<Node>::null(),
        prev: Ptr::<Node>::null(),
    }));
    let mut head: Ptr<Node> = (n0.as_pointer());
    let n1: Value<Node> = Rc::new(RefCell::new(Node {
        val: 4_(),
        next: Ptr::<Node>::null(),
        prev: Ptr::<Node>::null(),
    }));
    let n2: Value<Node> = Rc::new(RefCell::new(Node {
        val: 3_(),
        next: Ptr::<Node>::null(),
        prev: Ptr::<Node>::null(),
    }));
    let n3: Value<Node> = Rc::new(RefCell::new(Node {
        val: 2_(),
        next: Ptr::<Node>::null(),
        prev: Ptr::<Node>::null(),
    }));
    let n4: Value<Node> = Rc::new(RefCell::new(Node {
        val: 1_(),
        next: Ptr::<Node>::null(),
        prev: Ptr::<Node>::null(),
    }));
    let n5: Value<Node> = Rc::new(RefCell::new(Node {
        val: 0_(),
        next: Ptr::<Node>::null(),
        prev: Ptr::<Node>::null(),
    }));
    let n6: Value<Node> = Rc::new(RefCell::new(Node {
        val: -1_(),
        next: Ptr::<Node>::null(),
        prev: Ptr::<Node>::null(),
    }));
    let n7: Value<Node> = Rc::new(RefCell::new(Node {
        val: -2_(),
        next: Ptr::<Node>::null(),
        prev: Ptr::<Node>::null(),
    }));
    ({
        let _head: Ptr<Node> = (head).clone();
        let _new_node: Ptr<Node> = n1.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (head).clone();
        let _new_node: Ptr<Node> = n2.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (head).clone();
        let _new_node: Ptr<Node> = n3.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (head).clone();
        let _new_node: Ptr<Node> = n4.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (head).clone();
        let _new_node: Ptr<Node> = n5.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (head).clone();
        let _new_node: Ptr<Node> = n6.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (head).clone();
        let _new_node: Ptr<Node> = n7.as_pointer();
        Append_2(_head, _new_node)
    });
    let __rhs = ({ Delete_3((head).clone(), 5_()) });
    head = __rhs;
    let __rhs = ({ Delete_3((head).clone(), 0_()) });
    head = __rhs;
    let __rhs = ({ Delete_3((head).clone(), -2_()) });
    head = __rhs;
    let mut tail: Ptr<Node> = ({ Tail_4((head).clone()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    field!(
        ({ Find_0((head).clone(), 0_(),) }).with(|__s| __s.next.clone()),
        val
    )
    .write(30_());
    unimplemented!();
    let __rhs = (({ Find_0((head).clone(), 0_()) }).with(|__s| __s.val)
        + ({ Find_0((head).clone(), 3_()) }).with(|__s| __s.val));
    field!(
        ({ Find_0((head).clone(), 1_(),) }).with(|__s| __s.next.clone()),
        val
    )
    .write(__rhs);
    unimplemented!();
    let mut sum: () = ((((({ Find_0((head).clone(), 0_()) }).with(|__s| __s.val)
        + ({ Find_0((head).clone(), 1_()) }).with(|__s| __s.val))
        + ({ Find_0((head).clone(), 2_()) }).with(|__s| __s.val))
        + ({ Find_0((head).clone(), 3_()) }).with(|__s| __s.val))
        + ({ Find_0((head).clone(), 4_()) }).with(|__s| __s.val));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait NodeImpl {
    fn SetNext(&self, n: Ptr<Node>);
    fn SetPrev(&self, p: Ptr<Node>);
}
impl NodeImpl for Ptr<Node> {
    fn SetNext(&self, mut n: Ptr<Node>) {
        field!((*self), next).write((n).clone());
    }
    fn SetPrev(&self, mut p: Ptr<Node>) {
        field!((*self), prev).write((p).clone());
    }
}
pub fn __cpp2rust_init_globals() {}
