extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct StackArray {
    #[offset(0)]
    #[byte_size(24)]
    #[array_len(3)]
    pub arr: Box<[Ptr<i32>]>,
}
impl StackArray {
    pub fn copy_from(this: Ptr<StackArray>, _a0: Ptr<StackArray>) {
        field!(this, arr).write(Box::new(std::array::from_fn::<_, 3, _>(|__i: usize| {
            ((*_a0.upgrade().deref()).arr[(__i) as usize]).clone()
        })));
    }
}
impl Default for StackArray {
    fn default() -> Self {
        {
            StackArray {
                arr: (0..3)
                    .map(|_| Ptr::<i32>::null())
                    .collect::<Box<[Ptr<i32>]>>(),
            }
        }
    }
}
pub fn IncrementAll_0(s: Ptr<StackArray>) {
    {
        let mut i: i32 = 0;
        'loop_: while (i < 3) {
            {
                (array_elem!(s, arr, i).read()).with_mut(|__v| *__v = *__v + 1)
            };
            i.prefix_inc();
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(0));
    let s: Value<StackArray> = Rc::new(RefCell::new(<StackArray>::uninit()));
    {
        let __d = s.as_pointer();
        field!(__d, arr).write(Box::new([
            (x.as_pointer()),
            (x.as_pointer()),
            (x.as_pointer()),
        ]));
    };
    ({ IncrementAll_0(s.as_pointer()) });
    assert!(({ ((*x.borrow()) == 3) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
