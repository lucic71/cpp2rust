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
    let mut arr: Vec<libc::c_char> = {
        let __s = c"foo".as_ptr();
        std::slice::from_raw_parts(__s, (0..).take_while(|&__i| *__s.add(__i) != 0).count() + 1)
            .to_vec()
    };
    (*{
        let __idx = 1_usize as usize;
        let __v: &mut Vec<libc::c_char> = &mut arr;
        __v.as_mut_ptr().add(__idx)
    }) = ('b' as libc::c_char);
    let mut p: *const libc::c_char = arr.as_ptr().offset((1) as isize);
    assert!((((*p) as i32) == (('b' as libc::c_char) as i32)));
    assert!(
        arr == {
            let __s = c"fbo".as_ptr();
            std::slice::from_raw_parts(__s, (0..).take_while(|&__i| *__s.add(__i) != 0).count() + 1)
                .to_vec()
        }
    );
    let mut names: [*mut libc::c_char; 1] = [arr.as_mut_ptr()];
    let copy: Vec<libc::c_char> = {
        let __s = (names[(0) as usize]).cast_const();
        std::slice::from_raw_parts(__s, (0..).take_while(|&__i| *__s.add(__i) != 0).count() + 1)
            .to_vec()
    };
    assert!(
        copy == {
            let __s = c"fbo".as_ptr();
            std::slice::from_raw_parts(__s, (0..).take_while(|&__i| *__s.add(__i) != 0).count() + 1)
                .to_vec()
        }
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
