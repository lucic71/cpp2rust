extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static assigns_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, CopyAssign, Default)]
#[placement]
#[byte_size(8)]
pub struct Partial {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    pub keep: i32,
}
impl Partial {
    pub fn new(this: Ptr<Partial>, mut v: i32, mut keep: i32) {
        field!(this, v).write(v);
        field!(this, keep).write(keep);
    }
    pub fn copy_from(this: Ptr<Partial>, o: Ptr<Partial>) {
        field!(this, v).write(o.with(|__s| __s.v));
        field!(this, keep).write(o.with(|__s| __s.keep));
    }
}
impl Clone for Partial {
    fn clone(&self) -> Self {
        let __src: Value<Partial> = Rc::new(RefCell::new(Partial {
            v: self.v.clone(),
            keep: self.keep.clone(),
        }));
        let __dst: Value<Partial> = Rc::new(RefCell::new(<Partial>::uninit()));
        Partial::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[placement]
#[byte_size(4)]
pub struct NonConstAssign {
    #[offset(0)]
    pub mark: i32,
}
impl NonConstAssign {
    pub fn new(this: Ptr<NonConstAssign>) {
        field!(this, mark).write(0);
    }
    pub fn copy_from(this: Ptr<NonConstAssign>, _a0: Ptr<NonConstAssign>) {
        field!(this, mark).write(_a0.with(|__s| __s.mark));
    }
}
impl Default for NonConstAssign {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<NonConstAssign> = Rc::new(RefCell::new(<NonConstAssign>::uninit()));
            NonConstAssign::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CopyAssign)]
#[placement]
#[byte_size(4)]
pub struct RefQualified {
    #[offset(0)]
    pub mark: i32,
}
impl RefQualified {
    pub fn new(this: Ptr<RefQualified>) {
        field!(this, mark).write(0);
    }
    pub fn copy_from(this: Ptr<RefQualified>, _a0: Ptr<RefQualified>) {
        field!(this, mark).write(_a0.with(|__s| __s.mark));
    }
}
impl Default for RefQualified {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<RefQualified> = Rc::new(RefCell::new(<RefQualified>::uninit()));
            RefQualified::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CopyAssign)]
#[placement]
#[byte_size(24)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Partial,
    #[offset(8)]
    #[byte_size(16)]
    #[array_len(2)]
    pub arr: Box<[Partial]>,
}
impl Holder {
    pub fn copy_from(this: Ptr<Holder>, _a0: Ptr<Holder>) {
        Partial::copy_from(field_ptr!(this, p), { field_ptr!(_a0, p) });
        {
            let __d = array_field_ptr!(this, arr);
            for __i in 0..2_usize {
                Partial::copy_from(__d.offset(__i as isize), {
                    array_field_ptr!(_a0, arr).offset(__i)
                });
            }
        };
    }
}
impl Default for Holder {
    fn default() -> Self {
        {
            Holder {
                p: <Partial>::default(),
                arr: (0..2)
                    .map(|_| <Partial>::default())
                    .collect::<Box<[Partial]>>(),
            }
        }
    }
}
thread_local!(
    pub static copies_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, CopyAssign, Default)]
#[placement]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct CopyCounted {
    #[offset(0)]
    pub v: i32,
}
impl CopyCounted {
    pub fn new(this: Ptr<CopyCounted>, mut v: i32) {
        field!(this, v).write(v);
    }
    pub fn copy_from(this: Ptr<CopyCounted>, o: Ptr<CopyCounted>) {
        field!(this, v).write(o.with(|__s| __s.v));
        (*copies_1.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
impl Clone for CopyCounted {
    fn clone(&self) -> Self {
        let __src: Value<CopyCounted> = Rc::new(RefCell::new(CopyCounted { v: self.v.clone() }));
        let __dst: Value<CopyCounted> = Rc::new(RefCell::new(<CopyCounted>::uninit()));
        CopyCounted::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CopyAssign)]
#[placement]
#[byte_size(12)]
pub struct HasCopyCounted {
    #[offset(0)]
    #[byte_size(4)]
    pub c: CopyCounted,
    #[offset(4)]
    #[byte_size(8)]
    #[array_len(2)]
    pub arr: Box<[CopyCounted]>,
}
impl HasCopyCounted {
    pub fn copy_from(this: Ptr<HasCopyCounted>, _a0: Ptr<HasCopyCounted>) {
        CopyCounted::copy_from(field_ptr!(this, c), { field_ptr!(_a0, c) });
        {
            let __d = array_field_ptr!(this, arr);
            for __i in 0..2_usize {
                CopyCounted::copy_from(__d.offset(__i as isize), {
                    array_field_ptr!(_a0, arr).offset(__i)
                });
            }
        };
    }
}
impl Default for HasCopyCounted {
    fn default() -> Self {
        {
            HasCopyCounted {
                c: <CopyCounted>::default(),
                arr: (0..2)
                    .map(|_| <CopyCounted>::default())
                    .collect::<Box<[CopyCounted]>>(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Partial> = Rc::new(RefCell::new(<Partial>::uninit()));
    Partial::new(a.as_pointer(), { 1 }, { 100 });
    let b: Value<Partial> = Rc::new(RefCell::new(<Partial>::uninit()));
    Partial::new(b.as_pointer(), { 2 }, { 200 });
    let c: Value<Partial> = Rc::new(RefCell::new(<Partial>::uninit()));
    Partial::new(c.as_pointer(), { 3 }, { 300 });
    ({ PartialImpl::copy_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ ({ (*a.borrow()).v } == 2) && ({ (*a.borrow()).keep } == 100) }));
    assert!((assigns_0.with(|rc| *rc.borrow()) == 1));
    ({
        PartialImpl::copy_assign(
            &c.as_pointer(),
            ({ PartialImpl::copy_assign(&a.as_pointer(), b.as_pointer()) }),
        )
    });
    assert!(({ ({ (*c.borrow()).v } == 2) && ({ (*c.borrow()).keep } == 300) }));
    assert!((assigns_0.with(|rc| *rc.borrow()) == 3));
    ({
        let _o: Ptr<Partial> = a.as_pointer();
        PartialImpl::copy_assign(&a.as_pointer(), _o)
    });
    assert!((assigns_0.with(|rc| *rc.borrow()) == 3));
    ({
        let _o: Value<Partial> = {
            let __v: Value<Partial> = Rc::new(RefCell::new(<Partial>::uninit()));
            Partial::new(__v.as_pointer(), { 9 }, { 900 });
            __v
        };
        PartialImpl::copy_assign(&a.as_pointer(), _o.as_pointer())
    });
    assert!(({ ({ (*a.borrow()).v } == 9) && ({ (*a.borrow()).keep } == 100) }));
    assert!((assigns_0.with(|rc| *rc.borrow()) == 4));
    let ra: Ptr<Partial> = a.as_pointer();
    ({
        let _o: Ptr<Partial> = c.as_pointer();
        PartialImpl::copy_assign(&ra, _o)
    });
    assert!(({ ({ (*a.borrow()).v } == 2) }));
    let mut pa: Ptr<Partial> = (a.as_pointer());
    ({
        let _o: Ptr<Partial> = b.as_pointer();
        PartialImpl::copy_assign(&pa, _o)
    });
    assert!(({ ({ (*a.borrow()).v } == 2) }));
    assert!((assigns_0.with(|rc| *rc.borrow()) == 6));
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::uninit()));
    {
        let __d = h.as_pointer();
        Partial::new(field_ptr!(__d, p), { 4 }, { 40 });
        {
            let __d = array_field_ptr!(__d, arr);
            Partial::new(__d.offset(0), { 5 }, { 50 });
            Partial::new(__d.offset(1), { 6 }, { 60 });
        };
    };
    ({ PartialImpl::copy_assign(&field_ptr!(h.as_pointer(), p), b.as_pointer()) });
    ({
        PartialImpl::copy_assign(
            &array_field_ptr!(h.as_pointer(), arr).offset((1) as isize),
            c.as_pointer(),
        )
    });
    assert!(({ ({ (*h.borrow()).p.v } == 2) && ({ (*h.borrow()).p.keep } == 40) }));
    assert!(
        (array_elem!(h.as_pointer(), arr, 1).with(|__s| __s.v) == 2)
            && (array_elem!(h.as_pointer(), arr, 1).with(|__s| __s.keep) == 60)
    );
    assert!((assigns_0.with(|rc| *rc.borrow()) == 8));
    let n: Value<NonConstAssign> = Rc::new(RefCell::new(<NonConstAssign>::uninit()));
    NonConstAssign::new(n.as_pointer());
    let n1: Value<NonConstAssign> = Rc::new(RefCell::new(<NonConstAssign>::uninit()));
    NonConstAssign::new(n1.as_pointer());
    let n2: Value<NonConstAssign> = Rc::new(RefCell::new(<NonConstAssign>::uninit()));
    NonConstAssign::new(n2.as_pointer());
    let cn: Value<NonConstAssign> = Rc::new(RefCell::new(<NonConstAssign>::uninit()));
    NonConstAssign::new(cn.as_pointer());
    ({ NonConstAssignImpl::operator_assign_2(&n1.as_pointer(), n.as_pointer()) });
    ({ NonConstAssignImpl::operator_assign_3(&n2.as_pointer(), cn.as_pointer()) });
    assert!(({ ({ (*n1.borrow()).mark } == 1) }));
    assert!(({ ({ (*n2.borrow()).mark } == 10) }));
    let r: Value<RefQualified> = Rc::new(RefCell::new(<RefQualified>::uninit()));
    RefQualified::new(r.as_pointer());
    let r1: Value<RefQualified> = Rc::new(RefCell::new(<RefQualified>::uninit()));
    RefQualified::new(r1.as_pointer());
    ({ RefQualifiedImpl::copy_assign(&r1.as_pointer(), r.as_pointer()) });
    assert!(({ ({ (*r1.borrow()).mark } == 1) }));
    let h2: Value<Holder> = Rc::new(RefCell::new(<Holder>::uninit()));
    {
        let __d = h2.as_pointer();
        Partial::new(field_ptr!(__d, p), { 7 }, { 70 });
        {
            let __d = array_field_ptr!(__d, arr);
            Partial::new(__d.offset(0), { 8 }, { 80 });
            Partial::new(__d.offset(1), { 9 }, { 90 });
        };
    };
    ({ HolderImpl::copy_assign(&h2.as_pointer(), h.as_pointer()) });
    assert!(({ ({ (*h2.borrow()).p.v } == 2) && ({ (*h2.borrow()).p.keep } == 70) }));
    assert!(
        (array_elem!(h2.as_pointer(), arr, 1).with(|__s| __s.v) == 2)
            && (array_elem!(h2.as_pointer(), arr, 1).with(|__s| __s.keep) == 90)
    );
    assert!((assigns_0.with(|rc| *rc.borrow()) == 11));
    let cc1: Value<CopyCounted> = Rc::new(RefCell::new(<CopyCounted>::uninit()));
    CopyCounted::new(cc1.as_pointer(), { 1 });
    let cc2: Value<CopyCounted> = Rc::new(RefCell::new(<CopyCounted>::uninit()));
    CopyCounted::new(cc2.as_pointer(), { 2 });
    ({ CopyCountedImpl::copy_assign(&cc1.as_pointer(), cc2.as_pointer()) });
    assert!(({ ({ (*cc1.borrow()).v } == 2) }));
    let hc1: Value<HasCopyCounted> = Rc::new(RefCell::new(<HasCopyCounted>::uninit()));
    {
        let __d = hc1.as_pointer();
        CopyCounted::new(field_ptr!(__d, c), { 1 });
        {
            let __d = array_field_ptr!(__d, arr);
            CopyCounted::new(__d.offset(0), { 2 });
            CopyCounted::new(__d.offset(1), { 3 });
        };
    };
    let hc2: Value<HasCopyCounted> = Rc::new(RefCell::new(<HasCopyCounted>::uninit()));
    {
        let __d = hc2.as_pointer();
        CopyCounted::new(field_ptr!(__d, c), { 4 });
        {
            let __d = array_field_ptr!(__d, arr);
            CopyCounted::new(__d.offset(0), { 5 });
            CopyCounted::new(__d.offset(1), { 6 });
        };
    };
    ({ HasCopyCountedImpl::copy_assign(&hc1.as_pointer(), hc2.as_pointer()) });
    assert!(
        ({
            (({ (*hc1.borrow()).c.v } == 4)
                && (array_elem!(hc1.as_pointer(), arr, 0).with(|__s| __s.v) == 5))
                && (array_elem!(hc1.as_pointer(), arr, 1).with(|__s| __s.v) == 6)
        })
    );
    assert!((copies_1.with(|rc| *rc.borrow()) == 0));
    return 0;
}
pub trait CopyCountedImpl {
    fn copy_assign(&self, _a0: Ptr<CopyCounted>) -> Ptr<CopyCounted>;
}
impl CopyCountedImpl for Ptr<CopyCounted> {
    fn copy_assign(&self, _a0: Ptr<CopyCounted>) -> Ptr<CopyCounted> {
        field!((*self), v).write({ _a0.with(|__s| __s.v) });
        return (*self).clone();
    }
}
pub trait HasCopyCountedImpl {
    fn copy_assign(&self, _a0: Ptr<HasCopyCounted>) -> Ptr<HasCopyCounted>;
}
impl HasCopyCountedImpl for Ptr<HasCopyCounted> {
    fn copy_assign(&self, _a0: Ptr<HasCopyCounted>) -> Ptr<HasCopyCounted> {
        ({
            let _arg0: Ptr<CopyCounted> = field_ptr!(_a0, c);
            CopyCountedImpl::copy_assign(&field_ptr!((*self), c), _arg0)
        });
        {
            let mut __tmp_0 = { (array_field_ptr!((*self), arr)).to_any() };
            __tmp_0.memcpy(&(array_field_ptr!(_a0, arr)).to_any(), 8_usize);
            (__tmp_0).clone()
        };
        return (*self).clone();
    }
}
pub trait HolderImpl {
    fn copy_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
}
impl HolderImpl for Ptr<Holder> {
    fn copy_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        ({
            let _o: Ptr<Partial> = field_ptr!(_a0, p);
            PartialImpl::copy_assign(&field_ptr!((*self), p), _o)
        });
        {
            let mut __i0: usize = 0_usize;
            'loop_: while (__i0 != 2_usize) {
                ({
                    let _o: Ptr<Partial> = array_field_ptr!(_a0, arr).offset((__i0) as isize);
                    PartialImpl::copy_assign(
                        &array_field_ptr!((*self), arr).offset((__i0) as isize),
                        _o,
                    )
                });
                __i0.prefix_inc();
            }
        }
        return (*self).clone();
    }
}
pub trait NonConstAssignImpl {
    fn operator_assign_2(&self, o: Ptr<NonConstAssign>) -> Ptr<NonConstAssign>;
    fn operator_assign_3(&self, o: Ptr<NonConstAssign>) -> Ptr<NonConstAssign>;
}
impl NonConstAssignImpl for Ptr<NonConstAssign> {
    fn operator_assign_2(&self, o: Ptr<NonConstAssign>) -> Ptr<NonConstAssign> {
        field!((*self), mark).write({ (o.with(|__s| __s.mark) + 1) });
        return (*self).clone();
    }
    fn operator_assign_3(&self, o: Ptr<NonConstAssign>) -> Ptr<NonConstAssign> {
        field!((*self), mark).write({ (o.with(|__s| __s.mark) + 10) });
        return (*self).clone();
    }
}
pub trait PartialImpl {
    fn copy_assign(&self, o: Ptr<Partial>) -> Ptr<Partial>;
}
impl PartialImpl for Ptr<Partial> {
    fn copy_assign(&self, o: Ptr<Partial>) -> Ptr<Partial> {
        if ((*self) == (o)) {
            return (*self).clone();
        }
        field!((*self), v).write({ o.with(|__s| __s.v) });
        (*assigns_0.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
}
pub trait RefQualifiedImpl {
    fn copy_assign(&self, o: Ptr<RefQualified>) -> Ptr<RefQualified>;
}
impl RefQualifiedImpl for Ptr<RefQualified> {
    fn copy_assign(&self, o: Ptr<RefQualified>) -> Ptr<RefQualified> {
        field!((*self), mark).write({ (o.with(|__s| __s.mark) + 1) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = assigns_0.with(|_| ());
    let _ = copies_1.with(|_| ());
}
