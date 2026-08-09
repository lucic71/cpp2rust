extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Color = u32;
pub const Color_RED: Color = 0;
pub const Color_GREEN: Color = 1;
pub const Color_BLUE: Color = 2;
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(3));
    let c: Value<Color> = Rc::new(RefCell::new(((*n.borrow()) as Color)));
    assert!((((*c.borrow()) as i32) == (((3) as Color) as i32)));
    assert!((((*c.borrow()) as i32) != (Color_BLUE as i32)));
    return if ((*c.borrow()) as i32) == (((3) as Color) as i32) {
        0
    } else {
        1
    };
}
