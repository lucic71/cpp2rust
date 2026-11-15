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
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<DerivesDirectly> = Rc::new(RefCell::new(<DerivesDirectly>::default()));
    ({ HasCounterImpl::increment(&(d.as_pointer() as HasCounter)) });
    assert!(({ ({ ((*d.borrow()) as HasCounter).count } == 1) && ({ (*d.borrow()).own } == 0) }));
    ({ DerivesDirectlyImpl::incrementBoth(&d.as_pointer()) });
    assert!(({ ({ ((*d.borrow()) as HasCounter).count } == 2) && ({ (*d.borrow()).own } == 1) }));
    ((*d.borrow_mut()) as HasCounter).count += 5;
    assert!(({ ({ ((*d.borrow()) as HasCounter).count } == 7) }));
    ({ HasCounterImpl::increment(&(d.as_pointer() as HasCounter)) });
    assert!(({ ({ ((*d.borrow()) as HasCounter).count } == 8) }));
    let i: Value<DerivesIndirectly> = Rc::new(RefCell::new(<DerivesIndirectly>::default()));
    ({ DerivesIndirectlyImpl::incrementTwice(&i.as_pointer()) });
    assert!(
        ({
            ({ ((*i.borrow()) as HasCounter).count } == 2)
                && ({ ((*i.borrow()) as DerivesDirectly).own } == 1)
        })
    );
    ({ HasCounterImpl::increment(&(i.as_pointer() as HasCounter)) });
    assert!(
        ({
            ({ ((*i.borrow()) as HasCounter).count } == 3)
                && ({ ((*i.borrow()) as DerivesDirectly).own } == 1)
        })
    );
    assert!(({ ({ ((*d.borrow()) as HasCounter).count } == 8) }));
    return 0;
}
pub trait DerivesDirectlyImpl {
    fn incrementBoth(&self);
}
impl DerivesDirectlyImpl for Ptr<DerivesDirectly> {
    fn incrementBoth(&self) {
        ({ HasCounterImpl::increment(self) });
        field!((*self), own).with_mut(|__v| __v.prefix_inc());
    }
}
pub trait DerivesIndirectlyImpl {
    fn incrementTwice(&self);
}
impl DerivesIndirectlyImpl for Ptr<DerivesIndirectly> {
    fn incrementTwice(&self) {
        ({ HasCounterImpl::increment(self) });
        ({ DerivesDirectlyImpl::incrementBoth(self) });
    }
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
