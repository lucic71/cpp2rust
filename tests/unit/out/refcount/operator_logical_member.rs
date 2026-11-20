extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: i32,
}
impl S {
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let t: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = t.as_pointer();
        field!(__d, v).write(1);
    };
    let f: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = f.as_pointer();
        field!(__d, v).write(0);
    };
    assert!(({ SImpl::operator_not(&f.as_pointer(),) }));
    assert!(!({ SImpl::operator_not(&t.as_pointer(),) }));
    assert!(
        ({
            let _o: Ptr<S> = t.as_pointer();
            SImpl::operator_and(&t.as_pointer(), _o)
        })
    );
    assert!(!({ SImpl::operator_and(&t.as_pointer(), f.as_pointer(),) }));
    assert!(({ SImpl::operator_or(&t.as_pointer(), f.as_pointer(),) }));
    assert!(
        !({
            let _o: Ptr<S> = f.as_pointer();
            SImpl::operator_or(&f.as_pointer(), _o)
        })
    );
    return 0;
}
pub trait SImpl {
    fn operator_not(&self) -> bool;
    fn operator_and(&self, o: Ptr<S>) -> bool;
    fn operator_or(&self, o: Ptr<S>) -> bool;
}
impl SImpl for Ptr<S> {
    fn operator_not(&self) -> bool {
        return ((*self).with(|__s| __s.v) == 0);
    }
    fn operator_and(&self, o: Ptr<S>) -> bool {
        return ((*self).with(|__s| __s.v) != 0) && (o.with(|__s| __s.v) != 0);
    }
    fn operator_or(&self, o: Ptr<S>) -> bool {
        return ((*self).with(|__s| __s.v) != 0) || (o.with(|__s| __s.v) != 0);
    }
}
pub fn __cpp2rust_init_globals() {}
