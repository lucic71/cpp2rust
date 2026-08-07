extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Kind_enum = u32;
pub const Kind_enum_KIND_NONE: Kind_enum = 0;
pub const Kind_enum_KIND_DONE: Kind_enum = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub union anon_0 {
    pub obj: *mut ::libc::c_void,
    pub code: i32,
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Event {
    pub kind: Kind_enum,
    pub handle: *mut ::libc::c_void,
    pub payload: anon_0,
}
pub unsafe fn make_event_1(mut code: i32) -> Event {
    return Event {
        kind: Kind_enum_KIND_DONE,
        handle: std::ptr::null_mut(),
        payload: anon_0 { code: code },
    };
}
pub unsafe fn make_ref_2(mut p: *mut ::libc::c_void) -> Event {
    return Event {
        kind: Kind_enum_KIND_NONE,
        handle: std::ptr::null_mut(),
        payload: anon_0 { obj: p },
    };
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut dummy: i32 = 0;
    let mut m1: Event = <Event>::default();
    m1.kind = Kind_enum_KIND_DONE;
    m1.handle = (((&raw mut dummy as *mut i32) as *mut i32) as *mut ::libc::c_void);
    m1.payload.code = 42;
    assert!(((((m1.kind as u32) == ((Kind_enum_KIND_DONE as i32) as u32)) as i32) != 0));
    assert!(((((m1.payload.code) == (42)) as i32) != 0));
    let mut m2: Event = <Event>::default();
    m2.kind = Kind_enum_KIND_NONE;
    m2.handle = (((&raw mut dummy as *mut i32) as *mut i32) as *mut ::libc::c_void);
    m2.payload.obj = (((&raw mut dummy as *mut i32) as *mut i32) as *mut ::libc::c_void);
    assert!(
        ((((m2.payload.obj) == (((&raw mut dummy as *mut i32) as *mut i32) as *mut ::libc::c_void))
            as i32)
            != 0)
    );
    let mut m3: Event = (unsafe { make_event_1(((dummy) + (7))) });
    assert!(((((m3.kind as u32) == ((Kind_enum_KIND_DONE as i32) as u32)) as i32) != 0));
    assert!(((((m3.payload.code) == (7)) as i32) != 0));
    let mut m4: Event = (unsafe {
        make_ref_2((((&raw mut dummy as *mut i32) as *mut i32) as *mut ::libc::c_void))
    });
    assert!(
        ((((m4.payload.obj) == (((&raw mut dummy as *mut i32) as *mut i32) as *mut ::libc::c_void))
            as i32)
            != 0)
    );
    return 0;
}
