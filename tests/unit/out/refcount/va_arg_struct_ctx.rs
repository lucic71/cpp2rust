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
pub struct context {
    pub verbose: i32,
    pub last_error: i32,
}
impl ByteRepr for context {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.verbose.to_bytes(&mut buf[0..4]);
        self.last_error.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            verbose: <i32>::from_bytes(&buf[0..4]),
            last_error: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
pub fn set_error_0(ctx: Ptr<context>, fmt: Ptr<u8>, __args: &[VaArg]) {
    let ctx: Value<Ptr<context>> = Rc::new(RefCell::new(ctx));
    let fmt: Value<Ptr<u8>> = Rc::new(RefCell::new(fmt));
    if (*ctx.borrow()).with(|__v| __v.verbose) != 0 {
        let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
        (*ap.borrow_mut()) = VaList::new(__args);
        {
            let __rhs = (*ap.borrow_mut()).arg::<i32>();
            (*ctx.borrow()).with_mut(|__v| __v.last_error = __rhs)
        };
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let ctx: Value<context> = <Value<context>>::default();
    (*ctx.borrow_mut()).verbose = 1;
    (*ctx.borrow_mut()).last_error = 0;
    ({
        set_error_0(
            (ctx.as_pointer()),
            Ptr::from_string_literal(b"error %d\0"),
            &[(42).into()],
        )
    });
    assert!(((((*ctx.borrow()).last_error == 42) as i32) != 0));
    (*ctx.borrow_mut()).verbose = 0;
    ({
        set_error_0(
            (ctx.as_pointer()),
            Ptr::from_string_literal(b"error %d\0"),
            &[(99).into()],
        )
    });
    assert!(((((*ctx.borrow()).last_error == 42) as i32) != 0));
    return 0;
}
