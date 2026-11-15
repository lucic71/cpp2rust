extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct HasCounter {
    #[offset(0)]
    pub count: i32,
}
impl Default for HasCounter {
    fn default() -> Self {
        { HasCounter { count: 0 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct DerivesDirectly {
    #[offset(4)]
    pub own: i32,
}
impl Default for DerivesDirectly {
    fn default() -> Self {
        { DerivesDirectly { own: 0 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct DerivesIndirectly {}
pub fn incrementThroughPointer_0(mut p: Ptr<HasCounter>) {
    ({ HasCounterImpl::increment(&p) });
}
pub fn incrementThroughReference_1(r: Ptr<HasCounter>) {
    ({ HasCounterImpl::increment(&r) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<DerivesDirectly> = Rc::new(RefCell::new(<DerivesDirectly>::default()));
    ({ incrementThroughPointer_0((d.as_pointer())) });
    assert!(({ ({ ((*d.borrow()) as HasCounter).count } == 1) }));
    ({ incrementThroughReference_1(d.as_pointer()) });
    assert!(({ ({ ((*d.borrow()) as HasCounter).count } == 2) }));
    assert!(({ ({ (*d.borrow()).own } == 0) }));
    let mut p: Ptr<HasCounter> = (d.as_pointer());
    let r: Ptr<HasCounter> = d.as_pointer();
    assert!(({ (p).clone() } == { (r).clone() }));
    ({ HasCounterImpl::increment(&p) });
    assert!(({ ({ ((*d.borrow()) as HasCounter).count } == 3) }));
    ({ HasCounterImpl::increment(&r) });
    assert!(({ ({ ((*d.borrow()) as HasCounter).count } == 4) }));
    ({ HasCounterImpl::increment(&(d.as_pointer() as HasCounter)) });
    assert!((p.with(|__s| __s.count) == 5));
    assert!((r.with(|__s| __s.count) == 5));
    let i: Value<DerivesIndirectly> = Rc::new(RefCell::new(<DerivesIndirectly>::default()));
    ({ incrementThroughPointer_0((i.as_pointer())) });
    assert!(({ ({ ((*i.borrow()) as HasCounter).count } == 1) }));
    ({ incrementThroughReference_1(i.as_pointer()) });
    assert!(({ ({ ((*i.borrow()) as HasCounter).count } == 2) }));
    let mut asDirectBase: Ptr<DerivesDirectly> = (i.as_pointer());
    ({ HasCounterImpl::increment(&(asDirectBase as *mut HasCounter)) });
    assert!(({ ({ ((*i.borrow()) as HasCounter).count } == 3) }));
    assert!(({ ({ ((*d.borrow()) as HasCounter).count } == 5) }));
    return 0;
}
pub trait HasCounterImpl {
    fn increment(&self);
}
impl HasCounterImpl for Ptr<HasCounter> {
    fn increment(&self) {
        field!((*self), count).with_mut(|__v| __v.prefix_inc());
    }
}
pub fn __cpp2rust_init_globals() {}
