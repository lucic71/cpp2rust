extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct S {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: bool,
}
impl S {
    pub fn new(this: Ptr<S>) {
        field!(this, a).write(11);
        field!(this, b).write(true);
    }
}
impl Default for S {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
            S::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Declared {
    #[offset(0)]
    pub v: i32,
}
impl Declared {}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(16)]
    #[array_len(2)]
    pub items: Box<[S]>,
}
impl Holder {
    pub fn new_1(this: Ptr<Holder>) {
        {
            let __d = array_field_ptr!(this, items);
            for __i in 0..2_isize {
                S::new(__d.offset(__i));
            }
        };
    }
}
impl Default for Holder {
    fn default() -> Self {
        {
            Holder {
                items: (0..2).map(|_| <S>::default()).collect::<Box<[S]>>(),
            }
        }
    }
}
thread_local!(
    pub static kInit_0: Value<i32> = Rc::new(RefCell::new(3));
);
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct FromStatic {
    #[offset(0)]
    pub v: i32,
}
impl Default for FromStatic {
    fn default() -> Self {
        {
            FromStatic {
                v: kInit_0.with(|rc| *rc.borrow()),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut d: Ptr<Declared> = Ptr::<Declared>::null();
    assert!((d).is_null());
    let s: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    S::new(s.as_pointer());
    assert!(({ ({ (*s.borrow()).a } == 11) }));
    assert!(({ (({ (*s.borrow()).b } as i32) == (true as i32)) }));
    let mut h: Ptr<Holder> = Ptr::alloc_array(
        (0..1_usize)
            .map(|_| <Holder>::default())
            .collect::<Box<[Holder]>>(),
    );
    assert!((array_elem!(h.offset((0) as isize), items, 1).with(|__s| __s.a) == 11));
    h.delete();
    let mut fs: Ptr<FromStatic> = Ptr::alloc_array(
        (0..2_usize)
            .map(|_| <FromStatic>::default())
            .collect::<Box<[FromStatic]>>(),
    );
    assert!((elem!(fs, 1).with(|__s| __s.v) == 3));
    fs.delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = kInit_0.with(|_| ());
}
