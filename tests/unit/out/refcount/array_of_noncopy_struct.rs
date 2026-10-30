extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(32)]
pub struct NonCopy {
    #[offset(0)]
    #[byte_size(24)]
    pub data: Vec<i32>,
    #[offset(24)]
    pub tag: i32,
}
impl NonCopy {
    pub fn move_from(_a0: Ptr<NonCopy>) -> Self {
        Self {
            data: field!(_a0, data).with_mut(std::mem::take),
            tag: { (*_a0.upgrade().deref()).tag },
        }
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
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[NonCopy]>> = Rc::new(RefCell::new(
        (0..3)
            .map(|_| <NonCopy>::default())
            .collect::<Box<[NonCopy]>>(),
    ));
    (*arr.borrow_mut())[(0) as usize].tag = 7;
    {
        let __a1 = 42;
        (*arr.borrow_mut())[(1) as usize].data.push(__a1)
    };
    assert!(({ (*arr.borrow())[(0) as usize].tag } == 7));
    assert!(((*arr.borrow())[(1) as usize].data.len() == 1_usize));
    assert!(
        ((field_ref!((arr.as_pointer() as Ptr<NonCopy>).offset(1), data)
            .decay()
            .offset(0_usize as isize)
            .read())
            == 42)
    );
    assert!(({ (*arr.borrow())[(2) as usize].tag } == 0));
    assert!(((*arr.borrow())[(2) as usize].data.len() == 0_usize));
    return 0;
}
pub trait NonCopyImpl {
    fn move_assign(&self, _a0: Ptr<NonCopy>) -> Ptr<NonCopy>;
}
impl NonCopyImpl for Ptr<NonCopy> {
    fn move_assign(&self, _a0: Ptr<NonCopy>) -> Ptr<NonCopy> {
        field_ref!((*self), data)
            .as_pointer()
            .write(field!(_a0, data).with_mut(std::mem::take));
        field!((*self), tag).write({ { (*_a0.upgrade().deref()).tag } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
