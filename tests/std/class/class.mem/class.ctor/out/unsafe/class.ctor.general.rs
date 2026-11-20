extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct Tracker {
    pub self_: *mut Tracker,
    pub value: i32,
}
impl Tracker {
    pub unsafe fn new_2(this: *mut Tracker) {
        (&raw mut (*this).self_).write(this);
        (&raw mut (*this).value).write(-1_i32);
    }
    pub unsafe fn new_1(this: *mut Tracker, mut value: i32) {
        (&raw mut (*this).self_).write(this);
        (&raw mut (*this).value).write(value);
    }
    pub unsafe fn new_3(this: *mut Tracker, mut a: i32, mut b: i32) {
        Tracker::new_1(this.clone(), { ((a) + (b)) });
    }
    pub unsafe fn copy_from(this: *mut Tracker, other: *const Tracker) {
        (&raw mut (*this).self_).write(this);
        (&raw mut (*this).value).write((*other).value);
    }
    pub unsafe fn move_from(this: *mut Tracker, other: *mut Tracker) {
        (&raw mut (*this).self_).write(this);
        (&raw mut (*this).value).write((*other).value);
        (*other).value = -2_i32;
    }
    pub unsafe fn is_self(&self) -> bool {
        return (((self.self_).cast_const()) == (self as *const Tracker));
    }
}
impl Clone for Tracker {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: Tracker = ::std::mem::MaybeUninit::uninit().assume_init();
            Tracker::copy_from(&raw mut __v, self as *const Tracker);
            __v
        }
    }
}
impl Default for Tracker {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: Tracker = ::std::mem::MaybeUninit::uninit().assume_init();
                Tracker::new_2(&raw mut __v);
                __v
            }
        }
    }
}
pub unsafe fn test_0() {
    let mut a: Tracker = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracker::new_1(&raw mut a, { 1 });
    assert!(((a.self_) == (&mut a as *mut Tracker)) && ((a.value) == (1)));
}
pub unsafe fn test_1() {
    let mut a: Tracker = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracker::new_2(&raw mut a);
    assert!(((a.self_) == (&mut a as *mut Tracker)) && ((a.value) == (-1_i32)));
    let mut elements: [Tracker; 2] = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = (&raw mut elements).cast::<Tracker>();
        for __i in 0..2_isize {
            Tracker::new_2(__d.offset(__i));
        }
    };
    assert!(
        ((elements[(1) as usize].self_) == (&mut elements[(1) as usize] as *mut Tracker))
            && ((elements[(1) as usize].value) == (-1_i32))
    );
}
pub unsafe fn test_2() {
    let mut a: Tracker = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracker::new_3(&raw mut a, { 1 }, { 2 });
    assert!(((a.self_) == (&mut a as *mut Tracker)) && ((a.value) == (3)));
}
pub unsafe fn test_3() {
    let mut a: Tracker = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracker::new_1(&raw mut a, { 1 });
    let mut b: Tracker = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracker::copy_from(&raw mut b, { &a });
    assert!(
        ((((b.self_) == (&mut b as *mut Tracker)) && ((b.value) == (1)))
            && ((a.self_) == (&mut a as *mut Tracker)))
            && ((a.value) == (1))
    );
}
pub unsafe fn test_4() {
    let mut a: Tracker = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracker::new_1(&raw mut a, { 1 });
    let mut b: Tracker = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracker::move_from(&raw mut b, { &mut a });
    assert!(
        ((((b.self_) == (&mut b as *mut Tracker)) && ((b.value) == (1)))
            && ((a.self_) == (&mut a as *mut Tracker)))
            && ((a.value) == (-2_i32))
    );
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct implicit_constructors_Pair {
    pub first: Tracker,
    pub second: Tracker,
}
impl implicit_constructors_Pair {
    pub unsafe fn copy_from(
        this: *mut implicit_constructors_Pair,
        _a0: *const implicit_constructors_Pair,
    ) {
        Tracker::copy_from((&raw mut (*this).first), { &(*_a0).first });
        Tracker::copy_from((&raw mut (*this).second), { &(*_a0).second });
    }
    pub unsafe fn move_from(
        this: *mut implicit_constructors_Pair,
        _a0: *mut implicit_constructors_Pair,
    ) {
        Tracker::move_from((&raw mut (*this).first), { &mut (*_a0).first });
        Tracker::move_from((&raw mut (*this).second), { &mut (*_a0).second });
    }
    pub unsafe fn new_1(this: *mut implicit_constructors_Pair) {
        Tracker::new_2((&raw mut (*this).first));
        Tracker::new_2((&raw mut (*this).second));
    }
}
pub unsafe fn test_5() {
    let mut a: implicit_constructors_Pair = ::std::mem::MaybeUninit::uninit().assume_init();
    implicit_constructors_Pair::new_1(&raw mut a);
    assert!(
        ((a.first.self_) == (&mut a.first as *mut Tracker))
            && ((a.second.self_) == (&mut a.second as *mut Tracker))
    );
    a.second.value = 5;
    let mut b: implicit_constructors_Pair = ::std::mem::MaybeUninit::uninit().assume_init();
    implicit_constructors_Pair::copy_from(&raw mut b, { &a });
    assert!(
        ((b.first.self_) == (&mut b.first as *mut Tracker))
            && ((b.second.self_) == (&mut b.second as *mut Tracker))
    );
    assert!(((b.second.value) == (5)));
    let mut c: implicit_constructors_Pair = ::std::mem::MaybeUninit::uninit().assume_init();
    implicit_constructors_Pair::move_from(&raw mut c, { &mut a });
    assert!(((c.second.self_) == (&mut c.second as *mut Tracker)) && ((c.second.value) == (5)));
    assert!(((a.second.value) == (-2_i32)));
}
pub unsafe fn test_6() {
    let mut p: *mut Tracker = {
        let __p = Box::into_raw(Box::<Tracker>::new_uninit()) as *mut Tracker;
        Tracker::new_1(__p, { 4 });
        __p
    };
    assert!((((*p).self_) == (p)) && (((*p).value) == (4)));
    {
        let __p = p;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct member_Holder {
    pub before: i32,
    pub tracker: Tracker,
}
impl member_Holder {
    pub unsafe fn new(this: *mut member_Holder, mut value: i32) {
        (&raw mut (*this).before).write(value);
        Tracker::new_1((&raw mut (*this).tracker), { (((*this).before) + (1)) });
    }
    pub unsafe fn copy_from(this: *mut member_Holder, _a0: *const member_Holder) {
        (&raw mut (*this).before).write((*_a0).before);
        Tracker::copy_from((&raw mut (*this).tracker), { &(*_a0).tracker });
    }
    pub unsafe fn move_from(this: *mut member_Holder, _a0: *mut member_Holder) {
        (&raw mut (*this).before).write((*_a0).before);
        Tracker::move_from((&raw mut (*this).tracker), { &mut (*_a0).tracker });
    }
}
pub unsafe fn test_7() {
    let mut h: member_Holder = ::std::mem::MaybeUninit::uninit().assume_init();
    member_Holder::new(&raw mut h, { 5 });
    assert!(((h.tracker.self_) == (&mut h.tracker as *mut Tracker)) && ((h.tracker.value) == (6)));
}
pub unsafe fn test_8() {
    let mut elements: [Tracker; 2] = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = (&raw mut elements).cast::<Tracker>();
        Tracker::new_1(__d.offset(0), { 6 });
        Tracker::new_1(__d.offset(1), { 7 });
    };
    assert!(
        ((elements[(0) as usize].self_) == (&mut elements[(0) as usize] as *mut Tracker))
            && ((elements[(0) as usize].value) == (6))
    );
    assert!(
        ((elements[(1) as usize].self_) == (&mut elements[(1) as usize] as *mut Tracker))
            && ((elements[(1) as usize].value) == (7))
    );
}
pub unsafe fn is_self_9(tracker: *mut Tracker) -> bool {
    return (((*tracker).self_) == (&mut (*tracker) as *mut Tracker));
}
pub unsafe fn test_10() {
    assert!(
        (unsafe {
            is_self_9((|__p: *mut Tracker| {
                Tracker::new_1(__p, { 8 });
                __p
            })(
                (&mut ::std::mem::MaybeUninit::<Tracker>::uninit()).as_mut_ptr(),
            ))
        })
    );
}
pub unsafe fn make_11(__ret: *mut Tracker, mut value: i32) {
    Tracker::new_1(__ret.clone(), { value });
    return;
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct returned_object_Aggregate {
    pub tracker: Tracker,
}
impl returned_object_Aggregate {
    pub unsafe fn copy_from(
        this: *mut returned_object_Aggregate,
        _a0: *const returned_object_Aggregate,
    ) {
        Tracker::copy_from((&raw mut (*this).tracker), { &(*_a0).tracker });
    }
    pub unsafe fn move_from(
        this: *mut returned_object_Aggregate,
        _a0: *mut returned_object_Aggregate,
    ) {
        Tracker::move_from((&raw mut (*this).tracker), { &mut (*_a0).tracker });
    }
    pub unsafe fn new_1(this: *mut returned_object_Aggregate) {
        Tracker::new_2((&raw mut (*this).tracker));
    }
}
pub unsafe fn test_12() {
    let mut r: Tracker = ::std::mem::MaybeUninit::uninit().assume_init();
    (unsafe { make_11(&raw mut r, 9) });
    assert!(((r.self_) == (&mut r as *mut Tracker)) && ((r.value) == (9)));
    let mut aggregate: returned_object_Aggregate = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut aggregate;
        (unsafe { make_11((&raw mut (*__d).tracker), 10) });
    };
    assert!(((aggregate.tracker.self_) == (&mut aggregate.tracker as *mut Tracker)));
}
pub unsafe fn test_13() {
    assert!(
        (unsafe {
            Tracker::is_self(&mut *(|__p: *mut Tracker| {
                Tracker::new_1(__p, { 11 });
                __p
            })(
                (&mut ::std::mem::MaybeUninit::<Tracker>::uninit()).as_mut_ptr(),
            ))
        })
    );
    let mut __tmp_0: Tracker = ::std::mem::MaybeUninit::uninit().assume_init();
    let __tmp_1 = &raw mut __tmp_0;
    Tracker::new_1(__tmp_1, { 12 });
    let bound: *const Tracker = &mut __tmp_0;
    assert!(((((*bound).self_).cast_const()) == (bound)) && (((*bound).value) == (12)));
}
pub unsafe fn test_14() {
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (2)) {
            let mut chosen: Tracker = ::std::mem::MaybeUninit::uninit().assume_init();
            {
                let __d = &raw mut chosen;
                if ((i) == (0)) {
                    Tracker::new_1(__d.clone(), { 13 })
                } else {
                    Tracker::new_1(__d.clone(), { 14 })
                }
            };
            assert!(
                ((chosen.self_) == (&mut chosen as *mut Tracker))
                    && ((chosen.value) == ((13) + (i)))
            );
            i.prefix_inc();
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, DestructorUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct lifetime_Logged {
    pub id: i32,
}
impl lifetime_Logged {
    pub unsafe fn new(this: *mut lifetime_Logged, mut id: i32) {
        (&raw mut (*this).id).write(id);
        (unsafe { libc::printf(c"ctor %d\n".as_ptr() as *const libc::c_char, (id)) });
    }
    pub unsafe fn copy_from(this: *mut lifetime_Logged, other: *const lifetime_Logged) {
        (&raw mut (*this).id).write((((*other).id) + (100)));
        (unsafe { libc::printf(c"copy %d\n".as_ptr() as *const libc::c_char, ((*this).id)) });
    }
    pub unsafe fn destructor(&mut self) {
        (unsafe { libc::printf(c"dtor %d\n".as_ptr() as *const libc::c_char, (self.id)) });
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const lifetime_Logged) -> *mut lifetime_Logged {
        self.id = (*_a0).id;
        return &mut (*(self as *mut lifetime_Logged));
    }
}
impl Clone for lifetime_Logged {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: lifetime_Logged = ::std::mem::MaybeUninit::uninit().assume_init();
            lifetime_Logged::copy_from(&raw mut __v, self as *const lifetime_Logged);
            __v
        }
    }
}
pub unsafe fn make_15(__ret: *mut lifetime_Logged, mut id: i32) {
    lifetime_Logged::new(__ret.clone(), { id });
    return;
}
pub unsafe fn test_16() {
    let mut local: lifetime_Logged = ::std::mem::MaybeUninit::uninit().assume_init();
    lifetime_Logged::new(&raw mut local, { 1 });
    let _dtor_local =
        std::mem::DropGuard::new(&raw mut local, |__p| unsafe { (*__p).destructor() });
    let mut copy: lifetime_Logged = ::std::mem::MaybeUninit::uninit().assume_init();
    lifetime_Logged::copy_from(&raw mut copy, { &local });
    let _dtor_copy = std::mem::DropGuard::new(&raw mut copy, |__p| unsafe { (*__p).destructor() });
    let mut returned: lifetime_Logged = ::std::mem::MaybeUninit::uninit().assume_init();
    (unsafe { make_15(&raw mut returned, 3) });
    let _dtor_returned =
        std::mem::DropGuard::new(&raw mut returned, |__p| unsafe { (*__p).destructor() });
    (unsafe { libc::printf(c"end\n".as_ptr() as *const libc::c_char) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_16() });
    (unsafe { test_0() });
    (unsafe { test_1() });
    (unsafe { test_2() });
    (unsafe { test_3() });
    (unsafe { test_4() });
    (unsafe { test_5() });
    (unsafe { test_6() });
    (unsafe { test_7() });
    (unsafe { test_8() });
    (unsafe { test_10() });
    (unsafe { test_12() });
    (unsafe { test_13() });
    (unsafe { test_14() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
