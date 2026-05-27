extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Mode {
    #[default]
    MODE_NONE = 0,
    MODE_ONE = 1,
    MODE_TWO = 2,
}
impl From<i32> for Mode {
    fn from(n: i32) -> Mode {
        match n {
            0 => Mode::MODE_NONE,
            1 => Mode::MODE_ONE,
            2 => Mode::MODE_TWO,
            _ => panic!("invalid Mode value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(Mode);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Config {
    pub count: i32,
    pub mode: Mode,
}
pub static mut config_0: Config = unsafe {
    Config {
        count: 0_i32,
        mode: Mode::MODE_NONE,
    }
};
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((config_0.count) == (0)) as i32) != 0));
    assert!(((((config_0.mode as u32) == ((Mode::MODE_NONE as i32) as u32)) as i32) != 0));
    return 0;
}
