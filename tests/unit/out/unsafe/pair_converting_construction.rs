extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct Celsius {
    pub degrees: i32,
}
impl Celsius {
    pub unsafe fn new(this: *mut Celsius, mut d: i32) {
        (&raw mut (*this).degrees).write(d);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(i32, to_i32)]
pub struct Meters {
    pub value: i32,
}
impl Meters {
    pub unsafe fn to_i32(&self) -> i32 {
        return self.value;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(Celsius, new, ptr)]
pub struct Label {
    pub id: i32,
}
impl Label {
    pub unsafe fn new(this: *mut Label, c: *const Celsius) {
        (&raw mut (*this).id).write((*c).degrees);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(Celsius, new_1, ptr)]
#[cpp_from_move(Celsius, new_2, ptr)]
pub struct Owner {
    pub taken: i32,
    pub copied: i32,
}
impl Owner {
    pub unsafe fn new_1(this: *mut Owner, c: *const Celsius) {
        (&raw mut (*this).taken).write(0);
        (&raw mut (*this).copied).write((*c).degrees);
    }
    pub unsafe fn new_2(this: *mut Owner, c: *mut Celsius) {
        (&raw mut (*this).taken).write((*c).degrees);
        (&raw mut (*this).copied).write(0);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(Celsius, new, storage)]
pub struct Holder {
    pub value: i32,
}
impl Holder {
    pub unsafe fn new(this: *mut Holder, c: *mut Celsius) {
        (&raw mut (*this).value).write((*c).degrees);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new, value)]
pub struct Fallback {
    pub value: i32,
}
impl Fallback {
    pub unsafe fn new(this: *mut Fallback, mut v: i32) {
        (&raw mut (*this).value).write(v);
    }
}
impl Default for Fallback {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: Fallback = ::std::mem::MaybeUninit::uninit().assume_init();
                Fallback::new(&raw mut __v, { -1_i32 });
                __v
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Counter {
    pub hits: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(Counter, new, mut_ptr)]
pub struct Observer {
    pub seen: i32,
}
impl Observer {
    pub unsafe fn new(this: *mut Observer, c: *mut Counter) {
        (&raw mut (*this).seen).write((*c).hits);
        (*c).hits.postfix_inc();
    }
}
impl Tag {
    pub unsafe fn new(this: *mut Tag, u: *const Celsius) {
        (&raw mut (*this).code).write((((*u).degrees) + (100)));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(Celsius, new, ptr)]
pub struct Tag {
    pub code: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(*const libc::c_char , new, value)]
pub struct Name {
    pub len: i32,
}
impl Name {
    pub unsafe fn new(this: *mut Name, mut s: *const libc::c_char) {
        (&raw mut (*this).len).write(0);
        'loop_: while ((*s.offset(((*this).len) as isize)) != 0) {
            (*this).len.prefix_inc();
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut widened: (i64, u32) = ((-1_i32 as i64), 2_u32);
    assert!(((widened.0) == (-1_i64)));
    assert!(((widened.1) == (2_u32)));
    let mut from_ints: (Celsius, Celsius) = (
        {
            let mut __v: Celsius = ::std::mem::MaybeUninit::uninit().assume_init();
            Celsius::new(&raw mut __v, { 1 });
            __v
        },
        {
            let mut __v: Celsius = ::std::mem::MaybeUninit::uninit().assume_init();
            Celsius::new(&raw mut __v, { 2 });
            __v
        },
    );
    assert!(((from_ints.0.degrees) == (1)));
    assert!(((from_ints.1.degrees) == (2)));
    let mut m: Meters = Meters { value: 7 };
    let mut from_operators: (i32, i64) = (
        (unsafe { Meters::to_i32(&m) }),
        ((unsafe { Meters::to_i32(&m) }) as i64),
    );
    assert!(((from_operators.0) == (7)));
    assert!(((from_operators.1) == (7_i64)));
    let mut from_moved_pair: (i64, f64) = match ((&mut (3, 4) as *mut (i32, i32)),) {
        (__tmp_0,) => (((*&mut (*__tmp_0).0) as i64), ((*&mut (*__tmp_0).1) as f64)),
    };
    assert!(((from_moved_pair.0) == (3_i64)));
    assert!(((from_moved_pair.1) == (4_f64)));
    let mut ints: (i32, i32) = (5, 6);
    let mut from_copied_pair: (Celsius, i64) = (
        <Celsius as CppFromUnsafe<i32>>::cpp_from(&ints.0),
        ((*&ints.1) as i64),
    );
    assert!(((from_copied_pair.0.degrees) == (5)));
    assert!(((from_copied_pair.1) == (6_i64)));
    let mut meters: (Meters, Meters) = ((Meters { value: 8 }), (Meters { value: 9 }));
    let mut from_operator_pair: (i32, i32) = (
        <i32 as CppFromUnsafe<Meters>>::cpp_from(&meters.0),
        <i32 as CppFromUnsafe<Meters>>::cpp_from(&meters.1),
    );
    assert!(((from_operator_pair.0) == (8)));
    assert!(((from_operator_pair.1) == (9)));
    let mut celsius: (Celsius, Celsius) = (
        {
            let mut __v: Celsius = ::std::mem::MaybeUninit::uninit().assume_init();
            Celsius::new(&raw mut __v, { 10 });
            __v
        },
        {
            let mut __v: Celsius = ::std::mem::MaybeUninit::uninit().assume_init();
            Celsius::new(&raw mut __v, { 11 });
            __v
        },
    );
    let mut from_record_pair: (Label, Owner) = (
        <Label as CppFromUnsafe<Celsius>>::cpp_from(&celsius.0),
        <Owner as CppFromUnsafe<Celsius>>::cpp_from(&celsius.1),
    );
    assert!(((from_record_pair.0.id) == (10)));
    assert!(((from_record_pair.1.copied) == (11)));
    assert!(((from_record_pair.1.taken) == (0)));
    let mut from_moved_record_pair: (Label, Owner) = match ((&mut (
        {
            let mut __v: Celsius = ::std::mem::MaybeUninit::uninit().assume_init();
            Celsius::new(&raw mut __v, { 12 });
            __v
        },
        {
            let mut __v: Celsius = ::std::mem::MaybeUninit::uninit().assume_init();
            Celsius::new(&raw mut __v, { 13 });
            __v
        },
    ) as *mut (Celsius, Celsius)),)
    {
        (__tmp_1,) => (
            <Label as CppFromUnsafe<Celsius>>::cpp_from_move(&mut (*__tmp_1).0),
            <Owner as CppFromUnsafe<Celsius>>::cpp_from_move(&mut (*__tmp_1).1),
        ),
    };
    assert!(((from_moved_record_pair.0.id) == (12)));
    assert!(((from_moved_record_pair.1.taken) == (13)));
    assert!(((from_moved_record_pair.1.copied) == (0)));
    let mut from_by_value_pair: (Holder, Holder) = (
        <Holder as CppFromUnsafe<Celsius>>::cpp_from(&celsius.0),
        <Holder as CppFromUnsafe<Celsius>>::cpp_from(&celsius.1),
    );
    assert!(((from_by_value_pair.0.value) == (10)));
    assert!(((from_by_value_pair.1.value) == (11)));
    let mut from_moved_by_value_pair: (Holder, Holder) = match ((&mut (
        {
            let mut __v: Celsius = ::std::mem::MaybeUninit::uninit().assume_init();
            Celsius::new(&raw mut __v, { 14 });
            __v
        },
        {
            let mut __v: Celsius = ::std::mem::MaybeUninit::uninit().assume_init();
            Celsius::new(&raw mut __v, { 15 });
            __v
        },
    ) as *mut (Celsius, Celsius)),)
    {
        (__tmp_2,) => (
            <Holder as CppFromUnsafe<Celsius>>::cpp_from_move(&mut (*__tmp_2).0),
            <Holder as CppFromUnsafe<Celsius>>::cpp_from_move(&mut (*__tmp_2).1),
        ),
    };
    assert!(((from_moved_by_value_pair.0.value) == (14)));
    assert!(((from_moved_by_value_pair.1.value) == (15)));
    let mut defaulted: Fallback = ::std::mem::MaybeUninit::uninit().assume_init();
    Fallback::new(&raw mut defaulted, { -1_i32 });
    assert!(((defaulted.value) == (-1_i32)));
    let mut from_defaulted_ctor: (Fallback, i64) = (
        <Fallback as CppFromUnsafe<i32>>::cpp_from(&ints.0),
        ((*&ints.1) as i64),
    );
    assert!(((from_defaulted_ctor.0.value) == (5)));
    assert!(((from_defaulted_ctor.1) == (6_i64)));
    let mut counter: Counter = Counter { hits: 3 };
    let mut forwarded_mutable_ref: (Observer, i32) = (
        {
            let mut __v: Observer = ::std::mem::MaybeUninit::uninit().assume_init();
            Observer::new(&raw mut __v, { &mut counter });
            __v
        },
        0,
    );
    assert!(((forwarded_mutable_ref.0.seen) == (3)));
    assert!(((counter.hits) == (4)));
    let mut from_template_ctor: (Tag, Tag) = (
        <Tag as CppFromUnsafe<Celsius>>::cpp_from(&celsius.0),
        <Tag as CppFromUnsafe<Celsius>>::cpp_from(&celsius.1),
    );
    assert!(((from_template_ctor.0.code) == (110)));
    assert!(((from_template_ctor.1.code) == (111)));
    let mut lvalue_source: Celsius = ::std::mem::MaybeUninit::uninit().assume_init();
    Celsius::new(&raw mut lvalue_source, { 16 });
    let mut moved_source: Celsius = ::std::mem::MaybeUninit::uninit().assume_init();
    Celsius::new(&raw mut moved_source, { 17 });
    let mut forwarded_records: (Label, Owner) = (
        {
            let mut __v: Label = ::std::mem::MaybeUninit::uninit().assume_init();
            Label::new(&raw mut __v, { &lvalue_source });
            __v
        },
        {
            let mut __v: Owner = ::std::mem::MaybeUninit::uninit().assume_init();
            Owner::new_2(&raw mut __v, { &mut moved_source });
            __v
        },
    );
    assert!(((forwarded_records.0.id) == (16)));
    assert!(((forwarded_records.1.taken) == (17)));
    assert!(((forwarded_records.1.copied) == (0)));
    let mut from_string_literals: (Name, Name) = (
        {
            let mut __v: Name = ::std::mem::MaybeUninit::uninit().assume_init();
            Name::new(&raw mut __v, { c"ab".as_ptr() });
            __v
        },
        {
            let mut __v: Name = ::std::mem::MaybeUninit::uninit().assume_init();
            Name::new(&raw mut __v, { c"cde".as_ptr() });
            __v
        },
    );
    assert!(((from_string_literals.0.len) == (2)));
    assert!(((from_string_literals.1.len) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
