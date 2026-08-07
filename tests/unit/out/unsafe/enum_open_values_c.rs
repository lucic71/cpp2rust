extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Flags = u32;
pub const Flags_FLAG_A: Flags = 256;
pub const Flags_FLAG_B: Flags = 512;
pub const Flags_FLAG_A_ALIAS: Flags = 256;
pub unsafe fn with_a_0(mut f: Flags) -> Flags {
    return ((((f as u32) | ((Flags_FLAG_A as i32) as u32)) as i32) as Flags);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut f: Flags = (('x' as i32) as Flags);
    f = (unsafe { with_a_0(f) });
    f = (((f as u32) | ((Flags_FLAG_B as i32) as u32)) as Flags);
    assert!((((((f as u32) & (255_u32)) == (('x' as i32) as u32)) as i32) != 0));
    assert!(
        (((((f as u32) & (!255 as u32)) == (((Flags_FLAG_A as i32) | (Flags_FLAG_B as i32)) as u32))
            as i32)
            != 0)
    );
    let mut zero: Flags = ((0) as Flags);
    assert!(((!(zero != 0) as i32) != 0));
    assert!(((((zero as u32) != (f as u32)) as i32) != 0));
    let mut as_int: i32 = (f as i32);
    assert!(((((as_int) == (((256) | (512)) | ('x' as i32))) as i32) != 0));
    f = ((((f as u32) & (!(Flags_FLAG_B as i32) as u32)) as i32) as Flags);
    assert!(((((f as u32) == ((((256) | ('x' as i32)) as Flags) as u32)) as i32) != 0));
    let mut seq: Flags = Flags_FLAG_A;
    seq.postfix_inc();
    assert!(((((seq as u32) == (((257) as Flags) as u32)) as i32) != 0));
    seq.prefix_dec();
    assert!(((((seq as u32) == ((Flags_FLAG_A as i32) as u32)) as i32) != 0));
    assert!(((((Flags_FLAG_A_ALIAS as i32) == (Flags_FLAG_A as i32)) as i32) != 0));
    let mut alias: Flags = Flags_FLAG_A_ALIAS;
    assert!(((((alias as u32) == ((Flags_FLAG_A as i32) as u32)) as i32) != 0));
    assert!((((alias as u32) | ((Flags_FLAG_B as i32) as u32)) != 0));
    return 0;
}
