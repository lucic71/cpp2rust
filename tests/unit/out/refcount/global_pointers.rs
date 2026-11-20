extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Entry {
    #[offset(0)]
    #[byte_size(8)]
    pub name: Ptr<i8>,
    #[offset(8)]
    #[byte_size(8)]
    pub p: Ptr<i32>,
}
impl Entry {
    pub fn new_1(this: Ptr<Entry>) {}
    pub fn copy_from(this: Ptr<Entry>, _a0: Ptr<Entry>) {
        field!(this, name).write(_a0.with(|__s| __s.name.clone()));
        field!(this, p).write(_a0.with(|__s| __s.p.clone()));
    }
    pub fn move_from(this: Ptr<Entry>, _a0: Ptr<Entry>) {
        field!(this, name).write({ (*_a0.upgrade().deref()).name.clone() });
        field!(this, p).write({ (*_a0.upgrade().deref()).p.clone() });
    }
}
thread_local!(
    pub static single_entry_0: Value<Entry> = {
        let __v: Value<Entry> = Rc::new(RefCell::new(<Entry>::uninit()));
        {
            let __d = __v.as_pointer();
            field!(__d, name).write(Ptr::<i8>::from_string_literal(b"alone"));
            field!(__d, p).write(Ptr::<i32>::null());
        };
        __v
    };
);
thread_local!(
    pub static entries_1: Value<Box<[Entry]>> = {
        let __v: Value<Box<[Entry]>> = Rc::new(RefCell::new(
            (0..2).map(|_| <Entry>::uninit()).collect::<Box<[_]>>(),
        ));
        {
            let __d = __v.decay();
            {
                let __d = __d.offset(0);
                field!(__d, name).write(Ptr::<i8>::from_string_literal(b"first"));
                field!(__d, p).write(Ptr::<i32>::null());
            };
            {
                let __d = __d.offset(1);
                field!(__d, name).write(Ptr::<i8>::from_string_literal(b"second"));
                field!(__d, p).write(Ptr::<i32>::null());
            };
        };
        __v
    };
);
thread_local!(
    pub static arr_of_pointers_2: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<i8>::null(),
        Ptr::<i8>::null(),
        Ptr::<i8>::null(),
    ])));
);
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    assert!(({ ({ (*single_entry_0.with(Value::clone).borrow()).p.clone() }).is_null() }));
    {
        let mut i: i32 = 0;
        'loop_: while (i < 2) {
            assert!(
                ({
                    ({
                        (*entries_1.with(Value::clone).borrow())[(i) as usize]
                            .p
                            .clone()
                    })
                    .is_null()
                })
            );
            assert!(
                ({
                    let __idx0 = (i) as usize;
                    arr_of_pointers_2.with(|rc| rc.borrow()[__idx0].clone())
                })
                .is_null()
            );
            i.prefix_inc();
        }
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = single_entry_0.with(|_| ());
    let _ = entries_1.with(|_| ());
    let _ = arr_of_pointers_2.with(|_| ());
}
