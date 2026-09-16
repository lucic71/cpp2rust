extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Item {
    pub key: Value<i32>,
    pub value: Value<i32>,
}
impl Clone for Item {
    fn clone(&self) -> Self {
        let __this: Value<Item> = Rc::new(RefCell::new(Self {
            key: Rc::new(RefCell::new((*self.key.borrow()))),
            value: Rc::new(RefCell::new((*self.value.borrow()))),
        }));
        let this: Ptr<Item> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Item {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.key.borrow()).to_bytes(&mut buf[0..4]);
        (*self.value.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            key: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            value: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn CompareItem_0(a: Ptr<Item>, b: Ptr<Item>) -> bool {
    return {
        let _lhs = (*(*a.upgrade().deref()).key.borrow());
        _lhs < (*(*b.upgrade().deref()).key.borrow())
    };
}
pub fn CompareInt_1(a: i32, b: i32) -> bool {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return ((*a.borrow()) > (*b.borrow()));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<Item>> = Rc::new(RefCell::new(Vec::new()));
    (*v.borrow_mut()).push(Item {
        key: Rc::new(RefCell::new(3)),
        value: Rc::new(RefCell::new(30)),
    });
    (*v.borrow_mut()).push(Item {
        key: Rc::new(RefCell::new(1)),
        value: Rc::new(RefCell::new(10)),
    });
    (*v.borrow_mut()).push(Item {
        key: Rc::new(RefCell::new(2)),
        value: Rc::new(RefCell::new(20)),
    });
    (v.as_pointer() as Ptr<Item>).sort_with_cmp(
        (v.as_pointer() as Ptr<Item>).to_end().get_offset(),
        |x, y| CompareItem_0.call(x, y),
    );
    assert!(
        ((*(*(v.as_pointer() as Ptr<Item>)
            .offset(0_usize)
            .upgrade()
            .deref())
        .key
        .borrow())
            == 1)
    );
    (v.as_pointer() as Ptr<Item>).sort_with_cmp(
        (v.as_pointer() as Ptr<Item>).to_end().get_offset(),
        |x, y| {
            (|a: Ptr<Item>, b: Ptr<Item>| {
                return {
                    let _lhs = (*(*a.upgrade().deref()).key.borrow());
                    _lhs > (*(*b.upgrade().deref()).key.borrow())
                };
            })
            .call(x, y)
        },
    );
    assert!(
        ((*(*(v.as_pointer() as Ptr<Item>)
            .offset(0_usize)
            .upgrade()
            .deref())
        .key
        .borrow())
            == 3)
    );
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([5, 2, 8, 1, 3])));
    {
        let fun =
            |x: Ptr<i32>, y: Ptr<i32>| CompareInt_1.call((x.read()).clone(), (y.read()).clone());
        (arr.as_pointer() as Ptr<i32>).sort_with_cmp(
            (arr.as_pointer() as Ptr<i32>)
                .offset((5) as isize)
                .get_offset(),
            fun,
        )
    };
    assert!(((*arr.borrow())[(0) as usize] == 8));
    {
        let fun = |x: Ptr<i32>, y: Ptr<i32>| {
            (|x: i32, y: i32| {
                let x: Value<i32> = Rc::new(RefCell::new(x));
                let y: Value<i32> = Rc::new(RefCell::new(y));
                return ((*x.borrow()) < (*y.borrow()));
            })
            .call((x.read()).clone(), (y.read()).clone())
        };
        (arr.as_pointer() as Ptr<i32>).sort_with_cmp(
            (arr.as_pointer() as Ptr<i32>)
                .offset((5) as isize)
                .get_offset(),
            fun,
        )
    };
    assert!(((*arr.borrow())[(0) as usize] == 1));
    return 0;
}
