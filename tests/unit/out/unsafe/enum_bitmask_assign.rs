extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Flags_enum = u32;
pub const Flags_enum_F_NONE: Flags_enum = 0;
pub const Flags_enum_F_A: Flags_enum = 1;
pub const Flags_enum_F_B: Flags_enum = 2;
pub const Flags_enum_F_AB: Flags_enum = 3;
pub const Flags_enum_F_ALL: Flags_enum = 7;
pub unsafe fn add_b_0(mut f: Flags_enum) -> Flags_enum {
    f = (((f as u32) | ((Flags_enum_F_B as i32) as u32)) as Flags_enum);
    return f;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut f: Flags_enum = Flags_enum_F_A;
    f = (unsafe { add_b_0(f) });
    assert!(((((f as u32) == ((Flags_enum_F_AB as i32) as u32)) as i32) != 0));
    let mut g: Flags_enum = Flags_enum_F_NONE;
    g = (((g as u32) | ((Flags_enum_F_A as i32) as u32)) as Flags_enum);
    assert!(((((g as u32) == ((Flags_enum_F_A as i32) as u32)) as i32) != 0));
    g = (((g as u32) | ((Flags_enum_F_B as i32) as u32)) as Flags_enum);
    assert!(((((g as u32) == ((Flags_enum_F_AB as i32) as u32)) as i32) != 0));
    g = (((g as u32) & ((Flags_enum_F_ALL as i32) as u32)) as Flags_enum);
    assert!(((((g as u32) == ((Flags_enum_F_AB as i32) as u32)) as i32) != 0));
    return 0;
}
