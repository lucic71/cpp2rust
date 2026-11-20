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
pub struct list_head {
    pub next: *mut list_head,
}
impl list_head {
    pub unsafe fn new_1(this: *mut list_head) {}
    pub unsafe fn copy_from(this: *mut list_head, _a0: *const list_head) {
        (&raw mut (*this).next).write((*_a0).next);
    }
    pub unsafe fn move_from(this: *mut list_head, _a0: *mut list_head) {
        (&raw mut (*this).next).write((*_a0).next);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct node {
    pub value: i32,
    pub value_ptr: *mut i32,
}
impl node {
    pub unsafe fn new_1(this: *mut node) {}
    pub unsafe fn copy_from(this: *mut node, _a0: *const node) {
        (&raw mut (*this).value).write((*_a0).value);
        (&raw mut (*this).value_ptr).write((*_a0).value_ptr);
    }
    pub unsafe fn move_from(this: *mut node, _a0: *mut node) {
        (&raw mut (*this).value).write((*_a0).value);
        (&raw mut (*this).value_ptr).write((*_a0).value_ptr);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct node_with_defaults {
    pub next: *mut node_with_defaults,
    pub value: i32,
}
impl node_with_defaults {
    pub unsafe fn new_1(this: *mut node_with_defaults) {
        (&raw mut (*this).next).write(std::ptr::null_mut());
        (&raw mut (*this).value).write(3);
    }
    pub unsafe fn copy_from(this: *mut node_with_defaults, _a0: *const node_with_defaults) {
        (&raw mut (*this).next).write((*_a0).next);
        (&raw mut (*this).value).write((*_a0).value);
    }
    pub unsafe fn move_from(this: *mut node_with_defaults, _a0: *mut node_with_defaults) {
        (&raw mut (*this).next).write((*_a0).next);
        (&raw mut (*this).value).write((*_a0).value);
    }
}
impl Default for node_with_defaults {
    fn default() -> Self {
        unsafe {
            node_with_defaults {
                next: std::ptr::null_mut(),
                value: 3,
            }
        }
    }
}
pub unsafe fn init_0(mut l: *mut list_head) -> *mut list_head {
    (*l).next = std::ptr::null_mut();
    return l;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut list: list_head = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut list;
        (&raw mut (*__d).next).write((&mut list as *mut list_head));
    };
    assert!(((list.next) == (&mut list as *mut list_head)));
    let mut n: node = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut n;
        (&raw mut (*__d).value).write(42);
        (&raw mut (*__d).value_ptr).write((&mut n.value as *mut i32));
    };
    assert!(((n.value_ptr) == (&mut n.value as *mut i32)));
    (*n.value_ptr) = 7;
    assert!(((n.value) == (7)));
    let mut arr: [list_head; 2] = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = (&raw mut arr).cast::<list_head>();
        {
            let __d = __d.offset(0);
            (&raw mut (*__d).next).write((&mut arr[(1) as usize] as *mut list_head));
        };
        {
            let __d = __d.offset(1);
            (&raw mut (*__d).next).write((&mut arr[(0) as usize] as *mut list_head));
        };
    };
    assert!(((arr[(0) as usize].next) == (&mut arr[(1) as usize] as *mut list_head)));
    assert!(((arr[(1) as usize].next) == (&mut arr[(0) as usize] as *mut list_head)));
    let mut d: node_with_defaults = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut d;
        (&raw mut (*__d).next).write((&mut d as *mut node_with_defaults));
        (&raw mut (*__d).value).write(3);
    };
    assert!(((d.next) == (&mut d as *mut node_with_defaults)));
    assert!(((d.value) == (3)));
    let mut called: list_head = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut called;
        (&raw mut (*__d).next).write((unsafe { init_0((&mut called as *mut list_head)) }));
    };
    assert!(((called.next) == (&mut called as *mut list_head)));
    let mut size: usize = ::std::mem::size_of::<usize>();
    assert!(((size) == (::std::mem::size_of::<usize>())));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
