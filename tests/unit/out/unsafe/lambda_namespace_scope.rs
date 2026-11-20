extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut counter_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut inc_1: libcc2rs::LazyStorage<FnPtr<fn(i32) -> i32>> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        __p.write(FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
            unsafe {
                return ((x) + (1));
            }
        }))
    });
pub static mut bump_2: libcc2rs::LazyStorage<FnPtr<fn() -> i32>> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        __p.write(FnPtr::<fn() -> i32>::new(|| -> i32 {
            unsafe {
                (*(*&raw const counter_0).get()).postfix_inc();
                return (*(*&raw const counter_0).get());
            }
        }))
    });
pub unsafe fn apply_3(mut f: FnPtr<fn(i32) -> i32>, mut x: i32) -> i32 {
    return (unsafe { f.call(x) });
}
pub unsafe fn by_ref_4(v: *const i32) -> i32 {
    return ((*v) * (2));
}
pub static mut twice_next_5: libcc2rs::LazyStorage<FnPtr<fn(i32) -> i32>> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        __p.write(FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
            unsafe {
                return (unsafe {
                    let mut _v: i32 = ((x) + (1));
                    by_ref_4(&mut _v)
                });
            }
        }))
    });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { (*(*&raw const inc_1).get()).call(41,) }) == (42)));
    (unsafe { (*(*&raw const bump_2).get()).call() });
    assert!(((unsafe { (*(*&raw const bump_2).get()).call() }) == (2)));
    assert!(((*(*&raw const counter_0).get()) == (2)));
    assert!(((unsafe { apply_3((*(*&raw const inc_1).get()).copy_from(), 1,) }) == (2)));
    let mut copy: FnPtr<fn(i32) -> i32> = (*(*&raw const inc_1).get()).copy_from();
    assert!(((unsafe { copy.call(9,) }) == (10)));
    let mut fp: Option<unsafe fn(i32) -> i32> = Some(|mut x: i32| -> i32 {
        unsafe {
            return ((x) + (1));
        }
    });
    assert!(((unsafe { (fp).unwrap()(-1_i32,) }) == (0)));
    assert!(((unsafe { (*(*&raw const twice_next_5).get()).call(4,) }) == (10)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const counter_0).get();
    (*&raw const inc_1).get();
    (*&raw const bump_2).get();
    (*&raw const twice_next_5).get();
}
