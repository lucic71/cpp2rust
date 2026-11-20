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
    pub name: *const libc::c_char,
    pub p: *mut i32,
}
impl Entry {
    pub unsafe fn new_1(this: *mut Entry) {}
    pub unsafe fn copy_from(this: *mut Entry, _a0: *const Entry) {
        (&raw mut (*this).name).write((*_a0).name);
        (&raw mut (*this).p).write((*_a0).p);
    }
    pub unsafe fn move_from(this: *mut Entry, _a0: *mut Entry) {
        (&raw mut (*this).name).write((*_a0).name);
        (&raw mut (*this).p).write((*_a0).p);
    }
}
pub static mut single_entry_0: libcc2rs::LazyStorage<Entry> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __d = __p;
            (&raw mut (*__d).name).write(c"alone".as_ptr());
            (&raw mut (*__d).p).write(std::ptr::null_mut());
        }
    });
pub static mut entries_1: libcc2rs::LazyStorage<[Entry; 2]> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __d = __p.cast::<Entry>();
            {
                let __d = __d.offset(0);
                (&raw mut (*__d).name).write(c"first".as_ptr());
                (&raw mut (*__d).p).write(std::ptr::null_mut());
            };
            {
                let __d = __d.offset(1);
                (&raw mut (*__d).name).write(c"second".as_ptr());
                (&raw mut (*__d).p).write(std::ptr::null_mut());
            };
        }
    });
pub static mut arr_of_pointers_2: libcc2rs::LazyStorage<[*mut libc::c_char; 3]> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        __p.write([
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        ])
    });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((*(*&raw const single_entry_0).get()).p).is_null());
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (2)) {
            assert!(((*(*&raw const entries_1).get())[(i) as usize].p).is_null());
            assert!(((*(*&raw const arr_of_pointers_2).get())[(i) as usize]).is_null());
            i.prefix_inc();
        }
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const single_entry_0).get();
    (*&raw const entries_1).get();
    (*&raw const arr_of_pointers_2).get();
}
