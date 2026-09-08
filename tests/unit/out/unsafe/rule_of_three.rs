extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut alive_0: i32 = unsafe { 0 };
pub static mut copies_1: i32 = unsafe { 0 };
#[repr(C)]
#[derive(Clone, Default)]
pub struct Buffer {
    pub data: *mut i32,
    pub size: i32,
}
impl Buffer {
    pub unsafe fn Buffer(mut size: i32) -> Self {
        let mut this = Self {
            data: Box::leak((0..(size as usize)).map(|_| 0_i32).collect::<Box<[i32]>>())
                .as_mut_ptr(),
            size: size,
        };
        let mut i: i32 = 0;
        'loop_: while ((i) < (size)) {
            (*this.data.offset((i) as isize)) = i;
            i.prefix_inc();
        }
        alive_0.prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
            self.data,
            libcc2rs::malloc_usable_size(self.data as *mut ::libc::c_void)
                / ::std::mem::size_of::<i32>(),
        )));
        alive_0.prefix_dec();
    }
    pub unsafe fn Buffer_pconstBuffer(o: *const Buffer) -> Self {
        let mut this = Self {
            data: Box::leak(
                (0..((*o).size as usize))
                    .map(|_| 0_i32)
                    .collect::<Box<[i32]>>(),
            )
            .as_mut_ptr(),
            size: (*o).size,
        };
        let mut i: i32 = 0;
        'loop_: while ((i) < (this.size)) {
            (*this.data.offset((i) as isize)) = (*(*o).data.offset((i) as isize));
            i.prefix_inc();
        }
        alive_0.prefix_inc();
        copies_1.prefix_inc();
        this
    }
    pub unsafe fn operator_assign(&mut self, o: *const Buffer) -> *mut Buffer {
        if ((self) == (o)) {
            return &mut (*self) as *mut Buffer;
        }
        ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
            self.data,
            libcc2rs::malloc_usable_size(self.data as *mut ::libc::c_void)
                / ::std::mem::size_of::<i32>(),
        )));
        self.data = Box::leak(
            (0..((*o).size as usize))
                .map(|_| 0_i32)
                .collect::<Box<[i32]>>(),
        );
        self.size = (*o).size;
        let mut i: i32 = 0;
        'loop_: while ((i) < (self.size)) {
            (*self.data.offset((i) as isize)) = (*(*o).data.offset((i) as isize));
            i.prefix_inc();
        }
        copies_1.prefix_inc();
        return &mut (*self) as *mut Buffer;
    }
}
pub unsafe fn sum_2(mut b: Buffer) -> i32 {
    let mut s: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (b.size)) {
        s += (*b.data.offset((i) as isize));
        i.prefix_inc();
    }
    return s;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut a: Buffer = Buffer::Buffer({ 4 });
        let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, Buffer::destructor);
        let mut b: Buffer = a.clone();
        let _dtor_b = ScopedDestructorUnsafe::new(&raw mut b, Buffer::destructor);
        assert!(((alive_0) == (2)) && ((copies_1) == (1)));
        (*b.data.offset((0) as isize)) = 100;
        assert!(((*a.data.offset((0) as isize)) == (0)));
        let mut c: Buffer = Buffer::Buffer({ 2 });
        let _dtor_c = ScopedDestructorUnsafe::new(&raw mut c, Buffer::destructor);
        (unsafe { Buffer::operator_assign(&mut c, &a as *const Buffer) });
        assert!(((c.size) == (4)) && ((*c.data.offset((3) as isize)) == (3)));
        assert!(((alive_0) == (3)) && ((copies_1) == (2)));
        (unsafe {
            let _o: *const Buffer = &c as *const Buffer;
            Buffer::operator_assign(&mut c, _o)
        });
        assert!(((copies_1) == (2)));
        assert!(((unsafe { sum_2(a.clone(),) }) == (6)));
        assert!(((alive_0) == (3)) && ((copies_1) == (3)));
        let mut d: Buffer = a.clone();
        let _dtor_d = ScopedDestructorUnsafe::new(&raw mut d, Buffer::destructor);
        assert!(((alive_0) == (4)) && ((copies_1) == (4)));
        assert!((!((a.data).is_null())) && ((a.size) == (4)));
        (unsafe { Buffer::operator_assign(&mut d, &mut b) });
        assert!(((copies_1) == (5)));
        assert!(
            ((*b.data.offset((0) as isize)) == (100)) && ((*d.data.offset((0) as isize)) == (100))
        );
    }
    assert!(((alive_0) == (0)));
    return 0;
}
