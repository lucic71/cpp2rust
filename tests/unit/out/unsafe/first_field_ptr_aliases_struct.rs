extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct transfer {
    pub errbuf: [libc::c_char; 32],
    pub code: i32,
}
impl Default for transfer {
    fn default() -> Self {
        transfer {
            errbuf: [(0 as libc::c_char); 32],
            code: 0_i32,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct holder {
    pub xfer: *mut transfer,
    pub err: *mut libc::c_char,
}
#[repr(C)]
#[derive(Clone)]
pub struct tagged {
    pub errbuf: [libc::c_char; 32],
    pub code: i32,
    pub lookup: BTreeMap<i32, Box<i32>>,
}
impl Default for tagged {
    fn default() -> Self {
        tagged {
            errbuf: [(0 as libc::c_char); 32],
            code: 0_i32,
            lookup: BTreeMap::new(),
        }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut h: *mut holder =
        (libcc2rs::malloc_unsafe(::std::mem::size_of::<holder>()) as *mut holder);
    (*h).xfer = (libcc2rs::malloc_unsafe(::std::mem::size_of::<transfer>()) as *mut transfer);
    (*(*h).xfer).code = 7;
    (*h).err = (*(*h).xfer).errbuf.as_mut_ptr();
    {
        if 5_usize != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((c"boom".as_ptr() as *const libc::c_char) as *const ::libc::c_void),
                (((*h).err as *mut libc::c_char) as *mut ::libc::c_void),
                5_usize as usize,
            )
        }
        (((*h).err as *mut libc::c_char) as *mut ::libc::c_void)
    };
    assert!(
        ((libc::strcmp(
            ((*(*h).xfer).errbuf.as_mut_ptr()).cast_const(),
            c"boom".as_ptr()
        )) == (0))
    );
    assert!((((*(*h).xfer).code) == (7)));
    libcc2rs::free_unsafe((((*h).xfer as *mut transfer) as *mut ::libc::c_void));
    libcc2rs::free_unsafe(((h as *mut holder) as *mut ::libc::c_void));
    let mut t: tagged = <tagged>::default();
    t.code = 9;
    (*t.lookup.entry(1).or_default().as_mut()) = 100;
    let mut err: *mut libc::c_char = t.errbuf.as_mut_ptr();
    {
        if 5_usize != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((c"bang".as_ptr() as *const libc::c_char) as *const ::libc::c_void),
                ((err as *mut libc::c_char) as *mut ::libc::c_void),
                5_usize as usize,
            )
        }
        ((err as *mut libc::c_char) as *mut ::libc::c_void)
    };
    assert!(((libc::strcmp((t.errbuf.as_mut_ptr()).cast_const(), c"bang".as_ptr())) == (0)));
    assert!(((t.code) == (9)));
    assert!(((*t.lookup.entry(1).or_default().as_mut()) == (100)));
    return 0;
}
