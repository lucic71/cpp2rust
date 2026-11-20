extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct X {
    pub v: i32,
}
impl X {
    pub unsafe fn new_1(this: *mut X) {}
    pub unsafe fn copy_from(this: *mut X, _a0: *const X) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut X, _a0: *mut X) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
impl std::cmp::Ord for X {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if ((operator_lt_0(
                (&mut self.clone()) as *mut X,
                (&mut other.clone()) as *mut X,
            )) != 0)
            {
                std::cmp::Ordering::Less
            } else if ((operator_lt_0(
                (&mut other.clone()) as *mut X,
                (&mut self.clone()) as *mut X,
            )) != 0)
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for X {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for X {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ((operator_eq_1(
                (&mut self.clone()) as *mut X,
                (&mut other.clone()) as *mut X,
            )) != 0)
        }
    }
}
impl std::cmp::Eq for X {}
pub unsafe fn operator_eq_1(a: *mut X, b: *mut X) -> i32 {
    return if (((*a).v) == ((*b).v)) { 2 } else { 0 };
}
pub unsafe fn operator_ne_2(a: *mut X, b: *mut X) -> i32 {
    return if (((*a).v) != ((*b).v)) { 3 } else { 0 };
}
pub unsafe fn operator_lt_0(a: *mut X, b: *mut X) -> i32 {
    return if (((*a).v) < ((*b).v)) { 4 } else { 0 };
}
pub unsafe fn operator_gt_3(a: *mut X, b: *mut X) -> i32 {
    return if (((*a).v) > ((*b).v)) { 5 } else { 0 };
}
pub unsafe fn operator_le_4(a: *mut X, b: *mut X) -> i32 {
    return if (((*a).v) <= ((*b).v)) { 6 } else { 0 };
}
pub unsafe fn operator_ge_5(a: *mut X, b: *mut X) -> i32 {
    return if (((*a).v) >= ((*b).v)) { 7 } else { 0 };
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Result {
    pub r: i32,
}
impl Result {
    pub unsafe fn new_1(this: *mut Result) {}
    pub unsafe fn copy_from(this: *mut Result, _a0: *const Result) {
        (&raw mut (*this).r).write((*_a0).r);
    }
    pub unsafe fn move_from(this: *mut Result, _a0: *mut Result) {
        (&raw mut (*this).r).write((*_a0).r);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Custom {
    pub v: i32,
}
impl Custom {
    pub unsafe fn operator_cmp(&self, __ret: *mut Result, o: *const Custom) {
        {
            let __d = __ret.clone();
            (&raw mut (*__d).r).write(((self.v) - ((*o).v)));
        };
        return;
    }
    pub unsafe fn new_1(this: *mut Custom) {}
    pub unsafe fn copy_from(this: *mut Custom, _a0: *const Custom) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut Custom, _a0: *mut Custom) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Mixed {
    pub v: i32,
}
impl Mixed {
    pub unsafe fn operator_cmp(&self, __ret: *mut Result, o: *const Mixed) {
        {
            let __d = __ret.clone();
            (&raw mut (*__d).r).write(((self.v) - ((*o).v)));
        };
        return;
    }
    pub unsafe fn operator_lt(&self, o: *const Mixed) -> i32 {
        return if ((self.v) < ((*o).v)) { 8 } else { 0 };
    }
    pub unsafe fn operator_eq(&self, o: *const Mixed) -> i32 {
        return if ((self.v) == ((*o).v)) { 9 } else { 0 };
    }
    pub unsafe fn new_1(this: *mut Mixed) {}
    pub unsafe fn copy_from(this: *mut Mixed, _a0: *const Mixed) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut Mixed, _a0: *mut Mixed) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
impl std::cmp::Ord for Mixed {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if ((Mixed::operator_lt(self, other as *const Mixed)) != 0) {
                std::cmp::Ordering::Less
            } else if ((Mixed::operator_lt(other, self as *const Mixed)) != 0) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Mixed {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Mixed {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ((Mixed::operator_eq(self, other as *const Mixed)) != 0) }
    }
}
impl std::cmp::Eq for Mixed {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(bool, to_bool)]
pub struct MyBool {
    pub value: bool,
}
impl MyBool {
    pub unsafe fn new(this: *mut MyBool, mut v: bool) {
        (&raw mut (*this).value).write(v);
    }
    pub unsafe fn to_bool(&self) -> bool {
        return self.value;
    }
    pub unsafe fn copy_from(this: *mut MyBool, _a0: *const MyBool) {
        (&raw mut (*this).value).write((*_a0).value);
    }
    pub unsafe fn move_from(this: *mut MyBool, _a0: *mut MyBool) {
        (&raw mut (*this).value).write((*_a0).value);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Boolish {
    pub v: i32,
}
impl Boolish {
    pub unsafe fn new_1(this: *mut Boolish) {}
    pub unsafe fn copy_from(this: *mut Boolish, _a0: *const Boolish) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut Boolish, _a0: *mut Boolish) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
impl std::cmp::Ord for Boolish {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if {
                let mut __cmp: MyBool = ::std::mem::MaybeUninit::uninit().assume_init();
                operator_lt_6(
                    &raw mut __cmp,
                    (&mut self.clone()) as *mut Boolish,
                    (&mut other.clone()) as *mut Boolish,
                );
                MyBool::to_bool(&__cmp)
            } {
                std::cmp::Ordering::Less
            } else if {
                let mut __cmp: MyBool = ::std::mem::MaybeUninit::uninit().assume_init();
                operator_lt_6(
                    &raw mut __cmp,
                    (&mut other.clone()) as *mut Boolish,
                    (&mut self.clone()) as *mut Boolish,
                );
                MyBool::to_bool(&__cmp)
            } {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Boolish {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Boolish {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            {
                let mut __cmp: MyBool = ::std::mem::MaybeUninit::uninit().assume_init();
                operator_eq_7(
                    &raw mut __cmp,
                    (&mut self.clone()) as *mut Boolish,
                    (&mut other.clone()) as *mut Boolish,
                );
                MyBool::to_bool(&__cmp)
            }
        }
    }
}
impl std::cmp::Eq for Boolish {}
pub unsafe fn operator_eq_7(__ret: *mut MyBool, a: *mut Boolish, b: *mut Boolish) {
    MyBool::new(__ret.clone(), { (((*a).v) == ((*b).v)) });
    return;
}
pub unsafe fn operator_lt_6(__ret: *mut MyBool, a: *mut Boolish, b: *mut Boolish) {
    MyBool::new(__ret.clone(), { (((*a).v) < ((*b).v)) });
    return;
}
#[repr(C)]
#[derive(FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(bool, to_bool)]
pub struct RefBool {
    pub value: bool,
}
impl RefBool {
    pub unsafe fn new(this: *mut RefBool, mut v: bool) {
        (&raw mut (*this).value).write(v);
    }
    pub unsafe fn to_bool(&self) -> bool {
        return self.value;
    }
}
pub static mut ref_true_8: libcc2rs::LazyStorage<RefBool> =
    libcc2rs::LazyStorage::new(|__p| unsafe { RefBool::new(__p, { true }) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RefInt {
    pub v: i32,
}
impl RefInt {
    pub unsafe fn operator_eq(&self, _a0: *const RefInt) -> *mut RefBool {
        return &mut (*(*&raw const ref_true_8).get());
    }
    pub unsafe fn new_1(this: *mut RefInt) {}
    pub unsafe fn copy_from(this: *mut RefInt, _a0: *const RefInt) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut RefInt, _a0: *mut RefInt) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
impl std::cmp::PartialEq for RefInt {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            {
                let __cmp = RefInt::operator_eq(self, other as *const RefInt);
                RefBool::to_bool(&*__cmp)
            }
        }
    }
}
impl std::cmp::Eq for RefInt {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: X = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut a;
        (&raw mut (*__d).v).write(1);
    };
    let mut b: X = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut b;
        (&raw mut (*__d).v).write(2);
    };
    let mut c: X = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut c;
        (&raw mut (*__d).v).write(1);
    };
    assert!(
        ((unsafe {
            let mut __tmp_0: X = ::std::mem::MaybeUninit::uninit().assume_init();
            X::copy_from(&raw mut __tmp_0, { &a });
            let _a = &raw mut __tmp_0;
            operator_eq_1(
                _a,
                (|__p: *mut X| {
                    X::copy_from(__p, { &c });
                    __p
                })((&mut ::std::mem::MaybeUninit::<X>::uninit()).as_mut_ptr()),
            )
        }) == (2))
    );
    assert!(
        ((unsafe {
            let mut __tmp_1: X = ::std::mem::MaybeUninit::uninit().assume_init();
            X::copy_from(&raw mut __tmp_1, { &a });
            let _a = &raw mut __tmp_1;
            operator_eq_1(
                _a,
                (|__p: *mut X| {
                    X::copy_from(__p, { &b });
                    __p
                })((&mut ::std::mem::MaybeUninit::<X>::uninit()).as_mut_ptr()),
            )
        }) == (0))
    );
    assert!(
        ((unsafe {
            let mut __tmp_2: X = ::std::mem::MaybeUninit::uninit().assume_init();
            X::copy_from(&raw mut __tmp_2, { &a });
            let _a = &raw mut __tmp_2;
            operator_ne_2(
                _a,
                (|__p: *mut X| {
                    X::copy_from(__p, { &b });
                    __p
                })((&mut ::std::mem::MaybeUninit::<X>::uninit()).as_mut_ptr()),
            )
        }) == (3))
    );
    assert!(
        ((unsafe {
            let mut __tmp_3: X = ::std::mem::MaybeUninit::uninit().assume_init();
            X::copy_from(&raw mut __tmp_3, { &a });
            let _a = &raw mut __tmp_3;
            operator_lt_0(
                _a,
                (|__p: *mut X| {
                    X::copy_from(__p, { &b });
                    __p
                })((&mut ::std::mem::MaybeUninit::<X>::uninit()).as_mut_ptr()),
            )
        }) == (4))
    );
    assert!(
        ((unsafe {
            let mut __tmp_4: X = ::std::mem::MaybeUninit::uninit().assume_init();
            X::copy_from(&raw mut __tmp_4, { &b });
            let _a = &raw mut __tmp_4;
            operator_gt_3(
                _a,
                (|__p: *mut X| {
                    X::copy_from(__p, { &a });
                    __p
                })((&mut ::std::mem::MaybeUninit::<X>::uninit()).as_mut_ptr()),
            )
        }) == (5))
    );
    assert!(
        ((unsafe {
            let mut __tmp_5: X = ::std::mem::MaybeUninit::uninit().assume_init();
            X::copy_from(&raw mut __tmp_5, { &a });
            let _a = &raw mut __tmp_5;
            operator_le_4(
                _a,
                (|__p: *mut X| {
                    X::copy_from(__p, { &c });
                    __p
                })((&mut ::std::mem::MaybeUninit::<X>::uninit()).as_mut_ptr()),
            )
        }) == (6))
    );
    assert!(
        ((unsafe {
            let mut __tmp_6: X = ::std::mem::MaybeUninit::uninit().assume_init();
            X::copy_from(&raw mut __tmp_6, { &a });
            let _a = &raw mut __tmp_6;
            operator_ge_5(
                _a,
                (|__p: *mut X| {
                    X::copy_from(__p, { &c });
                    __p
                })((&mut ::std::mem::MaybeUninit::<X>::uninit()).as_mut_ptr()),
            )
        }) == (7))
    );
    let mut xs: [X; 3] = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = (&raw mut xs).cast::<X>();
        {
            let __d = __d.offset(0);
            (&raw mut (*__d).v).write(3);
        };
        {
            let __d = __d.offset(1);
            (&raw mut (*__d).v).write(1);
        };
        {
            let __d = __d.offset(2);
            (&raw mut (*__d).v).write(2);
        };
    };
    {
        let mut __tmp_7 = { xs.as_mut_ptr() };
        let __n = xs.as_mut_ptr().offset((3) as isize).iter_distance(&__tmp_7);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| (*__tmp_7.iter_at(__x)).cmp(&*__tmp_7.iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__tmp_7.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__tmp_7.iter_at(__i), __v);
        }
    };
    assert!(
        (((xs[(0) as usize].v) == (1)) && ((xs[(1) as usize].v) == (2)))
            && ((xs[(2) as usize].v) == (3))
    );
    let mut p: Custom = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut p;
        (&raw mut (*__d).v).write(5);
    };
    let mut q: Custom = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut q;
        (&raw mut (*__d).v).write(2);
    };
    let mut r: Result = ::std::mem::MaybeUninit::uninit().assume_init();
    (unsafe { Custom::operator_cmp(&p, &raw mut r, &q) });
    assert!(((r.r) == (3)));
    let mut m: Mixed = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut m;
        (&raw mut (*__d).v).write(4);
    };
    let mut n: Mixed = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut n;
        (&raw mut (*__d).v).write(4);
    };
    assert!(((unsafe { Mixed::operator_eq(&m, &n,) }) == (9)));
    assert!(
        ((({
            let mut __v: Result = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { Mixed::operator_cmp(&m, &raw mut __v, &n) });
            __v
        })
        .r) == (0))
    );
    let mut ms: [Mixed; 3] = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = (&raw mut ms).cast::<Mixed>();
        {
            let __d = __d.offset(0);
            (&raw mut (*__d).v).write(6);
        };
        {
            let __d = __d.offset(1);
            (&raw mut (*__d).v).write(4);
        };
        {
            let __d = __d.offset(2);
            (&raw mut (*__d).v).write(5);
        };
    };
    {
        let mut __tmp_8 = { ms.as_mut_ptr() };
        let __n = ms.as_mut_ptr().offset((3) as isize).iter_distance(&__tmp_8);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| (*__tmp_8.iter_at(__x)).cmp(&*__tmp_8.iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__tmp_8.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__tmp_8.iter_at(__i), __v);
        }
    };
    assert!(
        (((ms[(0) as usize].v) == (4)) && ((ms[(1) as usize].v) == (5)))
            && ((ms[(2) as usize].v) == (6))
    );
    assert!(
        ((unsafe {
            let _o: *const Mixed = &ms[(1) as usize];
            Mixed::operator_lt(&ms[(0) as usize], _o)
        }) == (8))
    );
    let mut b1: Boolish = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut b1;
        (&raw mut (*__d).v).write(1);
    };
    let mut b2: Boolish = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut b2;
        (&raw mut (*__d).v).write(2);
    };
    assert!(
        (unsafe {
            MyBool::to_bool(&mut *(|__p: *mut MyBool| {
                (unsafe {
                    let mut __tmp_9: Boolish = ::std::mem::MaybeUninit::uninit().assume_init();
                    Boolish::copy_from(&raw mut __tmp_9, { &b1 });
                    let _a = &raw mut __tmp_9;
                    operator_eq_7(
                        __p,
                        _a,
                        (|__p: *mut Boolish| {
                            {
                                let __d = __p;
                                (&raw mut (*__d).v).write(1);
                            };
                            __p
                        })(
                            (&mut ::std::mem::MaybeUninit::<Boolish>::uninit()).as_mut_ptr()
                        ),
                    )
                });
                __p
            })(
                (&mut ::std::mem::MaybeUninit::<MyBool>::uninit()).as_mut_ptr(),
            ))
        })
    );
    assert!(
        !(unsafe {
            MyBool::to_bool(&mut *(|__p: *mut MyBool| {
                (unsafe {
                    let mut __tmp_10: Boolish = ::std::mem::MaybeUninit::uninit().assume_init();
                    Boolish::copy_from(&raw mut __tmp_10, { &b2 });
                    let _a = &raw mut __tmp_10;
                    operator_lt_6(
                        __p,
                        _a,
                        (|__p: *mut Boolish| {
                            Boolish::copy_from(__p, { &b1 });
                            __p
                        })(
                            (&mut ::std::mem::MaybeUninit::<Boolish>::uninit()).as_mut_ptr()
                        ),
                    )
                });
                __p
            })(
                (&mut ::std::mem::MaybeUninit::<MyBool>::uninit()).as_mut_ptr(),
            ))
        })
    );
    let mut lt: MyBool = ::std::mem::MaybeUninit::uninit().assume_init();
    (unsafe {
        let mut __tmp_11: Boolish = ::std::mem::MaybeUninit::uninit().assume_init();
        Boolish::copy_from(&raw mut __tmp_11, { &b1 });
        let _a = &raw mut __tmp_11;
        operator_lt_6(
            &raw mut lt,
            _a,
            (|__p: *mut Boolish| {
                Boolish::copy_from(__p, { &b2 });
                __p
            })((&mut ::std::mem::MaybeUninit::<Boolish>::uninit()).as_mut_ptr()),
        )
    });
    assert!(lt.value);
    let mut bs: [Boolish; 3] = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = (&raw mut bs).cast::<Boolish>();
        {
            let __d = __d.offset(0);
            (&raw mut (*__d).v).write(3);
        };
        {
            let __d = __d.offset(1);
            (&raw mut (*__d).v).write(1);
        };
        {
            let __d = __d.offset(2);
            (&raw mut (*__d).v).write(2);
        };
    };
    {
        let mut __tmp_12 = { bs.as_mut_ptr() };
        let __n = bs
            .as_mut_ptr()
            .offset((3) as isize)
            .iter_distance(&__tmp_12);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| (*__tmp_12.iter_at(__x)).cmp(&*__tmp_12.iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__tmp_12.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__tmp_12.iter_at(__i), __v);
        }
    };
    assert!(
        (((bs[(0) as usize].v) == (1)) && ((bs[(1) as usize].v) == (2)))
            && ((bs[(2) as usize].v) == (3))
    );
    let mut ri1: RefInt = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut ri1;
        (&raw mut (*__d).v).write(1);
    };
    let mut ri2: RefInt = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut ri2;
        (&raw mut (*__d).v).write(2);
    };
    assert!((unsafe { RefBool::to_bool(&(*(unsafe { RefInt::operator_eq(&ri1, &ri2,) })),) }));
    let mut ris: [RefInt; 2] = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = (&raw mut ris).cast::<RefInt>();
        {
            let __d = __d.offset(0);
            (&raw mut (*__d).v).write(1);
        };
        {
            let __d = __d.offset(1);
            (&raw mut (*__d).v).write(2);
        };
    };
    assert!(
        (({
            let mut __tmp_13 = { ris.as_mut_ptr().offset((2) as isize) };
            let mut __it = ris.as_mut_ptr();
            while __it != __tmp_13 && *__it.iter_deref() != ri2 {
                __it.iter_inc();
            }
            __it
        }) == (ris.as_mut_ptr()))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const ref_true_8).get();
}
