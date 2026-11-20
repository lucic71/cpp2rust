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
    pub unsafe fn touch(&self) {
        (unsafe { libc::printf(c"touch %d\n".as_ptr() as *const libc::c_char, (self.tag)) });
    }
    pub unsafe fn copy_from(this: *mut Tracker, _a0: *const Tracker) {
        (&raw mut (*this).tag).write((*_a0).tag);
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct HoldsTracker {
    pub member: Tracker,
}
impl HoldsTracker {
    pub unsafe fn copy_from(this: *mut HoldsTracker, _a0: *const HoldsTracker) {
        Tracker::copy_from((&raw mut (*this).member), { &(*_a0).member });
    }
    pub unsafe fn move_from(this: *mut HoldsTracker, _a0: *mut HoldsTracker) {
        Tracker::copy_from((&raw mut (*this).member), { &(*_a0).member });
    }
}
impl HoldsTracker {
    pub unsafe fn destructor(&mut self) {
        Tracker::destructor(&mut self.member);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TriviallyDestructible {
    pub value: i32,
}
impl TriviallyDestructible {
    pub unsafe fn new_1(this: *mut TriviallyDestructible) {}
    pub unsafe fn copy_from(this: *mut TriviallyDestructible, _a0: *const TriviallyDestructible) {
        (&raw mut (*this).value).write((*_a0).value);
    }
    pub unsafe fn move_from(this: *mut TriviallyDestructible, _a0: *mut TriviallyDestructible) {
        (&raw mut (*this).value).write((*_a0).value);
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
pub unsafe fn never_called_1() {
    (unsafe { Tracker::touch(&(*(*&raw const never_constructed_2).get())) });
}
pub unsafe fn test_3() {
    (unsafe { Tracker::touch(&(*(*&raw const global_0).get())) });
}
static mut never_constructed_2: libcc2rs::LazyStorage<Tracker> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __static = Tracker::new(__p, { 100 });
            libcc2rs::register_static_destructor(|| {
                (*(*&raw const never_constructed_2).get()).destructor()
            });
            __static
        }
    });
pub unsafe fn local_static_during_global_init_4() -> *const Tracker {
    return &(*(*&raw const during_init_5).get());
}
pub static mut before_6: libcc2rs::LazyStorage<Tracker> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __static = Tracker::new(__p, { 2 });
            libcc2rs::register_static_destructor(|| (*(*&raw const before_6).get()).destructor());
            __static
        }
    });
pub static mut after_7: libcc2rs::LazyStorage<Tracker> = libcc2rs::LazyStorage::new(|__p| unsafe {
    {
        let __static = Tracker::new(__p, {
            (((*(unsafe { local_static_during_global_init_4() })).tag) + (1))
        });
        libcc2rs::register_static_destructor(|| (*(*&raw const after_7).get()).destructor());
        __static
    }
});
pub static mut holder_8: libcc2rs::LazyStorage<HoldsTracker> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __static = {
                let __d = __p;
                Tracker::new((&raw mut (*__d).member), { 5 });
            };
            libcc2rs::register_static_destructor(|| (*(*&raw const holder_8).get()).destructor());
            __static
        }
    });
pub static mut trivial_9: libcc2rs::LazyStorage<TriviallyDestructible> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __d = __p;
            (&raw mut (*__d).value).write(6);
        }
    });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p3_reverse_completion_order_WithStaticMember {}
impl p3_reverse_completion_order_WithStaticMember {
    pub unsafe fn new_1(this: *mut p3_reverse_completion_order_WithStaticMember) {}
    pub unsafe fn copy_from(
        this: *mut p3_reverse_completion_order_WithStaticMember,
        _a0: *const p3_reverse_completion_order_WithStaticMember,
    ) {
    }
    pub unsafe fn move_from(
        this: *mut p3_reverse_completion_order_WithStaticMember,
        _a0: *mut p3_reverse_completion_order_WithStaticMember,
    ) {
    }
}
pub static mut member_10: libcc2rs::LazyStorage<Tracker> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __static = Tracker::new(__p, { 7 });
            libcc2rs::register_static_destructor(|| (*(*&raw const member_10).get()).destructor());
            __static
        }
    });
pub static mut capturing_11: libcc2rs::LazyStorage<FnPtr<fn() -> i32>> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        __p.write({
            let __static = lambda_unsafe!(
                {
                    let captured: Tracker = {
                        let mut __v: Tracker = ::std::mem::MaybeUninit::uninit().assume_init();
                        Tracker::new(&raw mut __v, { 8 });
                        __v
                    };
                },
                || -> i32 {
                    return captured.tag;
                }
            );
            libcc2rs::register_static_destructor(|| (*(*&raw const capturing_11).get()).destroy());
            __static
        })
    });
pub unsafe fn run_local_static_12() {
    (unsafe { Tracker::touch(&(*(*&raw const during_main_13).get())) });
}
pub unsafe fn test_14() {
    (unsafe { run_local_static_12() });
    (unsafe { run_local_static_12() });
    (unsafe { Tracker::touch(&(*(unsafe { local_static_during_global_init_4() }))) });
    (unsafe {
        libc::printf(
            c"trivial %d\n".as_ptr() as *const libc::c_char,
            ((*(*&raw const trivial_9).get()).value),
        )
    });
    (unsafe { Tracker::touch(&(*(*&raw const member_10).get())) });
    (unsafe {
        libc::printf(
            c"captured %d\n".as_ptr() as *const libc::c_char,
            (unsafe { (*(*&raw const capturing_11).get()).call() }),
        )
    });
}
static mut during_init_5: libcc2rs::LazyStorage<Tracker> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __static = Tracker::new(__p, { 3 });
            libcc2rs::register_static_destructor(|| {
                (*(*&raw const during_init_5).get()).destructor()
            });
            __static
        }
    });
static mut during_main_13: libcc2rs::LazyStorage<Tracker> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __static = Tracker::new(__p, { 9 });
            libcc2rs::register_static_destructor(|| {
                (*(*&raw const during_main_13).get()).destructor()
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
    (unsafe { test_3() });
    (unsafe { test_14() });
    (unsafe { libc::printf(c"return from main\n".as_ptr() as *const libc::c_char) });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const global_0).get();
    (*&raw const before_6).get();
    (*&raw const after_7).get();
    (*&raw const holder_8).get();
    (*&raw const trivial_9).get();
    (*&raw const member_10).get();
    (*&raw const capturing_11).get();
}
