extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[repr(u32)]
pub enum Flags_enum {
    #[default]
    F_NONE = 0,
    F_A = 1,
    F_B = 2,
    F_AB = 3,
    F_ALL = 7,
}
impl From<i32> for Flags_enum {
    fn from(n: i32) -> Flags_enum {
        match n {
            0 => Flags_enum::F_NONE,
            1 => Flags_enum::F_A,
            2 => Flags_enum::F_B,
            3 => Flags_enum::F_AB,
            7 => Flags_enum::F_ALL,
            _ => panic!("invalid Flags_enum value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(Flags_enum);
pub unsafe fn add_b_0(mut f: Flags_enum) -> Flags_enum {
    f = Flags_enum::from((((f as u32) | ((Flags_enum::F_B as i32) as u32)) as i32));
    return f;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut f: Flags_enum = Flags_enum::F_A;
    f = (unsafe { add_b_0(f) });
    assert!(((((f as u32) == ((Flags_enum::F_AB as i32) as u32)) as i32) != 0));
    let mut g: Flags_enum = Flags_enum::F_NONE;
    g = Flags_enum::from((((g as u32) | ((Flags_enum::F_A as i32) as u32)) as i32));
    assert!(((((g as u32) == ((Flags_enum::F_A as i32) as u32)) as i32) != 0));
    g = Flags_enum::from((((g as u32) | ((Flags_enum::F_B as i32) as u32)) as i32));
    assert!(((((g as u32) == ((Flags_enum::F_AB as i32) as u32)) as i32) != 0));
    g = Flags_enum::from((((g as u32) & ((Flags_enum::F_ALL as i32) as u32)) as i32));
    assert!(((((g as u32) == ((Flags_enum::F_AB as i32) as u32)) as i32) != 0));
    return 0;
}
