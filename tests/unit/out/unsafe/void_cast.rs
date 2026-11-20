extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn unused_param_0(mut x: i32) {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct NonTrivial {
    pub data: Vec<i32>,
}
impl NonTrivial {
    pub unsafe fn copy_from(this: *mut NonTrivial, _a0: *const NonTrivial) {
        (&raw mut (*this).data).write((*_a0).data.clone());
    }
    pub unsafe fn move_from(this: *mut NonTrivial, _a0: *mut NonTrivial) {
        (&raw mut (*this).data).write(std::mem::take(&mut (*_a0).data));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut NonTrivial) -> *mut NonTrivial {
        {
            self.data = std::mem::take(&mut (*_a0).data);
        };
        return &mut (*(self as *mut NonTrivial));
    }
    pub unsafe fn new_1(this: *mut NonTrivial) {
        (&raw mut (*this).data).write(Vec::new());
    }
}
pub unsafe fn unused_ref_param_1(x: *const NonTrivial) {}
pub unsafe fn unused_ptr_param_2(mut p: *const NonTrivial) {}
pub static mut side_effect_counter_3: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub unsafe fn bump_and_return_4() -> i32 {
    (*(*&raw const side_effect_counter_3).get()).prefix_inc();
    return (*(*&raw const side_effect_counter_3).get());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Holder {
    pub field: i32,
}
impl Holder {
    pub unsafe fn new_1(this: *mut Holder) {}
    pub unsafe fn copy_from(this: *mut Holder, _a0: *const Holder) {
        (&raw mut (*this).field).write((*_a0).field);
    }
    pub unsafe fn move_from(this: *mut Holder, _a0: *mut Holder) {
        (&raw mut (*this).field).write((*_a0).field);
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct NonCopyable {
    pub value: Option<Box<i32>>,
}
impl NonCopyable {
    pub unsafe fn move_from(this: *mut NonCopyable, _a0: *mut NonCopyable) {
        (&raw mut (*this).value).write((*_a0).value.take());
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut NonCopyable) -> *mut NonCopyable {
        {
            self.value = (*_a0).value.take();
        };
        return &mut (*(self as *mut NonCopyable));
    }
    pub unsafe fn new_1(this: *mut NonCopyable) {
        (&raw mut (*this).value).write(None);
    }
}
pub unsafe fn unused_noncopyable_param_5(x: *const NonCopyable) {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { unused_param_0(42) });
    let mut y: i32 = 5;
    let mut z: i32 = {
        7
    };
    assert!(((z) == (7)));
    let mut counter: i32 = 0;
    let mut w: i32 = {
        {
            counter = 3
        };
        counter
    };
    assert!(((w) == (3)));
    assert!(((counter) == (3)));
    &(unsafe { bump_and_return_4() });
    assert!(((*(*&raw const side_effect_counter_3).get()) == (1)));
    let mut v: i32 = {
        &(unsafe { bump_and_return_4() });
        99
    };
    assert!(((*(*&raw const side_effect_counter_3).get()) == (2)));
    assert!(((v) == (99)));
    ();
    ();
    let mut err: i32 = 0;
    (&(err = 42));
    assert!(((err) == (42)));
    let mut chosen: i32 = {
        &(err = 7);
        123
    };
    assert!(((err) == (7)));
    assert!(((chosen) == (123)));
    assert!(((*(*&raw const side_effect_counter_3).get()) == (2)));
    assert!(((*(*&raw const side_effect_counter_3).get()) == (2)));
    assert!(((*(*&raw const side_effect_counter_3).get()) == (2)));
    let mut storage: i32 = 11;
    let mut p: *mut i32 = (&mut storage as *mut i32);
    let mut arr: [i32; 3] = [1, 2, 3];
    let mut h: Holder = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut h;
        (&raw mut (*__d).field).write(17);
    };
    let mut hp: *mut Holder = (&mut h as *mut Holder);
    let mut nt: NonTrivial = ::std::mem::MaybeUninit::uninit().assume_init();
    NonTrivial::new_1(&raw mut nt);
    (unsafe { unused_ref_param_1(&nt) });
    (unsafe { unused_ptr_param_2((&mut nt as *mut NonTrivial).cast_const()) });
    let mut g: NonCopyable = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut g;
        (&raw mut (*__d).value).write(Some(Box::new(9)));
    };
    ();
    (unsafe { unused_noncopyable_param_5(&g) });
    assert!(
        ((*g.value
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (9))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const side_effect_counter_3).get();
}
