extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut p_mut_0: libcc2rs::LazyStorage<*mut i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(std::ptr::null_mut()) });
pub static mut p_const_1: libcc2rs::LazyStorage<*const i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(std::ptr::null()) });
pub static mut cp_2: libcc2rs::LazyStorage<*const libc::c_char> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(std::ptr::null()) });
pub static mut arr_of_ptr_3: libcc2rs::LazyStorage<[*mut i32; 4]> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write([std::ptr::null_mut(); 4]) });
pub static mut pp_4: libcc2rs::LazyStorage<*mut *mut i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(std::ptr::null_mut()) });
pub static mut const_arr_of_ptr_5: libcc2rs::LazyStorage<[*const i32; 3]> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write([std::ptr::null(); 3]) });
pub static mut cp_explicit_null_6: libcc2rs::LazyStorage<*const libc::c_char> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(std::ptr::null()) });
pub static mut p_zero_7: libcc2rs::LazyStorage<*mut i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(std::ptr::null_mut()) });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((*(*&raw const p_mut_0).get()).is_null());
    assert!((*(*&raw const p_const_1).get()).is_null());
    assert!((*(*&raw const cp_2).get()).is_null());
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (4)) {
            assert!(((*(*&raw const arr_of_ptr_3).get())[(i) as usize]).is_null());
            i.prefix_inc();
        }
    }
    assert!((*(*&raw const pp_4).get()).is_null());
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (3)) {
            assert!(((*(*&raw const const_arr_of_ptr_5).get())[(i) as usize]).is_null());
            i.prefix_inc();
        }
    }
    assert!((*(*&raw const cp_explicit_null_6).get()).is_null());
    assert!((*(*&raw const p_zero_7).get()).is_null());
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const p_mut_0).get();
    (*&raw const p_const_1).get();
    (*&raw const cp_2).get();
    (*&raw const arr_of_ptr_3).get();
    (*&raw const pp_4).get();
    (*&raw const const_arr_of_ptr_5).get();
    (*&raw const cp_explicit_null_6).get();
    (*&raw const p_zero_7).get();
}
