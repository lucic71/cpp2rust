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
    let mut v: Vec<Vec<Vec<Vec<libc::c_char>>>> = Vec::new();
    {
        let __init = Vec::new();
        v.push(__init)
    };
    {
        let __init = Vec::new();
        v.push(__init)
    };
    let mut row: Vec<Vec<libc::c_char>> = Vec::new();
    {
        let __a1 = unimplemented!();
        row.push(__a1)
    };
    {
        let __a1 = unimplemented!();
        row.push(__a1)
    };
    (*unimplemented!()).push((row).clone());
    {
        let __a1 = unimplemented!();
        row.push(__a1)
    };
    (*unimplemented!()).push((row).clone());
    {
        let __init = Vec::new();
        (*unimplemented!()).push(__init)
    };
    {
        let __a1 = {
            let mut __tmp2 = (*unimplemented!()).clone();
            __tmp2.pop();
            let __from = c"x".as_ptr();
            __tmp2.extend_from_slice(::std::slice::from_raw_parts(
                __from,
                (0..).position(|i| *__from.add(i) == 0).unwrap(),
            ));
            __tmp2.push(0);
            __tmp2
        };
        (*((*unimplemented!()).last_mut().unwrap())).push(__a1)
    };
    assert!((((*((*unimplemented!()).first_mut().unwrap())).len()) == (2_usize)));
    assert!(
        (((*((*unimplemented!()).first_mut().unwrap())).len()) == (3_usize)) && (unimplemented!())
    );
    assert!(unimplemented!());
    let mut copy: Vec<Vec<Vec<Vec<libc::c_char>>>> = v.clone();
    (*unimplemented!()) = unimplemented!();
    (*if 0_usize as usize >= (*unimplemented!()).len() - 1 {
        panic!("out of bounds access")
    } else {
        (&mut (*unimplemented!())[0_usize as usize] as *mut libc::c_char)
    }) = (('B' as libc::c_char) as ());
    assert!((unimplemented!()) && (unimplemented!()));
    assert!((unimplemented!()) && (unimplemented!()));
    (*unimplemented!()).remove(0);
    assert!(unimplemented!());
    assert!((((*((*unimplemented!()).first_mut().unwrap())).len()) == (3_usize)));
    assert!(((row.len()) == (3_usize)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
