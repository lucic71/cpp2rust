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
pub static mut then_runs_4: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub static mut else_runs_5: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub static mut dtorCount_6: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct p1_1_condition_variable_visible_in_else_ConvertsToBool {
    pub value: i32,
}
impl p1_1_condition_variable_visible_in_else_ConvertsToBool {
    pub unsafe fn to_bool(&self) -> bool {
        return ((self.value) != (0));
    }
    pub unsafe fn destructor(&mut self) {
        (*(*&raw const dtorCount_6).get()).prefix_inc();
    }
    pub unsafe fn copy_from(
        this: *mut p1_1_condition_variable_visible_in_else_ConvertsToBool,
        _a0: *const p1_1_condition_variable_visible_in_else_ConvertsToBool,
    ) {
        (&raw mut (*this).value).write((*_a0).value);
    }
}
pub unsafe fn test_7() {
    {
        let mut condition: p1_1_condition_variable_visible_in_else_ConvertsToBool =
            ::std::mem::MaybeUninit::uninit().assume_init();
        {
            let __d = &raw mut condition;
            (&raw mut (*__d).value).write(0);
        };
        let _dtor_condition =
            std::mem::DropGuard::new(&raw mut condition, |__p| unsafe { (*__p).destructor() });
        if (unsafe { p1_1_condition_variable_visible_in_else_ConvertsToBool::to_bool(&condition) })
        {
            (*(*&raw const then_runs_4).get()).prefix_inc();
        } else {
            (*(*&raw const else_runs_5).get()).prefix_inc();
            assert!(((condition.value) == (0)));
            assert!(((*(*&raw const dtorCount_6).get()) == (0)));
        }
    }
    assert!(((*(*&raw const then_runs_4).get()) == (0)));
    assert!(((*(*&raw const else_runs_5).get()) == (1)));
    assert!(((*(*&raw const dtorCount_6).get()) == (1)));
    {
        let mut condition: p1_1_condition_variable_visible_in_else_ConvertsToBool =
            ::std::mem::MaybeUninit::uninit().assume_init();
        {
            let __d = &raw mut condition;
            (&raw mut (*__d).value).write(6);
        };
        let _dtor_condition =
            std::mem::DropGuard::new(&raw mut condition, |__p| unsafe { (*__p).destructor() });
        if (unsafe { p1_1_condition_variable_visible_in_else_ConvertsToBool::to_bool(&condition) })
        {
            (*(*&raw const then_runs_4).get()).prefix_inc();
            assert!(((condition.value) == (6)));
            assert!(((*(*&raw const dtorCount_6).get()) == (1)));
        } else {
            (*(*&raw const else_runs_5).get()).prefix_inc();
        }
    }
    assert!(((*(*&raw const then_runs_4).get()) == (1)));
    assert!(((*(*&raw const else_runs_5).get()) == (1)));
    assert!(((*(*&raw const dtorCount_6).get()) == (2)));
}
pub unsafe fn test_8() {
    let mut index: i32 = 42;
    let mut iterations: i32 = 0;
    let mut sum: i32 = 0;
    {
        let mut index: i32 = 0;
        'loop_: while ((index) < (3)) {
            iterations.prefix_inc();
            sum += index;
            index.prefix_inc();
        }
    }
    assert!(((iterations) == (3)));
    assert!(((sum) == (3)));
    assert!(((index) == (42)));
}
pub unsafe fn test_9() {
    (unsafe { LifetimeTracked::reset() });
    let mut scoped: LifetimeTracked = ::std::mem::MaybeUninit::uninit().assume_init();
    LifetimeTracked::new(&raw mut scoped);
    let _dtor_scoped =
        std::mem::DropGuard::new(&raw mut scoped, |__p| unsafe { (*__p).destructor() });
    assert!(((*(*&raw const ctorCount_0).get()) == (1)));
    {
        let mut index: i32 = 0;
        'loop_: while ((index) < (3)) {
            let mut scoped: LifetimeTracked = ::std::mem::MaybeUninit::uninit().assume_init();
            LifetimeTracked::new(&raw mut scoped);
            let _dtor_scoped =
                std::mem::DropGuard::new(&raw mut scoped, |__p| unsafe { (*__p).destructor() });
            index.prefix_inc();
        }
    }
    assert!(((*(*&raw const ctorCount_0).get()) == (4)));
    assert!(((*(*&raw const dtorCount_2).get()) == (3)));
    if ((*(*&raw const ctorCount_0).get()) == (4)) {
        let mut scoped: LifetimeTracked = ::std::mem::MaybeUninit::uninit().assume_init();
        LifetimeTracked::new(&raw mut scoped);
        let _dtor_scoped =
            std::mem::DropGuard::new(&raw mut scoped, |__p| unsafe { (*__p).destructor() });
    }
    assert!(((*(*&raw const ctorCount_0).get()) == (5)));
    assert!(((*(*&raw const dtorCount_2).get()) == (4)));
    if ((*(*&raw const ctorCount_0).get()) == (0)) {
    } else {
        let mut scoped: LifetimeTracked = ::std::mem::MaybeUninit::uninit().assume_init();
        LifetimeTracked::new(&raw mut scoped);
        let _dtor_scoped =
            std::mem::DropGuard::new(&raw mut scoped, |__p| unsafe { (*__p).destructor() });
    }
    assert!(((*(*&raw const ctorCount_0).get()) == (6)));
    assert!(((*(*&raw const dtorCount_2).get()) == (5)));
    (unsafe { LifetimeTracked::use_(&scoped) });
    assert!(((scoped.uses) == (1)));
}
pub unsafe fn test_10() {
    let mut depth: i32 = 1;
    let mut outermost: *mut i32 = (&mut depth as *mut i32);
    {
        let mut depth: i32 = 2;
        (*outermost) += 10;
        assert!(((depth) == (2)));
        {
            let mut depth: i32 = 3;
            (*outermost) += 100;
            assert!(((depth) == (3)));
        }
        assert!(((depth) == (2)));
    }
    assert!(((depth) == (111)));
}
pub unsafe fn test_11() {
    (unsafe { LifetimeTracked::reset() });
    let mut tracked: LifetimeTracked = ::std::mem::MaybeUninit::uninit().assume_init();
    LifetimeTracked::new(&raw mut tracked);
    let _dtor_tracked =
        std::mem::DropGuard::new(&raw mut tracked, |__p| unsafe { (*__p).destructor() });
    {
        let mut tracked: LifetimeTracked = ::std::mem::MaybeUninit::uninit().assume_init();
        LifetimeTracked::new(&raw mut tracked);
        let _dtor_tracked =
            std::mem::DropGuard::new(&raw mut tracked, |__p| unsafe { (*__p).destructor() });
        assert!(((*(*&raw const ctorCount_0).get()) == (2)));
        (unsafe { LifetimeTracked::use_(&tracked) });
        assert!(((tracked.uses) == (1)));
        assert!(((*(*&raw const dtorCount_2).get()) == (0)));
    }
    assert!(((*(*&raw const dtorCount_2).get()) == (1)));
    assert!(((tracked.uses) == (0)));
    (unsafe { LifetimeTracked::use_(&tracked) });
    assert!(((tracked.uses) == (1)));
    assert!(((*(*&raw const useCount_3).get()) == (2)));
}
pub unsafe fn test_12() {
    let mut outer_iterations: i32 = 0;
    let mut inner_iterations: i32 = 0;
    let mut sum: i32 = 0;
    {
        let mut index: i32 = 0;
        'loop_: while ((index) < (2)) {
            {
                let mut index: i32 = 10;
                'loop_: while ((index) < (13)) {
                    inner_iterations.prefix_inc();
                    sum += index;
                    index.prefix_inc();
                }
            }
            {
                index.prefix_inc();
                outer_iterations.prefix_inc()
            };
        }
    }
    assert!(((outer_iterations) == (2)));
    assert!(((inner_iterations) == (6)));
    assert!(((sum) == (66)));
}
pub static mut inner_then_runs_13: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub static mut inner_else_runs_14: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub static mut outer_else_runs_15: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe fn test_16() {
    {
        let mut choice: i32 = 4;
        if (choice != 0) {
            {
                let mut choice: i32 = 0;
                if (choice != 0) {
                    (*(*&raw const inner_then_runs_13).get()).prefix_inc();
                } else {
                    (*(*&raw const inner_else_runs_14).get()).prefix_inc();
                    assert!(((choice) == (0)));
                }
            }
        } else {
            (*(*&raw const outer_else_runs_15).get()).prefix_inc();
            assert!(((choice) == (0)));
        }
    }
    assert!(((*(*&raw const inner_then_runs_13).get()) == (0)));
    assert!(((*(*&raw const inner_else_runs_14).get()) == (1)));
    assert!(((*(*&raw const outer_else_runs_15).get()) == (0)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_7() });
    (unsafe { test_8() });
    (unsafe { test_9() });
    (unsafe { test_10() });
    (unsafe { test_11() });
    (unsafe { test_12() });
    (unsafe { test_16() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const ctorCount_0).get();
    (*&raw const copyCount_1).get();
    (*&raw const dtorCount_2).get();
    (*&raw const useCount_3).get();
    (*&raw const then_runs_4).get();
    (*&raw const else_runs_5).get();
    (*&raw const dtorCount_6).get();
    (*&raw const inner_then_runs_13).get();
    (*&raw const inner_else_runs_14).get();
    (*&raw const outer_else_runs_15).get();
}
