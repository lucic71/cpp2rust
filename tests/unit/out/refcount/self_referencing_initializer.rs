extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct list_head {
    #[offset(0)]
    #[byte_size(8)]
    pub next: Ptr<list_head>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct node {
    #[offset(0)]
    pub value: (),
    #[offset(8)]
    #[byte_size(8)]
    pub value_ptr: Ptr<()>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct node_with_defaults {
    #[offset(0)]
    #[byte_size(8)]
    pub next: Ptr<node_with_defaults>,
    #[offset(8)]
    pub value: (),
}
impl Default for node_with_defaults {
    fn default() -> Self {
        {
            node_with_defaults {
                next: Ptr::<node_with_defaults>::null(),
                value: 3_(),
            }
        }
    }
}
pub fn init_0(l: Ptr<list_head>) -> Ptr<list_head> {
    let l: Value<Ptr<list_head>> = Rc::new(RefCell::new(l));
    field!((*l.borrow()), next).write(Ptr::<list_head>::null());
    return (*l.borrow()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let list: Value<list_head> = <Value<list_head>>::default();
    *list.borrow_mut() = list_head {
        next: (list.as_pointer()),
    };
    unimplemented!();
    let n: Value<node> = <Value<node>>::default();
    *n.borrow_mut() = node {
        value: 42_(),
        value_ptr: (field_ptr!(n.as_pointer(), value)),
    };
    unimplemented!();
    { (*n.borrow()).value_ptr.clone() }.write(7_());
    unimplemented!();
    let arr: Value<Box<[list_head]>> = Rc::new(RefCell::new(
        (0..2)
            .map(|_| <list_head>::default())
            .collect::<Box<[list_head]>>(),
    ));
    *arr.borrow_mut() = Box::new([
        list_head {
            next: ((arr.as_pointer() as Ptr<list_head>).offset(1_())),
        },
        list_head {
            next: ((arr.as_pointer() as Ptr<list_head>).offset(0_())),
        },
    ]);
    unimplemented!();
    unimplemented!();
    let d: Value<node_with_defaults> = <Value<node_with_defaults>>::default();
    *d.borrow_mut() = node_with_defaults {
        next: (d.as_pointer()),
        value: 3_(),
    };
    unimplemented!();
    unimplemented!();
    let called: Value<list_head> = <Value<list_head>>::default();
    *called.borrow_mut() = list_head {
        next: ({ init_0((called.as_pointer())) }),
    };
    unimplemented!();
    let mut size: () = ::std::mem::size_of::<()>();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
