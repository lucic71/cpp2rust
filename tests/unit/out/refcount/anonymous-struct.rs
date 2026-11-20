extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Outer_Named {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
impl Outer_Named {
    pub fn copy_from(this: Ptr<Outer_Named>, _a0: Ptr<Outer_Named>) {
        field!(this, a).write(_a0.with(|__s| __s.a));
        field!(this, b).write(_a0.with(|__s| __s.b));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    pub c: i32,
    #[offset(4)]
    pub d: i32,
}
impl anon_0 {
    pub fn copy_from(this: Ptr<anon_0>, _a0: Ptr<anon_0>) {
        field!(this, c).write(_a0.with(|__s| __s.c));
        field!(this, d).write(_a0.with(|__s| __s.d));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct anon_1 {
    #[offset(0)]
    pub g: i32,
    #[offset(4)]
    pub h: i32,
}
impl anon_1 {
    pub fn copy_from(this: Ptr<anon_1>, _a0: Ptr<anon_1>) {
        field!(this, g).write(_a0.with(|__s| __s.g));
        field!(this, h).write(_a0.with(|__s| __s.h));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct anon_2 {
    #[offset(0)]
    pub e: i32,
    #[offset(4)]
    pub f: i32,
}
impl anon_2 {
    pub fn copy_from(this: Ptr<anon_2>, _a0: Ptr<anon_2>) {
        field!(this, e).write(_a0.with(|__s| __s.e));
        field!(this, f).write(_a0.with(|__s| __s.f));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct anon_4 {
    #[offset(0)]
    pub j: i32,
}
impl anon_4 {
    pub fn copy_from(this: Ptr<anon_4>, _a0: Ptr<anon_4>) {
        field!(this, j).write(_a0.with(|__s| __s.j));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct anon_5 {
    #[offset(0)]
    pub k: i32,
}
impl anon_5 {
    pub fn copy_from(this: Ptr<anon_5>, _a0: Ptr<anon_5>) {
        field!(this, k).write(_a0.with(|__s| __s.k));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct anon_3 {
    #[offset(0)]
    pub i: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub inner_named: anon_4,
    #[offset(8)]
    #[byte_size(4)]
    pub anon_5: anon_5,
}
impl anon_3 {
    pub fn copy_from(this: Ptr<anon_3>, _a0: Ptr<anon_3>) {
        field!(this, i).write(_a0.with(|__s| __s.i));
        anon_4::copy_from(field_ptr!(this, inner_named), {
            field_ptr!(_a0, inner_named)
        });
        anon_5::copy_from(field_ptr!(this, anon_5), { field_ptr!(_a0, anon_5) });
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(44)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(8)]
    pub named: Outer_Named,
    #[offset(8)]
    #[byte_size(8)]
    pub anonymous_named_0: anon_0,
    #[offset(16)]
    #[byte_size(8)]
    pub anonymous_named_1: anon_1,
    #[offset(24)]
    #[byte_size(8)]
    pub anon_2: anon_2,
    #[offset(32)]
    #[byte_size(12)]
    pub anon_3: anon_3,
}
impl Outer {
    pub fn copy_from(this: Ptr<Outer>, _a0: Ptr<Outer>) {
        Outer_Named::copy_from(field_ptr!(this, named), { field_ptr!(_a0, named) });
        anon_0::copy_from(field_ptr!(this, anonymous_named_0), {
            field_ptr!(_a0, anonymous_named_0)
        });
        anon_1::copy_from(field_ptr!(this, anonymous_named_1), {
            field_ptr!(_a0, anonymous_named_1)
        });
        anon_2::copy_from(field_ptr!(this, anon_2), { field_ptr!(_a0, anon_2) });
        anon_3::copy_from(field_ptr!(this, anon_3), { field_ptr!(_a0, anon_3) });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Outer> = Rc::new(RefCell::new(<Outer>::uninit()));
    {
        let __d = o.as_pointer();
        {
            let __d = field_ptr!(__d, named);
            field!(__d, a).write(0_i32);
            field!(__d, b).write(0_i32);
        };
        {
            let __d = field_ptr!(__d, anonymous_named_0);
            field!(__d, c).write(0_i32);
            field!(__d, d).write(0_i32);
        };
        {
            let __d = field_ptr!(__d, anonymous_named_1);
            field!(__d, g).write(0_i32);
            field!(__d, h).write(0_i32);
        };
        {
            let __d = field_ptr!(__d, anon_2);
            field!(__d, e).write(0_i32);
            field!(__d, f).write(0_i32);
        };
        {
            let __d = field_ptr!(__d, anon_3);
            field!(__d, i).write(0_i32);
            {
                let __d = field_ptr!(__d, inner_named);
                field!(__d, j).write(0_i32);
            };
            {
                let __d = field_ptr!(__d, anon_5);
                field!(__d, k).write(0_i32);
            };
        };
    };
    (*o.borrow_mut()).named.a = 1;
    (*o.borrow_mut()).named.b = 2;
    (*o.borrow_mut()).anonymous_named_0.c = 3;
    (*o.borrow_mut()).anonymous_named_0.d = 4;
    (*o.borrow_mut()).anonymous_named_1.g = 5;
    (*o.borrow_mut()).anonymous_named_1.h = 6;
    (*o.borrow_mut()).anon_2.e = 7;
    (*o.borrow_mut()).anon_2.f = 8;
    (*o.borrow_mut()).anon_3.i = 9;
    (*o.borrow_mut()).anon_3.inner_named.j = 10;
    (*o.borrow_mut()).anon_3.anon_5.k = 11;
    assert!(({ ({ (*o.borrow()).named.a } == 1) }));
    assert!(({ ({ (*o.borrow()).named.b } == 2) }));
    assert!(({ ({ (*o.borrow()).anonymous_named_0.c } == 3) }));
    assert!(({ ({ (*o.borrow()).anonymous_named_0.d } == 4) }));
    assert!(({ ({ (*o.borrow()).anonymous_named_1.g } == 5) }));
    assert!(({ ({ (*o.borrow()).anonymous_named_1.h } == 6) }));
    assert!(({ ({ (*o.borrow()).anon_2.e } == 7) }));
    assert!(({ ({ (*o.borrow()).anon_2.f } == 8) }));
    assert!(({ ({ (*o.borrow()).anon_3.i } == 9) }));
    assert!(({ ({ (*o.borrow()).anon_3.inner_named.j } == 10) }));
    assert!(({ ({ (*o.borrow()).anon_3.anon_5.k } == 11) }));
    let s: Value<anon_6> = Rc::new(RefCell::new(<anon_6>::uninit()));
    anon_6::new_1(s.as_pointer());
    (*s.borrow_mut()).x = 1;
    (*s.borrow_mut()).z = 2;
    assert!(
        ({
            ({
                (*s.borrow_mut()).x = 1;
                { (*s.borrow()).x }
            } != 0)
        })
    );
    assert!(
        ({
            ({
                (*s.borrow_mut()).z = 2;
                { (*s.borrow()).z }
            } != 0)
        })
    );
    return 0;
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct anon_6 {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub z: i32,
}
impl anon_6 {
    pub fn new_1(this: Ptr<anon_6>) {}
    pub fn copy_from(this: Ptr<anon_6>, _a0: Ptr<anon_6>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
        field!(this, z).write(_a0.with(|__s| __s.z));
    }
}
pub fn __cpp2rust_init_globals() {}
