extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct node {
    #[offset(0)]
    pub value: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub next: Ptr<node>,
}
impl node {
    pub fn copy_from(this: Ptr<node>, _a0: Ptr<node>) {
        field!(this, value).write(_a0.with(|__s| __s.value));
        field!(this, next).write(_a0.with(|__s| __s.next.clone()));
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct pair_t {
    #[offset(0)]
    pub a: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub n: Ptr<node>,
}
impl pair_t {
    pub fn new_1(this: Ptr<pair_t>) {}
    pub fn copy_from(this: Ptr<pair_t>, _a0: Ptr<pair_t>) {
        field!(this, a).write(_a0.with(|__s| __s.a));
        field!(this, n).write(_a0.with(|__s| __s.n.clone()));
    }
    pub fn move_from(this: Ptr<pair_t>, _a0: Ptr<pair_t>) {
        field!(this, a).write({ (*_a0.upgrade().deref()).a });
        field!(this, n).write({ (*_a0.upgrade().deref()).n.clone() });
    }
}
thread_local!(
    pub static global_node_0: Value<Ptr<node>> = Rc::new(RefCell::new(Ptr::<node>::null()));
);
pub fn id_1(mut n: Ptr<node>) -> Ptr<node> {
    return n;
}
pub fn pick_2(mut a: Ptr<node>, mut b: Ptr<node>) -> Ptr<node> {
    return if !(a).is_null() {
        (a).clone()
    } else {
        (b).clone()
    };
}
pub fn local_ptr_3(mut n: Ptr<node>) -> Ptr<node> {
    let mut p: Ptr<node> = n.with(|__s| __s.next.clone());
    return p;
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
pub fn ret_struct_9(__ret: Ptr<pair_t>, mut a: i32, mut n: Ptr<node>) {
    let p: Value<pair_t> = Rc::new(RefCell::new(<pair_t>::uninit()));
    pair_t::new_1(p.as_pointer());
    (*p.borrow_mut()).a = a;
    (*p.borrow_mut()).n = (n).clone();
    pair_t::move_from(__ret.clone(), { p.as_pointer() });
    return;
}
pub fn ret_struct_param_10(__ret: Ptr<pair_t>, p: Value<pair_t>) {
    pair_t::move_from(__ret.clone(), { p.as_pointer() });
    return;
}
pub fn ret_struct_ref_11(__ret: Ptr<pair_t>, p: Ptr<pair_t>) {
    pair_t::copy_from(__ret.clone(), { (p).clone() });
    return;
}
pub fn ret_vec_12(mut v: Vec<i32>) -> Vec<i32> {
    return std::mem::take(&mut v);
}
pub fn ret_vec_local_13() -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    {
        let __a1 = 1;
        v.push(__a1)
    };
    return std::mem::take(&mut v);
}
pub fn ret_loop_14(mut n: Ptr<node>) -> Ptr<node> {
    'loop_: while !(n.with(|__s| __s.next.clone())).is_null() {
        if (n.with(|__s| __s.value) == 2) {
            return n;
        }
        n = { n.with(|__s| __s.next.clone()) };
    }
    return n;
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<node> = Rc::new(RefCell::new(<node>::uninit()));
    {
        let __d = c.as_pointer();
        field!(__d, value).write(3);
        field!(__d, next).write(Ptr::<node>::null());
    };
    let b: Value<node> = Rc::new(RefCell::new(<node>::uninit()));
    {
        let __d = b.as_pointer();
        field!(__d, value).write(2);
        field!(__d, next).write((c.as_pointer()));
    };
    let a: Value<node> = Rc::new(RefCell::new(<node>::uninit()));
    {
        let __d = a.as_pointer();
        field!(__d, value).write(1);
        field!(__d, next).write((b.as_pointer()));
    };
    global_node_0.with(|rc| *rc.borrow_mut() = (a.as_pointer()));
    assert!((({ id_1((a.as_pointer()),) }) == (a.as_pointer())));
    assert!((({ local_ptr_3((a.as_pointer()),) }) == (b.as_pointer())));
    assert!((({ call_once_4(Ptr::<node>::null(), (b.as_pointer()),) }) == (b.as_pointer())));
    assert!((({ call_twice_5((c.as_pointer()),) }) == (c.as_pointer())));
    assert!((({ next_of_6((b.as_pointer()),) }) == (c.as_pointer())));
    assert!((({ ret_global_7() }) == (a.as_pointer())));
    assert!((({ address_taken_8((a.as_pointer()),) }) == (a.as_pointer())));
    let p: Value<pair_t> = Rc::new(RefCell::new(<pair_t>::uninit()));
    ({ ret_struct_9(p.as_pointer(), 4, (a.as_pointer())) });
    assert!(
        ({
            ({ (*p.borrow()).a } == 4) && ({ { (*p.borrow()).n.clone() } } == { (a.as_pointer()) })
        })
    );
    let q: Value<pair_t> = Rc::new(RefCell::new(<pair_t>::uninit()));
    ({
        ret_struct_param_10(q.as_pointer(), {
            let __v: Value<pair_t> = Rc::new(RefCell::new(<pair_t>::uninit()));
            pair_t::copy_from(__v.as_pointer(), { p.as_pointer() });
            __v
        })
    });
    assert!(
        ({
            ({ (*q.borrow()).a } == 4) && ({ { (*q.borrow()).n.clone() } } == { (a.as_pointer()) })
        })
    );
    let r: Value<pair_t> = Rc::new(RefCell::new(<pair_t>::uninit()));
    ({ ret_struct_ref_11(r.as_pointer(), q.as_pointer()) });
    assert!(
        ({
            ({ (*r.borrow()).a } == 4) && ({ { (*r.borrow()).n.clone() } } == { (a.as_pointer()) })
        })
    );
    assert!((({ ret_vec_12(({ ret_vec_local_13() }),) }).len() == 1_usize));
    assert!((({ ret_loop_14((a.as_pointer()),) }) == (b.as_pointer())));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = global_node_0.with(|_| ());
}
