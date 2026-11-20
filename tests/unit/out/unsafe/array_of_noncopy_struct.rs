extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct NonCopy {
    pub data: Vec<i32>,
    pub tag: i32,
}
impl NonCopy {
    pub unsafe fn copy_from(this: *mut NonCopy, _a0: *const NonCopy) {
        (&raw mut (*this).data).write((*_a0).data.clone());
        (&raw mut (*this).tag).write((*_a0).tag);
    }
    pub unsafe fn move_from(this: *mut NonCopy, _a0: *mut NonCopy) {
        (&raw mut (*this).data).write(std::mem::take(&mut (*_a0).data));
        (&raw mut (*this).tag).write((*_a0).tag);
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut NonCopy) -> *mut NonCopy {
        {
            self.data = std::mem::take(&mut (*_a0).data);
        };
        self.tag = (*_a0).tag;
        return &mut (*(self as *mut NonCopy));
    }
    pub unsafe fn new_1(this: *mut NonCopy) {
        (&raw mut (*this).data).write(Vec::new());
        (&raw mut (*this).tag).write(0);
    }
}
impl Default for NonCopy {
    fn default() -> Self {
        unsafe {
            NonCopy {
                data: Default::default(),
                tag: 0,
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
    let mut arr: [NonCopy; 3] = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = (&raw mut arr).cast::<NonCopy>();
        for __i in 0..3_isize {
            NonCopy::new_1(__d.offset(__i));
        }
    };
    arr[(0) as usize].tag = 7;
    {
        let __a1 = 42;
        arr[(1) as usize].data.push(__a1)
    };
    assert!(((arr[(0) as usize].tag) == (7)));
    assert!(((arr[(1) as usize].data.len()) == (1_usize)));
    assert!(((arr[(1) as usize].data[(0_usize)]) == (42)));
    assert!(((arr[(2) as usize].tag) == (0)));
    assert!(((arr[(2) as usize].data.len()) == (0_usize)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
