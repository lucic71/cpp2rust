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
pub struct peer {
    pub port: i32,
    pub hostname: [libc::c_char; 1],
}
impl Default for peer {
    fn default() -> Self {
        peer {
            port: 0_i32,
            hostname: [(0 as libc::c_char); 1],
        }
    }
}
pub unsafe fn peer_create_0(mut host: *const libc::c_char) -> *mut peer {
    let mut p: *mut peer = (libcc2rs::malloc_unsafe(
        (::std::mem::size_of::<peer>() as usize).wrapping_add(libc::strlen(host)),
    ) as *mut peer);
    (*p).port = 443;
    {
        if (libc::strlen(host)).wrapping_add(1_usize) != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((host as *const libc::c_char) as *const ::libc::c_void),
                (((*p).hostname.as_mut_ptr() as *mut libc::c_char) as *mut ::libc::c_void),
                (libc::strlen(host)).wrapping_add(1_usize) as usize,
            )
        }
        (((*p).hostname.as_mut_ptr() as *mut libc::c_char) as *mut ::libc::c_void)
    };
    return p;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: *mut peer =
        (unsafe { peer_create_0((c"example.com".as_ptr().cast_mut()).cast_const()) });
    assert!((((((*p).port) == (443)) as i32) != 0));
    assert!(
        ((((libc::strcmp(
            ((*p).hostname.as_mut_ptr()).cast_const(),
            (c"example.com".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        (((((*(*p).hostname.as_mut_ptr().add((0) as usize)) as i32) == ('e' as i32)) as i32) != 0)
    );
    assert!(
        (((((*(*p).hostname.as_mut_ptr().add((7) as usize)) as i32) == ('.' as i32)) as i32) != 0)
    );
    (*(*p).hostname.as_mut_ptr().add((0) as usize)) = (('E' as i32) as libc::c_char);
    assert!(
        ((((libc::strcmp(
            ((*p).hostname.as_mut_ptr()).cast_const(),
            (c"Example.com".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!((((((*p).port) == (443)) as i32) != 0));
    let mut h: *mut libc::c_char = ((*p).hostname.as_mut_ptr().add((8) as usize));
    assert!(
        ((((libc::strcmp((h).cast_const(), (c"com".as_ptr().cast_mut()).cast_const())) == (0))
            as i32)
            != 0)
    );
    libcc2rs::free_unsafe(((p as *mut peer) as *mut ::libc::c_void));
    return 0;
}
