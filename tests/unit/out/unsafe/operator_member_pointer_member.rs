extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Inner {
    pub x: i32,
}
impl Inner {
    pub unsafe fn new_1(this: *mut Inner) {}
    pub unsafe fn copy_from(this: *mut Inner, _a0: *const Inner) {
        (&raw mut (*this).x).write((*_a0).x);
    }
    pub unsafe fn move_from(this: *mut Inner, _a0: *mut Inner) {
        (&raw mut (*this).x).write((*_a0).x);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Table {}
impl Table {
    pub unsafe fn operator_index(mut i: i32) -> *mut i32 {
        return &mut (*(*&raw const table_0).get())[(i) as usize];
    }
    pub unsafe fn new_1(this: *mut Table) {}
    pub unsafe fn copy_from(this: *mut Table, _a0: *const Table) {}
    pub unsafe fn move_from(this: *mut Table, _a0: *mut Table) {}
}
pub static mut table_0: libcc2rs::LazyStorage<[i32; 3]> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write([7, 8, 9]) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub data: [i32; 3],
    pub inner: Inner,
}
impl S {
    pub unsafe fn operator_index_1(&mut self, mut i: i32) -> *mut i32 {
        return &mut self.data[(i) as usize];
    }
    pub unsafe fn operator_index_2(&self, mut i: i32) -> *const i32 {
        return &self.data[(i) as usize];
    }
    pub unsafe fn operator_deref(&mut self) -> *mut Inner {
        return &mut self.inner;
    }
    pub unsafe fn operator_arrow(&mut self) -> *mut Inner {
        return (&mut self.inner as *mut Inner);
    }
    pub unsafe fn operator_addr(&mut self) -> *mut i32 {
        return (&mut self.data[(0) as usize] as *mut i32);
    }
    pub unsafe fn new_1(this: *mut S) {
        Inner::new_1((&raw mut (*this).inner));
    }
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).data).write(std::array::from_fn::<_, 3, _>(|__i: usize| {
            (*_a0).data[(__i)]
        }));
        Inner::copy_from((&raw mut (*this).inner), { &(*_a0).inner });
    }
    pub unsafe fn move_from(this: *mut S, _a0: *mut S) {
        (&raw mut (*this).data).write(std::array::from_fn::<_, 3, _>(|__i: usize| {
            (*_a0).data[(__i)]
        }));
        Inner::move_from((&raw mut (*this).inner), { &mut (*_a0).inner });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut s;
        (&raw mut (*__d).data).write([1, 2, 3]);
        {
            let __d = (&raw mut (*__d).inner);
            (&raw mut (*__d).x).write(9);
        };
    };
    assert!(((*(unsafe { S::operator_index_1(&mut s, 1,) })) == (2)));
    (*(unsafe { S::operator_index_1(&mut s, 1) })) = 20;
    assert!(((*(unsafe { S::operator_index_1(&mut s, 1,) })) == (20)));
    let cs: *const S = &s;
    assert!(((*(unsafe { S::operator_index_2(&(*cs), 2,) })) == (3)));
    assert!((((*(unsafe { S::operator_deref(&mut s,) })).x) == (9)));
    (*(unsafe { S::operator_deref(&mut s) })).x = 10;
    assert!((((*(unsafe { S::operator_arrow(&mut s,) })).x) == (10)));
    (*(unsafe { S::operator_arrow(&mut s) })).x = 11;
    assert!(((s.inner.x) == (11)));
    let mut p: *mut i32 = (unsafe { S::operator_addr(&mut s) });
    assert!(((*p) == (1)));
    (*p) = 5;
    assert!(((s.data[(0) as usize]) == (5)));
    let mut t: Table = ::std::mem::MaybeUninit::uninit().assume_init();
    Table::new_1(&raw mut t);
    assert!(((*(unsafe { Table::operator_index(1,) })) == (8)));
    (*(unsafe { Table::operator_index(1) })) = 80;
    assert!((((*(*&raw const table_0).get())[(1) as usize]) == (80)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const table_0).get();
}
