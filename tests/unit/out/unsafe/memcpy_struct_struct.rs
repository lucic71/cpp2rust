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
pub struct Entry {
    pub bits: u8,
    pub value: u16,
}
impl Entry {
    pub unsafe fn copy_from(this: *mut Entry, _a0: *const Entry) {
        (&raw mut (*this).bits).write((*_a0).bits);
        (&raw mut (*this).value).write((*_a0).value);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut table: [Entry; 8] = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = (&raw mut table).cast::<Entry>();
        {
            let __d = __d.offset(0);
            (&raw mut (*__d).bits).write(1_u8);
            (&raw mut (*__d).value).write(4369_u16);
        };
        {
            let __d = __d.offset(1);
            (&raw mut (*__d).bits).write(2_u8);
            (&raw mut (*__d).value).write(8738_u16);
        };
        {
            let __d = __d.offset(2);
            (&raw mut (*__d).bits).write(3_u8);
            (&raw mut (*__d).value).write(13107_u16);
        };
        {
            let __d = __d.offset(3);
            (&raw mut (*__d).bits).write(4_u8);
            (&raw mut (*__d).value).write(17476_u16);
        };
        {
            let __d = __d.offset(4);
            (&raw mut (*__d).bits).write(0_u8);
            (&raw mut (*__d).value).write(0_u16);
        };
        {
            let __d = __d.offset(5);
            (&raw mut (*__d).bits).write(0_u8);
            (&raw mut (*__d).value).write(0_u16);
        };
        {
            let __d = __d.offset(6);
            (&raw mut (*__d).bits).write(0_u8);
            (&raw mut (*__d).value).write(0_u16);
        };
        {
            let __d = __d.offset(7);
            (&raw mut (*__d).bits).write(0_u8);
            (&raw mut (*__d).value).write(0_u16);
        };
    };
    let mut table_size: usize = 4_usize;
    {
        let mut __tmp_0 = { ((&mut table[(table_size)] as *mut Entry) as *mut ::libc::c_void) };
        let mut __tmp_1 = { ((&mut table[(0) as usize] as *mut Entry) as *const ::libc::c_void) };
        let mut __tmp_2 =
            { (table_size as u64).wrapping_mul((::std::mem::size_of::<Entry>() as u64)) };
        if (__tmp_2 as usize) != 0 {
            ::std::ptr::copy_nonoverlapping(__tmp_1, __tmp_0, (__tmp_2 as usize))
        }
        __tmp_0
    };
    assert!(
        ((table[(4) as usize].bits as i32) == (1))
            && ((table[(4) as usize].value as i32) == (4369))
    );
    assert!(
        ((table[(5) as usize].bits as i32) == (2))
            && ((table[(5) as usize].value as i32) == (8738))
    );
    assert!(
        ((table[(6) as usize].bits as i32) == (3))
            && ((table[(6) as usize].value as i32) == (13107))
    );
    assert!(
        ((table[(7) as usize].bits as i32) == (4))
            && ((table[(7) as usize].value as i32) == (17476))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
