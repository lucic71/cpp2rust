extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn pick_0() -> i32 {
    return 1;
}
pub unsafe fn test_1() {
    assert!(((unsafe { pick_0() }) == (1)));
    assert!(((unsafe { pick_in_b_2() }) == (2)));
}
pub static mut limit_3: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(10) });
pub unsafe fn test_4() {
    assert!(((*(*&raw const limit_3).get()) == (10)));
    assert!(((unsafe { limit_in_b_5() }) == (20)));
    assert!(
        ((&raw const (*(*&raw const limit_3).get()) as *const i32)
            != (unsafe { limit_address_in_b_6() }))
    );
}
pub unsafe fn test_8() {
    assert!(((*(*&raw const threshold_7).get()) == (30)));
    assert!(
        ((&raw const (*(*&raw const threshold_7).get()) as *const i32)
            == (unsafe { threshold_address_in_b_9() }))
    );
}
pub unsafe fn test_11() {
    assert!(((*(*&raw const ceiling_10).get()) == (40)));
    assert!(
        ((&raw const (*(*&raw const ceiling_10).get()) as *const i32)
            == (unsafe { ceiling_address_in_b_12() }))
    );
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_13 {
    pub whole: i32,
    pub fraction: f32,
}
impl Default for anon_13 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub static mut anon_13: libcc2rs::LazyStorage<anon_13> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(<anon_13>::default()) });
pub unsafe fn test_14() {
    (*(*&raw const anon_13).get()).whole = 5;
    (unsafe { set_whole_in_b_15(6) });
    assert!((((*(*&raw const anon_13).get()).whole) == (5)));
    assert!(((unsafe { whole_in_b_16() }) == (6)));
}
pub static mut counter_17: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(1) });
pub unsafe fn bump_18() -> i32 {
    return (*(*(*&raw const counter_17).get()).prefix_inc());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Record_19 {
    pub id: i32,
}
impl Record_19 {
    pub unsafe fn new_1(this: *mut Record_19) {
        (&raw mut (*this).id).write(100);
    }
    pub unsafe fn copy_from(this: *mut Record_19, _a0: *const Record_19) {
        (&raw mut (*this).id).write((*_a0).id);
    }
}
impl Default for Record_19 {
    fn default() -> Self {
        unsafe { Record_19 { id: 100 } }
    }
}
pub static mut depth_20: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(3) });
pub unsafe fn test_21() {
    assert!(((unsafe { bump_18() }) == (2)));
    assert!(((unsafe { bump_in_b_22() }) == (51)));
    assert!(((*(*&raw const counter_17).get()) == (2)));
    let mut record: Record_19 = ::std::mem::MaybeUninit::uninit().assume_init();
    Record_19::new_1(&raw mut record);
    assert!(((record.id) == (100)));
    assert!(((unsafe { record_id_in_b_23() }) == (200)));
    assert!(((*(*&raw const depth_20).get()) == (3)));
    assert!(((unsafe { depth_in_b_24() }) == (4)));
}
pub unsafe fn test_26() {
    assert!(((*(*&raw const shared_total_25).get()) == (5)));
    (*(*&raw const shared_total_25).get()) = 7;
    (unsafe { add_in_b_27(3) });
    assert!(((*(*&raw const shared_total_25).get()) == (10)));
}
pub unsafe fn test_29() {
    assert!(((*(*&raw const level_28).get()) == (1)));
    assert!(((unsafe { raise_30(2,) }) == (3)));
    (*(*&raw const level_28).get()) = 10;
    assert!(((unsafe { raise_30(1,) }) == (11)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Tally_31 {}
impl Tally_31 {
    pub unsafe fn add(&mut self, mut amount: i32) -> i32 {
        (*(*&raw const total_32).get()) += amount;
        return (*(*&raw const total_32).get());
    }
    pub unsafe fn new_1(this: *mut Tally_31) {}
    pub unsafe fn copy_from(this: *mut Tally_31, _a0: *const Tally_31) {}
}
pub static mut total_32: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(10) });
pub unsafe fn test_33() {
    let mut tally: Tally_31 = ::std::mem::MaybeUninit::uninit().assume_init();
    Tally_31::new_1(&raw mut tally);
    assert!(((unsafe { Tally_31::add(&mut tally, 1,) }) == (11)));
    assert!(((unsafe { add_in_b_34(1,) }) == (102)));
    assert!(((*(*&raw const total_32).get()) == (11)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p5_members_of_class_with_external_linkage_Ledger {}
impl p5_members_of_class_with_external_linkage_Ledger {
    pub unsafe fn copy_from(
        this: *mut p5_members_of_class_with_external_linkage_Ledger,
        _a0: *const p5_members_of_class_with_external_linkage_Ledger,
    ) {
    }
}
pub unsafe fn test_36() {
    assert!(((*(*&raw const balance_35).get()) == (50)));
    (*(*&raw const balance_35).get()) = 60;
    assert!(((unsafe { p5_members_of_class_with_external_linkage_Ledger::deposit(5,) }) == (65)));
    assert!(((*(*&raw const balance_35).get()) == (65)));
}
pub unsafe fn read_remote_37() -> i32 {
    return (*(*&raw const remote_38).get());
}
pub unsafe fn test_39() {
    assert!(((unsafe { read_remote_37() }) == (70)));
    (unsafe { set_remote_in_b_40(71) });
    assert!(((unsafe { read_remote_37() }) == (71)));
}
pub static mut hidden_41: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(80) });
pub unsafe fn read_hidden_through_extern_42() -> i32 {
    let mut hidden: i32 = 1;
    {
        return (*(*&raw const hidden_41).get());
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_43() {
    assert!(((unsafe { read_hidden_through_extern_42() }) == (80)));
    (*(*&raw const hidden_41).get()) = 81;
    assert!(((unsafe { read_hidden_through_extern_42() }) == (81)));
    assert!(((unsafe { hidden_in_b_44() }) == (90)));
}
pub unsafe fn next_even_45() -> i32 {
    (*(*&raw const count_46).get()) += 2;
    return (*(*&raw const count_46).get());
}
pub unsafe fn next_odd_47() -> i32 {
    (*(*&raw const count_48).get()) += 2;
    return (*(*&raw const count_48).get());
}
pub unsafe fn test_49() {
    assert!(((unsafe { next_even_45() }) == (2)));
    assert!(((unsafe { next_odd_47() }) == (3)));
    assert!(((unsafe { next_in_b_50() }) == (1000)));
    assert!(((unsafe { next_even_45() }) == (4)));
    assert!(((unsafe { next_odd_47() }) == (5)));
    assert!(((unsafe { next_in_b_50() }) == (2000)));
}
static mut count_46: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
static mut count_48: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(1) });
pub unsafe fn first_key_51() -> i32 {
    let mut entry: Entry_52 = ::std::mem::MaybeUninit::uninit().assume_init();
    Entry_52::new_1(&raw mut entry);
    return entry.key;
}
pub unsafe fn second_key_plus_value_53() -> i32 {
    let mut entry: Entry_54 = ::std::mem::MaybeUninit::uninit().assume_init();
    Entry_54::new_1(&raw mut entry);
    return ((entry.key) + (entry.value));
}
pub unsafe fn test_55() {
    assert!(((unsafe { first_key_51() }) == (1)));
    assert!(((unsafe { second_key_plus_value_53() }) == (32)));
    assert!(((unsafe { key_in_b_56() }) == (400)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Entry_52 {
    pub key: i32,
}
impl Entry_52 {
    pub unsafe fn new_1(this: *mut Entry_52) {
        (&raw mut (*this).key).write(1);
    }
    pub unsafe fn copy_from(this: *mut Entry_52, _a0: *const Entry_52) {
        (&raw mut (*this).key).write((*_a0).key);
    }
}
impl Default for Entry_52 {
    fn default() -> Self {
        unsafe { Entry_52 { key: 1 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Entry_54 {
    pub key: i32,
    pub value: i32,
}
impl Entry_54 {
    pub unsafe fn new_1(this: *mut Entry_54) {
        (&raw mut (*this).key).write(2);
        (&raw mut (*this).value).write(30);
    }
    pub unsafe fn copy_from(this: *mut Entry_54, _a0: *const Entry_54) {
        (&raw mut (*this).key).write((*_a0).key);
        (&raw mut (*this).value).write((*_a0).value);
    }
}
impl Default for Entry_54 {
    fn default() -> Self {
        unsafe { Entry_54 { key: 2, value: 30 } }
    }
}
pub unsafe fn test_58() {
    assert!((((*(*&raw const table_57).get())[(1) as usize]) == (2)));
    (*(*&raw const table_57).get())[(2) as usize] = 30;
    assert!(((unsafe { table_sum_in_b_59() }) == (33)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_1() });
    (unsafe { test_4() });
    (unsafe { test_8() });
    (unsafe { test_11() });
    (unsafe { test_14() });
    (unsafe { test_21() });
    (unsafe { test_26() });
    (unsafe { test_29() });
    (unsafe { test_33() });
    (unsafe { test_36() });
    (unsafe { test_39() });
    (unsafe { test_43() });
    (unsafe { test_49() });
    (unsafe { test_55() });
    (unsafe { test_58() });
    return 0;
}
pub unsafe fn pick_60() -> i32 {
    return 2;
}
pub unsafe fn pick_in_b_2() -> i32 {
    return (unsafe { pick_60() });
}
pub static mut limit_61: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(20) });
pub unsafe fn limit_in_b_5() -> i32 {
    return (*(*&raw const limit_61).get());
}
pub unsafe fn limit_address_in_b_6() -> *const i32 {
    return (&raw const (*(*&raw const limit_61).get()) as *const i32);
}
pub static mut threshold_7: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(30) });
pub unsafe fn threshold_address_in_b_9() -> *const i32 {
    return (&raw const (*(*&raw const threshold_7).get()) as *const i32);
}
pub static mut ceiling_10: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(40) });
pub unsafe fn ceiling_address_in_b_12() -> *const i32 {
    return (&raw const (*(*&raw const ceiling_10).get()) as *const i32);
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_62 {
    pub whole: i32,
    pub fraction: f32,
}
impl Default for anon_62 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub static mut anon_62: libcc2rs::LazyStorage<anon_62> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(<anon_62>::default()) });
pub unsafe fn set_whole_in_b_15(mut value: i32) {
    (*(*&raw const anon_62).get()).whole = value;
}
pub unsafe fn whole_in_b_16() -> i32 {
    return (*(*&raw const anon_62).get()).whole;
}
pub static mut counter_63: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(50) });
pub unsafe fn bump_64() -> i32 {
    return (*(*(*&raw const counter_63).get()).prefix_inc());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Record_65 {
    pub id: i64,
    pub extra: i32,
}
impl Record_65 {
    pub unsafe fn new_1(this: *mut Record_65) {
        (&raw mut (*this).id).write(200_i64);
        (&raw mut (*this).extra).write(1);
    }
    pub unsafe fn copy_from(this: *mut Record_65, _a0: *const Record_65) {
        (&raw mut (*this).id).write((*_a0).id);
        (&raw mut (*this).extra).write((*_a0).extra);
    }
}
impl Default for Record_65 {
    fn default() -> Self {
        unsafe {
            Record_65 {
                id: 200_i64,
                extra: 1,
            }
        }
    }
}
pub static mut depth_66: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(4) });
pub unsafe fn bump_in_b_22() -> i32 {
    return (unsafe { bump_64() });
}
pub unsafe fn record_id_in_b_23() -> i32 {
    let mut record: Record_65 = ::std::mem::MaybeUninit::uninit().assume_init();
    Record_65::new_1(&raw mut record);
    return (record.id as i32);
}
pub unsafe fn depth_in_b_24() -> i32 {
    return (*(*&raw const depth_66).get());
}
pub static mut shared_total_25: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(5) });
pub unsafe fn add_in_b_27(mut amount: i32) {
    (*(*&raw const shared_total_25).get()) += amount;
}
pub static mut level_28: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(1) });
pub unsafe fn raise_30(mut by: i32) -> i32 {
    (*(*&raw const level_28).get()) += by;
    return (*(*&raw const level_28).get());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Tally_67 {}
impl Tally_67 {
    pub unsafe fn add(&mut self, mut amount: i32) -> i32 {
        (*(*&raw const total_68).get()) += ((amount) * (2));
        return (*(*&raw const total_68).get());
    }
    pub unsafe fn new_1(this: *mut Tally_67) {}
    pub unsafe fn copy_from(this: *mut Tally_67, _a0: *const Tally_67) {}
}
pub static mut total_68: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(100) });
pub unsafe fn add_in_b_34(mut amount: i32) -> i32 {
    let mut tally: Tally_67 = ::std::mem::MaybeUninit::uninit().assume_init();
    Tally_67::new_1(&raw mut tally);
    return (unsafe { Tally_67::add(&mut tally, amount) });
}
pub static mut balance_35: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(50) });
impl p5_members_of_class_with_external_linkage_Ledger {
    pub unsafe fn deposit(mut amount: i32) -> i32 {
        (*(*&raw const balance_35).get()) += amount;
        return (*(*&raw const balance_35).get());
    }
}
pub static mut remote_38: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(70) });
pub unsafe fn set_remote_in_b_40(mut value: i32) {
    (*(*&raw const remote_38).get()) = value;
}
pub static mut hidden_69: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(90) });
pub unsafe fn hidden_in_b_44() -> i32 {
    return (*(*&raw const hidden_69).get());
}
pub unsafe fn next_in_b_50() -> i32 {
    (*(*&raw const count_70).get()) += 1000;
    return (*(*&raw const count_70).get());
}
static mut count_70: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub unsafe fn key_in_b_56() -> i32 {
    let mut entry: Entry_71 = ::std::mem::MaybeUninit::uninit().assume_init();
    Entry_71::new_1(&raw mut entry);
    return (entry.key as i32);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Entry_71 {
    pub key: i64,
}
impl Entry_71 {
    pub unsafe fn new_1(this: *mut Entry_71) {
        (&raw mut (*this).key).write(400_i64);
    }
    pub unsafe fn copy_from(this: *mut Entry_71, _a0: *const Entry_71) {
        (&raw mut (*this).key).write((*_a0).key);
    }
}
impl Default for Entry_71 {
    fn default() -> Self {
        unsafe { Entry_71 { key: 400_i64 } }
    }
}
pub static mut table_57: libcc2rs::LazyStorage<[i32; 3]> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write([1, 2, 3]) });
pub unsafe fn table_sum_in_b_59() -> i32 {
    return ((((*(*&raw const table_57).get())[(0) as usize])
        + ((*(*&raw const table_57).get())[(1) as usize]))
        + ((*(*&raw const table_57).get())[(2) as usize]));
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const limit_3).get();
    (*&raw const anon_13).get();
    (*&raw const counter_17).get();
    (*&raw const depth_20).get();
    (*&raw const total_32).get();
    (*&raw const hidden_41).get();
    (*&raw const limit_61).get();
    (*&raw const threshold_7).get();
    (*&raw const ceiling_10).get();
    (*&raw const anon_62).get();
    (*&raw const counter_63).get();
    (*&raw const depth_66).get();
    (*&raw const shared_total_25).get();
    (*&raw const level_28).get();
    (*&raw const total_68).get();
    (*&raw const balance_35).get();
    (*&raw const remote_38).get();
    (*&raw const hidden_69).get();
    (*&raw const table_57).get();
}
