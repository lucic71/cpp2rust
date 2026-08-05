extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn scaleA_0(mut x: i32) -> i32 {
    return ((x) * (2));
}
pub unsafe fn shiftA_1(mut x: i32) -> i32 {
    return ((x) - (2));
}
pub unsafe fn scaleB_2(mut x: i32) -> i32 {
    return ((x) * (3));
}
pub unsafe fn shiftB_3(mut x: i32) -> i32 {
    return ((x) - (3));
}
pub unsafe fn pmin_int_4(mut a: i32, mut b: i32) -> i32 {
    return if ((((a) < (b)) as i32) != 0) { a } else { b };
}
pub unsafe fn pmax_int_5(mut a: i32, mut b: i32) -> i32 {
    return if ((((a) > (b)) as i32) != 0) { a } else { b };
}
pub unsafe fn pmin_long_6(mut a: i64, mut b: i64) -> i64 {
    return if ((((a) < (b)) as i32) != 0) { a } else { b };
}
pub unsafe fn pmax_long_7(mut a: i64, mut b: i64) -> i64 {
    return if ((((a) > (b)) as i32) != 0) { a } else { b };
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((unsafe { scaleA_0(5) }) == (10)) as i32) != 0));
    assert!(((((unsafe { scaleB_2(5) }) == (15)) as i32) != 0));
    assert!(((((unsafe { shiftA_1(5) }) == (3)) as i32) != 0));
    assert!(((((unsafe { shiftB_3(5) }) == (2)) as i32) != 0));
    assert!(((((unsafe { pmin_int_4(3, 4) }) == (3)) as i32) != 0));
    assert!(((((unsafe { pmax_int_5(3, 4) }) == (4)) as i32) != 0));
    assert!(((((unsafe { pmin_long_6(3_i64, 4_i64) }) == (3_i64)) as i32) != 0));
    assert!(((((unsafe { pmax_long_7(3_i64, 4_i64) }) == (4_i64)) as i32) != 0));
    assert!(((((unsafe { combine_8(5) }) == (35)) as i32) != 0));
    return 0;
}
pub unsafe fn combine_8(mut x: i32) -> i32 {
    return (((((unsafe { scaleA_0(x) }) + (unsafe { scaleB_2(x) })) + (unsafe { shiftA_1(x) }))
        + (unsafe { shiftB_3(x) }))
        + ((unsafe { pmax_long_7((x as i64), 0_i64) }) as i32));
}
