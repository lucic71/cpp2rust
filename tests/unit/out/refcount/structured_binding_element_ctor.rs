extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static copies_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static moves_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Elem {
    #[offset(0)]
    pub v: i32,
}
impl Elem {
    pub fn new(this: Ptr<Elem>, mut v: i32) {
        field!(this, v).write(v);
    }
    pub fn copy_from(this: Ptr<Elem>, other: Ptr<Elem>) {
        field!(this, v).write((other.with(|__s| __s.v) + 100));
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
    pub fn move_from(this: Ptr<Elem>, other: Ptr<Elem>) {
        field!(this, v).write((other.with(|__s| __s.v) + 1000));
        field!(other, v).write(-1_i32);
        (*moves_1.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
impl Clone for Elem {
    fn clone(&self) -> Self {
        let __src: Value<Elem> = Rc::new(RefCell::new(Elem { v: self.v.clone() }));
        let __dst: Value<Elem> = Rc::new(RefCell::new(<Elem>::uninit()));
        Elem::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(8)]
pub struct Two {
    #[offset(0)]
    #[byte_size(4)]
    pub a: Elem,
    #[offset(4)]
    #[byte_size(4)]
    pub b: Elem,
}
impl Two {
    pub fn copy_from(this: Ptr<Two>, _a0: Ptr<Two>) {
        Elem::copy_from(field_ptr!(this, a), { field_ptr!(_a0, a) });
        Elem::copy_from(field_ptr!(this, b), { field_ptr!(_a0, b) });
    }
    pub fn move_from(this: Ptr<Two>, _a0: Ptr<Two>) {
        Elem::move_from(field_ptr!(this, a), { field_ptr!(_a0, a) });
        Elem::move_from(field_ptr!(this, b), { field_ptr!(_a0, b) });
    }
}
pub fn make_two_2(__ret: Ptr<Two>) {
    {
        let __d = __ret.clone();
        Elem::new(field_ptr!(__d, a), { 1 });
        Elem::new(field_ptr!(__d, b), { 2 });
    };
    return;
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let t: Value<Two> = Rc::new(RefCell::new(<Two>::uninit()));
    {
        let __d = t.as_pointer();
        Elem::new(field_ptr!(__d, a), { 1 });
        Elem::new(field_ptr!(__d, b), { 2 });
    };
    let mut base_copies: i32 = copies_0.with(|rc| *rc.borrow());
    let mut base_moves: i32 = moves_1.with(|rc| *rc.borrow());
    let __decomp_3: Value<Two> = Rc::new(RefCell::new(<Two>::uninit()));
    Two::copy_from(__decomp_3.as_pointer(), { t.as_pointer() });
    assert!((copies_0.with(|rc| *rc.borrow()) == (base_copies + 2)));
    assert!((moves_1.with(|rc| *rc.borrow()) == base_moves));
    assert!(({ ({ (*__decomp_3.borrow()).a.v } == 101) }));
    assert!(({ ({ (*__decomp_3.borrow()).b.v } == 102) }));
    let __decomp_4: Ptr<Two> = t.as_pointer();
    assert!((copies_0.with(|rc| *rc.borrow()) == (base_copies + 2)));
    assert!(({ ({ (*__decomp_4.upgrade().deref()).a.v } == 1) }));
    field!(field!(__decomp_4, a), v).write(50);
    assert!(({ ({ (*t.borrow()).a.v } == 50) }));
    let __decomp_5: Value<Two> = Rc::new(RefCell::new(<Two>::uninit()));
    Two::move_from(__decomp_5.as_pointer(), { t.as_pointer() });
    assert!((moves_1.with(|rc| *rc.borrow()) == (base_moves + 2)));
    assert!(({ ({ (*__decomp_5.borrow()).a.v } == 1050) }));
    assert!(({ ({ (*__decomp_5.borrow()).b.v } == 1002) }));
    assert!(({ ({ (*t.borrow()).a.v } == -1_i32) }));
    let mut before_copies: i32 = copies_0.with(|rc| *rc.borrow());
    let mut before_moves: i32 = moves_1.with(|rc| *rc.borrow());
    let __decomp_6: Value<Two> = Rc::new(RefCell::new(<Two>::uninit()));
    ({ make_two_2(__decomp_6.as_pointer()) });
    assert!((copies_0.with(|rc| *rc.borrow()) == before_copies));
    assert!((moves_1.with(|rc| *rc.borrow()) == before_moves));
    assert!(({ ({ (*__decomp_6.borrow()).a.v } == 1) }));
    let p: Value<(Value<Elem>, Value<Elem>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(
            Rc::try_unwrap({
                let __v: Value<Elem> = Rc::new(RefCell::new(<Elem>::uninit()));
                Elem::new(__v.as_pointer(), { 7 });
                __v
            })
            .ok()
            .unwrap()
            .into_inner(),
        )),
        Rc::new(RefCell::new(
            Rc::try_unwrap({
                let __v: Value<Elem> = Rc::new(RefCell::new(<Elem>::uninit()));
                Elem::new(__v.as_pointer(), { 8 });
                __v
            })
            .ok()
            .unwrap()
            .into_inner(),
        )),
    )));
    before_copies = copies_0.with(|rc| *rc.borrow());
    let __decomp_7: Value<(Value<Elem>, Value<Elem>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new((*p.borrow()).0.borrow().clone())),
        Rc::new(RefCell::new((*p.borrow()).1.borrow().clone())),
    )));
    let x: Ptr<Elem> = ({ (*__decomp_7.borrow()).get_value::<0>().as_pointer() });
    let y: Ptr<Elem> = ({ (*__decomp_7.borrow()).get_value::<1>().as_pointer() });
    assert!((copies_0.with(|rc| *rc.borrow()) == (before_copies + 2)));
    assert!(({ ({ (*x.upgrade().deref()).v } == ({ (*(*p.borrow()).0.borrow()).v } + 100)) }));
    assert!(({ ({ (*y.upgrade().deref()).v } == ({ (*(*p.borrow()).1.borrow()).v } + 100)) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
    let _ = moves_1.with(|_| ());
}
