extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static sequence_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn nextStamp_1() -> i32 {
    return (*sequence_0.with(Value::clone).borrow_mut()).prefix_inc();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(8)]
pub struct HasIntConstructor {
    #[offset(0)]
    pub value: i32,
    #[offset(4)]
    pub stamp: i32,
}
impl HasIntConstructor {
    pub fn new(mut v: i32) -> Self {
        Self {
            value: v,
            stamp: ({ nextStamp_1() }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct HasDefaultConstructor {
    #[offset(0)]
    pub stamp: i32,
}
impl HasDefaultConstructor {
    pub fn new() -> Self {
        Self {
            stamp: ({ nextStamp_1() }),
        }
    }
}
impl Default for HasDefaultConstructor {
    fn default() -> Self {
        { HasDefaultConstructor::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct InheritsConstructor {
    #[offset(8)]
    pub memberStamp: i32,
}
impl Default for InheritsConstructor {
    fn default() -> Self {
        {
            InheritsConstructor {
                memberStamp: ({ nextStamp_1() }),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct InheritsConstructorWithOtherBase {
    #[offset(12)]
    pub memberStamp: i32,
}
impl Default for InheritsConstructorWithOtherBase {
    fn default() -> Self {
        {
            InheritsConstructorWithOtherBase {
                memberStamp: ({ nextStamp_1() }),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<InheritsConstructor> = Rc::new(RefCell::new(InheritsConstructor::new_1({ 2 })));
    assert!(({ ({ ((*d.borrow()) as HasIntConstructor).value } == 2) }));
    assert!(({ ({ ((*d.borrow()) as HasIntConstructor).stamp } == 1) }));
    assert!(({ ({ (*d.borrow()).memberStamp } == 2) }));
    sequence_0.with(|rc| *rc.borrow_mut() = 0);
    let e: Value<InheritsConstructorWithOtherBase> =
        Rc::new(RefCell::new(InheritsConstructorWithOtherBase::new_1({ 3 })));
    assert!(({ ({ ((*e.borrow()) as HasIntConstructor).value } == 3) }));
    assert!(({ ({ ((*e.borrow()) as HasDefaultConstructor).stamp } == 1) }));
    assert!(({ ({ ((*e.borrow()) as HasIntConstructor).stamp } == 2) }));
    assert!(({ ({ (*e.borrow()).memberStamp } == 3) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = sequence_0.with(|_| ());
}
