extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut scaled_calls_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub static mut rate_1: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(1) });
pub unsafe fn scaled_2(mut amount: i32) -> i32 {
    (*(*&raw const scaled_calls_0).get()).prefix_inc();
    return ((amount) * (10));
}
pub unsafe fn total_3(mut base: i32) -> i32 {
    return base;
}
pub unsafe fn test_4() {
    (*(*&raw const rate_1).get()) = 2;
    {
        let mut rate: i32 = 3;
        assert!(
            ((unsafe { total_3((unsafe { scaled_2((*(*&raw const rate_1).get()),) }),) }) == (20))
        );
        assert!(((*(*&raw const scaled_calls_0).get()) == (1)));
        assert!(((rate) == (3)));
    }
    assert!(((unsafe { total_3(5,) }) == (5)));
    assert!(((*(*&raw const scaled_calls_0).get()) == (1)));
}
pub static mut next_ticket_calls_5: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe fn next_ticket_6() -> i32 {
    return ((*(*(*&raw const next_ticket_calls_5).get()).prefix_inc()) * (100));
}
pub unsafe fn issue_7(mut ticket: i32) -> i32 {
    return ticket;
}
pub unsafe fn test_8() {
    assert!(((*(*&raw const next_ticket_calls_5).get()) == (0)));
    assert!(((unsafe { issue_7((unsafe { next_ticket_6() }),) }) == (100)));
    assert!(((*(*&raw const next_ticket_calls_5).get()) == (1)));
    assert!(((unsafe { issue_7(7,) }) == (7)));
    assert!(((*(*&raw const next_ticket_calls_5).get()) == (1)));
    assert!(((unsafe { issue_7((unsafe { next_ticket_6() }),) }) == (200)));
    assert!(((*(*&raw const next_ticket_calls_5).get()) == (2)));
}
pub static mut step_9: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(100) });
pub static mut step_10: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(4) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p9_static_member_hides_namespace_variable_HasStaticStep {
    pub total: i32,
}
impl p9_static_member_hides_namespace_variable_HasStaticStep {
    pub unsafe fn advance(&mut self, mut amount: i32) -> i32 {
        self.total += amount;
        return self.total;
    }
    pub unsafe fn new_1(this: *mut p9_static_member_hides_namespace_variable_HasStaticStep) {
        (&raw mut (*this).total).write(1);
    }
    pub unsafe fn copy_from(
        this: *mut p9_static_member_hides_namespace_variable_HasStaticStep,
        _a0: *const p9_static_member_hides_namespace_variable_HasStaticStep,
    ) {
        (&raw mut (*this).total).write((*_a0).total);
    }
}
impl Default for p9_static_member_hides_namespace_variable_HasStaticStep {
    fn default() -> Self {
        unsafe { p9_static_member_hides_namespace_variable_HasStaticStep { total: 1 } }
    }
}
pub unsafe fn test_11() {
    let mut object: p9_static_member_hides_namespace_variable_HasStaticStep =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p9_static_member_hides_namespace_variable_HasStaticStep::new_1(&raw mut object);
    assert!(
        ((unsafe {
            p9_static_member_hides_namespace_variable_HasStaticStep::advance(
                &mut object,
                (*(*&raw const step_10).get()),
            )
        }) == (5))
    );
    (*(*&raw const step_10).get()) = 6;
    assert!(
        ((unsafe {
            p9_static_member_hides_namespace_variable_HasStaticStep::advance(
                &mut object,
                (*(*&raw const step_10).get()),
            )
        }) == (11))
    );
    assert!(
        ((unsafe {
            p9_static_member_hides_namespace_variable_HasStaticStep::advance(&mut object, 2)
        }) == (13))
    );
    assert!(((*(*&raw const step_9).get()) == (100)));
}
pub static mut defaults_evaluated_12: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe fn count_default_13() -> i32 {
    return (*(*(*&raw const defaults_evaluated_12).get()).prefix_inc());
}
pub unsafe fn combine_14(mut first: i32, mut second: i32) -> i32 {
    return (((first) * (100)) + (second));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new, value)]
pub struct p9_evaluated_each_time_called_Widget {
    pub id: i32,
}
impl p9_evaluated_each_time_called_Widget {
    pub unsafe fn new(this: *mut p9_evaluated_each_time_called_Widget, mut id: i32) {
        (&raw mut (*this).id).write(id);
    }
    pub unsafe fn copy_from(
        this: *mut p9_evaluated_each_time_called_Widget,
        _a0: *const p9_evaluated_each_time_called_Widget,
    ) {
        (&raw mut (*this).id).write((*_a0).id);
    }
}
impl Default for p9_evaluated_each_time_called_Widget {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: p9_evaluated_each_time_called_Widget =
                    ::std::mem::MaybeUninit::uninit().assume_init();
                p9_evaluated_each_time_called_Widget::new(&raw mut __v, {
                    (unsafe { count_default_13() })
                });
                __v
            }
        }
    }
}
static mut base_15: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(10) });
pub type anon_16 = u32;
pub const anon_16_kBonus: anon_16 = 5;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p9_evaluated_each_time_called_Account {
    pub balance: i32,
}
impl p9_evaluated_each_time_called_Account {
    unsafe fn twice(mut value: i32) -> i32 {
        return ((2) * (value));
    }
    pub unsafe fn deposit(&mut self, mut amount: i32) -> i32 {
        self.balance += amount;
        return self.balance;
    }
    pub unsafe fn set_base(mut value: i32) {
        (*(*&raw const base_15).get()) = value;
    }
    pub unsafe fn new_1(this: *mut p9_evaluated_each_time_called_Account) {
        (&raw mut (*this).balance).write(0);
    }
    pub unsafe fn copy_from(
        this: *mut p9_evaluated_each_time_called_Account,
        _a0: *const p9_evaluated_each_time_called_Account,
    ) {
        (&raw mut (*this).balance).write((*_a0).balance);
    }
}
impl Default for p9_evaluated_each_time_called_Account {
    fn default() -> Self {
        unsafe { p9_evaluated_each_time_called_Account { balance: 0 } }
    }
}
pub unsafe fn test_17() {
    (*(*&raw const defaults_evaluated_12).get()) = 0;
    assert!(((unsafe { combine_14(1, (unsafe { count_default_13() }),) }) == (101)));
    assert!(((unsafe { combine_14(2, (unsafe { count_default_13() }),) }) == (202)));
    assert!(((unsafe { combine_14(3, 9,) }) == (309)));
    assert!(((*(*&raw const defaults_evaluated_12).get()) == (2)));
    let mut first: i32 = 0;
    assert!(
        ((unsafe { combine_14((*first.prefix_inc()), (unsafe { count_default_13() }),) }) == (103))
    );
    assert!(((first) == (1)));
    assert!(((*(*&raw const defaults_evaluated_12).get()) == (3)));
    let mut a: p9_evaluated_each_time_called_Widget =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p9_evaluated_each_time_called_Widget::new(&raw mut a, { (unsafe { count_default_13() }) });
    let mut b: p9_evaluated_each_time_called_Widget =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p9_evaluated_each_time_called_Widget::new(&raw mut b, { 40 });
    let mut c: p9_evaluated_each_time_called_Widget =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p9_evaluated_each_time_called_Widget::new(&raw mut c, { (unsafe { count_default_13() }) });
    assert!(((a.id) == (4)));
    assert!(((b.id) == (40)));
    assert!(((c.id) == (5)));
    assert!(((*(*&raw const defaults_evaluated_12).get()) == (5)));
    let mut account: p9_evaluated_each_time_called_Account =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p9_evaluated_each_time_called_Account::new_1(&raw mut account);
    assert!(
        ((unsafe {
            p9_evaluated_each_time_called_Account::deposit(
                &mut account,
                ((unsafe {
                    p9_evaluated_each_time_called_Account::twice((*(*&raw const base_15).get()))
                }) + (anon_16_kBonus as i32)),
            )
        }) == (25))
    );
    (unsafe { p9_evaluated_each_time_called_Account::set_base(1) });
    assert!(
        ((unsafe {
            p9_evaluated_each_time_called_Account::deposit(
                &mut account,
                ((unsafe {
                    p9_evaluated_each_time_called_Account::twice((*(*&raw const base_15).get()))
                }) + (anon_16_kBonus as i32)),
            )
        }) == (32))
    );
    assert!(
        ((unsafe { p9_evaluated_each_time_called_Account::deposit(&mut account, 3,) }) == (35))
    );
}
pub unsafe fn add_18(mut a: i32, mut b: i32) -> i32 {
    return ((a) + (b));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p9_not_part_of_the_type_Scale {}
impl p9_not_part_of_the_type_Scale {
    pub unsafe fn times(mut a: i32, mut b: i32) -> i32 {
        return ((a) * (b));
    }
    pub unsafe fn copy_from(
        this: *mut p9_not_part_of_the_type_Scale,
        _a0: *const p9_not_part_of_the_type_Scale,
    ) {
    }
}
pub unsafe fn descending_19(a: *const i32, b: *const i32) -> bool {
    return ((*a) > (*b));
}
pub unsafe fn test_20() {
    let mut p: Option<unsafe fn(i32, i32) -> i32> = Some(add_18);
    let mut q: Option<unsafe fn(i32, i32) -> i32> = (Some(add_18));
    assert!(((p) == (q)));
    assert!(((unsafe { (p).unwrap()(1, 5,) }) == (6)));
    assert!(((unsafe { (q).unwrap()(2, 5,) }) == (7)));
    assert!(((unsafe { add_18(1, 2,) }) == (3)));
    let mut s: Option<unsafe fn(i32, i32) -> i32> = (Some(p9_not_part_of_the_type_Scale::times));
    assert!(((unsafe { (s).unwrap()(2, 5,) }) == (10)));
    assert!(((unsafe { p9_not_part_of_the_type_Scale::times(2, 3,) }) == (6)));
    let mut values: Vec<i32> = Vec::from([3, -1_i32, 4, 1, 5]);
    {
        let mut __tmp_0 = { values.begin() };
        let __n = values.end().iter_distance(&__tmp_0);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            let __a = __tmp_0.iter_at(__x) as *const _;
            let __b = __tmp_0.iter_at(__y) as *const _;
            if (descending_19 as unsafe fn(*const i32, *const i32) -> bool).call(__a, __b) {
                std::cmp::Ordering::Less
            } else if (descending_19 as unsafe fn(*const i32, *const i32) -> bool).call(__b, __a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__tmp_0.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__tmp_0.iter_at(__i), __v);
        }
    };
    assert!(((values[(0_usize)]) == (5)));
    assert!(((values[(1_usize)]) == (4)));
    assert!(((values[(2_usize)]) == (3)));
    assert!(((values[(3_usize)]) == (1)));
    assert!(((values[(4_usize)]) == (-1_i32)));
    assert!(
        (unsafe {
            let mut _a: i32 = 1;
            let mut _b: i32 = 0;
            descending_19(&mut _a, &mut _b)
        })
    );
    assert!(
        !(unsafe {
            let mut _a: i32 = -1_i32;
            let mut _b: i32 = 0;
            descending_19(&mut _a, &mut _b)
        })
    );
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_4() });
    (unsafe { test_8() });
    (unsafe { test_11() });
    (unsafe { test_17() });
    (unsafe { test_20() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const scaled_calls_0).get();
    (*&raw const rate_1).get();
    (*&raw const next_ticket_calls_5).get();
    (*&raw const step_9).get();
    (*&raw const step_10).get();
    (*&raw const defaults_evaluated_12).get();
    (*&raw const base_15).get();
}
