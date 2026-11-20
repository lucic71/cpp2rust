extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut x: i32) -> i32 {
    return x;
}
pub unsafe fn foo_1(mut x: f64) -> f64 {
    return x;
}
pub unsafe fn pick_2() -> i32 {
    return 0;
}
pub unsafe fn pick_3() -> i32 {
    return 1;
}
pub unsafe fn bar_4(mut p: *mut i32, mut flag: bool) -> *mut i32 {
    return if flag { p } else { std::ptr::null_mut() };
}
pub unsafe fn bar_5(mut p: *mut f64, mut flag: bool) -> *mut f64 {
    return if flag { p } else { std::ptr::null_mut() };
}
pub unsafe fn func_6(mut x1: i32, mut x2: i32, mut x3: i32) -> i32 {
    return (((x1) + (x2)) + (x3));
}
pub unsafe fn func_7(mut x1: f64, mut x2: i32, mut x3: f64) -> i32 {
    return ((((x1) + (x2 as f64)) + (x3)) as i32);
}
pub static mut half_8: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(((1) / (2))) });
pub static mut half_9: libcc2rs::LazyStorage<f64> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(((1_f64) / (2_f64))) });
pub static mut half_10: libcc2rs::LazyStorage<*mut i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(std::ptr::null_mut()) });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 10;
    let mut y: f64 = (x as f64);
    assert!(
        ((((((((unsafe { foo_0(x,) }) as f64) + (unsafe { foo_1(y,) }))
            + ((*(unsafe { bar_4((&mut x as *mut i32), true,) })) as f64))
            + (*(unsafe { bar_5((&mut y as *mut f64), true,) })))
            + ((unsafe { func_6(1, 2, 3,) }) as f64))
            + ((unsafe { func_7(2_f64, x, y,) }) as f64))
            == (68_f64))
    );
    assert!(((*(*&raw const half_8).get()) == (0)));
    assert!(((*(*&raw const half_9).get()) == (0.5_f64)));
    (*(*&raw const half_8).get()) = 7;
    assert!(((*(*&raw const half_8).get()) == (7)));
    assert!(((*(*&raw const half_9).get()) == (0.5_f64)));
    assert!((*(*&raw const half_10).get()).is_null());
    (*(*&raw const half_10).get()) = (&mut x as *mut i32);
    assert!(((*(*(*&raw const half_10).get())) == (10)));
    assert!(((unsafe { pick_3() }) == (1)));
    assert!(((unsafe { pick_2() }) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const half_8).get();
    (*&raw const half_9).get();
    (*&raw const half_10).get();
}
