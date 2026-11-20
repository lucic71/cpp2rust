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
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, DestructorUnsafe, Default)]
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
pub unsafe fn sum_2(b: *const Buffer) -> i32 {
    let mut s: i32 = 0;
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < ((*b).size)) {
            s += (*b).data[(i) as usize];
            i.prefix_inc();
        }
    }
    return s;
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
            ((*(*&raw const alive_0).get()) == (2)) && ((*(*&raw const copies_1).get()) == (1))
        );
        b.data[(0) as usize] = 100;
        assert!(((a.data[(0) as usize]) == (0)));
        let mut c: Buffer = ::std::mem::MaybeUninit::uninit().assume_init();
        Buffer::new(&raw mut c, { 2 });
        let _dtor_c = std::mem::DropGuard::new(&raw mut c, |__p| unsafe { (*__p).destructor() });
        (unsafe { Buffer::copy_assign(&mut c, &a) });
        assert!(((c.size) == (4)) && ((c.data[(3) as usize]) == (3)));
        assert!(
            ((*(*&raw const alive_0).get()) == (3)) && ((*(*&raw const copies_1).get()) == (2))
        );
        (unsafe {
            let _o: *const Buffer = &c;
            Buffer::copy_assign(&mut c, _o)
        });
        assert!(((*(*&raw const copies_1).get()) == (2)));
        assert!(((unsafe { sum_2(&a,) }) == (6)));
        assert!(((unsafe { sum_2(&b,) }) == (106)));
        let mut d: Buffer = ::std::mem::MaybeUninit::uninit().assume_init();
        Buffer::copy_from(&raw mut d, { &a });
        let _dtor_d = std::mem::DropGuard::new(&raw mut d, |__p| unsafe { (*__p).destructor() });
        assert!(
            ((*(*&raw const alive_0).get()) == (4)) && ((*(*&raw const copies_1).get()) == (3))
        );
        assert!(((a.size) == (4)) && ((a.data[(3) as usize]) == (3)));
        (unsafe { Buffer::copy_assign(&mut d, &b) });
        assert!(((*(*&raw const copies_1).get()) == (4)));
        assert!(((b.data[(0) as usize]) == (100)) && ((d.data[(0) as usize]) == (100)));
    }
    assert!(((*(*&raw const alive_0).get()) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const alive_0).get();
    (*&raw const copies_1).get();
}
