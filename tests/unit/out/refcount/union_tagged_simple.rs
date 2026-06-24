extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Kind_enum {
    #[default]
    KIND_NONE = 0,
    KIND_DONE = 1,
}
impl From<i32> for Kind_enum {
    fn from(n: i32) -> Kind_enum {
        match n {
            0 => Kind_enum::KIND_NONE,
            1 => Kind_enum::KIND_DONE,
            _ => panic!("invalid Kind_enum value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(Kind_enum);
#[derive(Clone)]
pub struct anon_0 {
    __store: libcc2rs::UnionStorage,
}
impl anon_0 {
    pub fn obj(&self) -> Ptr<AnyPtr> {
        self.__store.reinterpret(0)
    }
    pub fn code(&self) -> Ptr<i32> {
        self.__store.reinterpret(0)
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __store: libcc2rs::UnionStorage::new(8),
        }
    }
}
impl ByteRepr for anon_0 {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.__store.to_bytes(buf);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        anon_0 {
            __store: libcc2rs::UnionStorage::from_bytes(buf),
        }
    }
}
#[derive(Default)]
pub struct Event {
    pub kind: Value<Kind_enum>,
    pub handle: Value<AnyPtr>,
    pub payload: Value<anon_0>,
}
impl ByteRepr for Event {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let dummy: Value<i32> = Rc::new(RefCell::new(0));
    let m1: Value<Event> = <Value<Event>>::default();
    (*(*m1.borrow()).kind.borrow_mut()) = Kind_enum::KIND_DONE;
    (*(*m1.borrow()).handle.borrow_mut()) = ((dummy.as_pointer()) as Ptr<i32>).to_any();
    (*(*m1.borrow()).payload.borrow_mut()).code().write(42);
    assert!(
        (((((*(*m1.borrow()).kind.borrow()) as u32) == ((Kind_enum::KIND_DONE as i32) as u32))
            as i32)
            != 0)
    );
    assert!((((((*(*m1.borrow()).payload.borrow()).code().read()) == 42) as i32) != 0));
    let m2: Value<Event> = <Value<Event>>::default();
    (*(*m2.borrow()).kind.borrow_mut()) = Kind_enum::KIND_NONE;
    (*(*m2.borrow()).handle.borrow_mut()) = ((dummy.as_pointer()) as Ptr<i32>).to_any();
    (*(*m2.borrow()).payload.borrow_mut())
        .obj()
        .write(((dummy.as_pointer()) as Ptr<i32>).to_any());
    assert!(
        ((({
            let _lhs = ((*(*m2.borrow()).payload.borrow()).obj().read()).clone();
            _lhs == ((dummy.as_pointer()) as Ptr<i32>).to_any()
        }) as i32)
            != 0)
    );
    return 0;
}
