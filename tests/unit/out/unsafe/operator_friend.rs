extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(_a0: *const Defaulted, _a1: *const Defaulted) -> bool {
    return (((*_a0).a) == ((*_a1).a)) && (((*_a0).b) == ((*_a1).b));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Defaulted {
    pub a: i32,
    pub b: i32,
}
impl Defaulted {
    pub unsafe fn copy_from(this: *mut Defaulted, _a0: *const Defaulted) {
        (&raw mut (*this).a).write((*_a0).a);
        (&raw mut (*this).b).write((*_a0).b);
    }
}
impl std::cmp::PartialEq for Defaulted {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const Defaulted, other as *const Defaulted) }
    }
}
impl std::cmp::Eq for Defaulted {}
pub unsafe fn operator_cmp_1(
    _a0: *const DefaultedOrd,
    _a1: *const DefaultedOrd,
) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).a), &((*_a1).a));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
pub unsafe fn operator_eq_2(_a0: *const DefaultedOrd, _a1: *const DefaultedOrd) -> bool {
    return (((*_a0).a) == ((*_a1).a));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DefaultedOrd {
    pub a: i32,
}
impl DefaultedOrd {
    pub unsafe fn copy_from(this: *mut DefaultedOrd, _a0: *const DefaultedOrd) {
        (&raw mut (*this).a).write((*_a0).a);
    }
}
impl std::cmp::Ord for DefaultedOrd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_1(self as *const DefaultedOrd, other as *const DefaultedOrd) }
    }
}
impl std::cmp::PartialOrd for DefaultedOrd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for DefaultedOrd {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_2(self as *const DefaultedOrd, other as *const DefaultedOrd) }
    }
}
impl std::cmp::Eq for DefaultedOrd {}
pub unsafe fn operator_eq_3(x: *const Inline, y: *const Inline) -> bool {
    return (((*x).a) == ((*y).a));
}
pub unsafe fn operator_lt_4(x: *const Inline, y: *const Inline) -> bool {
    return (((*x).a) < ((*y).a));
}
pub unsafe fn operator_add_5(__ret: *mut Inline, x: *const Inline, y: *const Inline) {
    {
        let __d = __ret.clone();
        (&raw mut (*__d).a).write((((*x).a) + ((*y).a)));
    };
    return;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Inline {
    pub a: i32,
}
impl Inline {
    pub unsafe fn copy_from(this: *mut Inline, _a0: *const Inline) {
        (&raw mut (*this).a).write((*_a0).a);
    }
}
impl std::cmp::Ord for Inline {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_4(self as *const Inline, other as *const Inline) {
                std::cmp::Ordering::Less
            } else if operator_lt_4(other as *const Inline, self as *const Inline) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Inline {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Inline {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_3(self as *const Inline, other as *const Inline) }
    }
}
impl std::cmp::Eq for Inline {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OutOfLine {
    pub a: i32,
}
impl OutOfLine {
    pub unsafe fn copy_from(this: *mut OutOfLine, _a0: *const OutOfLine) {
        (&raw mut (*this).a).write((*_a0).a);
    }
}
impl std::cmp::PartialEq for OutOfLine {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_6(self as *const OutOfLine, other as *const OutOfLine) }
    }
}
impl std::cmp::Eq for OutOfLine {}
pub unsafe fn operator_eq_6(x: *const OutOfLine, y: *const OutOfLine) -> bool {
    return (((*x).a) == ((*y).a));
}
pub unsafe fn operator_ne_7(x: *const OutOfLine, y: *const OutOfLine) -> bool {
    return !(unsafe {
        let _x: *const OutOfLine = x;
        let _y: *const OutOfLine = y;
        operator_eq_6(_x, _y)
    });
}
pub unsafe fn operator_eq_8(x: *const Tmpl_int_, y: *const Tmpl_int_) -> bool {
    return (((*x).v) == ((*y).v));
}
pub unsafe fn operator_lt_9(x: *const Tmpl_int_, y: *const Tmpl_int_) -> bool {
    return (((*x).v) < ((*y).v));
}
pub unsafe fn operator_eq_10(x: *const Tmpl_int_, y: *const Tmpl_long_) -> bool {
    return (((*x).v as i64) == ((*y).v));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Tmpl_int_ {
    pub v: i32,
}
impl Tmpl_int_ {
    pub unsafe fn copy_from(this: *mut Tmpl_int_, _a0: *const Tmpl_int_) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
impl std::cmp::Ord for Tmpl_int_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_9(self as *const Tmpl_int_, other as *const Tmpl_int_) {
                std::cmp::Ordering::Less
            } else if operator_lt_9(other as *const Tmpl_int_, self as *const Tmpl_int_) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Tmpl_int_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Tmpl_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_8(self as *const Tmpl_int_, other as *const Tmpl_int_) }
    }
}
impl std::cmp::Eq for Tmpl_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Tmpl_long_ {
    pub v: i64,
}
impl Tmpl_long_ {
    pub unsafe fn copy_from(this: *mut Tmpl_long_, _a0: *const Tmpl_long_) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
pub unsafe fn operator_eq_11(
    _a0: *const TmplDefaulted_int_,
    _a1: *const TmplDefaulted_int_,
) -> bool {
    return (((*_a0).v) == ((*_a1).v));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TmplDefaulted_int_ {
    pub v: i32,
}
impl TmplDefaulted_int_ {
    pub unsafe fn copy_from(this: *mut TmplDefaulted_int_, _a0: *const TmplDefaulted_int_) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
impl std::cmp::PartialEq for TmplDefaulted_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_11(
                self as *const TmplDefaulted_int_,
                other as *const TmplDefaulted_int_,
            )
        }
    }
}
impl std::cmp::Eq for TmplDefaulted_int_ {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d1: Defaulted = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut d1;
        (&raw mut (*__d).a).write(1);
        (&raw mut (*__d).b).write(2);
    };
    let mut d2: Defaulted = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut d2;
        (&raw mut (*__d).a).write(1);
        (&raw mut (*__d).b).write(2);
    };
    let mut d3: Defaulted = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut d3;
        (&raw mut (*__d).a).write(1);
        (&raw mut (*__d).b).write(3);
    };
    assert!(
        (unsafe {
            let _arg0: *const Defaulted = &d1;
            operator_eq_0(_arg0, &d2)
        })
    );
    assert!(
        !(unsafe {
            let _arg0: *const Defaulted = &d1;
            operator_eq_0(_arg0, &d3)
        })
    );
    assert!(
        !(unsafe {
            let _arg0: *const Defaulted = &d1;
            operator_eq_0(_arg0, &d3)
        })
    );
    let mut o1: DefaultedOrd = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut o1;
        (&raw mut (*__d).a).write(1);
    };
    let mut o2: DefaultedOrd = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut o2;
        (&raw mut (*__d).a).write(2);
    };
    assert!(
        (unsafe {
            let _arg0: *const DefaultedOrd = &o1;
            operator_cmp_1(_arg0, &o2)
        }) == std::cmp::Ordering::Less
    );
    assert!(
        (unsafe {
            let _arg0: *const DefaultedOrd = &o2;
            operator_cmp_1(_arg0, &o1)
        }) == std::cmp::Ordering::Greater
    );
    assert!(
        (unsafe {
            let _arg0: *const DefaultedOrd = &o1;
            let _arg1: *const DefaultedOrd = &o1;
            operator_eq_2(_arg0, _arg1)
        })
    );
    assert!(
        (unsafe {
            let _arg0: *const DefaultedOrd = &o1;
            operator_cmp_1(_arg0, &o2)
        }) == std::cmp::Ordering::Less
    );
    let mut i1: Inline = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut i1;
        (&raw mut (*__d).a).write(1);
    };
    let mut i2: Inline = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut i2;
        (&raw mut (*__d).a).write(2);
    };
    let mut i3: Inline = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut i3;
        (&raw mut (*__d).a).write(1);
    };
    assert!(
        (unsafe {
            let _x: *const Inline = &i1;
            operator_eq_3(_x, &i3)
        })
    );
    assert!(
        !(unsafe {
            let _x: *const Inline = &i1;
            operator_eq_3(_x, &i2)
        })
    );
    assert!(
        (unsafe {
            let _x: *const Inline = &i1;
            operator_lt_4(_x, &i2)
        })
    );
    assert!(
        !(unsafe {
            let _x: *const Inline = &i2;
            operator_lt_4(_x, &i1)
        })
    );
    assert!(
        (unsafe {
            let mut _x: Inline = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe {
                let _x: *const Inline = &i1;
                operator_add_5(&raw mut _x, _x, &i2)
            });
            let mut _y: Inline = ::std::mem::MaybeUninit::uninit().assume_init();
            {
                let __d = &raw mut _y;
                (&raw mut (*__d).a).write(3);
            };
            operator_eq_3(&mut _x, &mut _y)
        })
    );
    let mut f1: OutOfLine = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut f1;
        (&raw mut (*__d).a).write(4);
    };
    let mut f2: OutOfLine = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut f2;
        (&raw mut (*__d).a).write(4);
    };
    let mut f3: OutOfLine = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut f3;
        (&raw mut (*__d).a).write(5);
    };
    assert!(
        (unsafe {
            let _x: *const OutOfLine = &f1;
            operator_eq_6(_x, &f2)
        })
    );
    assert!(
        (unsafe {
            let _x: *const OutOfLine = &f1;
            operator_ne_7(_x, &f3)
        })
    );
    assert!(
        !(unsafe {
            let _x: *const OutOfLine = &f1;
            operator_eq_6(_x, &f3)
        })
    );
    let mut t1: Tmpl_int_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut t1;
        (&raw mut (*__d).v).write(1);
    };
    let mut t2: Tmpl_int_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut t2;
        (&raw mut (*__d).v).write(2);
    };
    let mut t3: Tmpl_int_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut t3;
        (&raw mut (*__d).v).write(1);
    };
    assert!(
        (unsafe {
            let _x: *const Tmpl_int_ = &t1;
            operator_eq_8(_x, &t3)
        })
    );
    assert!(
        !(unsafe {
            let _x: *const Tmpl_int_ = &t1;
            operator_eq_8(_x, &t2)
        })
    );
    assert!(
        (unsafe {
            let _x: *const Tmpl_int_ = &t1;
            operator_lt_9(_x, &t2)
        })
    );
    let mut u1: Tmpl_long_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut u1;
        (&raw mut (*__d).v).write(1_i64);
    };
    let mut u2: Tmpl_long_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut u2;
        (&raw mut (*__d).v).write(2_i64);
    };
    assert!(
        (unsafe {
            let _x: *const Tmpl_int_ = &t1;
            operator_eq_10(_x, &u1)
        })
    );
    assert!(
        !(unsafe {
            let _x: *const Tmpl_int_ = &t1;
            operator_eq_10(_x, &u2)
        })
    );
    let mut v1: TmplDefaulted_int_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut v1;
        (&raw mut (*__d).v).write(7);
    };
    let mut v2: TmplDefaulted_int_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut v2;
        (&raw mut (*__d).v).write(7);
    };
    let mut v3: TmplDefaulted_int_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut v3;
        (&raw mut (*__d).v).write(8);
    };
    assert!(
        (unsafe {
            let _arg0: *const TmplDefaulted_int_ = &v1;
            operator_eq_11(_arg0, &v2)
        })
    );
    assert!(
        !(unsafe {
            let _arg0: *const TmplDefaulted_int_ = &v1;
            operator_eq_11(_arg0, &v3)
        })
    );
    assert!(
        !(unsafe {
            let _arg0: *const TmplDefaulted_int_ = &v1;
            operator_eq_11(_arg0, &v3)
        })
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
