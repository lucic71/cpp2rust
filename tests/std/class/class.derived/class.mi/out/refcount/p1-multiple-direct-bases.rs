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
pub struct CountsFirst {
    #[offset(0)]
    pub first: i32,
}
impl Default for CountsFirst {
    fn default() -> Self {
        { CountsFirst { first: 0 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct CountsSecond {
    #[offset(0)]
    pub second: i32,
}
impl Default for CountsSecond {
    fn default() -> Self {
        { CountsSecond { second: 0 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct CountsThird {
    #[offset(0)]
    pub third: i32,
}
impl Default for CountsThird {
    fn default() -> Self {
        { CountsThird { third: 0 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct HasThreeBases {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<HasThreeBases> = Rc::new(RefCell::new(<HasThreeBases>::default()));
    ({ HasThreeBasesImpl::touchAll(&d.as_pointer()) });
    assert!(
        ({
            (({ ((*d.borrow()) as CountsFirst).first } == 1)
                && ({ ((*d.borrow()) as CountsSecond).second } == 1))
                && ({ ((*d.borrow()) as CountsThird).third } == 1)
        })
    );
    ({ CountsSecondImpl::touchSecond(&(d.as_pointer() as CountsSecond)) });
    assert!(
        ({
            (({ ((*d.borrow()) as CountsFirst).first } == 1)
                && ({ ((*d.borrow()) as CountsSecond).second } == 2))
                && ({ ((*d.borrow()) as CountsThird).third } == 1)
        })
    );
    let mut first: Ptr<CountsFirst> = (d.as_pointer());
    ({ CountsFirstImpl::touchFirst(&first) });
    assert!(
        ({
            (({ ((*d.borrow()) as CountsFirst).first } == 2)
                && ({ ((*d.borrow()) as CountsSecond).second } == 2))
                && ({ ((*d.borrow()) as CountsThird).third } == 1)
        })
    );
    let mut second: Ptr<CountsSecond> = (d.as_pointer());
    ({ CountsSecondImpl::touchSecond(&second) });
    assert!(
        ({
            (({ ((*d.borrow()) as CountsFirst).first } == 2)
                && ({ ((*d.borrow()) as CountsSecond).second } == 3))
                && ({ ((*d.borrow()) as CountsThird).third } == 1)
        })
    );
    let mut third: Ptr<CountsThird> = (d.as_pointer());
    ({ CountsThirdImpl::touchThird(&third) });
    assert!(
        ({
            (({ ((*d.borrow()) as CountsFirst).first } == 2)
                && ({ ((*d.borrow()) as CountsSecond).second } == 3))
                && ({ ((*d.borrow()) as CountsThird).third } == 2)
        })
    );
    assert!((first.with(|__s| __s.first) == 2));
    assert!((second.with(|__s| __s.second) == 3));
    assert!((third.with(|__s| __s.third) == 2));
    return 0;
}
pub trait CountsFirstImpl {
    fn touchFirst(&self);
}
impl CountsFirstImpl for Ptr<CountsFirst> {
    fn touchFirst(&self) {
        field!((*self), first).with_mut(|__v| __v.prefix_inc());
    }
}
pub trait CountsSecondImpl {
    fn touchSecond(&self);
}
impl CountsSecondImpl for Ptr<CountsSecond> {
    fn touchSecond(&self) {
        field!((*self), second).with_mut(|__v| __v.prefix_inc());
    }
}
pub trait CountsThirdImpl {
    fn touchThird(&self);
}
impl CountsThirdImpl for Ptr<CountsThird> {
    fn touchThird(&self) {
        field!((*self), third).with_mut(|__v| __v.prefix_inc());
    }
}
pub trait HasThreeBasesImpl {
    fn touchAll(&self);
}
impl HasThreeBasesImpl for Ptr<HasThreeBases> {
    fn touchAll(&self) {
        ({ CountsFirstImpl::touchFirst(self) });
        ({ CountsSecondImpl::touchSecond(self) });
        ({ CountsThirdImpl::touchThird(self) });
    }
}
pub fn __cpp2rust_init_globals() {}
