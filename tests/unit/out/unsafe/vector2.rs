extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fn_0(v: *mut Vec<i32>, mut v3: Vec<i32>) {
    {
        let __a1 = 20;
        (*v).push(__a1)
    };
    let mut x: i32 = 0_i32;
    let mut v2: Vec<i32> = Vec::new();
    let mut v4: *mut Vec<i32> = (&mut v3 as *mut Vec<i32>);
    {
        let __a1 = 0;
        v2.push(__a1)
    };
    {
        let __a1 = 1;
        v2.push(__a1)
    };
    {
        let __a1 = 3;
        v2.push(__a1)
    };
    x = (*{
        let __idx = 2_usize as usize;
        let __v: &mut Vec<i32> = &mut (*v);
        __v.as_mut_ptr().add(__idx)
    });
    (*{
        let __idx = 0_usize as usize;
        let __v: &mut Vec<i32> = &mut v2;
        __v.as_mut_ptr().add(__idx)
    }) = 1;
    (*{
        let __idx = 0_usize as usize;
        let __v: &mut Vec<i32> = &mut (*if true {
            (&mut v3 as *mut Vec<i32>)
        } else {
            (v)
        });
        __v.as_mut_ptr().add(__idx)
    }) = 7;
    v2 = (*v).clone();
    (*{
        let __idx = 1_usize as usize;
        let __v: &mut Vec<i32> = &mut (*v4);
        __v.as_mut_ptr().add(__idx)
    }) = 13;
    assert!(((x) == (6)));
    assert!(((*((*v).first_mut().unwrap())) == (4)));
    assert!(
        ((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<i32> = &mut (*v);
            __v.as_mut_ptr().add(__idx)
        }) == (5))
    );
    assert!(
        ((*{
            let __idx = 2_usize as usize;
            let __v: &mut Vec<i32> = &mut (*v);
            __v.as_mut_ptr().add(__idx)
        }) == (6))
    );
    assert!(((*((*v).last_mut().unwrap())) == (20)));
    assert!(
        ((*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (4))
    );
    assert!(
        ((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (5))
    );
    assert!(
        ((*{
            let __idx = 2_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (6))
    );
    assert!(
        ((*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<i32> = &mut v3;
            __v.as_mut_ptr().add(__idx)
        }) == (7))
    );
    assert!(
        ((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<i32> = &mut v3;
            __v.as_mut_ptr().add(__idx)
        }) == (13))
    );
    {
        let __a1 = 20;
        (*v).push(__a1)
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();
    {
        let __a1 = 4;
        v.push(__a1)
    };
    {
        let __a1 = 5;
        v.push(__a1)
    };
    {
        let __a1 = 6;
        v.push(__a1)
    };
    {
        let __a1 = 8;
        v2.push(__a1)
    };
    {
        let __a1 = 9;
        v2.push(__a1)
    };
    (unsafe { fn_0(&mut v, v2.clone()) });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
