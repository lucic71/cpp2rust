extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut alive_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut copies_1: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut moves_2: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(
    VaArg,
    FnPtrArg,
    MoveCtorUnsafe,
    CppFromUnsafe,
    CopyAssignUnsafe,
    MoveAssignUnsafe,
    DestructorUnsafe,
    Default,
)]
#[cpp_from(i32, new, value)]
pub struct Buffer {
    pub data: [i32; 4],
    pub size: i32,
}
impl Buffer {
    pub unsafe fn new(this: *mut Buffer, mut size: i32) {
        (&raw mut (*this).size).write(size);
        {
            let mut i: i32 = 0;
            'loop_: while ((i) < (4)) {
                (*this).data[(i) as usize] = if ((i) < (size)) { i } else { -1_i32 };
                i.prefix_inc();
            }
        }
        (*(*&raw const alive_0).get()).prefix_inc();
    }
    pub unsafe fn destructor(&mut self) {
        (*(*&raw const alive_0).get()).prefix_dec();
    }
    pub unsafe fn copy_from(this: *mut Buffer, o: *const Buffer) {
        (&raw mut (*this).size).write((*o).size);
        {
            let mut i: i32 = 0;
            'loop_: while ((i) < (4)) {
                (*this).data[(i) as usize] = (*o).data[(i) as usize];
                i.prefix_inc();
            }
        }
        (*(*&raw const alive_0).get()).prefix_inc();
        (*(*&raw const copies_1).get()).prefix_inc();
    }
    pub unsafe fn move_from(this: *mut Buffer, o: *mut Buffer) {
        (&raw mut (*this).size).write((*o).size);
        {
            let mut i: i32 = 0;
            'loop_: while ((i) < (4)) {
                (*this).data[(i) as usize] = (*o).data[(i) as usize];
                (*o).data[(i) as usize] = -1_i32;
                i.prefix_inc();
            }
        }
        (*o).size = 0;
        (*(*&raw const alive_0).get()).prefix_inc();
        (*(*&raw const moves_2).get()).prefix_inc();
    }
    pub unsafe fn copy_assign(&mut self, o: *const Buffer) -> *mut Buffer {
        if (((self as *mut Buffer).cast_const()) == (o)) {
            return &mut (*(self as *mut Buffer));
        }
        self.size = (*o).size;
        {
            let mut i: i32 = 0;
            'loop_: while ((i) < (4)) {
                self.data[(i) as usize] = (*o).data[(i) as usize];
                i.prefix_inc();
            }
        }
        (*(*&raw const copies_1).get()).prefix_inc();
        return &mut (*(self as *mut Buffer));
    }
    pub unsafe fn move_assign(&mut self, o: *mut Buffer) -> *mut Buffer {
        if ((self as *mut Buffer) == (o)) {
            return &mut (*(self as *mut Buffer));
        }
        self.size = (*o).size;
        {
            let mut i: i32 = 0;
            'loop_: while ((i) < (4)) {
                self.data[(i) as usize] = (*o).data[(i) as usize];
                (*o).data[(i) as usize] = -1_i32;
                i.prefix_inc();
            }
        }
        (*o).size = 0;
        (*(*&raw const moves_2).get()).prefix_inc();
        return &mut (*(self as *mut Buffer));
    }
}
impl Clone for Buffer {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: Buffer = ::std::mem::MaybeUninit::uninit().assume_init();
            Buffer::copy_from(&raw mut __v, self as *const Buffer);
            __v
        }
    }
}
pub unsafe fn make_3(__ret: *mut Buffer, mut size: i32) {
    let mut b: Buffer = ::std::mem::MaybeUninit::uninit().assume_init();
    Buffer::new(&raw mut b, { size });
    let _dtor_b = std::mem::DropGuard::new(&raw mut b, |__p| unsafe { (*__p).destructor() });
    Buffer::move_from(__ret.clone(), { &mut b });
    return;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut a: Buffer = ::std::mem::MaybeUninit::uninit().assume_init();
        Buffer::new(&raw mut a, { 4 });
        let _dtor_a = std::mem::DropGuard::new(&raw mut a, |__p| unsafe { (*__p).destructor() });
        let mut b: Buffer = ::std::mem::MaybeUninit::uninit().assume_init();
        Buffer::copy_from(&raw mut b, { &a });
        let _dtor_b = std::mem::DropGuard::new(&raw mut b, |__p| unsafe { (*__p).destructor() });
        assert!(
            (((*(*&raw const alive_0).get()) == (2)) && ((*(*&raw const copies_1).get()) == (1)))
                && ((*(*&raw const moves_2).get()) == (0))
        );
        b.data[(0) as usize] = 100;
        assert!(((a.data[(0) as usize]) == (0)));
        let mut c: Buffer = ::std::mem::MaybeUninit::uninit().assume_init();
        Buffer::move_from(&raw mut c, { &mut a });
        let _dtor_c = std::mem::DropGuard::new(&raw mut c, |__p| unsafe { (*__p).destructor() });
        assert!(((*(*&raw const alive_0).get()) == (3)) && ((*(*&raw const moves_2).get()) == (1)));
        assert!(((a.size) == (0)) && ((a.data[(0) as usize]) == (-1_i32)));
        assert!(((c.size) == (4)) && ((c.data[(3) as usize]) == (3)));
        let mut d: Buffer = ::std::mem::MaybeUninit::uninit().assume_init();
        (unsafe { make_3(&raw mut d, 2) });
        let _dtor_d = std::mem::DropGuard::new(&raw mut d, |__p| unsafe { (*__p).destructor() });
        assert!(((d.size) == (2)) && ((*(*&raw const moves_2).get()) == (2)));
        (unsafe { Buffer::copy_assign(&mut d, &b) });
        assert!(
            (((d.size) == (4)) && ((d.data[(0) as usize]) == (100)))
                && ((*(*&raw const copies_1).get()) == (2))
        );
        (unsafe { Buffer::move_assign(&mut d, &mut c) });
        assert!(
            (((d.data[(0) as usize]) == (0)) && ((c.size) == (0)))
                && ((*(*&raw const moves_2).get()) == (3))
        );
        (unsafe {
            let _o: *mut Buffer = &mut d;
            Buffer::move_assign(&mut d, _o)
        });
        assert!(((d.size) == (4)) && ((*(*&raw const moves_2).get()) == (3)));
    }
    assert!(((*(*&raw const alive_0).get()) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const alive_0).get();
    (*&raw const copies_1).get();
    (*&raw const moves_2).get();
}
