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
pub struct p7_4_pointer_to_member_and_null_HasTwoInts {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
impl p7_4_pointer_to_member_and_null_HasTwoInts {
    pub fn copy_from(
        this: Ptr<p7_4_pointer_to_member_and_null_HasTwoInts>,
        _a0: Ptr<p7_4_pointer_to_member_and_null_HasTwoInts>,
    ) {
        field!(this, first).write(_a0.with(|__s| __s.first));
        field!(this, second).write(_a0.with(|__s| __s.second));
    }
}
pub fn second_or_null_0(mut choose_second: bool) -> isize {
    return if choose_second {
        (offset_of_field!(p7_4_pointer_to_member_and_null_HasTwoInts, second) as isize)
    } else {
        -1_isize
    };
}
pub fn test_1() {
    let obj: Value<p7_4_pointer_to_member_and_null_HasTwoInts> = Rc::new(RefCell::new(
        <p7_4_pointer_to_member_and_null_HasTwoInts>::uninit(),
    ));
    {
        let __d = obj.as_pointer();
        field!(__d, first).write(3);
        field!(__d, second).write(4);
    };
    assert!((({ second_or_null_0(false,) }) == -1_isize));
    assert!(
        (({ second_or_null_0(true,) })
            == (offset_of_field!(p7_4_pointer_to_member_and_null_HasTwoInts, second) as isize))
    );
    assert!(
        (((obj.as_pointer())
            .member::<i32>(({ second_or_null_0(true,) }))
            .read())
            == 4)
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
pub fn __cpp2rust_init_globals() {}
