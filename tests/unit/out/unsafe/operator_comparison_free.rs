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
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn new_1(this: *mut S) {}
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut S, _a0: *mut S) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
impl std::cmp::Ord for S {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_0(self as *const S, other as *const S) {
                std::cmp::Ordering::Less
            } else if operator_lt_0(other as *const S, self as *const S) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for S {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_1(self as *const S, other as *const S) }
    }
}
impl std::cmp::Eq for S {}
pub unsafe fn operator_eq_1(a: *const S, b: *const S) -> bool {
    return (((*a).v) == ((*b).v));
}
pub unsafe fn operator_ne_2(a: *const S, b: *const S) -> bool {
    return (((*a).v) != ((*b).v));
}
pub unsafe fn operator_lt_0(a: *const S, b: *const S) -> bool {
    return (((*a).v) < ((*b).v));
}
pub unsafe fn operator_gt_3(a: *const S, b: *const S) -> bool {
    return (((*a).v) > ((*b).v));
}
pub unsafe fn operator_le_4(a: *const S, b: *const S) -> bool {
    return (((*a).v) <= ((*b).v));
}
pub unsafe fn operator_ge_5(a: *const S, b: *const S) -> bool {
    return (((*a).v) >= ((*b).v));
}
pub unsafe fn operator_lt_6(a: *const S, mut b: i32) -> bool {
    return (((*a).v) < (b));
}
pub unsafe fn operator_lt_7(mut a: i32, b: *const S) -> bool {
    return ((a) < ((*b).v));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct V {
    pub v: i32,
}
impl V {
    pub unsafe fn new_1(this: *mut V) {}
    pub unsafe fn copy_from(this: *mut V, _a0: *const V) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut V, _a0: *mut V) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
impl std::cmp::Ord for V {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_8(
                (&mut self.clone()) as *mut V,
                (&mut other.clone()) as *mut V,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_8(
                (&mut other.clone()) as *mut V,
                (&mut self.clone()) as *mut V,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for V {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for V {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_9(
                (&mut self.clone()) as *mut V,
                (&mut other.clone()) as *mut V,
            )
        }
    }
}
impl std::cmp::Eq for V {}
pub unsafe fn operator_eq_9(a: *mut V, b: *mut V) -> bool {
    return (((*a).v) == ((*b).v));
}
pub unsafe fn operator_ne_10(a: *mut V, b: *mut V) -> bool {
    return (((*a).v) != ((*b).v));
}
pub unsafe fn operator_lt_8(a: *mut V, b: *mut V) -> bool {
    return (((*a).v) < ((*b).v));
}
pub unsafe fn operator_gt_11(a: *mut V, b: *mut V) -> bool {
    return (((*a).v) > ((*b).v));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: V = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut x;
        (&raw mut (*__d).v).write(1);
    };
    let mut y: V = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut y;
        (&raw mut (*__d).v).write(2);
    };
    let mut z: V = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut z;
        (&raw mut (*__d).v).write(1);
    };
    assert!(
        (unsafe {
            let mut __tmp_0: V = ::std::mem::MaybeUninit::uninit().assume_init();
            V::copy_from(&raw mut __tmp_0, { &x });
            let _a = &raw mut __tmp_0;
            operator_eq_9(
                _a,
                (|__p: *mut V| {
                    V::copy_from(__p, { &z });
                    __p
                })((&mut ::std::mem::MaybeUninit::<V>::uninit()).as_mut_ptr()),
            )
        })
    );
    assert!(
        (unsafe {
            let mut __tmp_1: V = ::std::mem::MaybeUninit::uninit().assume_init();
            V::copy_from(&raw mut __tmp_1, { &x });
            let _a = &raw mut __tmp_1;
            operator_ne_10(
                _a,
                (|__p: *mut V| {
                    V::copy_from(__p, { &y });
                    __p
                })((&mut ::std::mem::MaybeUninit::<V>::uninit()).as_mut_ptr()),
            )
        })
    );
    assert!(
        (unsafe {
            let mut __tmp_2: V = ::std::mem::MaybeUninit::uninit().assume_init();
            V::copy_from(&raw mut __tmp_2, { &x });
            let _a = &raw mut __tmp_2;
            operator_lt_8(
                _a,
                (|__p: *mut V| {
                    V::copy_from(__p, { &y });
                    __p
                })((&mut ::std::mem::MaybeUninit::<V>::uninit()).as_mut_ptr()),
            )
        })
    );
    assert!(
        (unsafe {
            let mut __tmp_3: V = ::std::mem::MaybeUninit::uninit().assume_init();
            V::copy_from(&raw mut __tmp_3, { &y });
            let _a = &raw mut __tmp_3;
            operator_gt_11(
                _a,
                (|__p: *mut V| {
                    V::copy_from(__p, { &x });
                    __p
                })((&mut ::std::mem::MaybeUninit::<V>::uninit()).as_mut_ptr()),
            )
        })
    );
    assert!(
        !(unsafe {
            let mut __tmp_4: V = ::std::mem::MaybeUninit::uninit().assume_init();
            V::copy_from(&raw mut __tmp_4, { &y });
            let _a = &raw mut __tmp_4;
            operator_lt_8(
                _a,
                (|__p: *mut V| {
                    V::copy_from(__p, { &x });
                    __p
                })((&mut ::std::mem::MaybeUninit::<V>::uninit()).as_mut_ptr()),
            )
        })
    );
    let mut a: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut a;
        (&raw mut (*__d).v).write(1);
    };
    let mut b: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut b;
        (&raw mut (*__d).v).write(2);
    };
    let mut c: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut c;
        (&raw mut (*__d).v).write(1);
    };
    assert!(
        (unsafe {
            let _a: *const S = &a;
            operator_eq_1(_a, &c)
        })
    );
    assert!(
        (unsafe {
            let _a: *const S = &a;
            operator_ne_2(_a, &b)
        })
    );
    assert!(
        (unsafe {
            let _a: *const S = &a;
            operator_lt_0(_a, &b)
        })
    );
    assert!(
        (unsafe {
            let _a: *const S = &b;
            operator_gt_3(_a, &a)
        })
    );
    assert!(
        (unsafe {
            let _a: *const S = &a;
            operator_le_4(_a, &c)
        })
    );
    assert!(
        (unsafe {
            let _a: *const S = &a;
            operator_ge_5(_a, &c)
        })
    );
    assert!(
        !(unsafe {
            let _a: *const S = &b;
            operator_lt_0(_a, &a)
        })
    );
    assert!(
        (unsafe {
            let _a: *const S = &a;
            operator_lt_6(_a, 5)
        })
    );
    assert!((unsafe { operator_lt_7(0, &a,) }));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
