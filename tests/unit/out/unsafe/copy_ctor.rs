extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct Counted {
    pub v: i32,
}
impl Counted {
    pub unsafe fn new(this: *mut Counted, mut v: i32) {
        (&raw mut (*this).v).write(v);
    }
    pub unsafe fn copy_from(this: *mut Counted, o: *const Counted) {
        (&raw mut (*this).v).write((*o).v);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Counted) -> *mut Counted {
        self.v = (*_a0).v;
        return &mut (*(self as *mut Counted));
    }
}
impl Clone for Counted {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: Counted = ::std::mem::MaybeUninit::uninit().assume_init();
            Counted::copy_from(&raw mut __v, self as *const Counted);
            __v
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct NonConst {
    pub mark: i32,
}
impl NonConst {
    pub unsafe fn new(this: *mut NonConst) {
        (&raw mut (*this).mark).write(0);
    }
    pub unsafe fn new_1(this: *mut NonConst, o: *mut NonConst) {
        (&raw mut (*this).mark).write((((*o).mark) + (1)));
    }
    pub unsafe fn new_2(this: *mut NonConst, o: *const NonConst) {
        (&raw mut (*this).mark).write((((*o).mark) + (10)));
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const NonConst) -> *mut NonConst {
        self.mark = (*_a0).mark;
        return &mut (*(self as *mut NonConst));
    }
}
impl Clone for NonConst {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: NonConst = ::std::mem::MaybeUninit::uninit().assume_init();
            NonConst::new_1(&raw mut __v, self as *const NonConst as *mut NonConst);
            __v
        }
    }
}
impl Default for NonConst {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: NonConst = ::std::mem::MaybeUninit::uninit().assume_init();
                NonConst::new(&raw mut __v);
                __v
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct Ignored {
    pub v: i32,
}
impl Ignored {
    pub unsafe fn new(this: *mut Ignored, mut v: i32) {
        (&raw mut (*this).v).write(v);
    }
    pub unsafe fn copy_from(this: *mut Ignored, _a0: *const Ignored) {
        (&raw mut (*this).v).write(-1_i32);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Ignored) -> *mut Ignored {
        self.v = (*_a0).v;
        return &mut (*(self as *mut Ignored));
    }
}
impl Clone for Ignored {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: Ignored = ::std::mem::MaybeUninit::uninit().assume_init();
            Ignored::copy_from(&raw mut __v, self as *const Ignored);
            __v
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct Holder {
    pub c: Counted,
    pub arr: [Counted; 2],
}
impl Holder {
    pub unsafe fn copy_from(this: *mut Holder, _a0: *const Holder) {
        Counted::copy_from((&raw mut (*this).c), { &(*_a0).c });
        {
            let __d = (&raw mut (*this).arr).cast::<Counted>();
            for __i in 0..2_usize {
                Counted::copy_from(__d.offset(__i as isize), { &(*_a0).arr[(__i)] });
            }
        };
    }
    pub unsafe fn move_from(this: *mut Holder, _a0: *mut Holder) {
        Counted::copy_from((&raw mut (*this).c), { &(*_a0).c });
        {
            let __d = (&raw mut (*this).arr).cast::<Counted>();
            for __i in 0..2_usize {
                Counted::copy_from(__d.offset(__i as isize), { &(*_a0).arr[(__i)] });
            }
        };
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Holder) -> *mut Holder {
        (unsafe {
            let _arg0: *const Counted = &(*_a0).c;
            Counted::copy_assign(&mut self.c, _arg0)
        });
        {
            let mut __tmp_0 = { ((&mut self.arr as *mut [Counted; 2]) as *mut ::libc::c_void) };
            let mut __tmp_1 = { ((&(*_a0).arr as *const [Counted; 2]) as *const ::libc::c_void) };
            if 8_usize != 0 {
                ::std::ptr::copy_nonoverlapping(__tmp_1, __tmp_0, 8_usize)
            }
            __tmp_0
        };
        return &mut (*(self as *mut Holder));
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct Box_int_ {
    pub val: i32,
}
impl Box_int_ {
    pub unsafe fn new(this: *mut Box_int_, mut v: i32) {
        (&raw mut (*this).val).write(v);
    }
    pub unsafe fn copy_from(this: *mut Box_int_, o: *const Box_int_) {
        (&raw mut (*this).val).write((*o).val);
    }
    pub unsafe fn move_from(this: *mut Box_int_, _a0: *mut Box_int_) {
        (&raw mut (*this).val).write((*_a0).val);
    }
}
impl Clone for Box_int_ {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: Box_int_ = ::std::mem::MaybeUninit::uninit().assume_init();
            Box_int_::copy_from(&raw mut __v, self as *const Box_int_);
            __v
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct Owner {
    pub box_: Box_int_,
}
impl Owner {
    pub unsafe fn copy_from(this: *mut Owner, _a0: *const Owner) {
        Box_int_::copy_from((&raw mut (*this).box_), { &(*_a0).box_ });
    }
    pub unsafe fn move_from(this: *mut Owner, _a0: *mut Owner) {
        Box_int_::move_from((&raw mut (*this).box_), { &mut (*_a0).box_ });
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct OutOfLine {
    pub v: i32,
}
impl OutOfLine {
    pub unsafe fn new(this: *mut OutOfLine, mut v: i32) {
        (&raw mut (*this).v).write(v);
    }
    pub unsafe fn copy_from(this: *mut OutOfLine, o: *const OutOfLine) {
        (&raw mut (*this).v).write((((*o).v) + (100)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const OutOfLine) -> *mut OutOfLine {
        self.v = (*_a0).v;
        return &mut (*(self as *mut OutOfLine));
    }
}
impl Clone for OutOfLine {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: OutOfLine = ::std::mem::MaybeUninit::uninit().assume_init();
            OutOfLine::copy_from(&raw mut __v, self as *const OutOfLine);
            __v
        }
    }
}
impl OutOfLine {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct Tracked {
    pub copied_from: i32,
}
impl Tracked {
    pub unsafe fn new(this: *mut Tracked) {
        (&raw mut (*this).copied_from).write(0);
    }
    pub unsafe fn copy_from(this: *mut Tracked, o: *const Tracked) {
        (&raw mut (*this).copied_from).write(0);
        (*(&raw const (*o).copied_from).cast_mut()).prefix_inc();
    }
    pub unsafe fn copy_assign(&mut self, o: *const Tracked) -> *mut Tracked {
        (*(&raw const (*o).copied_from).cast_mut()).prefix_inc();
        return &mut (*(self as *mut Tracked));
    }
}
impl Clone for Tracked {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
            Tracked::copy_from(&raw mut __v, self as *const Tracked);
            __v
        }
    }
}
impl Default for Tracked {
    fn default() -> Self {
        unsafe { Tracked { copied_from: 0 } }
    }
}
pub unsafe fn touch_1(mut t: *const Tracked) {
    (*(&raw const (*t).copied_from).cast_mut()).prefix_inc();
}
pub unsafe fn by_value_2(c: *mut Counted) -> i32 {
    return (*c).v;
}
pub unsafe fn make_3(__ret: *mut Counted, mut v: i32) {
    let mut c: Counted = ::std::mem::MaybeUninit::uninit().assume_init();
    Counted::new(&raw mut c, { v });
    Counted::copy_from(__ret.clone(), { &c });
    return;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Counted = ::std::mem::MaybeUninit::uninit().assume_init();
    Counted::new(&raw mut a, { 1 });
    let mut b: Counted = ::std::mem::MaybeUninit::uninit().assume_init();
    Counted::copy_from(&raw mut b, { &a });
    let mut c: Counted = ::std::mem::MaybeUninit::uninit().assume_init();
    Counted::copy_from(&raw mut c, { &a });
    let mut d: Counted = ::std::mem::MaybeUninit::uninit().assume_init();
    Counted::copy_from(&raw mut d, { &a });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (3)));
    assert!((((b.v) == (1)) && ((c.v) == (1))) && ((d.v) == (1)));
    assert!(
        ((unsafe {
            by_value_2((|__p: *mut Counted| {
                Counted::copy_from(__p, { &a });
                __p
            })(
                (&mut ::std::mem::MaybeUninit::<Counted>::uninit()).as_mut_ptr(),
            ))
        }) == (1))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (4)));
    let mut e: Counted = ::std::mem::MaybeUninit::uninit().assume_init();
    (unsafe { make_3(&raw mut e, 5) });
    assert!(((e.v) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (5)));
    let mut f: Counted = ::std::mem::MaybeUninit::uninit().assume_init();
    Counted::new(&raw mut f, { 6 });
    assert!(((f.v) == (6)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (5)));
    let mut g: Counted = ::std::mem::MaybeUninit::uninit().assume_init();
    Counted::new(&raw mut g, { 7 });
    let mut h: Counted = ::std::mem::MaybeUninit::uninit().assume_init();
    Counted::copy_from(&raw mut h, { &g });
    assert!(((h.v) == (7)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (6)));
    let mut hold: Holder = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut hold;
        Counted::new((&raw mut (*__d).c), { 8 });
        {
            let __d = (&raw mut (*__d).arr).cast::<Counted>();
            Counted::new(__d.offset(0), { 9 });
            Counted::new(__d.offset(1), { 10 });
        };
    };
    let mut hold2: Holder = ::std::mem::MaybeUninit::uninit().assume_init();
    Holder::copy_from(&raw mut hold2, { &hold });
    assert!(
        (((hold2.c.v) == (8)) && ((hold2.arr[(0) as usize].v) == (9)))
            && ((hold2.arr[(1) as usize].v) == (10))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (9)));
    let mut vec_: Vec<Counted> = Vec::new();
    {
        let a0_clone = a.clone();
        vec_.push(a0_clone)
    };
    assert!(((vec_[(0_usize)].v) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (10)));
    let mut i1: Ignored = ::std::mem::MaybeUninit::uninit().assume_init();
    Ignored::new(&raw mut i1, { 1 });
    let mut i2: Ignored = ::std::mem::MaybeUninit::uninit().assume_init();
    Ignored::copy_from(&raw mut i2, { &i1 });
    assert!(((i1.v) == (1)) && ((i2.v) == (-1_i32)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (11)));
    let mut o: Owner = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut o;
        Box_int_::new((&raw mut (*__d).box_), { 3 });
    };
    let mut o2: Owner = ::std::mem::MaybeUninit::uninit().assume_init();
    Owner::move_from(&raw mut o2, { &mut o });
    assert!(((o2.box_.val) == (3)));
    let mut ol1: OutOfLine = ::std::mem::MaybeUninit::uninit().assume_init();
    OutOfLine::new(&raw mut ol1, { 5 });
    let mut ol2: OutOfLine = ::std::mem::MaybeUninit::uninit().assume_init();
    OutOfLine::copy_from(&raw mut ol2, { &ol1 });
    assert!(((ol2.v) == (105)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (12)));
    let mut n: NonConst = ::std::mem::MaybeUninit::uninit().assume_init();
    NonConst::new(&raw mut n);
    let mut n1: NonConst = ::std::mem::MaybeUninit::uninit().assume_init();
    NonConst::new_1(&raw mut n1, { &mut n });
    let mut cn: NonConst = ::std::mem::MaybeUninit::uninit().assume_init();
    NonConst::new(&raw mut cn);
    let mut n2: NonConst = ::std::mem::MaybeUninit::uninit().assume_init();
    NonConst::new_2(&raw mut n2, { &cn });
    assert!(((n1.mark) == (1)));
    assert!(((n2.mark) == (10)));
    let mut t1: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracked::new(&raw mut t1);
    let mut t2: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracked::copy_from(&raw mut t2, { &t1 });
    (unsafe { Tracked::copy_assign(&mut t2, &t1) });
    assert!(((t1.copied_from) == (2)));
    let mut ct: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracked::new(&raw mut ct);
    let mut t3: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracked::copy_from(&raw mut t3, { &ct });
    (unsafe { touch_1((&ct as *const Tracked)) });
    assert!(((ct.copied_from) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
}
