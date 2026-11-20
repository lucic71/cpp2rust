extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(144)]
pub struct Pointers {
    #[offset(0)]
    #[byte_size(8)]
    pub x1: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub x2: Ptr<i32>,
    #[offset(16)]
    #[byte_size(40)]
    #[array_len(5)]
    pub x3: Box<[Ptr<i32>]>,
    #[offset(56)]
    #[byte_size(80)]
    #[array_len(10)]
    pub x4: Box<[Ptr<i32>]>,
    #[offset(136)]
    pub x5: i32,
}
impl Pointers {
    pub fn new_1(this: Ptr<Pointers>) {}
    pub fn copy_from(this: Ptr<Pointers>, _a0: Ptr<Pointers>) {
        field!(this, x1).write(_a0.with(|__s| __s.x1.clone()));
        field!(this, x2).write(_a0.with(|__s| __s.x2.clone()));
        field!(this, x3).write(Box::new(std::array::from_fn::<_, 5, _>(|__i: usize| {
            ((*_a0.upgrade().deref()).x3[(__i) as usize]).clone()
        })));
        field!(this, x4).write(Box::new(std::array::from_fn::<_, 10, _>(|__i: usize| {
            ((*_a0.upgrade().deref()).x4[(__i) as usize]).clone()
        })));
        field!(this, x5).write(_a0.with(|__s| __s.x5));
    }
}
impl Default for Pointers {
    fn default() -> Self {
        {
            Pointers {
                x1: Ptr::<i32>::null(),
                x2: Ptr::<i32>::null(),
                x3: (0..5)
                    .map(|_| Ptr::<i32>::null())
                    .collect::<Box<[Ptr<i32>]>>(),
                x4: (0..10)
                    .map(|_| Ptr::<i32>::null())
                    .collect::<Box<[Ptr<i32>]>>(),
                x5: 0_i32,
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(440)]
pub struct SmallArrays {
    #[offset(0)]
    #[byte_size(128)]
    #[array_len(32)]
    pub a: Box<[i32]>,
    #[offset(128)]
    #[byte_size(8)]
    pub f: FnPtr<fn(i32) -> i32>,
    #[offset(136)]
    #[byte_size(16)]
    #[array_len(2)]
    pub fs: Box<[FnPtr<fn(i32) -> i32>]>,
    #[offset(152)]
    #[byte_size(288)]
    #[array_len(2)]
    pub p: Box<[Pointers]>,
}
impl SmallArrays {
    pub fn new_1(this: Ptr<SmallArrays>) {
        {
            let __d = array_field_ptr!(this, p);
            for __i in 0..2_isize {
                Pointers::new_1(__d.offset(__i));
            }
        };
    }
    pub fn copy_from(this: Ptr<SmallArrays>, _a0: Ptr<SmallArrays>) {
        field!(this, a).write(Box::new(std::array::from_fn::<_, 32, _>(|__i: usize| {
            (*_a0.upgrade().deref()).a[(__i) as usize]
        })));
        field!(this, f).write(_a0.with(|__s| __s.f.clone()));
        field!(this, fs).write(Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
            ((*_a0.upgrade().deref()).fs[(__i) as usize]).clone()
        })));
        {
            let __d = array_field_ptr!(this, p);
            for __i in 0..2_usize {
                Pointers::copy_from(__d.offset(__i as isize), {
                    array_field_ptr!(_a0, p).offset(__i)
                });
            }
        };
    }
}
impl Default for SmallArrays {
    fn default() -> Self {
        {
            SmallArrays {
                a: (0..32).map(|_| 0_i32).collect::<Box<[i32]>>(),
                f: FnPtr::<fn(i32) -> i32>::null(),
                fs: (0..2)
                    .map(|_| FnPtr::<fn(i32) -> i32>::null())
                    .collect::<Box<[FnPtr<fn(i32) -> i32>]>>(),
                p: (0..2)
                    .map(|_| <Pointers>::default())
                    .collect::<Box<[Pointers]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(132)]
pub struct BigArray {
    #[offset(0)]
    #[byte_size(132)]
    #[array_len(33)]
    pub a: Box<[i32]>,
}
impl BigArray {
    pub fn new_1(this: Ptr<BigArray>) {}
    pub fn copy_from(this: Ptr<BigArray>, _a0: Ptr<BigArray>) {
        field!(this, a).write(Box::new(std::array::from_fn::<_, 33, _>(|__i: usize| {
            (*_a0.upgrade().deref()).a[(__i) as usize]
        })));
    }
}
impl Default for BigArray {
    fn default() -> Self {
        {
            BigArray {
                a: (0..33).map(|_| 0_i32).collect::<Box<[i32]>>(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut default_pointers: Ptr<Pointers> = Ptr::alloc_array(
        (0..10_usize)
            .map(|_| <Pointers>::default())
            .collect::<Box<[Pointers]>>(),
    );
    default_pointers.delete();
    let mut small: Ptr<SmallArrays> = Ptr::alloc_array(
        (0..2_usize)
            .map(|_| <SmallArrays>::default())
            .collect::<Box<[SmallArrays]>>(),
    );
    small.delete();
    let mut big: Ptr<BigArray> = Ptr::alloc_array(
        (0..2_usize)
            .map(|_| <BigArray>::default())
            .collect::<Box<[BigArray]>>(),
    );
    big.delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
