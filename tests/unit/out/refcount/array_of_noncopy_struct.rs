extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(32)]
pub struct NonCopy {
    #[offset(0)]
    #[byte_size(24)]
    pub data: Vec<i32>,
    #[offset(24)]
    pub tag: i32,
}
impl NonCopy {
    pub fn copy_from(this: Ptr<NonCopy>, _a0: Ptr<NonCopy>) {
        field!(this, data).write((*_a0.upgrade().deref()).data.clone());
        field!(this, tag).write(_a0.with(|__s| __s.tag));
    }
    pub fn move_from(this: Ptr<NonCopy>, _a0: Ptr<NonCopy>) {
        field!(this, data).write(field!(_a0, data).with_mut(std::mem::take));
        field!(this, tag).write({ (*_a0.upgrade().deref()).tag });
    }
    pub fn new_1(this: Ptr<NonCopy>) {
        field!(this, data).write(Vec::new());
        field!(this, tag).write(0);
    }
}
impl Default for NonCopy {
    fn default() -> Self {
        {
            NonCopy {
                data: Default::default(),
                tag: 0,
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[NonCopy]>> = Rc::new(RefCell::new(
        (0..3).map(|_| <NonCopy>::uninit()).collect::<Box<[_]>>(),
    ));
    {
        let __d = arr.decay();
        for __i in 0..3_isize {
            NonCopy::new_1(__d.offset(__i));
        }
    };
    (*arr.borrow_mut())[(0) as usize].tag = 7;
    {
        let __a1 = 42;
        (*arr.borrow_mut())[(1) as usize].data.push(__a1)
    };
    assert!(({ ({ (*arr.borrow())[(0) as usize].tag } == 7) }));
    assert!(({ ((*arr.borrow())[(1) as usize].data.len() == 1_usize) }));
    assert!(((vec_elem!(arr.decay().offset(1), data, 0_usize).read()) == 42));
    assert!(({ ({ (*arr.borrow())[(2) as usize].tag } == 0) }));
    assert!(({ ((*arr.borrow())[(2) as usize].data.len() == 0_usize) }));
    return 0;
}
pub trait NonCopyImpl {
    fn move_assign(&self, _a0: Ptr<NonCopy>) -> Ptr<NonCopy>;
}
impl NonCopyImpl for Ptr<NonCopy> {
    fn move_assign(&self, _a0: Ptr<NonCopy>) -> Ptr<NonCopy> {
        field!((*self), data).with_mut(|__b0| {
            (*__b0) = field!(_a0, data).with_mut(std::mem::take);
        });
        field!((*self), tag).write({ { (*_a0.upgrade().deref()).tag } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
