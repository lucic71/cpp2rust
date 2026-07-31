extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Outer_Named {
    pub a: i32,
    pub b: i32,
}
impl Clone for Outer_Named {
    fn clone(&self) -> Self {
        let mut this = Self {
            a: self.a,
            b: self.b,
        };
        this
    }
}
impl ByteRepr for Outer_Named {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.a.to_bytes(&mut buf[0..4]);
        self.b.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: <i32>::from_bytes(&buf[0..4]),
            b: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
#[derive(Default)]
pub struct anon_0 {
    pub c: i32,
    pub d: i32,
}
impl Clone for anon_0 {
    fn clone(&self) -> Self {
        let mut this = Self {
            c: self.c,
            d: self.d,
        };
        this
    }
}
impl ByteRepr for anon_0 {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.c.to_bytes(&mut buf[0..4]);
        self.d.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            c: <i32>::from_bytes(&buf[0..4]),
            d: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
#[derive(Default)]
pub struct anon_1 {
    pub g: i32,
    pub h: i32,
}
impl Clone for anon_1 {
    fn clone(&self) -> Self {
        let mut this = Self {
            g: self.g,
            h: self.h,
        };
        this
    }
}
impl ByteRepr for anon_1 {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.g.to_bytes(&mut buf[0..4]);
        self.h.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            g: <i32>::from_bytes(&buf[0..4]),
            h: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
#[derive(Default)]
pub struct anon_2 {
    pub e: i32,
    pub f: i32,
}
impl Clone for anon_2 {
    fn clone(&self) -> Self {
        let mut this = Self {
            e: self.e,
            f: self.f,
        };
        this
    }
}
impl ByteRepr for anon_2 {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.e.to_bytes(&mut buf[0..4]);
        self.f.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            e: <i32>::from_bytes(&buf[0..4]),
            f: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
#[derive(Default)]
pub struct anon_4 {
    pub j: i32,
}
impl Clone for anon_4 {
    fn clone(&self) -> Self {
        let mut this = Self { j: self.j };
        this
    }
}
impl ByteRepr for anon_4 {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.j.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            j: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Default)]
pub struct anon_5 {
    pub k: i32,
}
impl Clone for anon_5 {
    fn clone(&self) -> Self {
        let mut this = Self { k: self.k };
        this
    }
}
impl ByteRepr for anon_5 {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.k.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            k: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Default)]
pub struct anon_3 {
    pub i: i32,
    pub inner_named: anon_4,
    pub anon_5: anon_5,
}
impl Clone for anon_3 {
    fn clone(&self) -> Self {
        let mut this = Self {
            i: self.i,
            inner_named: (self.inner_named).clone(),
            anon_5: (self.anon_5).clone(),
        };
        this
    }
}
impl ByteRepr for anon_3 {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.i.to_bytes(&mut buf[0..4]);
        self.inner_named.to_bytes(&mut buf[4..8]);
        self.anon_5.to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            i: <i32>::from_bytes(&buf[0..4]),
            inner_named: <anon_4>::from_bytes(&buf[4..8]),
            anon_5: <anon_5>::from_bytes(&buf[8..12]),
        }
    }
}
#[derive(Default)]
pub struct Outer {
    pub named: Outer_Named,
    pub anonymous_named_0: anon_0,
    pub anonymous_named_1: anon_1,
    pub anon_2: anon_2,
    pub anon_3: anon_3,
}
impl Clone for Outer {
    fn clone(&self) -> Self {
        let mut this = Self {
            named: (self.named).clone(),
            anonymous_named_0: (self.anonymous_named_0).clone(),
            anonymous_named_1: (self.anonymous_named_1).clone(),
            anon_2: (self.anon_2).clone(),
            anon_3: (self.anon_3).clone(),
        };
        this
    }
}
impl ByteRepr for Outer {
    fn byte_size() -> usize {
        44
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.named.to_bytes(&mut buf[0..8]);
        self.anonymous_named_0.to_bytes(&mut buf[8..16]);
        self.anonymous_named_1.to_bytes(&mut buf[16..24]);
        self.anon_2.to_bytes(&mut buf[24..32]);
        self.anon_3.to_bytes(&mut buf[32..44]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            named: <Outer_Named>::from_bytes(&buf[0..8]),
            anonymous_named_0: <anon_0>::from_bytes(&buf[8..16]),
            anonymous_named_1: <anon_1>::from_bytes(&buf[16..24]),
            anon_2: <anon_2>::from_bytes(&buf[24..32]),
            anon_3: <anon_3>::from_bytes(&buf[32..44]),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Outer> = Rc::new(RefCell::new(Outer {
        named: Outer_Named {
            a: <i32>::default(),
            b: <i32>::default(),
        },
        anonymous_named_0: anon_0 {
            c: <i32>::default(),
            d: <i32>::default(),
        },
        anonymous_named_1: anon_1 {
            g: <i32>::default(),
            h: <i32>::default(),
        },
        anon_2: anon_2 {
            e: <i32>::default(),
            f: <i32>::default(),
        },
        anon_3: anon_3 {
            i: <i32>::default(),
            inner_named: anon_4 {
                j: <i32>::default(),
            },
            anon_5: anon_5 {
                k: <i32>::default(),
            },
        },
    }));
    (*o.borrow_mut()).named.a = 1;
    (*o.borrow_mut()).named.b = 2;
    (*o.borrow_mut()).anonymous_named_0.c = 3;
    (*o.borrow_mut()).anonymous_named_0.d = 4;
    (*o.borrow_mut()).anonymous_named_1.g = 5;
    (*o.borrow_mut()).anonymous_named_1.h = 6;
    (*o.borrow_mut()).anon_2.e = 7;
    (*o.borrow_mut()).anon_2.f = 8;
    (*o.borrow_mut()).anon_3.i = 9;
    (*o.borrow_mut()).anon_3.inner_named.j = 10;
    (*o.borrow_mut()).anon_3.anon_5.k = 11;
    assert!(((*o.borrow()).named.a == 1));
    assert!(((*o.borrow()).named.b == 2));
    assert!(((*o.borrow()).anonymous_named_0.c == 3));
    assert!(((*o.borrow()).anonymous_named_0.d == 4));
    assert!(((*o.borrow()).anonymous_named_1.g == 5));
    assert!(((*o.borrow()).anonymous_named_1.h == 6));
    assert!(((*o.borrow()).anon_2.e == 7));
    assert!(((*o.borrow()).anon_2.f == 8));
    assert!(((*o.borrow()).anon_3.i == 9));
    assert!(((*o.borrow()).anon_3.inner_named.j == 10));
    assert!(((*o.borrow()).anon_3.anon_5.k == 11));
    #[derive(Default)]
    pub struct anon_6 {
        pub x: i32,
        pub z: i32,
    }
    impl Clone for anon_6 {
        fn clone(&self) -> Self {
            let mut this = Self {
                x: self.x,
                z: self.z,
            };
            this
        }
    }
    impl ByteRepr for anon_6 {
        fn byte_size() -> usize {
            8
        }
        fn to_bytes(&self, buf: &mut [u8]) {
            self.x.to_bytes(&mut buf[0..4]);
            self.z.to_bytes(&mut buf[4..8]);
        }
        fn from_bytes(buf: &[u8]) -> Self {
            Self {
                x: <i32>::from_bytes(&buf[0..4]),
                z: <i32>::from_bytes(&buf[4..8]),
            }
        }
    };
    let s: Value<anon_6> = Rc::new(RefCell::new(<anon_6>::default()));
    (*s.borrow_mut()).x = 1;
    (*s.borrow_mut()).z = 2;
    assert!(
        ({
            (*s.borrow_mut()).x = 1;
            (*s.borrow()).x
        } != 0)
    );
    assert!(
        ({
            (*s.borrow_mut()).z = 2;
            (*s.borrow()).z
        } != 0)
    );
    return 0;
}
