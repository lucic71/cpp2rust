extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Layout {
    pub a: u8,
    pub b: u32,
    pub c: u16,
}
impl Layout {
    pub unsafe fn copy_from(this: *mut Layout, _a0: *const Layout) {
        (&raw mut (*this).a).write((*_a0).a);
        (&raw mut (*this).b).write((*_a0).b);
        (&raw mut (*this).c).write((*_a0).c);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Frame {
    pub tag: u16,
    pub body: [libc::c_char; 64],
}
impl Frame {
    pub unsafe fn copy_from(this: *mut Frame, _a0: *const Frame) {
        (&raw mut (*this).tag).write((*_a0).tag);
        (&raw mut (*this).body).write(std::array::from_fn::<_, 64, _>(|__i: usize| {
            (*_a0).body[(__i)]
        }));
    }
}
impl Default for Frame {
    fn default() -> Self {
        unsafe {
            Frame {
                tag: 0_u16,
                body: [(0 as libc::c_char); 64],
            }
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((::std::mem::offset_of!(Layout, a)) == (0_usize)));
    assert!(((::std::mem::offset_of!(Layout, b)) == (4_usize)));
    assert!(((::std::mem::offset_of!(Layout, c)) == (8_usize)));
    let mut v: Layout = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut v;
        (&raw mut (*__d).a).write(0_u8);
        (&raw mut (*__d).b).write(0_u32);
        (&raw mut (*__d).c).write(0_u16);
    };
    v.b = 3735928559_u32;
    let mut base: *mut u8 = ((&mut v as *mut Layout) as *mut u8);
    let mut bp: *mut u32 =
        ((base.offset((::std::mem::offset_of!(Layout, b)) as isize)) as *mut u32);
    assert!(((*bp) == (3735928559_u32)));
    (*((base.offset((::std::mem::offset_of!(Layout, b)) as isize)) as *mut u32)) = 305419896_u32;
    assert!(((v.b) == (305419896_u32)));
    let mut text: *const libc::c_char = c"example-body".as_ptr();
    let mut len: usize = (libc::strlen(text)).wrapping_add(1_usize);
    let mut total: usize =
        ((::std::mem::offset_of!(Frame, body) as u64).wrapping_add((len as u64)) as usize);
    assert!(((total) == ((2_usize).wrapping_add(len))));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
