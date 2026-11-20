extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut ctorCount_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut copyCount_1: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut dtorCount_2: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut useCount_3: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct LifetimeTracked {
    pub uses: i32,
}
impl LifetimeTracked {
    pub unsafe fn reset() {
        (*(*&raw const ctorCount_0).get()) = {
            (*(*&raw const copyCount_1).get()) = {
                (*(*&raw const dtorCount_2).get()) = {
                    (*(*&raw const useCount_3).get()) = 0;
                    (*(*&raw const useCount_3).get())
                };
                (*(*&raw const dtorCount_2).get())
            };
            (*(*&raw const copyCount_1).get())
        };
    }
    pub unsafe fn new(this: *mut LifetimeTracked) {
        (&raw mut (*this).uses).write(0);
        (*(*&raw const ctorCount_0).get()).prefix_inc();
    }
    pub unsafe fn copy_from(this: *mut LifetimeTracked, _a0: *const LifetimeTracked) {
        (&raw mut (*this).uses).write(0);
        (*(*&raw const copyCount_1).get()).prefix_inc();
    }
    pub unsafe fn destructor(&mut self) {
        (*(*&raw const dtorCount_2).get()).prefix_inc();
    }
    pub unsafe fn use_(&self) {
        (*(&raw const self.uses).cast_mut()).prefix_inc();
        (*(*&raw const useCount_3).get()).prefix_inc();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const LifetimeTracked) -> *mut LifetimeTracked {
        self.uses = (*_a0).uses;
        return &mut (*(self as *mut LifetimeTracked));
    }
}
impl Clone for LifetimeTracked {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: LifetimeTracked = ::std::mem::MaybeUninit::uninit().assume_init();
            LifetimeTracked::copy_from(&raw mut __v, self as *const LifetimeTracked);
            __v
        }
    }
}
impl Default for LifetimeTracked {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: LifetimeTracked = ::std::mem::MaybeUninit::uninit().assume_init();
                LifetimeTracked::new(&raw mut __v);
                __v
            }
        }
    }
}
pub static mut matching_case_runs_4: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub static mut other_case_runs_5: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub static mut dtorCount_6: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CppFromUnsafe, DestructorUnsafe, Default)]
#[cpp_into(i32, to_i32)]
pub struct p2_condition_variable_lives_for_whole_statement_ConvertsToInt {
    pub value: i32,
}
impl p2_condition_variable_lives_for_whole_statement_ConvertsToInt {
    pub unsafe fn to_i32(&self) -> i32 {
        return self.value;
    }
    pub unsafe fn destructor(&mut self) {
        (*(*&raw const dtorCount_6).get()).prefix_inc();
    }
    pub unsafe fn new_1(this: *mut p2_condition_variable_lives_for_whole_statement_ConvertsToInt) {}
    pub unsafe fn copy_from(
        this: *mut p2_condition_variable_lives_for_whole_statement_ConvertsToInt,
        _a0: *const p2_condition_variable_lives_for_whole_statement_ConvertsToInt,
    ) {
        (&raw mut (*this).value).write((*_a0).value);
    }
}
pub unsafe fn test_7() {
    {
        let mut selector: p2_condition_variable_lives_for_whole_statement_ConvertsToInt =
            ::std::mem::MaybeUninit::uninit().assume_init();
        {
            let __d = &raw mut selector;
            (&raw mut (*__d).value).write(2);
        };
        let _dtor_selector =
            std::mem::DropGuard::new(&raw mut selector, |__p| unsafe { (*__p).destructor() });
        'switch: {
            match {
                (unsafe {
                    p2_condition_variable_lives_for_whole_statement_ConvertsToInt::to_i32(&selector)
                })
            } {
                __v if __v == 2 => {
                    (*(*&raw const matching_case_runs_4).get()).prefix_inc();
                    assert!(((selector.value) == (2)));
                    assert!(((*(*&raw const dtorCount_6).get()) == (0)));
                    break 'switch;
                }
                _ => {
                    (*(*&raw const other_case_runs_5).get()).prefix_inc();
                    break 'switch;
                }
            }
        }
    };
    assert!(((*(*&raw const matching_case_runs_4).get()) == (1)));
    assert!(((*(*&raw const other_case_runs_5).get()) == (0)));
    assert!(((*(*&raw const dtorCount_6).get()) == (1)));
}
pub static mut matching_case_runs_8: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub static mut other_case_runs_9: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe fn test_10() {
    (unsafe { LifetimeTracked::reset() });
    let mut selector: i32 = 9;
    {
        let mut guard: LifetimeTracked = ::std::mem::MaybeUninit::uninit().assume_init();
        LifetimeTracked::new(&raw mut guard);
        let _dtor_guard =
            std::mem::DropGuard::new(&raw mut guard, |__p| unsafe { (*__p).destructor() });
        let mut selector: i32 = 2;
        'switch: {
            match { selector } {
                __v if __v == 2 => {
                    (*(*&raw const matching_case_runs_8).get()).prefix_inc();
                    (unsafe { LifetimeTracked::use_(&guard) });
                    assert!(((guard.uses) == (1)));
                    assert!(((selector) == (2)));
                    assert!(((*(*&raw const dtorCount_2).get()) == (0)));
                    break 'switch;
                }
                _ => {
                    (*(*&raw const other_case_runs_9).get()).prefix_inc();
                    break 'switch;
                }
            }
        }
    };
    assert!(((*(*&raw const matching_case_runs_8).get()) == (1)));
    assert!(((*(*&raw const other_case_runs_9).get()) == (0)));
    assert!(((*(*&raw const ctorCount_0).get()) == (1)));
    assert!(((*(*&raw const dtorCount_2).get()) == (1)));
    assert!(((selector) == (9)));
}
pub unsafe fn classify_11(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        match { x } {
            __v if __v == 1 => {
                std::hint::cold_path();
                return -1_i32;
            }
            __v if __v == 2 => {
                r = 2;
                break 'switch;
            }
            __v if __v == 3 => {
                std::hint::cold_path();
                r = 3;
                break 'switch;
            }
            _ => {
                break 'switch;
            }
        }
    };
    'loop_: while ((x.postfix_dec()) > (0)) {
        std::hint::cold_path();
        {
            r += 10;
        };
    }
    return r;
}
pub unsafe fn test_12() {
    assert!(((unsafe { classify_11(1,) }) == (-1_i32)));
    assert!(((unsafe { classify_11(2,) }) == (22)));
    assert!(((unsafe { classify_11(3,) }) == (33)));
    assert!(((unsafe { classify_11(0,) }) == (0)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_12() });
    (unsafe { test_7() });
    (unsafe { test_10() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const ctorCount_0).get();
    (*&raw const copyCount_1).get();
    (*&raw const dtorCount_2).get();
    (*&raw const useCount_3).get();
    (*&raw const matching_case_runs_4).get();
    (*&raw const other_case_runs_5).get();
    (*&raw const dtorCount_6).get();
    (*&raw const matching_case_runs_8).get();
    (*&raw const other_case_runs_9).get();
}
