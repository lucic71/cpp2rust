extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
#[derive(Default)]
pub struct Config {
    pub count: Value<i32>,
    pub mode: Value<Mode>,
}
impl ByteRepr for Config {}
thread_local!(
    pub static config_0: Value<Config> = <Value<Config>>::default();
);
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(((((*(*config_0.with(Value::clone).borrow()).count.borrow()) == 0) as i32) != 0));
    assert!(
        (((((*(*config_0.with(Value::clone).borrow()).mode.borrow()) as u32)
            == ((Mode::MODE_NONE as i32) as u32)) as i32)
            != 0)
    );
    return 0;
}
