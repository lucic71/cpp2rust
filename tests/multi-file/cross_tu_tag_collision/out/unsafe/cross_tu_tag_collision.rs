extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct widget {
    pub id: i32,
}
pub unsafe fn a_value_0() -> i32 {
    let mut w: widget = <widget>::default();
    w.id = 11;
    return w.id;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((unsafe { a_value_0() }) == (11)) as i32) != 0));
    assert!(((((unsafe { b_value_1() }) == (2)) as i32) != 0));
    return 0;
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum widget_enum {
    #[default]
    WIDGET_A = 0,
    WIDGET_B = 1,
    WIDGET_C = 2,
}
impl From<i32> for widget_enum {
    fn from(n: i32) -> widget_enum {
        match n {
            0 => widget_enum::WIDGET_A,
            1 => widget_enum::WIDGET_B,
            2 => widget_enum::WIDGET_C,
            _ => panic!("invalid widget_enum value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(widget_enum);
pub unsafe fn b_value_1() -> i32 {
    let mut w: widget_enum = widget_enum::WIDGET_C;
    return (w as i32);
}
