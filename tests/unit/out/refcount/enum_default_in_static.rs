extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Mode = u32;
pub const Mode_MODE_NONE: Mode = 0;
pub const Mode_MODE_ONE: Mode = 1;
pub const Mode_MODE_TWO: Mode = 2;
#[repr(C)]
#[derive(Clone, Default)]
pub struct Config {
    pub count: i32,
    pub mode: Mode,
}
impl ByteRepr for Config {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.count.to_bytes(&mut buf[0..4]);
        self.mode.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            count: <i32>::from_bytes(&buf[0..4]),
            mode: <Mode>::from_bytes(&buf[4..8]),
        }
    }
}
thread_local!(
    pub static config_0: Value<Config> = <Value<Config>>::default();
);
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!(((((*config_0.with(Value::clone).borrow()).count == 0) as i32) != 0));
    assert!(
        (((((*config_0.with(Value::clone).borrow()).mode as u32)
            == ((Mode_MODE_NONE as i32) as u32)) as i32)
            != 0)
    );
    return 0;
}
