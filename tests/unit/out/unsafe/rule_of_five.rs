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
pub static mut moves_2: i32 = unsafe { 0 };
#[repr(C)]
#[derive(Default)]
pub struct Buffer {
    pub data: *mut i32,
    pub size: i32,
}
impl Buffer {
    pub unsafe fn Buffer(mut size: i32) -> Self {
        let mut __this = Self {
            data: Box::leak((0..(size as usize)).map(|_| 0_i32).collect::<Box<[i32]>>())
                .as_mut_ptr(),
            size: size,
        };
        let this = &raw mut __this;
        let mut i: i32 = 0;
        'loop_: while ((i) < (size)) {
            (*(*this).data.offset((i) as isize)) = i;
            i.prefix_inc();
        }
        alive_0.prefix_inc();
        __this
    }
    pub unsafe fn destructor(&mut self) {
        let this = self as *mut Buffer;

        ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
            (*this).data,
            libcc2rs::malloc_usable_size((*this).data as *mut ::libc::c_void)
                / ::std::mem::size_of::<i32>(),
        )));
        alive_0.prefix_dec();
    }
    pub unsafe fn Buffer_pconstBuffer(o: *const Buffer) -> Self {
        let mut __this = Self {
            data: Box::leak(
                (0..((*o).size as usize))
                    .map(|_| 0_i32)
                    .collect::<Box<[i32]>>(),
            )
            .as_mut_ptr(),
            size: (*o).size,
        };
        let this = &raw mut __this;
        let mut i: i32 = 0;
        'loop_: while ((i) < ((*this).size)) {
            (*(*this).data.offset((i) as isize)) = (*(*o).data.offset((i) as isize));
            i.prefix_inc();
        }
        alive_0.prefix_inc();
        copies_1.prefix_inc();
        __this
    }
    pub unsafe fn Buffer_pmutBuffer(o: *mut Buffer) -> Self {
        let mut __this = Self {
            data: (*o).data,
            size: (*o).size,
        };
        let this = &raw mut __this;
        (*o).data = std::ptr::null_mut();
        (*o).size = 0;
        alive_0.prefix_inc();
        moves_2.prefix_inc();
        __this
    }
    pub unsafe fn operator_assign_pconstBuffer(&mut self, o: *const Buffer) -> *mut Buffer {
        let this = self as *mut Buffer;
        if ((this) == (o)) {
            return &mut (*this) as *mut Buffer;
        }
        ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
            (*this).data,
            libcc2rs::malloc_usable_size((*this).data as *mut ::libc::c_void)
                / ::std::mem::size_of::<i32>(),
        )));
        (*this).data = Box::leak(
            (0..((*o).size as usize))
                .map(|_| 0_i32)
                .collect::<Box<[i32]>>(),
        );
        (*this).size = (*o).size;
        let mut i: i32 = 0;
        'loop_: while ((i) < ((*this).size)) {
            (*(*this).data.offset((i) as isize)) = (*(*o).data.offset((i) as isize));
            i.prefix_inc();
        }
        copies_1.prefix_inc();
        return &mut (*this) as *mut Buffer;
    }
    pub unsafe fn operator_assign_pmutBuffer(&mut self, o: *mut Buffer) -> *mut Buffer {
        let this = self as *mut Buffer;
        if ((this) == (o)) {
            return &mut (*this) as *mut Buffer;
        }
        ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
            (*this).data,
            libcc2rs::malloc_usable_size((*this).data as *mut ::libc::c_void)
                / ::std::mem::size_of::<i32>(),
        )));
        (*this).data = (*o).data;
        (*this).size = (*o).size;
        (*o).data = std::ptr::null_mut();
        (*o).size = 0;
        moves_2.prefix_inc();
        return &mut (*this) as *mut Buffer;
    }
}
impl Clone for Buffer {
    fn clone(&self) -> Self {
        unsafe { Buffer::Buffer_pconstBuffer(self as *const Buffer) }
    }
}
pub unsafe fn make_3(mut size: i32) -> Buffer {
    let mut b: Buffer = Buffer::Buffer({ size });
    let _dtor_b = ScopedDestructorUnsafe::new(&raw mut b, Buffer::destructor);
    return Buffer::Buffer_pmutBuffer({ &mut b });
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
        let mut b: Buffer = Buffer::Buffer_pconstBuffer({ &a as *const Buffer });
        let _dtor_b = ScopedDestructorUnsafe::new(&raw mut b, Buffer::destructor);
        assert!((((alive_0) == (2)) && ((copies_1) == (1))) && ((moves_2) == (0)));
        (*b.data.offset((0) as isize)) = 100;
        assert!(((*a.data.offset((0) as isize)) == (0)));
        let mut c: Buffer = Buffer::Buffer_pmutBuffer({ &mut a });
        let _dtor_c = ScopedDestructorUnsafe::new(&raw mut c, Buffer::destructor);
        assert!(((alive_0) == (3)) && ((moves_2) == (1)));
        assert!(((a.data).is_null()) && ((a.size) == (0)));
        assert!(((c.size) == (4)) && ((*c.data.offset((3) as isize)) == (3)));
        let mut d: Buffer = (unsafe { make_3(2) });
        let _dtor_d = ScopedDestructorUnsafe::new(&raw mut d, Buffer::destructor);
        assert!(((d.size) == (2)) && ((moves_2) == (2)));
        (unsafe { Buffer::operator_assign_pconstBuffer(&mut d, &b as *const Buffer) });
        assert!(
            (((d.size) == (4)) && ((*d.data.offset((0) as isize)) == (100))) && ((copies_1) == (2))
        );
        (unsafe { Buffer::operator_assign_pmutBuffer(&mut d, &mut c) });
        assert!(
            (((*d.data.offset((0) as isize)) == (0)) && ((c.data).is_null())) && ((moves_2) == (3))
        );
        (unsafe {
            let _o: *mut Buffer = &mut d;
            Buffer::operator_assign_pmutBuffer(&mut d, _o)
        });
        assert!(((d.size) == (4)) && ((moves_2) == (3)));
        let mut vec_: Vec<Buffer> = Vec::new();
        vec_.push(Buffer::Buffer({ 3 }));
        {
            let a0_clone = b.clone();
            vec_.push(a0_clone)
        };
        assert!(
            ((vec_[(0_usize)].size) == (3))
                && ((*vec_[(1_usize)].data.offset((0) as isize)) == (100))
        );
    }
    assert!(((alive_0) == (0)));
    return 0;
}
