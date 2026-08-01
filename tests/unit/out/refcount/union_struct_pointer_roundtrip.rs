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
pub struct pair {
    pub x: i32,
    pub y: i32,
}
impl ByteRepr for pair {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
        self.y.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <i32>::from_bytes(&buf[0..4]),
            y: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[pair]>> = Rc::new(RefCell::new(
        (0..3).map(|_| <pair>::default()).collect::<Box<[pair]>>(),
    ));
    (*arr.borrow_mut())[(0) as usize].x = 10;
    (*arr.borrow_mut())[(1) as usize].x = 20;
    (*arr.borrow_mut())[(2) as usize].x = 30;
    pub struct anon_0 {
        __bytes: Value<Box<[u8]>>,
    }
    impl anon_0 {
        pub fn p(&self) -> Ptr<Ptr<pair>> {
            (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
        }
        pub fn bits(&self) -> Ptr<u64> {
            (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
        }
    }
    impl Clone for anon_0 {
        fn clone(&self) -> Self {
            anon_0 {
                __bytes: Rc::new(RefCell::new(self.__bytes.borrow().clone())),
            }
        }
    }
    impl Default for anon_0 {
        fn default() -> Self {
            anon_0 {
                __bytes: Rc::new(RefCell::new(Box::from([0u8; 8]))),
            }
        }
    }
    impl ByteRepr for anon_0 {
        fn byte_size() -> usize {
            8
        }
        fn to_bytes(&self, buf: &mut [u8]) {
            buf.copy_from_slice(&self.__bytes.borrow());
        }
        fn from_bytes(buf: &[u8]) -> Self {
            anon_0 {
                __bytes: Rc::new(RefCell::new(Box::from(buf))),
            }
        }
    };
    let u: Value<anon_0> = <Value<anon_0>>::default();
    (u.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(0usize)
        .reinterpret_cast::<Ptr<pair>>() as Ptr<Ptr<pair>>)
        .write(((arr.as_pointer() as Ptr<pair>).offset(1)));
    let q: Value<Ptr<pair>> = Rc::new(RefCell::new(
        ((u.as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<Ptr<pair>>() as Ptr<Ptr<pair>>)
            .read())
        .clone(),
    ));
    assert!(((((*q.borrow()).with(|__v| (*__v).x) == 20) as i32) != 0));
    assert!(
        ((({
            let _lhs = (*q.borrow()).clone();
            _lhs == ((arr.as_pointer() as Ptr<pair>).offset(1))
        }) as i32)
            != 0)
    );
    {
        let rhs_0 = (u
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<u64>() as Ptr<u64>)
            .with(|__v| {
                (*__v)
                    .wrapping_add((::std::mem::size_of::<pair>() as u64))
                    .clone()
            });
        (u.as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<u64>() as Ptr<u64>)
            .write(rhs_0)
    };
    assert!(
        (((((u
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<Ptr<pair>>() as Ptr<Ptr<pair>>)
            .read())
        .with(|__v| (*__v).x)
            == 30) as i32)
            != 0)
    );
    assert!(
        ((({
            let _lhs = ((u
                .as_pointer()
                .reinterpret_cast::<u8>()
                .offset(0usize)
                .reinterpret_cast::<Ptr<pair>>() as Ptr<Ptr<pair>>)
                .read())
            .clone();
            _lhs == ((arr.as_pointer() as Ptr<pair>).offset(2))
        }) as i32)
            != 0)
    );
    {
        let rhs_0 = (u
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<u64>() as Ptr<u64>)
            .with(|__v| {
                (*__v)
                    .wrapping_sub(
                        ((2_usize).wrapping_mul((::std::mem::size_of::<pair>() as usize)) as u64),
                    )
                    .clone()
            });
        (u.as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<u64>() as Ptr<u64>)
            .write(rhs_0)
    };
    assert!(
        (((((u
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<Ptr<pair>>() as Ptr<Ptr<pair>>)
            .read())
        .with(|__v| (*__v).x)
            == 10) as i32)
            != 0)
    );
    assert!(
        ((({
            let _lhs = ((u
                .as_pointer()
                .reinterpret_cast::<u8>()
                .offset(0usize)
                .reinterpret_cast::<Ptr<pair>>() as Ptr<Ptr<pair>>)
                .read())
            .clone();
            _lhs == ((arr.as_pointer() as Ptr<pair>).offset(0))
        }) as i32)
            != 0)
    );
    return 0;
}
