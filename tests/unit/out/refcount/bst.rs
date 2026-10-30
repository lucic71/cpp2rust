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
pub fn insert_1(node: Ptr<node_t>, new_node: Ptr<node_t>) -> Ptr<node_t> {
    let node: Value<Ptr<node_t>> = Rc::new(RefCell::new(node));
    let new_node: Value<Ptr<node_t>> = Rc::new(RefCell::new(new_node));
    if (*node.borrow()).is_null() {
        return (*new_node.borrow()).clone();
    }
    if ({ (*new_node.borrow()).with(|__s| __s.value) } < { (*node.borrow()).with(|__s| __s.value) })
    {
        let __rhs = ({
            insert_1(
                (*node.borrow()).with(|__s| __s.left.clone()),
                (*new_node.borrow()).clone(),
            )
        });
        field!((*node.borrow()), left).write(__rhs);
    } else if ({ (*new_node.borrow()).with(|__s| __s.value) } > {
        (*node.borrow()).with(|__s| __s.value)
    }) {
        let __rhs = ({
            insert_1(
                (*node.borrow()).with(|__s| __s.right.clone()),
                (*new_node.borrow()).clone(),
            )
        });
        field!((*node.borrow()), right).write(__rhs);
    }
    return (*node.borrow()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut tree: () = unimplemented!();
    let mut n1: () = unimplemented!();
    let mut n2: () = unimplemented!();
    let mut n3: () = unimplemented!();
    let mut n4: () = unimplemented!();
    let mut ptr1: Ptr<node_t> = (unimplemented!());
    let __rhs = ({ insert_1((ptr1).clone(), (unimplemented!())) });
    ptr1 = __rhs;
    let __rhs = ({ insert_1((ptr1).clone(), (unimplemented!())) });
    ptr1 = __rhs;
    let __rhs = ({ insert_1((ptr1).clone(), (unimplemented!())) });
    ptr1 = __rhs;
    let __rhs = ({ insert_1((ptr1).clone(), (unimplemented!())) });
    ptr1 = __rhs;
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
