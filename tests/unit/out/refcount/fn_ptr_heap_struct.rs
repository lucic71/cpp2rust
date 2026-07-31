extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static total_0: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn bump_1(by: i32) {
    let by: Value<i32> = Rc::new(RefCell::new(by));
    (*total_0.with(Value::clone).borrow_mut()) += (*by.borrow());
}
pub fn reset_2(ignored: i32) {
    let ignored: Value<i32> = Rc::new(RefCell::new(ignored));
    (*ignored.borrow());
    (*total_0.with(Value::clone).borrow_mut()) = 0;
}
#[derive(Clone)]
pub struct handlers {
    pub cb: FnPtr<fn(i32)>,
    pub n: i32,
}
impl Default for handlers {
    fn default() -> Self {
        handlers {
            cb: FnPtr::<fn(i32)>::null(),
            n: <i32>::default(),
        }
    }
}
impl ByteRepr for handlers {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.cb.to_bytes(&mut buf[0..8]);
        self.n.to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            cb: <FnPtr<fn(i32)>>::from_bytes(&buf[0..8]),
            n: <i32>::from_bytes(&buf[8..12]),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let h: Value<Ptr<handlers>> = Rc::new(RefCell::new(
        libcc2rs::calloc_refcount(1_usize, 16usize).reinterpret_cast::<handlers>(),
    ));
    assert!(!(*h.borrow()).is_null());
    assert!((((((*h.borrow()).with(|__v| (*__v).cb.clone())).is_null()) as i32) != 0));
    (*h.borrow()).with_mut(|__v| __v.cb = FnPtr::<fn(i32)>::new(bump_1));
    (*h.borrow()).with_mut(|__v| __v.n = 7);
    assert!(
        ((({
            let _lhs = ((*h.borrow()).with(|__v| (*__v).cb.clone())).clone();
            _lhs == FnPtr::<fn(i32)>::new(bump_1)
        }) as i32)
            != 0)
    );
    assert!(
        ((({
            let _lhs = ((*h.borrow()).with(|__v| (*__v).cb.clone())).clone();
            _lhs != FnPtr::<fn(i32)>::new(reset_2)
        }) as i32)
            != 0)
    );
    ({ (*(*h.borrow()).with(|__v| (*__v).cb.clone()))(3) });
    assert!(((((*total_0.with(Value::clone).borrow()) == 3) as i32) != 0));
    ({ (*(*h.borrow()).with(|__v| (*__v).cb.clone()))(4) });
    assert!(((((*total_0.with(Value::clone).borrow()) == 7) as i32) != 0));
    (*h.borrow()).with_mut(|__v| __v.cb = FnPtr::<fn(i32)>::new(reset_2));
    ({ (*(*h.borrow()).with(|__v| (*__v).cb.clone()))(0) });
    assert!(((((*total_0.with(Value::clone).borrow()) == 0) as i32) != 0));
    assert!(((((*h.borrow()).with(|__v| (*__v).n) == 7) as i32) != 0));
    (*h.borrow()).with_mut(|__v| __v.cb = FnPtr::<fn(i32)>::null());
    assert!((((((*h.borrow()).with(|__v| (*__v).cb.clone())).is_null()) as i32) != 0));
    libcc2rs::free_refcount(((*h.borrow()).clone() as Ptr<handlers>).to_any());
    return 0;
}
