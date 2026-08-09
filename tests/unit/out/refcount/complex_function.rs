extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn ptr_1(x: Ptr<i32>) -> Ptr<i32> {
    let x: Value<Ptr<i32>> = Rc::new(RefCell::new(x));
    return (*x.borrow()).clone();
}
pub fn bar_2(x: Ptr<i32>) -> Ptr<i32> {
    return (x).clone();
}
#[repr(C)]
#[derive(Default)]
pub struct X1 {
    pub v: i32,
}
impl Clone for X1 {
    fn clone(&self) -> Self {
        let mut this = Self { v: self.v };
        this
    }
}
impl ByteRepr for X1 {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct X2 {
    pub v: Ptr<X1>,
}
pub trait X2Methods {
    fn get(&self) -> Ptr<X1>;
}
impl Clone for X2 {
    fn clone(&self) -> Self {
        let mut this = Self {
            v: (self.v).clone(),
        };
        this
    }
}
impl ByteRepr for X2 {}
#[repr(C)]
#[derive(Default)]
pub struct X3 {
    pub v: Ptr<X2>,
}
pub trait X3Methods {
    fn get(&self) -> Ptr<X2>;
}
impl Clone for X3 {
    fn clone(&self) -> Self {
        let mut this = Self {
            v: (self.v).clone(),
        };
        this
    }
}
impl ByteRepr for X3 {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <Ptr<X2>>::from_bytes(&buf[0..8]),
        }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct X4 {
    pub v: X3,
}
pub trait X4Methods {
    fn get(&self) -> Ptr<X3>;
}
impl Clone for X4 {
    fn clone(&self) -> Self {
        let mut this = Self {
            v: (self.v).clone(),
        };
        this
    }
}
impl ByteRepr for X4 {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <X3>::from_bytes(&buf[0..8]),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let x1: Value<i32> = Rc::new(RefCell::new(0));
    let x2: Value<i32> = Rc::new(RefCell::new(({ foo_0((*x1.borrow())) })));
    let x3: Value<i32> = Rc::new(RefCell::new(
        ((({ foo_0((*x2.borrow())) }) + ({ foo_0((*x1.borrow())) })) + 1),
    ));
    (*x2.borrow_mut()) += 1;
    (*x2.borrow_mut()) += ({ foo_0((*x1.borrow())) });
    {
        let __rhs = ((({ foo_0((*x2.borrow())) }) + ({ foo_0((*x3.borrow())) })) + 1);
        (*x3.borrow_mut()) += __rhs
    };
    let p1: Value<Ptr<i32>> = Rc::new(RefCell::new((x1.as_pointer())));
    let p2: Value<Ptr<i32>> = Rc::new(RefCell::new(({ ptr_1((*p1.borrow()).clone()) })));
    (*p1.borrow_mut()) = (*p2.borrow()).clone();
    (*p2.borrow_mut()) = ({ ptr_1((*p1.borrow()).clone()) });
    let r1: Ptr<i32> = x1.as_pointer();
    let r2: Ptr<i32> = ({ bar_2(x1.as_pointer()) });
    let r3: Ptr<i32> = ({ bar_2((r1).clone()) });
    {
        let __rhs = (*x1.borrow());
        {
            let _ptr = r2.clone();
            {
                let __rhs = (_ptr.read()) + __rhs;
                _ptr.write(__rhs)
            }
        }
    };
    {
        let __rhs = (r1.read());
        {
            let _ptr = r3.clone();
            {
                let __rhs = (_ptr.read()) + __rhs;
                _ptr.write(__rhs)
            }
        }
    };
    let x4: Value<i32> = Rc::new(RefCell::new(
        ((({ foo_0((*x3.borrow())) }) + (({ ptr_1((x3.as_pointer())) }).read()))
            + (({ bar_2(x2.as_pointer()) }).read())),
    ));
    let a: Value<X1> = Rc::new(RefCell::new(X1 { v: 0 }));
    let b: Value<X2> = Rc::new(RefCell::new(X2 { v: a.as_pointer() }));
    let c: Value<X3> = Rc::new(RefCell::new(X3 {
        v: (b.as_pointer()),
    }));
    let d: Value<X4> = Rc::new(RefCell::new(X4 {
        v: (*c.borrow()).clone(),
    }));
    {
        let __obj = (*d.borrow()).v.v.with(|__v| __v.v.clone());
        __obj.with_mut(|__v| __v.v = 0)
    };
    ({ ({ ({ d.as_pointer().get() }).get() }).get() }).with_mut(|__v| __v.v = 0);
    (*d.borrow_mut()).v.v = (b.as_pointer());
    let r4: Ptr<i32> = ({ ({ ({ d.as_pointer().get() }).get() }).get() }).field_ptr(
        0,
        |__v: &X1| ::std::slice::from_ref(&__v.v),
        |__v: &mut X1| ::std::slice::from_mut(&mut __v.v),
    );
    let r5: Ptr<X1> = ({ ({ ({ d.as_pointer().get() }).get() }).get() });
    let p: Value<Ptr<X2>> = Rc::new(RefCell::new(({ ({ d.as_pointer().get() }).get() })));
    let r6: Ptr<X3> = ({ d.as_pointer().get() });
    let r7: Ptr<X3> = d.as_pointer().field_ptr(
        0,
        |__v: &X4| ::std::slice::from_ref(&__v.v),
        |__v: &mut X4| ::std::slice::from_mut(&mut __v.v),
    );
    let r8: Ptr<i32> = ({
        ({
            d.as_pointer()
                .field_ptr(
                    0,
                    |__v: &X4| ::std::slice::from_ref(&__v.v),
                    |__v: &mut X4| ::std::slice::from_mut(&mut __v.v),
                )
                .get()
        })
        .get()
    })
    .field_ptr(
        0,
        |__v: &X1| ::std::slice::from_ref(&__v.v),
        |__v: &mut X1| ::std::slice::from_mut(&mut __v.v),
    );
    let x5: Value<i32> = Rc::new(RefCell::new(
        ({ ({ ({ d.as_pointer().get() }).get() }).get() }).with(|__v| __v.v),
    ));
    {
        let _ptr = ({ bar_2(x1.as_pointer()) }).clone();
        {
            let __rhs = (_ptr.read()) + 10;
            _ptr.write(__rhs)
        }
    };
    ({ bar_2(x1.as_pointer()) }).with_mut(|__v| __v.postfix_inc());
    let bar_out: Value<i32> = Rc::new(RefCell::new(
        (({
            bar_2(
                ({ ({ ({ d.as_pointer().get() }).get() }).get() }).field_ptr(
                    0,
                    |__v: &X1| ::std::slice::from_ref(&__v.v),
                    |__v: &mut X1| ::std::slice::from_mut(&mut __v.v),
                ),
            )
        })
        .read()),
    ));
    let bar_inc: Value<i32> = Rc::new(RefCell::new(
        ({ bar_2(x1.as_pointer()) }).with_mut(|__v| __v.prefix_inc()),
    ));
    (*bar_inc.borrow_mut()) = ({ bar_2(x1.as_pointer()) }).with_mut(|__v| __v.postfix_inc());
    (*bar_inc.borrow_mut()) =
        (((({ bar_2(x1.as_pointer()) }).read()) + ({ foo_0((*x4.borrow())) })) + 1);
    {
        let _ptr = ({
            bar_2(
                ({ ({ ({ d.as_pointer().get() }).get() }).get() }).field_ptr(
                    0,
                    |__v: &X1| ::std::slice::from_ref(&__v.v),
                    |__v: &mut X1| ::std::slice::from_mut(&mut __v.v),
                ),
            )
        })
        .clone();
        {
            let __rhs = (_ptr.read()) + 10;
            _ptr.write(__rhs)
        }
    };
    ({
        bar_2(
            ({ ({ ({ d.as_pointer().get() }).get() }).get() }).field_ptr(
                0,
                |__v: &X1| ::std::slice::from_ref(&__v.v),
                |__v: &mut X1| ::std::slice::from_mut(&mut __v.v),
            ),
        )
    })
    .with_mut(|__v| __v.postfix_inc());
    let bar_inc2: Value<i32> = Rc::new(RefCell::new(
        ({
            bar_2(
                ({ ({ ({ d.as_pointer().get() }).get() }).get() }).field_ptr(
                    0,
                    |__v: &X1| ::std::slice::from_ref(&__v.v),
                    |__v: &mut X1| ::std::slice::from_mut(&mut __v.v),
                ),
            )
        })
        .with_mut(|__v| __v.prefix_inc()),
    ));
    (*bar_inc2.borrow_mut()) = ({
        bar_2(
            ({ ({ ({ d.as_pointer().get() }).get() }).get() }).field_ptr(
                0,
                |__v: &X1| ::std::slice::from_ref(&__v.v),
                |__v: &mut X1| ::std::slice::from_mut(&mut __v.v),
            ),
        )
    })
    .with_mut(|__v| __v.postfix_inc());
    ({ ptr_1((x1.as_pointer())) }).with_mut(|__v| __v.prefix_inc());
    {
        let _ptr = ({ ptr_1((x1.as_pointer())) }).clone();
        {
            let __rhs = (_ptr.read()) + 1;
            _ptr.write(__rhs)
        }
    };
    ({
        ptr_1(
            (({ ({ ({ d.as_pointer().get() }).get() }).get() }).field_ptr(
                0,
                |__v: &X1| ::std::slice::from_ref(&__v.v),
                |__v: &mut X1| ::std::slice::from_mut(&mut __v.v),
            )),
        )
    })
    .with_mut(|__v| __v.prefix_inc());
    {
        let _ptr = ({
            ptr_1(
                (({ ({ ({ d.as_pointer().get() }).get() }).get() }).field_ptr(
                    0,
                    |__v: &X1| ::std::slice::from_ref(&__v.v),
                    |__v: &mut X1| ::std::slice::from_mut(&mut __v.v),
                )),
            )
        })
        .clone();
        {
            let __rhs = (_ptr.read()) + 1;
            _ptr.write(__rhs)
        }
    };
    {
        let _ptr = ({
            ptr_1(
                (({ ({ ({ d.as_pointer().get() }).get() }).get() }).field_ptr(
                    0,
                    |__v: &X1| ::std::slice::from_ref(&__v.v),
                    |__v: &mut X1| ::std::slice::from_mut(&mut __v.v),
                )),
            )
        })
        .clone();
        {
            let __rhs = (_ptr.read()) + 1;
            _ptr.write(__rhs)
        }
    };
    let ptr1: Value<i32> = Rc::new(RefCell::new(
        ({
            ptr_1(
                (({ ({ ({ d.as_pointer().get() }).get() }).get() }).field_ptr(
                    0,
                    |__v: &X1| ::std::slice::from_ref(&__v.v),
                    |__v: &mut X1| ::std::slice::from_mut(&mut __v.v),
                )),
            )
        })
        .with_mut(|__v| __v.postfix_inc()),
    ));
    let ptr2: Ptr<i32> = ({
        ptr_1(
            (({ ({ ({ d.as_pointer().get() }).get() }).get() }).field_ptr(
                0,
                |__v: &X1| ::std::slice::from_ref(&__v.v),
                |__v: &mut X1| ::std::slice::from_mut(&mut __v.v),
            )),
        )
    });
    let ptr3: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ({
            ptr_1(
                (({ ({ ({ d.as_pointer().get() }).get() }).get() }).field_ptr(
                    0,
                    |__v: &X1| ::std::slice::from_ref(&__v.v),
                    |__v: &mut X1| ::std::slice::from_mut(&mut __v.v),
                )),
            )
        }),
    ));
    let vptr: Value<i32> = Rc::new(RefCell::new(
        (({
            ptr_1(
                (({ ({ ({ d.as_pointer().get() }).get() }).get() }).field_ptr(
                    0,
                    |__v: &X1| ::std::slice::from_ref(&__v.v),
                    |__v: &mut X1| ::std::slice::from_mut(&mut __v.v),
                )),
            )
        })
        .read()),
    ));
    let pref: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ({
            bar_2(
                ({ ({ ({ d.as_pointer().get() }).get() }).get() }).field_ptr(
                    0,
                    |__v: &X1| ::std::slice::from_ref(&__v.v),
                    |__v: &mut X1| ::std::slice::from_mut(&mut __v.v),
                ),
            )
        }),
    ));
    ({
        bar_2(
            ({ ({ ({ d.as_pointer().get() }).get() }).get() }).field_ptr(
                0,
                |__v: &X1| ::std::slice::from_ref(&__v.v),
                |__v: &mut X1| ::std::slice::from_mut(&mut __v.v),
            ),
        )
    })
    .with_mut(|__v| __v.postfix_inc());
    return (((({
        ptr_1(
            (({ ({ ({ d.as_pointer().get() }).get() }).get() }).field_ptr(
                0,
                |__v: &X1| ::std::slice::from_ref(&__v.v),
                |__v: &mut X1| ::std::slice::from_mut(&mut __v.v),
            )),
        )
    })
    .read())
        + (({
            bar_2(
                ({ ({ ({ d.as_pointer().get() }).get() }).get() }).field_ptr(
                    0,
                    |__v: &X1| ::std::slice::from_ref(&__v.v),
                    |__v: &mut X1| ::std::slice::from_mut(&mut __v.v),
                ),
            )
        })
        .read()))
        + ({ foo_0(({ ({ ({ d.as_pointer().get() }).get() }).get() }).with(|__v| __v.v)) }));
}
impl X2Methods for Ptr<X2> {
    fn get(&self) -> Ptr<X1> {
        return (self.with(|__v| __v.v.clone()));
    }
}
impl X3Methods for Ptr<X3> {
    fn get(&self) -> Ptr<X2> {
        return (self.with(|__v| __v.v.clone()));
    }
}
impl X4Methods for Ptr<X4> {
    fn get(&self) -> Ptr<X3> {
        return self.field_ptr(
            0,
            |__v: &X4| ::std::slice::from_ref(&__v.v),
            |__v: &mut X4| ::std::slice::from_mut(&mut __v.v),
        );
    }
}
