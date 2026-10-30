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
    let mut str: Vec<libc::c_char> = {
        let __s = c"Hello, world!\n".as_ptr();
        std::slice::from_raw_parts(__s, (0..).take_while(|&__i| *__s.add(__i) != 0).count() + 1)
            .to_vec()
    };
    let file: [libc::c_char; 25] = std::mem::transmute(*b"test_stdcopy_ostream.txt\0");
    {
        let mut ofs: ::std::fs::File = unimplemented!();
        {
            let mut __first = UnsafeRandomAccessIterator::Vec(str.as_mut_ptr());
            let __last = UnsafeRandomAccessIterator::Vec(str.as_mut_ptr().add(str.len() - 1));
            let mut __out = (ofs).clone();
            while __first != __last {
                *__out.iter_deref() = (*__first.iter_deref());
                __first.iter_inc();
                __out.iter_inc();
            }
            __out
        };
    }
    libc::unlink(file.as_ptr());
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
