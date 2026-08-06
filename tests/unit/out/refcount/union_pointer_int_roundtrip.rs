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
pub struct cb {
    pub ctx: AnyPtr,
}
impl ByteRepr for cb {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.ctx.to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            ctx: <AnyPtr>::from_bytes(&buf[0..8]),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([10, 20, 30, 40])));
    pub struct anon_0 {
        __bytes: Value<Box<[u8]>>,
    }
    impl anon_0 {
        pub fn p(&self) -> Ptr<Ptr<i32>> {
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
        .reinterpret_cast::<Ptr<i32>>() as Ptr<Ptr<i32>>)
        .write(((arr.as_pointer() as Ptr<i32>).offset(1)));
    {
        let rhs_0 = (u
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<u64>() as Ptr<u64>)
            .with(|__v| {
                (*__v)
                    .wrapping_add(
                        ((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64),
                    )
                    .clone()
            });
        (u.as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<u64>() as Ptr<u64>)
            .write(rhs_0)
    };
    let q: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ((u.as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<Ptr<i32>>() as Ptr<Ptr<i32>>)
            .read())
        .clone(),
    ));
    assert!((((((*q.borrow()).read()) == 40) as i32) != 0));
    assert!(
        ((({
            let _lhs = (*q.borrow()).clone();
            _lhs == ((arr.as_pointer() as Ptr<i32>).offset(3))
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
                        ((3_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64),
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
        ((({
            let _lhs = ((u
                .as_pointer()
                .reinterpret_cast::<u8>()
                .offset(0usize)
                .reinterpret_cast::<Ptr<i32>>() as Ptr<Ptr<i32>>)
                .read())
            .clone();
            _lhs == ((arr.as_pointer() as Ptr<i32>).offset(0))
        }) as i32)
            != 0)
    );
    assert!(
        ((((((u
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<Ptr::<i32>>() as Ptr<Ptr::<i32>>)
            .read())
        .read())
            == 10) as i32)
            != 0)
    );
    (u.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(0usize)
        .reinterpret_cast::<Ptr<i32>>() as Ptr<Ptr<i32>>)
        .write((arr.as_pointer() as Ptr<i32>).offset(((4) as isize)));
    assert!(
        ((({
            let _lhs = ((u
                .as_pointer()
                .reinterpret_cast::<u8>()
                .offset(0usize)
                .reinterpret_cast::<Ptr<i32>>() as Ptr<Ptr<i32>>)
                .read())
            .clone();
            _lhs == (arr.as_pointer() as Ptr<i32>).offset(((4) as isize))
        }) as i32)
            != 0)
    );
    let c: Value<cb> = Rc::new(RefCell::new(cb {
        ctx: (<AnyPtr>::from_int(((99) as i64) as usize)),
    }));
    assert!(((((((*c.borrow()).ctx).to_int() as i32) == 99) as i32) != 0));
    let m: Value<AnyPtr> = Rc::new(RefCell::new(
        (<AnyPtr>::from_int(((-1_i32) as i64) as usize)),
    ));
    assert!((((((*m.borrow()).to_int() as i32) == -1_i32) as i32) != 0));
    assert!((((!((*m.borrow()).is_null())) as i32) != 0));
    (*c.borrow_mut()).ctx = (AnyPtr::default());
    assert!((((((*c.borrow()).ctx).is_null()) as i32) != 0));
    return 0;
}
