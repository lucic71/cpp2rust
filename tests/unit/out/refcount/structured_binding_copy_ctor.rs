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
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(8)]
pub struct Counted {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl Counted {
    pub fn new(this: Ptr<Counted>, mut x: i32, mut y: i32) {
        field!(this, x).write(x);
        field!(this, y).write(y);
    }
    pub fn copy_from(this: Ptr<Counted>, other: Ptr<Counted>) {
        field!(this, x).write((other.with(|__s| __s.x) * 2));
        field!(this, y).write((other.with(|__s| __s.y) * 2));
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
impl Clone for Counted {
    fn clone(&self) -> Self {
        let __src: Value<Counted> = Rc::new(RefCell::new(Counted {
            x: self.x.clone(),
            y: self.y.clone(),
        }));
        let __dst: Value<Counted> = Rc::new(RefCell::new(<Counted>::uninit()));
        Counted::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
pub fn make_counted_1(__ret: Ptr<Counted>) {
    Counted::new(__ret.clone(), { 5 }, { 6 });
    return;
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Counted> = Rc::new(RefCell::new(<Counted>::uninit()));
    Counted::new(s.as_pointer(), { 1 }, { 2 });
    let __decomp_2: Value<Counted> = {
        let __v: Value<Counted> = Rc::new(RefCell::new(<Counted>::uninit()));
        Counted::copy_from(__v.as_pointer(), { s.as_pointer() });
        __v
    };
    assert!((copies_0.with(|rc| *rc.borrow()) == 1));
    assert!(({ ({ (*__decomp_2.borrow()).x } == 2) }));
    assert!(({ ({ (*__decomp_2.borrow()).y } == 4) }));
    let __decomp_3: Ptr<Counted> = s.as_pointer();
    assert!((copies_0.with(|rc| *rc.borrow()) == 1));
    assert!((__decomp_3.with(|__s| __s.x) == 1));
    assert!((__decomp_3.with(|__s| __s.y) == 2));
    let __decomp_4: Ptr<Counted> = s.as_pointer();
    assert!((copies_0.with(|rc| *rc.borrow()) == 1));
    assert!((__decomp_4.with(|__s| __s.x) == 1));
    let __decomp_5: Value<Counted> = {
        let __v: Value<Counted> = Rc::new(RefCell::new(<Counted>::uninit()));
        ({ make_counted_1(__v.as_pointer()) });
        __v
    };
    assert!((copies_0.with(|rc| *rc.borrow()) == 1));
    assert!(({ ({ (*__decomp_5.borrow()).x } == 5) }));
    assert!(({ ({ (*__decomp_5.borrow()).y } == 6) }));
    return 0;
}
pub trait CountedImpl {
    fn copy_assign(&self, _a0: Ptr<Counted>) -> Ptr<Counted>;
}
impl CountedImpl for Ptr<Counted> {
    fn copy_assign(&self, _a0: Ptr<Counted>) -> Ptr<Counted> {
        field!((*self), x).write({ _a0.with(|__s| __s.x) });
        field!((*self), y).write({ _a0.with(|__s| __s.y) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
}
