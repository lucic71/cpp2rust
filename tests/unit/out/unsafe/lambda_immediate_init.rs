extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn pick_0(mut x: i32) -> i32 {
    return x;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct S {
    pub j: i32,
    pub k: i32,
}
impl S {
    pub unsafe fn new_1(this: *mut S) {
        (&raw mut (*this).j).write(10);
        (&raw mut (*this).k).write(
            ((unsafe {
                FnPtr::<fn() -> i32>::new(|| -> i32 {
                    unsafe {
                        return 3;
                    }
                })
                .call()
            }) + (1)),
        );
    }
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).j).write((*_a0).j);
        (&raw mut (*this).k).write((*_a0).k);
    }
    pub unsafe fn move_from(this: *mut S, _a0: *mut S) {
        (&raw mut (*this).j).write((*_a0).j);
        (&raw mut (*this).k).write((*_a0).k);
    }
}
impl Default for S {
    fn default() -> Self {
        unsafe {
            S {
                j: 10,
                k: ((unsafe {
                    FnPtr::<fn() -> i32>::new(|| -> i32 {
                        unsafe {
                            return 3;
                        }
                    })
                    .call()
                }) + (1)),
            }
        }
    }
}
pub static mut g_1: libcc2rs::LazyStorage<i32> = libcc2rs::LazyStorage::new(|__p| unsafe {
    __p.write(
        (unsafe {
            FnPtr::<fn() -> i32>::new(|| -> i32 {
                unsafe {
                    let mut s: i32 = 0;
                    {
                        let mut i: i32 = 1;
                        'loop_: while ((i) <= (4)) {
                            s += i;
                            i.postfix_inc();
                        }
                    }
                    return s;
                }
            })
            .call()
        }),
    )
});
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((unsafe {
            pick_0(
                (unsafe {
                    FnPtr::<fn() -> i32>::new(|| -> i32 {
                        unsafe {
                            return 237;
                        }
                    })
                    .call()
                }),
            )
        }) == (237))
    );
    assert!(((unsafe { pick_0(1,) }) == (1)));
    let mut s: S = ::std::mem::MaybeUninit::uninit().assume_init();
    S::new_1(&raw mut s);
    assert!(((s.k) == (4)));
    assert!(((*(*&raw const g_1).get()) == (10)));
    let mut a: i32 = 2;
    let c: i32 = (unsafe {
        lambda_unsafe!(
            {
                let a: *mut i32 = &mut a;
            },
            || -> i32 {
                (*a).postfix_inc();
                return ((*a) * (10));
            }
        )
        .call()
    });
    assert!(((c) == (30)));
    assert!(((a) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const g_1).get();
}
