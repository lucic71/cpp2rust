extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct p6_1_zero_initialized_pointer_to_member_HasTwoInts {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
impl p6_1_zero_initialized_pointer_to_member_HasTwoInts {
    pub fn copy_from(
        this: Ptr<p6_1_zero_initialized_pointer_to_member_HasTwoInts>,
        _a0: Ptr<p6_1_zero_initialized_pointer_to_member_HasTwoInts>,
    ) {
        field!(this, first).write(_a0.with(|__s| __s.first));
        field!(this, second).write(_a0.with(|__s| __s.second));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct p6_1_zero_initialized_pointer_to_member_HoldsSelector {
    #[offset(0)]
    #[byte_size(8)]
    pub selector: isize,
}
impl p6_1_zero_initialized_pointer_to_member_HoldsSelector {
    pub fn copy_from(
        this: Ptr<p6_1_zero_initialized_pointer_to_member_HoldsSelector>,
        _a0: Ptr<p6_1_zero_initialized_pointer_to_member_HoldsSelector>,
    ) {
        field!(this, selector).write(_a0.with(|__s| __s.selector));
    }
}
impl Default for p6_1_zero_initialized_pointer_to_member_HoldsSelector {
    fn default() -> Self {
        { p6_1_zero_initialized_pointer_to_member_HoldsSelector { selector: -1_isize } }
    }
}
thread_local!(
    pub static static_pm_0: Value<isize> = Rc::new(RefCell::new(-1_isize));
);
pub fn test_1() {
    assert!(((static_pm_0.with(|rc| *rc.borrow())) == -1_isize));
    assert!(
        (static_pm_0.with(|rc| *rc.borrow())
            != (offset_of_field!(p6_1_zero_initialized_pointer_to_member_HasTwoInts, first)
                as isize))
    );
    let mut value_initialized: isize = -1_isize;
    assert!(((value_initialized) == -1_isize));
    assert!(
        (value_initialized
            != (offset_of_field!(p6_1_zero_initialized_pointer_to_member_HasTwoInts, first)
                as isize))
    );
    let holder: Value<p6_1_zero_initialized_pointer_to_member_HoldsSelector> = Rc::new(
        RefCell::new(<p6_1_zero_initialized_pointer_to_member_HoldsSelector>::uninit()),
    );
    {
        let __d = holder.as_pointer();
        field!(__d, selector).write(-1_isize);
    };
    assert!(({ (({ (*holder.borrow()).selector }) == -1_isize) }));
    assert!(
        ({
            ({ (*holder.borrow()).selector }
                != (offset_of_field!(p6_1_zero_initialized_pointer_to_member_HasTwoInts, first)
                    as isize))
        })
    );
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_1() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = static_pm_0.with(|_| ());
}
