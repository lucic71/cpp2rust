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
pub struct node {
    #[offset(0)]
    pub value: (),
    #[offset(8)]
    #[byte_size(8)]
    pub next: Ptr<node>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct pair_t {
    #[offset(0)]
    pub a: (),
    #[offset(8)]
    #[byte_size(8)]
    pub n: Ptr<node>,
}
thread_local!(
    pub static global_node_0: Value<Ptr<node>> = Rc::new(RefCell::new(Ptr::<node>::null()));
);
pub fn id_1(n: Ptr<node>) -> Ptr<node> {
    let n: Value<Ptr<node>> = Rc::new(RefCell::new(n));
    return (*n.borrow()).clone();
}
pub fn pick_2(mut a: Ptr<node>, mut b: Ptr<node>) -> Ptr<node> {
    return if !(a).is_null() {
        (a).clone()
    } else {
        (b).clone()
    };
}
pub fn local_ptr_3(mut n: Ptr<node>) -> Ptr<node> {
    let p: Value<Ptr<node>> = Rc::new(RefCell::new(n.with(|__s| __s.next.clone())));
    return (*p.borrow()).clone();
}
pub fn call_once_4(mut n: Ptr<node>, mut m: Ptr<node>) -> Ptr<node> {
    return ({ pick_2(n, m) });
}
pub fn call_twice_5(mut n: Ptr<node>) -> Ptr<node> {
    return ({
        let _a: Ptr<node> = (n).clone();
        let _b: Ptr<node> = (n).clone();
        pick_2(_a, _b)
    });
}
pub fn next_of_6(mut n: Ptr<node>) -> Ptr<node> {
    return n.with(|__s| __s.next.clone());
}
pub fn ret_global_7() -> Ptr<node> {
    return (*global_node_0.with(Value::clone).borrow()).clone();
}
pub fn address_taken_8(n: Ptr<node>) -> Ptr<node> {
    let n: Value<Ptr<node>> = Rc::new(RefCell::new(n));
    let mut pp: Ptr<Ptr<node>> = (n.as_pointer());
    return if !(pp.read()).is_null() {
        (*n.borrow()).clone()
    } else {
        Ptr::<node>::null()
    };
}
pub fn ret_struct_9(mut a: (), mut n: Ptr<node>) -> pair_t {
    let mut p: pair_t = <pair_t>::default();
    p.a = a;
    p.n = (n).clone();
    return p;
}
pub fn ret_struct_param_10(mut p: pair_t) -> pair_t {
    return p;
}
pub fn ret_struct_ref_11(p: Ptr<pair_t>) -> pair_t {
    return (*p.upgrade().deref()).clone();
}
pub fn ret_vec_12(mut v: ()) -> () {
    return unimplemented!();
}
pub fn ret_vec_local_13() -> () {
    let mut v: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn ret_loop_14(n: Ptr<node>) -> Ptr<node> {
    let n: Value<Ptr<node>> = Rc::new(RefCell::new(n));
    'loop_: while !((*n.borrow()).with(|__s| __s.next.clone())).is_null() {
        if ((*n.borrow()).with(|__s| __s.value) == 2_()) {
            return (*n.borrow()).clone();
        }
        (*n.borrow_mut()) = { (*n.borrow()).with(|__s| __s.next.clone()) };
    }
    return (*n.borrow()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let c: Value<node> = Rc::new(RefCell::new(node {
        value: 3_(),
        next: Ptr::<node>::null(),
    }));
    let b: Value<node> = Rc::new(RefCell::new(node {
        value: 2_(),
        next: (c.as_pointer()),
    }));
    let a: Value<node> = Rc::new(RefCell::new(node {
        value: 1_(),
        next: (b.as_pointer()),
    }));
    global_node_0.with(|rc| *rc.borrow_mut() = (a.as_pointer()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: pair_t = ({ ret_struct_9(4_(), (a.as_pointer())) });
    unimplemented!();
    let q: Value<pair_t> = Rc::new(RefCell::new(({ ret_struct_param_10((p).clone()) })));
    unimplemented!();
    let mut r: pair_t = ({ ret_struct_ref_11(q.as_pointer()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = global_node_0.with(|_| ());
}
