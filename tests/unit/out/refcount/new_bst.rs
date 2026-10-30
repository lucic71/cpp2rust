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
pub struct node_t {
    #[offset(0)]
    #[byte_size(8)]
    pub left: Ptr<node_t>,
    #[offset(8)]
    #[byte_size(8)]
    pub right: Ptr<node_t>,
    #[offset(16)]
    pub value: (),
}
pub fn find_0(node: Ptr<node_t>, mut value: ()) -> Ptr<node_t> {
    let node: Value<Ptr<node_t>> = Rc::new(RefCell::new(node));
    if ({ value } < { (*node.borrow()).with(|__s| __s.value) })
        && (!(((*node.borrow()).with(|__s| __s.left.clone())).is_null()))
    {
        return ({ find_0((*node.borrow()).with(|__s| __s.left.clone()), value) });
    } else if ({ value } > { (*node.borrow()).with(|__s| __s.value) })
        && (!(((*node.borrow()).with(|__s| __s.right.clone())).is_null()))
    {
        return ({ find_0((*node.borrow()).with(|__s| __s.right.clone()), value) });
    } else if ({ value } == { (*node.borrow()).with(|__s| __s.value) }) {
        return (*node.borrow()).clone();
    }
    return Ptr::<node_t>::null();
}
pub fn insert_1(node: Ptr<node_t>, mut value: ()) -> Ptr<node_t> {
    let node: Value<Ptr<node_t>> = Rc::new(RefCell::new(node));
    if (*node.borrow()).is_null() {
        return Ptr::alloc(node_t {
            left: Ptr::<node_t>::null(),
            right: Ptr::<node_t>::null(),
            value: value,
        });
    }
    if ({ value } < { (*node.borrow()).with(|__s| __s.value) }) {
        let __rhs = ({ insert_1((*node.borrow()).with(|__s| __s.left.clone()), value) });
        field!((*node.borrow()), left).write(__rhs);
    } else if ({ value } > { (*node.borrow()).with(|__s| __s.value) }) {
        let __rhs = ({ insert_1((*node.borrow()).with(|__s| __s.right.clone()), value) });
        field!((*node.borrow()), right).write(__rhs);
    }
    return (*node.borrow()).clone();
}
pub fn del_2(mut node: Ptr<node_t>) {
    if !((node.with(|__s| __s.left.clone())).is_null()) {
        ({ del_2(node.with(|__s| __s.left.clone())) });
    }
    if !((node.with(|__s| __s.right.clone())).is_null()) {
        ({ del_2(node.with(|__s| __s.right.clone())) });
    }
    node.delete();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut root: Ptr<node_t> = Ptr::alloc(node_t {
        left: Ptr::<node_t>::null(),
        right: Ptr::<node_t>::null(),
        value: 0_(),
    });
    let __rhs = ({ insert_1((root).clone(), 1_()) });
    root = __rhs;
    let __rhs = ({ insert_1((root).clone(), 2_()) });
    root = __rhs;
    let __rhs = ({ insert_1((root).clone(), 3_()) });
    root = __rhs;
    let __rhs = ({ insert_1((root).clone(), 4_()) });
    root = __rhs;
    let mut out: () = (((((({ find_0((root).clone(), 0_()) }).with(|__s| __s.value) == 0_())
        && (({ find_0((root).clone(), 1_()) }).with(|__s| __s.value) == 1_()))
        && (({ find_0((root).clone(), 2_()) }).with(|__s| __s.value) == 2_()))
        && (({ find_0((root).clone(), 3_()) }).with(|__s| __s.value) == 3_()))
        && (({ find_0((root).clone(), 4_()) }).with(|__s| __s.value) == 4_()))
        && (({ find_0((root).clone(), 5_()) }).is_null());
    ({ del_2((root).clone()) });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
