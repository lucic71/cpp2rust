extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct list_head {
    #[offset(0)]
    #[byte_size(8)]
    pub next: Ptr<list_head>,
}
impl list_head {
    pub fn new_1(this: Ptr<list_head>) {}
    pub fn copy_from(this: Ptr<list_head>, _a0: Ptr<list_head>) {
        field!(this, next).write(_a0.with(|__s| __s.next.clone()));
    }
    pub fn move_from(this: Ptr<list_head>, _a0: Ptr<list_head>) {
        field!(this, next).write({ (*_a0.upgrade().deref()).next.clone() });
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct node {
    #[offset(0)]
    pub value: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub value_ptr: Ptr<i32>,
}
impl node {
    pub fn new_1(this: Ptr<node>) {}
    pub fn copy_from(this: Ptr<node>, _a0: Ptr<node>) {
        field!(this, value).write(_a0.with(|__s| __s.value));
        field!(this, value_ptr).write(_a0.with(|__s| __s.value_ptr.clone()));
    }
    pub fn move_from(this: Ptr<node>, _a0: Ptr<node>) {
        field!(this, value).write({ (*_a0.upgrade().deref()).value });
        field!(this, value_ptr).write({ (*_a0.upgrade().deref()).value_ptr.clone() });
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct node_with_defaults {
    #[offset(0)]
    #[byte_size(8)]
    pub next: Ptr<node_with_defaults>,
    #[offset(8)]
    pub value: i32,
}
impl node_with_defaults {
    pub fn new_1(this: Ptr<node_with_defaults>) {
        field!(this, next).write(Ptr::<node_with_defaults>::null());
        field!(this, value).write(3);
    }
    pub fn copy_from(this: Ptr<node_with_defaults>, _a0: Ptr<node_with_defaults>) {
        field!(this, next).write(_a0.with(|__s| __s.next.clone()));
        field!(this, value).write(_a0.with(|__s| __s.value));
    }
    pub fn move_from(this: Ptr<node_with_defaults>, _a0: Ptr<node_with_defaults>) {
        field!(this, next).write({ (*_a0.upgrade().deref()).next.clone() });
        field!(this, value).write({ (*_a0.upgrade().deref()).value });
    }
}
impl Default for node_with_defaults {
    fn default() -> Self {
        {
            node_with_defaults {
                next: Ptr::<node_with_defaults>::null(),
                value: 3,
            }
        }
    }
}
pub fn init_0(mut l: Ptr<list_head>) -> Ptr<list_head> {
    field!(l, next).write(Ptr::<list_head>::null());
    return l;
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let list: Value<list_head> = Rc::new(RefCell::new(<list_head>::uninit()));
    {
        let __d = list.as_pointer();
        field!(__d, next).write((list.as_pointer()));
    };
    assert!(({ ({ { (*list.borrow()).next.clone() } } == { (list.as_pointer()) }) }));
    let n: Value<node> = Rc::new(RefCell::new(<node>::uninit()));
    {
        let __d = n.as_pointer();
        field!(__d, value).write(42);
        field!(__d, value_ptr).write((field_ptr!(n.as_pointer(), value)));
    };
    assert!(
        ({ ({ { (*n.borrow()).value_ptr.clone() } } == { (field_ptr!(n.as_pointer(), value)) }) })
    );
    { (*n.borrow()).value_ptr.clone() }.write(7);
    assert!(({ ({ (*n.borrow()).value } == 7) }));
    let arr: Value<Box<[list_head]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <list_head>::uninit()).collect::<Box<[_]>>(),
    ));
    {
        let __d = arr.decay();
        {
            let __d = __d.offset(0);
            field!(__d, next).write((arr.decay().offset(1)));
        };
        {
            let __d = __d.offset(1);
            field!(__d, next).write((arr.decay().offset(0)));
        };
    };
    assert!(
        ({ ({ { (*arr.borrow())[(0) as usize].next.clone() } } == { (arr.decay().offset(1)) }) })
    );
    assert!(
        ({ ({ { (*arr.borrow())[(1) as usize].next.clone() } } == { (arr.decay().offset(0)) }) })
    );
    let d: Value<node_with_defaults> = Rc::new(RefCell::new(<node_with_defaults>::uninit()));
    {
        let __d = d.as_pointer();
        field!(__d, next).write((d.as_pointer()));
        field!(__d, value).write(3);
    };
    assert!(({ ({ { (*d.borrow()).next.clone() } } == { (d.as_pointer()) }) }));
    assert!(({ ({ (*d.borrow()).value } == 3) }));
    let called: Value<list_head> = Rc::new(RefCell::new(<list_head>::uninit()));
    {
        let __d = called.as_pointer();
        field!(__d, next).write(({ init_0((called.as_pointer())) }));
    };
    assert!(({ ({ { (*called.borrow()).next.clone() } } == { (called.as_pointer()) }) }));
    let mut size: usize = ::std::mem::size_of::<usize>();
    assert!((size == ::std::mem::size_of::<usize>()));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
