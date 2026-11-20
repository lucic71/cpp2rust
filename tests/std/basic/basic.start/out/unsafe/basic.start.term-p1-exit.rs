extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Tracker {
    pub tag: i32,
}
impl Tracker {
    pub unsafe fn new(this: *mut Tracker, mut tag: i32) {
        (&raw mut (*this).tag).write(tag);
        (unsafe { libc::printf(c"construct %d\n".as_ptr() as *const libc::c_char, (tag)) });
    }
    pub unsafe fn destructor(&mut self) {
        (unsafe { libc::printf(c"destroy %d\n".as_ptr() as *const libc::c_char, (self.tag)) });
    }
    pub unsafe fn copy_from(this: *mut Tracker, _a0: *const Tracker) {
        (&raw mut (*this).tag).write((*_a0).tag);
    }
}
pub static mut global_0: libcc2rs::LazyStorage<Tracker> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __static = Tracker::new(__p, { 1 });
            libcc2rs::register_static_destructor(|| (*(*&raw const global_0).get()).destructor());
            __static
        }
    });
pub unsafe fn exit_from_nested_call_1() {
    (unsafe {
        libc::printf(
            c"local static %d\n".as_ptr() as *const libc::c_char,
            ((*(*&raw const local_static_2).get()).tag),
        )
    });
    let mut automatic: Tracker = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracker::new(&raw mut automatic, { 3 });
    let _dtor_automatic =
        std::mem::DropGuard::new(&raw mut automatic, |__p| unsafe { (*__p).destructor() });
    libcc2rs::cpp2rust_exit(0);
}
static mut local_static_2: libcc2rs::LazyStorage<Tracker> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __static = Tracker::new(__p, { 2 });
            libcc2rs::register_static_destructor(|| {
                (*(*&raw const local_static_2).get()).destructor()
            });
            __static
        }
    });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { exit_from_nested_call_1() });
    (unsafe { libc::printf(c"not reached\n".as_ptr() as *const libc::c_char) });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const global_0).get();
}
