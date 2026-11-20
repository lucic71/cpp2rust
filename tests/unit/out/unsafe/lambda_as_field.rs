extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct Guard__lambda_at_lambda_as_field_cpp_34_13__ {
    pub f: FnPtr<fn()>,
}
impl Guard__lambda_at_lambda_as_field_cpp_34_13__ {
    pub unsafe fn destructor(&mut self) {
        (unsafe { self.f.call() });
    }
    pub unsafe fn copy_from(
        this: *mut Guard__lambda_at_lambda_as_field_cpp_34_13__,
        _a0: *const Guard__lambda_at_lambda_as_field_cpp_34_13__,
    ) {
        (&raw mut (*this).f).write((*_a0).f.copy_from());
    }
}
impl Default for Guard__lambda_at_lambda_as_field_cpp_34_13__ {
    fn default() -> Self {
        unsafe {
            Guard__lambda_at_lambda_as_field_cpp_34_13__ {
                f: FnPtr::<fn()>::new(|| unsafe {
                    (*(*&raw const count_0).get()).postfix_inc();
                }),
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Guard__lambda_at_lambda_as_field_cpp_45_13__ {
    pub f: FnPtr<fn()>,
}
impl Guard__lambda_at_lambda_as_field_cpp_45_13__ {
    pub unsafe fn destructor(&mut self) {
        (unsafe { self.f.call() });
    }
    pub unsafe fn copy_from(
        this: *mut Guard__lambda_at_lambda_as_field_cpp_45_13__,
        _a0: *const Guard__lambda_at_lambda_as_field_cpp_45_13__,
    ) {
        (&raw mut (*this).f).write((*_a0).f.copy_from());
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct Holder__lambda_at_lambda_as_field_cpp_39_12__ {
    pub f: FnPtr<fn(i32) -> i32>,
    pub calls: i32,
}
impl Holder__lambda_at_lambda_as_field_cpp_39_12__ {
    pub unsafe fn call(&mut self, mut x: i32) -> i32 {
        self.calls.postfix_inc();
        return (unsafe { self.f.call(x) });
    }
    pub unsafe fn copy_from(
        this: *mut Holder__lambda_at_lambda_as_field_cpp_39_12__,
        _a0: *const Holder__lambda_at_lambda_as_field_cpp_39_12__,
    ) {
        (&raw mut (*this).f).write((*_a0).f.copy_from());
        (&raw mut (*this).calls).write((*_a0).calls);
    }
}
impl Default for Holder__lambda_at_lambda_as_field_cpp_39_12__ {
    fn default() -> Self {
        unsafe {
            Holder__lambda_at_lambda_as_field_cpp_39_12__ {
                f: FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
                    unsafe {
                        return ((x) + (1));
                    }
                }),
                calls: 0_i32,
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct Holder__lambda_at_lambda_as_field_cpp_51_12__ {
    pub f: FnPtr<fn(i32) -> i32>,
    pub calls: i32,
}
impl Holder__lambda_at_lambda_as_field_cpp_51_12__ {
    pub unsafe fn call(&mut self, mut x: i32) -> i32 {
        self.calls.postfix_inc();
        return (unsafe { self.f.call(x) });
    }
    pub unsafe fn copy_from(
        this: *mut Holder__lambda_at_lambda_as_field_cpp_51_12__,
        _a0: *const Holder__lambda_at_lambda_as_field_cpp_51_12__,
    ) {
        (&raw mut (*this).f).write((*_a0).f.copy_from());
        (&raw mut (*this).calls).write((*_a0).calls);
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe)]
#[cpp_from(FnPtr<fn(i32 ,) -> i32 > , new, value)]
pub struct Moving__lambda_at_lambda_as_field_cpp_64_12__ {
    pub f: FnPtr<fn(i32) -> i32>,
    pub moves: i32,
}
impl Moving__lambda_at_lambda_as_field_cpp_64_12__ {
    pub unsafe fn new(
        this: *mut Moving__lambda_at_lambda_as_field_cpp_64_12__,
        mut f: FnPtr<fn(i32) -> i32>,
    ) {
        (&raw mut (*this).f).write(f.copy_from());
        (&raw mut (*this).moves).write(0);
    }
    pub unsafe fn move_from(
        this: *mut Moving__lambda_at_lambda_as_field_cpp_64_12__,
        o: *mut Moving__lambda_at_lambda_as_field_cpp_64_12__,
    ) {
        (&raw mut (*this).f).write((*o).f.move_from());
        (&raw mut (*this).moves).write((((*o).moves) + (1)));
    }
}
impl Default for Moving__lambda_at_lambda_as_field_cpp_64_12__ {
    fn default() -> Self {
        unsafe {
            Moving__lambda_at_lambda_as_field_cpp_64_12__ {
                f: FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
                    unsafe {
                        return ((x) * (2));
                    }
                }),
                moves: 0,
            }
        }
    }
}
pub unsafe fn wrap_1(mut fn_: FnPtr<fn(i32) -> i32>) -> FnPtr<fn(i32) -> i32> {
    return lambda_unsafe!(
        {
            let fn_: FnPtr<fn(i32) -> i32> = fn_.copy_from();
        },
        |mut x: i32| -> i32 {
            return ((unsafe { fn_.call(x) }) + (1));
        }
    );
}
pub unsafe fn wrap_2(mut fn_: FnPtr<fn(i32) -> i32>) -> FnPtr<fn(i32) -> i32> {
    return lambda_unsafe!(
        {
            let fn_: FnPtr<fn(i32) -> i32> = fn_.copy_from();
        },
        |mut x: i32| -> i32 {
            return ((unsafe { fn_.call(x) }) + (1));
        }
    );
}
pub static mut count_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut g: Guard__lambda_at_lambda_as_field_cpp_34_13__ =
            ::std::mem::MaybeUninit::uninit().assume_init();
        {
            let __d = &raw mut g;
            (&raw mut (*__d).f).write(FnPtr::<fn()>::new(|| unsafe {
                (*(*&raw const count_0).get()).postfix_inc();
            }));
        };
        let _dtor_g = std::mem::DropGuard::new(&raw mut g, |__p| unsafe { (*__p).destructor() });
        assert!(((*(*&raw const count_0).get()) == (0)));
    }
    assert!(((*(*&raw const count_0).get()) == (1)));
    let mut n: Holder__lambda_at_lambda_as_field_cpp_39_12__ =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut n;
        (&raw mut (*__d).f).write(FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
            unsafe {
                return ((x) + (1));
            }
        }));
        (&raw mut (*__d).calls).write(0);
    };
    assert!(((unsafe { Holder__lambda_at_lambda_as_field_cpp_39_12__::call(&mut n, 2,) }) == (3)));
    assert!(((n.calls) == (1)));
    let mut cleaned: i32 = 0;
    {
        let mut g: Guard__lambda_at_lambda_as_field_cpp_45_13__ =
            ::std::mem::MaybeUninit::uninit().assume_init();
        {
            let __d = &raw mut g;
            (&raw mut (*__d).f).write(lambda_unsafe!(
                {
                    let cleaned: *mut i32 = &mut cleaned;
                },
                || {
                    (*cleaned).postfix_inc();
                }
            ));
        };
        let _dtor_g = std::mem::DropGuard::new(&raw mut g, |__p| unsafe { (*__p).destructor() });
        assert!(((cleaned) == (0)));
    }
    assert!(((cleaned) == (1)));
    let mut factor: i32 = 3;
    let mut h: Holder__lambda_at_lambda_as_field_cpp_51_12__ =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut h;
        (&raw mut (*__d).f).write(lambda_unsafe!(
            {
                let factor: i32 = factor;
            },
            |mut x: i32| -> i32 {
                return ((x) * (factor));
            }
        ));
        (&raw mut (*__d).calls).write(0);
    };
    factor = 100;
    assert!(((unsafe { Holder__lambda_at_lambda_as_field_cpp_51_12__::call(&mut h, 2,) }) == (6)));
    assert!(((unsafe { Holder__lambda_at_lambda_as_field_cpp_51_12__::call(&mut h, 5,) }) == (15)));
    assert!(((h.calls) == (2)));
    let mut w: FnPtr<fn(i32) -> i32> = (unsafe {
        wrap_1(lambda_unsafe!(
            {
                let factor: i32 = factor;
            },
            |mut x: i32| -> i32 {
                return ((x) * (factor));
            }
        ))
    });
    factor = 7;
    assert!(((unsafe { w.call(2,) }) == (201)));
    let mut ww: FnPtr<fn(i32) -> i32> = (unsafe { wrap_2(w.copy_from()) });
    assert!(((unsafe { ww.call(2,) }) == (202)));
    let mut m: Moving__lambda_at_lambda_as_field_cpp_64_12__ =
        ::std::mem::MaybeUninit::uninit().assume_init();
    Moving__lambda_at_lambda_as_field_cpp_64_12__::new(&raw mut m, {
        FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
            unsafe {
                return ((x) * (2));
            }
        })
    });
    let mut m2: Moving__lambda_at_lambda_as_field_cpp_64_12__ =
        ::std::mem::MaybeUninit::uninit().assume_init();
    Moving__lambda_at_lambda_as_field_cpp_64_12__::move_from(&raw mut m2, { &mut m });
    assert!(((unsafe { m2.f.call(3,) }) == (6)));
    assert!(((m2.moves) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const count_0).get();
}
