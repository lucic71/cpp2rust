extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Inner {
    #[offset(0)]
    pub x: i32,
}
impl Inner {
    pub fn copy_from(this: Ptr<Inner>, _a0: Ptr<Inner>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct S {
    #[offset(0)]
    #[byte_size(12)]
    #[array_len(3)]
    pub data: Box<[i32]>,
    #[offset(12)]
    #[byte_size(4)]
    pub inner: Inner,
}
impl S {
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, data).write(Box::new(std::array::from_fn::<_, 3, _>(|__i: usize| {
            (*_a0.upgrade().deref()).data[(__i) as usize]
        })));
        Inner::copy_from(field_ptr!(this, inner), { field_ptr!(_a0, inner) });
    }
}
impl Default for S {
    fn default() -> Self {
        {
            S {
                data: (0..3).map(|_| 0_i32).collect::<Box<[i32]>>(),
                inner: <Inner>::default(),
            }
        }
    }
}
pub fn operator_deref_0(s: Ptr<S>) -> Ptr<Inner> {
    return field_ptr!(s, inner);
}
pub fn operator_addr_1(s: Ptr<S>) -> Ptr<i32> {
    return (array_field_ptr!(s, data).offset((0) as isize));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = s.as_pointer();
        field!(__d, data).write(Box::new([1, 2, 3]));
        {
            let __d = field_ptr!(__d, inner);
            field!(__d, x).write(9);
        };
    };
    assert!(
        (({
            let _s: Ptr<S> = s.as_pointer();
            operator_deref_0(_s)
        })
        .with(|__s| (__s).x)
            == 9)
    );
    field!(
        ({
            let _s: Ptr<S> = s.as_pointer();
            operator_deref_0(_s)
        }),
        x
    )
    .write(10);
    assert!(({ ({ (*s.borrow()).inner.x } == 10) }));
    let mut p: Ptr<i32> = ({
        let _s: Ptr<S> = s.as_pointer();
        operator_addr_1(_s)
    });
    assert!(((p.read()) == 1));
    p.write(5);
    assert!(((array_elem!(s.as_pointer(), data, 0).read()) == 5));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
