extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(x: ()) -> () {
    let x: Value<()> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn ptr_1(x: Ptr<()>) -> Ptr<()> {
    let x: Value<Ptr<()>> = Rc::new(RefCell::new(x));
    return (*x.borrow()).clone();
}
pub fn bar_2(x: Ptr<()>) -> Ptr<()> {
    return (x).clone();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct X1 {
    #[offset(0)]
    pub v: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct X2 {
    #[offset(0)]
    #[byte_size(8)]
    pub v: Ptr<X1>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct X3 {
    #[offset(0)]
    #[byte_size(8)]
    pub v: Ptr<X2>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct X4 {
    #[offset(0)]
    #[byte_size(8)]
    pub v: X3,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let x1: Value<()> = Rc::new(RefCell::new(0_()));
    let x2: Value<()> = Rc::new(RefCell::new(({ foo_0((*x1.borrow())) })));
    let x3: Value<()> = Rc::new(RefCell::new(
        ((({ foo_0((*x2.borrow())) }) + ({ foo_0((*x1.borrow())) })) + 1_()),
    ));
    (*x2.borrow_mut()) += 1_();
    (*x2.borrow_mut()) += ({ foo_0((*x1.borrow())) });
    let __rhs = ((({ foo_0((*x2.borrow())) }) + ({ foo_0((*x3.borrow())) })) + 1_());
    (*x3.borrow_mut()) += __rhs;
    let mut p1: Ptr<()> = (x1.as_pointer());
    let mut p2: Ptr<()> = ({ ptr_1((p1).clone()) });
    p1 = (p2).clone();
    p2 = ({ ptr_1((p1).clone()) });
    let r1: Ptr<()> = x1.as_pointer();
    let r2: Ptr<()> = ({ bar_2(x1.as_pointer()) });
    let r3: Ptr<()> = ({ bar_2((r1).clone()) });
    {
        let __rhs = { (*x1.borrow()) };
        r2.with_mut(|__v| *__v = *__v + __rhs)
    };
    {
        let __rhs = { (r1.read()) };
        r3.with_mut(|__v| *__v = *__v + __rhs)
    };
    let mut x4: () = ((({ foo_0((*x3.borrow())) }) + (({ ptr_1((x3.as_pointer())) }).read()))
        + (({ bar_2(x2.as_pointer()) }).read()));
    let a: Value<X1> = Rc::new(RefCell::new(X1 { v: 0_() }));
    let b: Value<X2> = Rc::new(RefCell::new(X2 { v: a.as_pointer() }));
    let mut c: X3 = X3 {
        v: (b.as_pointer()),
    };
    let d: Value<X4> = Rc::new(RefCell::new(X4 { v: (c).clone() }));
    field!({ (*d.borrow()).v.v.clone() }.with(|__s| __s.v.clone()), v).write(0_());
    field!(
        ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
        v
    )
    .write(0_());
    (*d.borrow_mut()).v.v = (b.as_pointer());
    let r4: Ptr<()> = field_ptr!(
        ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
        v
    );
    let r5: Ptr<X1> = ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer()) })) })) });
    let mut p: Ptr<X2> = ({ X3Impl::get(&({ X4Impl::get(&d.as_pointer()) })) });
    let r6: Ptr<X3> = ({ X4Impl::get(&d.as_pointer()) });
    let r7: Ptr<X3> = field_ptr!(d.as_pointer(), v);
    let r8: Ptr<()> = field_ptr!(
        ({ X2Impl::get(&({ X3Impl::get(&field_ptr!(d.as_pointer(), v),) }),) }),
        v
    );
    let mut x5: () = ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer()) })) })) })
        .with(|__s| __s.v);
    {
        ({ bar_2(x1.as_pointer()) }).with_mut(|__v| *__v = *__v + 10_())
    };
    ({ bar_2(x1.as_pointer()) }).with_mut(|__v| __v.postfix_inc());
    let mut bar_out: () = (({
        bar_2(field_ptr!(
            ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
            v
        ))
    })
    .read());
    let mut bar_inc: () = ({ bar_2(x1.as_pointer()) }).with_mut(|__v| __v.prefix_inc());
    bar_inc = ({ bar_2(x1.as_pointer()) }).with_mut(|__v| __v.postfix_inc());
    bar_inc = (((({ bar_2(x1.as_pointer()) }).read()) + ({ foo_0(x4) })) + 1_());
    {
        ({
            bar_2(field_ptr!(
                ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
                v
            ))
        })
        .with_mut(|__v| *__v = *__v + 10_())
    };
    ({
        bar_2(field_ptr!(
            ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
            v
        ))
    })
    .with_mut(|__v| __v.postfix_inc());
    let mut bar_inc2: () = ({
        bar_2(field_ptr!(
            ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
            v
        ))
    })
    .with_mut(|__v| __v.prefix_inc());
    bar_inc2 = ({
        bar_2(field_ptr!(
            ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
            v
        ))
    })
    .with_mut(|__v| __v.postfix_inc());
    ({ ptr_1((x1.as_pointer())) }).with_mut(|__v| __v.prefix_inc());
    {
        ({ ptr_1((x1.as_pointer())) }).with_mut(|__v| *__v = *__v + 1_())
    };
    ({
        ptr_1(
            (field_ptr!(
                ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
                v
            )),
        )
    })
    .with_mut(|__v| __v.prefix_inc());
    {
        ({
            ptr_1(
                (field_ptr!(
                    ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
                    v
                )),
            )
        })
        .with_mut(|__v| *__v = *__v + 1_())
    };
    {
        ({
            ptr_1(
                (field_ptr!(
                    ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
                    v
                )),
            )
        })
        .with_mut(|__v| *__v = *__v + 1_())
    };
    let mut ptr1: () = ({
        ptr_1(
            (field_ptr!(
                ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
                v
            )),
        )
    })
    .with_mut(|__v| __v.postfix_inc());
    let ptr2: Ptr<()> = ({
        ptr_1(
            (field_ptr!(
                ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
                v
            )),
        )
    });
    let mut ptr3: Ptr<()> = ({
        ptr_1(
            (field_ptr!(
                ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
                v
            )),
        )
    });
    let mut vptr: () = (({
        ptr_1(
            (field_ptr!(
                ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
                v
            )),
        )
    })
    .read());
    let mut pref: Ptr<()> = ({
        bar_2(field_ptr!(
            ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
            v
        ))
    });
    ({
        bar_2(field_ptr!(
            ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
            v
        ))
    })
    .with_mut(|__v| __v.postfix_inc());
    unimplemented!();
    return 0_();
}
pub trait X2Impl {
    fn get(&self) -> Ptr<X1>;
}
impl X2Impl for Ptr<X2> {
    fn get(&self) -> Ptr<X1> {
        return ((*self).with(|__s| __s.v.clone())).clone();
    }
}
pub trait X3Impl {
    fn get(&self) -> Ptr<X2>;
}
impl X3Impl for Ptr<X3> {
    fn get(&self) -> Ptr<X2> {
        return (*self).with(|__s| __s.v.clone());
    }
}
pub trait X4Impl {
    fn get(&self) -> Ptr<X3>;
}
impl X4Impl for Ptr<X4> {
    fn get(&self) -> Ptr<X3> {
        return field_ptr!((*self), v);
    }
}
pub fn __cpp2rust_init_globals() {}
