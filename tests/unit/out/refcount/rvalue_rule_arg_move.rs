extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static copies_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static moves_1: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, Default)]
#[byte_size(4)]
pub struct Elem {
    #[offset(0)]
    pub v: (),
}
impl Elem {
    pub fn new(mut v: ()) -> Self {
        Self { v: v }
    }
    pub fn copy_from(o: Ptr<Elem>) -> Self {
        let __this: Elem = Self {
            v: o.with(|__s| __s.v),
        };
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn move_from(o: Ptr<Elem>) -> Self {
        let __this: Elem = Self {
            v: o.with(|__s| __s.v),
        };
        (*moves_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for Elem {
    fn clone(&self) -> Self {
        let __src: Value<Elem> = Rc::new(RefCell::new(Elem { v: self.v.clone() }));
        Elem::copy_from(__src.as_pointer())
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut v: () = unimplemented!();
    unimplemented!();
    let a: Value<Elem> = Rc::new(RefCell::new(Elem::new({ 1_() })));
    let mut b: Elem = Elem::new({ 2_() });
    unimplemented!();
    unimplemented!();
    let mut ok: () = (((copies_0.with(|rc| *rc.borrow()) == 1_())
        && (moves_1.with(|rc| *rc.borrow()) == 1_()))
        && (__s.v == 1_()))
        && (__s.v == 2_());
    let mut pa: Ptr<Elem> = (a.as_pointer());
    println!(
        "{} {} {}",
        copies_0.with(|rc| *rc.borrow()),
        moves_1.with(|rc| *rc.borrow()),
        pa.with(|__s| __s.v)
    );
    return if ok { 0_() } else { 1_() };
}
pub trait ElemImpl {
    fn copy_assign(&self, _a0: Ptr<Elem>) -> Ptr<Elem>;
}
impl ElemImpl for Ptr<Elem> {
    fn copy_assign(&self, _a0: Ptr<Elem>) -> Ptr<Elem> {
        field!((*self), v).write({ _a0.with(|__s| __s.v) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
    let _ = moves_1.with(|_| ());
}
