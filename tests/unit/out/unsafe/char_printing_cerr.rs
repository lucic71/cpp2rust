extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut vec_: Vec<u8> = Vec::from([195_u8, 167_u8]);
    let mut i: i32 = 27;
    let mut str: Vec<libc::c_char> = {
        let __s = c"bar.".as_ptr();
        std::slice::from_raw_parts(__s, (0..).take_while(|&__i| *__s.add(__i) != 0).count() + 1)
            .to_vec()
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
