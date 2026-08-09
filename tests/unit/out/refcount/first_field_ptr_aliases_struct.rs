extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive()]
pub struct transfer {
    pub errbuf: Box<[u8]>,
    pub code: i32,
}
impl Clone for transfer {
    fn clone(&self) -> Self {
        let mut this = Self {
            errbuf: (self.errbuf).clone(),
            code: self.code,
        };
        this
    }
}
impl Default for transfer {
    fn default() -> Self {
        transfer {
            errbuf: (0..32).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
            code: <i32>::default(),
        }
    }
}
impl ByteRepr for transfer {
    fn byte_size() -> usize {
        36
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.errbuf.to_bytes(&mut buf[0..32]);
        self.code.to_bytes(&mut buf[32..36]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            errbuf: <Box<[u8]>>::from_bytes(&buf[0..32]),
            code: <i32>::from_bytes(&buf[32..36]),
        }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct holder {
    pub xfer: Ptr<transfer>,
    pub err: Ptr<u8>,
}
impl Clone for holder {
    fn clone(&self) -> Self {
        let mut this = Self {
            xfer: (self.xfer).clone(),
            err: (self.err).clone(),
        };
        this
    }
}
impl ByteRepr for holder {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.xfer.to_bytes(&mut buf[0..8]);
        self.err.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            xfer: <Ptr<transfer>>::from_bytes(&buf[0..8]),
            err: <Ptr<u8>>::from_bytes(&buf[8..16]),
        }
    }
}
#[repr(C)]
#[derive()]
pub struct tagged {
    pub errbuf: Box<[u8]>,
    pub code: i32,
    pub lookup: BTreeMap<i32, Value<i32>>,
}
impl Clone for tagged {
    fn clone(&self) -> Self {
        let mut this = Self {
            errbuf: (self.errbuf).clone(),
            code: self.code,
            lookup: self
                .lookup
                .iter()
                .map(|(k, v)| (k.clone(), Rc::new(RefCell::new(v.borrow().clone()))))
                .collect(),
        };
        this
    }
}
impl Default for tagged {
    fn default() -> Self {
        tagged {
            errbuf: (0..32).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
            code: <i32>::default(),
            lookup: BTreeMap::new(),
        }
    }
}
impl ByteRepr for tagged {
    fn byte_size() -> usize {
        88
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.errbuf.to_bytes(&mut buf[0..32]);
        self.code.to_bytes(&mut buf[32..36]);
        self.lookup.to_bytes(&mut buf[40..88]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            errbuf: <Box<[u8]>>::from_bytes(&buf[0..32]),
            code: <i32>::from_bytes(&buf[32..36]),
            lookup: <BTreeMap<i32, Value<i32>>>::from_bytes(&buf[40..88]),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let h: Value<Ptr<holder>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(16usize).reinterpret_cast::<holder>(),
    ));
    (*h.borrow()).with_mut(|__v| {
        __v.xfer = libcc2rs::malloc_refcount(36usize).reinterpret_cast::<transfer>()
    });
    {
        let __obj = (*h.borrow()).with(|__v| __v.xfer.clone());
        __obj.with_mut(|__v| __v.code = 7)
    };
    {
        let __rhs = ((*h.borrow()).with(|__v| {
            __v.xfer
                .field_ptr(
                    0,
                    |__v: &transfer| &__v.errbuf[..],
                    |__v: &mut transfer| &mut __v.errbuf[..],
                )
                .clone()
        }) as Ptr<u8>);
        (*h.borrow()).with_mut(|__v| __v.err = __rhs)
    };
    {
        (((*h.borrow()).with(|__v| __v.err.clone())).clone() as Ptr<u8>)
            .to_any()
            .memcpy(
                &Ptr::from_string_literal(b"boom\0").to_any(),
                5_usize as usize,
            );
        (((*h.borrow()).with(|__v| __v.err.clone())).clone() as Ptr<u8>)
            .to_any()
            .clone()
    };
    assert!(
        ({
            let mut __it1 = ((*h.borrow()).with(|__v| {
                __v.xfer
                    .field_ptr(
                        0,
                        |__v: &transfer| &__v.errbuf[..],
                        |__v: &mut transfer| &mut __v.errbuf[..],
                    )
                    .clone()
            }) as Ptr<u8>)
                .to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"boom\0").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0)
    );
    assert!(((*h.borrow()).with(|__v| __v.xfer.clone().with(|__v| __v.code)) == 7));
    libcc2rs::free_refcount(
        (((*h.borrow()).with(|__v| __v.xfer.clone())).clone() as Ptr<transfer>)
            .to_any()
            .clone(),
    );
    libcc2rs::free_refcount(((*h.borrow()).clone() as Ptr<holder>).to_any().clone());
    let t: Value<tagged> = Rc::new(RefCell::new(<tagged>::default()));
    (*t.borrow_mut()).code = 9;
    (t.as_pointer().field_ptr(
        40,
        |__v: &tagged| ::std::slice::from_ref(&__v.lookup),
        |__v: &mut tagged| ::std::slice::from_mut(&mut __v.lookup),
    ) as Ptr<BTreeMap<i32, Value<i32>>>)
        .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
            __v.entry(1.clone())
                .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                .as_pointer()
        })
        .write(100);
    let err: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (t.as_pointer().field_ptr(
            0,
            |__v: &tagged| &__v.errbuf[..],
            |__v: &mut tagged| &mut __v.errbuf[..],
        ) as Ptr<u8>),
    ));
    {
        ((*err.borrow()).clone() as Ptr<u8>).to_any().memcpy(
            &Ptr::from_string_literal(b"bang\0").to_any(),
            5_usize as usize,
        );
        ((*err.borrow()).clone() as Ptr<u8>).to_any().clone()
    };
    assert!(
        ({
            let mut __it1 = (t.as_pointer().field_ptr(
                0,
                |__v: &tagged| &__v.errbuf[..],
                |__v: &mut tagged| &mut __v.errbuf[..],
            ) as Ptr<u8>)
                .to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"bang\0").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0)
    );
    assert!(((*t.borrow()).code == 9));
    assert!(
        (((t.as_pointer().field_ptr(
            40,
            |__v: &tagged| ::std::slice::from_ref(&__v.lookup),
            |__v: &mut tagged| ::std::slice::from_mut(&mut __v.lookup)
        ) as Ptr<BTreeMap<i32, Value<i32>>>)
            .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
                __v.entry(1.clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                    .as_pointer()
            })
            .read())
            == 100)
    );
    return 0;
}
