extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct MyContainer_int_ {
    vec_: Vec<i32>,
}
impl MyContainer_int_ {
    pub unsafe fn empty(&self) -> bool {
        return self.vec_.is_empty();
    }
    pub unsafe fn size(&self) -> usize {
        return self.vec_.len();
    }
    pub unsafe fn back_4(&mut self) -> *mut i32 {
        return {
            let __n = self.vec_.len() - 1;
            &raw mut self.vec_[__n]
        };
    }
    pub unsafe fn pop_back(&mut self) {
        self.vec_.pop();
        return;
    }
    pub unsafe fn push_back(&mut self, item: *const i32) {
        {
            let a0_clone = (*item).clone();
            self.vec_.push(a0_clone)
        };
    }
    pub unsafe fn copy_from(this: *mut MyContainer_int_, _a0: *const MyContainer_int_) {
        (&raw mut (*this).vec_).write((*_a0).vec_.clone());
    }
    pub unsafe fn move_from(this: *mut MyContainer_int_, _a0: *mut MyContainer_int_) {
        (&raw mut (*this).vec_).write(std::mem::take(&mut (*_a0).vec_));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut MyContainer_int_) -> *mut MyContainer_int_ {
        {
            self.vec_ = std::mem::take(&mut (*_a0).vec_);
        };
        return &mut (*(self as *mut MyContainer_int_));
    }
    pub unsafe fn new_1(this: *mut MyContainer_int_) {
        (&raw mut (*this).vec_).write(Vec::new());
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct MyContainer_char_ {
    vec_: Vec<libc::c_char>,
}
impl MyContainer_char_ {
    pub unsafe fn empty(&self) -> bool {
        return self.vec_.is_empty();
    }
    pub unsafe fn size(&self) -> usize {
        return self.vec_.len();
    }
    pub unsafe fn back_4(&mut self) -> *mut libc::c_char {
        return {
            let __n = self.vec_.len() - 1;
            &raw mut self.vec_[__n]
        };
    }
    pub unsafe fn pop_back(&mut self) {
        self.vec_.pop();
        return;
    }
    pub unsafe fn push_back(&mut self, item: *const libc::c_char) {
        {
            let a0_clone = (*item).clone();
            self.vec_.push(a0_clone)
        };
    }
    pub unsafe fn copy_from(this: *mut MyContainer_char_, _a0: *const MyContainer_char_) {
        (&raw mut (*this).vec_).write((*_a0).vec_.clone());
    }
    pub unsafe fn move_from(this: *mut MyContainer_char_, _a0: *mut MyContainer_char_) {
        (&raw mut (*this).vec_).write(std::mem::take(&mut (*_a0).vec_));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut MyContainer_char_) -> *mut MyContainer_char_ {
        {
            self.vec_ = std::mem::take(&mut (*_a0).vec_);
        };
        return &mut (*(self as *mut MyContainer_char_));
    }
    pub unsafe fn new_1(this: *mut MyContainer_char_) {
        (&raw mut (*this).vec_).write(Vec::new());
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct MyContainer_float_ {
    vec_: Vec<f32>,
}
impl MyContainer_float_ {
    pub unsafe fn empty(&self) -> bool {
        return self.vec_.is_empty();
    }
    pub unsafe fn size(&self) -> usize {
        return self.vec_.len();
    }
    pub unsafe fn back_4(&mut self) -> *mut f32 {
        return {
            let __n = self.vec_.len() - 1;
            &raw mut self.vec_[__n]
        };
    }
    pub unsafe fn pop_back(&mut self) {
        self.vec_.pop();
        return;
    }
    pub unsafe fn push_back(&mut self, item: *const f32) {
        {
            let a0_clone = (*item).clone();
            self.vec_.push(a0_clone)
        };
    }
    pub unsafe fn copy_from(this: *mut MyContainer_float_, _a0: *const MyContainer_float_) {
        (&raw mut (*this).vec_).write((*_a0).vec_.clone());
    }
    pub unsafe fn move_from(this: *mut MyContainer_float_, _a0: *mut MyContainer_float_) {
        (&raw mut (*this).vec_).write(std::mem::take(&mut (*_a0).vec_));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut MyContainer_float_) -> *mut MyContainer_float_ {
        {
            self.vec_ = std::mem::take(&mut (*_a0).vec_);
        };
        return &mut (*(self as *mut MyContainer_float_));
    }
    pub unsafe fn new_1(this: *mut MyContainer_float_) {
        (&raw mut (*this).vec_).write(Vec::new());
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Boxed_short_ {
    pub value: i16,
}
impl Boxed_short_ {
    pub unsafe fn twice(mut v: i16) -> i16 {
        return (((v as i32) + (v as i32)) as i16);
    }
    pub unsafe fn plus(&self, mut other: i16) -> i16 {
        return (((self.value as i32) + (other as i32)) as i16);
    }
    pub unsafe fn new_1(this: *mut Boxed_short_) {}
    pub unsafe fn copy_from(this: *mut Boxed_short_, _a0: *const Boxed_short_) {
        (&raw mut (*this).value).write((*_a0).value);
    }
    pub unsafe fn move_from(this: *mut Boxed_short_, _a0: *mut Boxed_short_) {
        (&raw mut (*this).value).write((*_a0).value);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Boxed_int_ {
    pub value: i32,
}
impl Boxed_int_ {
    pub unsafe fn twice(mut v: i32) -> i32 {
        return ((v) + (v));
    }
    pub unsafe fn plus(&self, mut other: i32) -> i32 {
        return ((self.value) + (other));
    }
    pub unsafe fn new_1(this: *mut Boxed_int_) {}
    pub unsafe fn copy_from(this: *mut Boxed_int_, _a0: *const Boxed_int_) {
        (&raw mut (*this).value).write((*_a0).value);
    }
    pub unsafe fn move_from(this: *mut Boxed_int_, _a0: *mut Boxed_int_) {
        (&raw mut (*this).value).write((*_a0).value);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Boxed_long_ {
    pub value: i64,
}
impl Boxed_long_ {
    pub unsafe fn twice(mut v: i64) -> i64 {
        return ((v) + (v));
    }
    pub unsafe fn plus(&self, mut other: i64) -> i64 {
        return ((self.value) + (other));
    }
    pub unsafe fn new_1(this: *mut Boxed_long_) {}
    pub unsafe fn copy_from(this: *mut Boxed_long_, _a0: *const Boxed_long_) {
        (&raw mut (*this).value).write((*_a0).value);
    }
    pub unsafe fn move_from(this: *mut Boxed_long_, _a0: *mut Boxed_long_) {
        (&raw mut (*this).value).write((*_a0).value);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer_int__Inner_int_ {
    pub t: i32,
    pub u: i32,
}
impl Outer_int__Inner_int_ {
    pub unsafe fn sum(&self) -> i32 {
        return ((self.t as i32) + (self.u as i32));
    }
    pub unsafe fn new_1(this: *mut Outer_int__Inner_int_) {}
    pub unsafe fn copy_from(this: *mut Outer_int__Inner_int_, _a0: *const Outer_int__Inner_int_) {
        (&raw mut (*this).t).write((*_a0).t);
        (&raw mut (*this).u).write((*_a0).u);
    }
    pub unsafe fn move_from(this: *mut Outer_int__Inner_int_, _a0: *mut Outer_int__Inner_int_) {
        (&raw mut (*this).t).write((*_a0).t);
        (&raw mut (*this).u).write((*_a0).u);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer_int_ {
    pub v: i32,
}
impl Outer_int_ {
    pub unsafe fn with(&self, __ret: *mut Outer_int__Inner_int_, mut n: i32) {
        {
            let __d = __ret.clone();
            (&raw mut (*__d).t).write(self.v);
            (&raw mut (*__d).u).write(n);
        };
        return;
    }
    pub unsafe fn new_1(this: *mut Outer_int_) {}
    pub unsafe fn copy_from(this: *mut Outer_int_, _a0: *const Outer_int_) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut Outer_int_, _a0: *mut Outer_int_) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer_long__Inner_int_ {
    pub t: i64,
    pub u: i32,
}
impl Outer_long__Inner_int_ {
    pub unsafe fn sum(&self) -> i32 {
        return ((self.t as i32) + (self.u as i32));
    }
    pub unsafe fn new_1(this: *mut Outer_long__Inner_int_) {}
    pub unsafe fn copy_from(this: *mut Outer_long__Inner_int_, _a0: *const Outer_long__Inner_int_) {
        (&raw mut (*this).t).write((*_a0).t);
        (&raw mut (*this).u).write((*_a0).u);
    }
    pub unsafe fn move_from(this: *mut Outer_long__Inner_int_, _a0: *mut Outer_long__Inner_int_) {
        (&raw mut (*this).t).write((*_a0).t);
        (&raw mut (*this).u).write((*_a0).u);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer_long__Inner_char_ {
    pub t: i64,
    pub u: libc::c_char,
}
impl Outer_long__Inner_char_ {
    pub unsafe fn sum(&self) -> i32 {
        return ((self.t as i32) + (self.u as i32));
    }
    pub unsafe fn new_1(this: *mut Outer_long__Inner_char_) {}
    pub unsafe fn copy_from(
        this: *mut Outer_long__Inner_char_,
        _a0: *const Outer_long__Inner_char_,
    ) {
        (&raw mut (*this).t).write((*_a0).t);
        (&raw mut (*this).u).write((*_a0).u);
    }
    pub unsafe fn move_from(this: *mut Outer_long__Inner_char_, _a0: *mut Outer_long__Inner_char_) {
        (&raw mut (*this).t).write((*_a0).t);
        (&raw mut (*this).u).write((*_a0).u);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer_long_ {
    pub v: i64,
}
impl Outer_long_ {
    pub unsafe fn with(&self, __ret: *mut Outer_long__Inner_int_, mut n: i32) {
        {
            let __d = __ret.clone();
            (&raw mut (*__d).t).write(self.v);
            (&raw mut (*__d).u).write(n);
        };
        return;
    }
    pub unsafe fn new_1(this: *mut Outer_long_) {}
    pub unsafe fn copy_from(this: *mut Outer_long_, _a0: *const Outer_long_) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut Outer_long_, _a0: *mut Outer_long_) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Elem {
    pub i: i32,
}
impl Elem {
    pub unsafe fn new_1(this: *mut Elem) {}
    pub unsafe fn copy_from(this: *mut Elem, _a0: *const Elem) {
        (&raw mut (*this).i).write((*_a0).i);
    }
    pub unsafe fn move_from(this: *mut Elem, _a0: *mut Elem) {
        (&raw mut (*this).i).write((*_a0).i);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut ev: Vec<Elem> = Vec::new();
    {
        let __a1 = (Elem { i: 2 });
        ev.push(__a1)
    };
    assert!(((ev[(0_usize)].i) == (2)));
    assert!((((unsafe { Boxed_short_::twice(2_i16,) }) as i32) == (4)));
    let mut oi: Outer_int_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut oi;
        (&raw mut (*__d).v).write(3);
    };
    assert!(
        ((unsafe {
            Outer_int__Inner_int_::sum(&mut *(|__p: *mut Outer_int__Inner_int_| {
                (unsafe { Outer_int_::with(&oi, __p, 4) });
                __p
            })(
                (&mut ::std::mem::MaybeUninit::<Outer_int__Inner_int_>::uninit()).as_mut_ptr(),
            ))
        }) == (7))
    );
    let mut ol: Outer_long_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut ol;
        (&raw mut (*__d).v).write(5_i64);
    };
    let mut ic: Outer_long__Inner_char_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut ic;
        (&raw mut (*__d).t).write(6_i64);
        (&raw mut (*__d).u).write(('a' as libc::c_char));
    };
    assert!(
        ((unsafe {
            Outer_long__Inner_int_::sum(&mut *(|__p: *mut Outer_long__Inner_int_| {
                (unsafe { Outer_long_::with(&ol, __p, 2) });
                __p
            })(
                (&mut ::std::mem::MaybeUninit::<Outer_long__Inner_int_>::uninit()).as_mut_ptr(),
            ))
        }) == (7))
    );
    assert!(
        ((unsafe { Outer_long__Inner_char_::sum(&ic,) }) == ((6) + (('a' as libc::c_char) as i32)))
    );
    assert!(((unsafe { Boxed_int_::twice(3,) }) == (6)));
    let mut bi: Boxed_int_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut bi;
        (&raw mut (*__d).value).write(4);
    };
    assert!(((unsafe { Boxed_int_::plus(&bi, 5,) }) == (9)));
    assert!(((unsafe { Boxed_long_::twice(10_i64,) }) == (20_i64)));
    let mut bl: Boxed_long_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut bl;
        (&raw mut (*__d).value).write(7_i64);
    };
    assert!(((unsafe { Boxed_long_::plus(&bl, 1_i64,) }) == (8_i64)));
    let mut imc: MyContainer_int_ = ::std::mem::MaybeUninit::uninit().assume_init();
    MyContainer_int_::new_1(&raw mut imc);
    assert!((unsafe { MyContainer_int_::empty(&imc,) }));
    (unsafe {
        let mut _item: i32 = 1;
        MyContainer_int_::push_back(&mut imc, &mut _item)
    });
    assert!(
        ((unsafe { MyContainer_int_::size(&imc,) }) == (1_usize))
            && ((*(unsafe { MyContainer_int_::back_4(&mut imc,) })) == (1))
    );
    (unsafe { MyContainer_int_::pop_back(&mut imc) });
    assert!((unsafe { MyContainer_int_::empty(&imc,) }));
    let mut cmc: MyContainer_char_ = ::std::mem::MaybeUninit::uninit().assume_init();
    MyContainer_char_::new_1(&raw mut cmc);
    assert!((unsafe { MyContainer_char_::empty(&cmc,) }));
    (unsafe {
        let mut _item: libc::c_char = ('a' as libc::c_char);
        MyContainer_char_::push_back(&mut cmc, &mut _item)
    });
    assert!(
        ((unsafe { MyContainer_char_::size(&cmc,) }) == (1_usize))
            && (((*(unsafe { MyContainer_char_::back_4(&mut cmc,) })) as i32)
                == (('a' as libc::c_char) as i32))
    );
    (unsafe { MyContainer_char_::pop_back(&mut cmc) });
    assert!((unsafe { MyContainer_char_::empty(&cmc,) }));
    let mut fmc: MyContainer_float_ = ::std::mem::MaybeUninit::uninit().assume_init();
    MyContainer_float_::new_1(&raw mut fmc);
    assert!((unsafe { MyContainer_float_::empty(&fmc,) }));
    (unsafe {
        let mut _item: f32 = (1_f64 as f32);
        MyContainer_float_::push_back(&mut fmc, &mut _item)
    });
    assert!(
        ((unsafe { MyContainer_float_::size(&fmc,) }) == (1_usize))
            && (((*(unsafe { MyContainer_float_::back_4(&mut fmc,) })) as f64) == (1_f64))
    );
    (unsafe { MyContainer_float_::pop_back(&mut fmc) });
    assert!((unsafe { MyContainer_float_::empty(&fmc,) }));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
