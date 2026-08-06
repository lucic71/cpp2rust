extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive(Clone, Default)]
pub struct handle {
    pub value: i32,
}
impl ByteRepr for handle {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.value.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
pub fn configure_0(h: Ptr<handle>, op: i32, __args: &[VaArg]) -> i32 {
    let h: Value<Ptr<handle>> = Rc::new(RefCell::new(h));
    let op: Value<i32> = Rc::new(RefCell::new(op));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    let rc: Value<i32> = Rc::new(RefCell::new(0));
    (*ap.borrow_mut()) = VaList::new(__args);
    let onoff: Value<i32> = Rc::new(RefCell::new((*ap.borrow_mut()).arg::<i32>()));
    let pOut: Value<Ptr<i32>> = Rc::new(RefCell::new((*ap.borrow_mut()).arg::<Ptr<i32>>()));
    (*h.borrow()).with_mut(|__v| __v.value = (*onoff.borrow()));
    if !(*pOut.borrow()).is_null() {
        {
            let __rhs = (*onoff.borrow());
            (*pOut.borrow()).write(__rhs)
        };
        (*rc.borrow_mut()) = 1;
    };
    return (*rc.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let h: Value<handle> = Rc::new(RefCell::new(handle { value: 0 }));
    assert!(
        (((({ configure_0((h.as_pointer()), 7, &[(1).into(), (0).into(),]) }) == 0) as i32) != 0)
    );
    assert!(((((*h.borrow()).value == 1) as i32) != 0));
    let out: Value<i32> = Rc::new(RefCell::new(-1_i32));
    assert!(
        (((({
            configure_0(
                (h.as_pointer()),
                7,
                &[(5).into(), (out.as_pointer()).into()],
            )
        }) == 1) as i32)
            != 0)
    );
    assert!(((((*out.borrow()) == 5) as i32) != 0));
    assert!(((((*h.borrow()).value == 5) as i32) != 0));
    return 0;
}
