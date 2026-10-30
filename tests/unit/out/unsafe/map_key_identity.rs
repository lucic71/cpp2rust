extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut default_constructions_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Key {
    pub order: (),
    pub identity: (),
}
impl std::cmp::Ord for Key {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_1(self as *const Key, other as *const Key) {
                std::cmp::Ordering::Less
            } else if operator_lt_1(other as *const Key, self as *const Key) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_2(self as *const Key, other as *const Key) }
    }
}
impl std::cmp::Eq for Key {}
pub unsafe fn operator_lt_1(a: *const Key, b: *const Key) -> () {
    return (((*a).order) < ((*b).order));
}
pub unsafe fn operator_eq_2(a: *const Key, b: *const Key) -> () {
    return (((*a).identity) == ((*b).identity));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Mapped {
    pub value: (),
}
impl Mapped {
    pub unsafe fn new() -> Self {
        let mut this = Self { value: 0_() };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0)).prefix_inc();
        this
    }
}
impl Default for Mapped {
    fn default() -> Self {
        unsafe { Mapped::new() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut ok: () = true;
    let distinct_a: Key = Key {
        order: 1_(),
        identity: 7_(),
    };
    let distinct_b: Key = Key {
        order: 2_(),
        identity: 7_(),
    };
    ok &= (unsafe {
        let _a: *const Key = &distinct_a;
        operator_eq_2(_a, &distinct_b)
    });
    let mut distinct: () = unimplemented!();
    let a: *mut Mapped = &mut (*unimplemented!());
    (*a).value = 11_();
    let mut a_it: () = unimplemented!();
    let mut a_value: *mut Mapped = (&mut (*a_it).second as *mut Mapped);
    ok &= ((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0)) == (1_()));
    let b: *mut Mapped = &mut (*unimplemented!());
    (*b).value = 22_();
    let mut b_it: () = unimplemented!();
    ok &= ((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0)) == (2_()));
    ok &= ((unimplemented!()) == (2_()));
    ok &= ((a) != (b));
    ok &= ((((*a).value) == (11_())) && (((*b).value) == (22_())));
    ok &= (unimplemented!());
    ok &= (unimplemented!());
    ok &= (!(unimplemented!()));
    ok &= ((&mut (*a_it).second as *mut Mapped) == (a_value));
    ok &= (((*a_it).second.value) == (11_()));
    ok &= (unsafe {
        let _a: *const Key = &distinct_a;
        operator_eq_2(_a, &distinct_b)
    });
    let mut distinct_count: () = 0_();
    'loop_: for entry in IterRange::new(unimplemented!(), unimplemented!()) {
        let entry: *const () = &(*unimplemented!());
        if ((distinct_count) == (0_())) {
            ok &= ((((*entry).first.order) == (1_())) && (((*entry).second.value) == (11_())));
        } else if ((distinct_count) == (1_())) {
            ok &= ((((*entry).first.order) == (2_())) && (((*entry).second.value) == (22_())));
        }
        distinct_count.prefix_inc();
    }
    ok &= ((distinct_count) == (2_()));
    let equivalent_a: Key = Key {
        order: 3_(),
        identity: 8_(),
    };
    let equivalent_b: Key = Key {
        order: 3_(),
        identity: 9_(),
    };
    ok &= !(unsafe {
        let _a: *const Key = &equivalent_a;
        operator_eq_2(_a, &equivalent_b)
    });
    let mut equivalent: () = unimplemented!();
    let before_equivalent: () =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0));
    let first: *mut Mapped = &mut (*unimplemented!());
    (*first).value = 31_();
    let mut first_it: () = unimplemented!();
    let mut first_value: *mut Mapped = (&mut (*first_it).second as *mut Mapped);
    ok &= ((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0))
        == ((before_equivalent) + (1_())));
    let second: *mut Mapped = &mut (*unimplemented!());
    let reused: () = ((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0))
        == ((before_equivalent) + (1_())));
    ok &= reused;
    ok &= ((unimplemented!()) == (1_()));
    if reused {
        ok &= (!(unimplemented!()));
        ok &= ((first) == (second));
        ok &= (unimplemented!());
        ok &= (unimplemented!());
        ok &= ((&mut (*first_it).second as *mut Mapped) == (first_value));
        ok &= ((((*first_it).second.value) == (31_())) && (((*second).value) == (31_())));
        let mut equivalent_count: () = 0_();
        'loop_: for entry in IterRange::new(unimplemented!(), unimplemented!()) {
            let entry: *const () = &(*unimplemented!());
            ok &= ((((*entry).first.order) == (3_())) && (((*entry).second.value) == (31_())));
            equivalent_count.prefix_inc();
        }
        ok &= ((equivalent_count) == (1_()));
    }
    ok &= !(unsafe {
        let _a: *const Key = &equivalent_a;
        operator_eq_2(_a, &equivalent_b)
    });
    let later_key: Key = Key {
        order: 4_(),
        identity: 10_(),
    };
    let later: *mut Mapped = &mut (*unimplemented!());
    (*later).value = 41_();
    ok &= ((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0))
        == ((before_equivalent) + (2_())));
    ok &= ((unimplemented!()) == (2_()));
    if reused {
        ok &= (unimplemented!());
        ok &= ((&mut (*first_it).second as *mut Mapped) == (first_value));
        ok &= (((*first_it).second.value) == (31_()));
    }
    let matching_a: Key = Key {
        order: 5_(),
        identity: 5_(),
    };
    let matching_b: Key = Key {
        order: 5_(),
        identity: 5_(),
    };
    let matching_c: Key = Key {
        order: 6_(),
        identity: 6_(),
    };
    let before_matching: () =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0));
    let mut matching: () = unimplemented!();
    let matched: *mut Mapped = &mut (*unimplemented!());
    (*matched).value = 51_();
    let mut matched_it: () = unimplemented!();
    let mut matched_value: *mut Mapped = (&mut (*matched_it).second as *mut Mapped);
    ok &= ((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0))
        == ((before_matching) + (1_())));
    let matched_again: *mut Mapped = &mut (*unimplemented!());
    ok &= ((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0))
        == ((before_matching) + (1_())));
    ok &= ((unimplemented!()) == (1_()));
    ok &= ((matched) == (matched_again));
    ok &= (unimplemented!());
    ok &= ((&mut (*matched_it).second as *mut Mapped) == (matched_value));
    ok &= (((*matched_it).second.value) == (51_()));
    let other: *mut Mapped = &mut (*unimplemented!());
    (*other).value = 61_();
    ok &= ((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0))
        == ((before_matching) + (2_())));
    ok &= ((unimplemented!()) == (2_()));
    ok &= (unimplemented!());
    ok &= ((&mut (*matched_it).second as *mut Mapped) == (matched_value));
    ok &= (((*matched_it).second.value) == (51_()));
    return if ok { 0_() } else { 1_() };
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const default_constructions_0);
}
