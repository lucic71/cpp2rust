extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, Uninit, FnPtrArg, MoveCtor, CppFrom, MoveAssign, Default)]
#[placement]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct NoCopy {
    #[offset(0)]
    pub v: i32,
}
impl NoCopy {
    pub fn new(this: Ptr<NoCopy>, mut v: i32) {
        field!(this, v).write(v);
    }
    pub fn move_from(this: Ptr<NoCopy>, o: Ptr<NoCopy>) {
        field!(this, v).write(o.with(|__s| __s.v));
        field!(o, v).write(0);
    }
}
#[derive(Record, ByteRepr, Uninit, FnPtrArg, MoveCtor, MoveAssign)]
#[placement]
#[byte_size(4)]
pub struct PrivateCopy {
    #[offset(0)]
    pub v: i32,
}
impl PrivateCopy {
    pub fn new(this: Ptr<PrivateCopy>) {
        field!(this, v).write(0);
    }
    pub fn move_from(this: Ptr<PrivateCopy>, o: Ptr<PrivateCopy>) {
        field!(this, v).write(o.with(|__s| __s.v));
        field!(o, v).write(0);
    }
}
impl Default for PrivateCopy {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<PrivateCopy> = Rc::new(RefCell::new(<PrivateCopy>::uninit()));
            PrivateCopy::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
#[derive(Record, ByteRepr, Uninit, FnPtrArg)]
#[placement]
#[byte_size(4)]
pub struct Immovable {
    #[offset(0)]
    pub v: i32,
}
impl Immovable {
    pub fn new(this: Ptr<Immovable>) {
        field!(this, v).write(0);
    }
}
impl Default for Immovable {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<Immovable> = Rc::new(RefCell::new(<Immovable>::uninit()));
            Immovable::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
#[derive(Record, ByteRepr, Uninit, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[placement]
#[byte_size(8)]
pub struct Container {
    #[offset(0)]
    #[byte_size(4)]
    pub inner: NoCopy,
    #[offset(4)]
    pub tag: i32,
}
impl Container {
    pub fn move_from(this: Ptr<Container>, _a0: Ptr<Container>) {
        NoCopy::move_from(field_ptr!(this, inner), { field_ptr!(_a0, inner) });
        field!(this, tag).write({ (*_a0.upgrade().deref()).tag });
    }
}
#[derive(Record, ByteRepr, Uninit, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct VecHolder {
    #[offset(0)]
    #[byte_size(24)]
    pub items: Vec<NoCopy>,
}
impl VecHolder {
    pub fn move_from(_a0: Ptr<VecHolder>) -> Self {
        Self {
            items: field!(_a0, items).with_mut(std::mem::take),
        }
    }
}
pub fn bump_0(mut p: Ptr<NoCopy>) {
    field!(p, v).with_mut(|__v| __v.postfix_inc());
}
pub fn bump_ref_1(r: Ptr<Immovable>) {
    field!(r, v).with_mut(|__v| __v.postfix_inc());
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<NoCopy> = Rc::new(RefCell::new(<NoCopy>::uninit()));
    NoCopy::new(a.as_pointer(), { 1 });
    let b: Value<NoCopy> = Rc::new(RefCell::new(<NoCopy>::uninit()));
    NoCopy::move_from(b.as_pointer(), { a.as_pointer() });
    assert!(({ ({ (*b.borrow()).v } == 1) && ({ (*a.borrow()).v } == 0) }));
    ({ NoCopyImpl::move_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ ({ (*a.borrow()).v } == 1) && ({ (*b.borrow()).v } == 0) }));
    ({ bump_0((a.as_pointer())) });
    assert!(({ ({ (*a.borrow()).v } == 2) }));
    let p: Value<PrivateCopy> = Rc::new(RefCell::new(<PrivateCopy>::uninit()));
    PrivateCopy::new(p.as_pointer());
    (*p.borrow_mut()).v = 3;
    let q: Value<PrivateCopy> = Rc::new(RefCell::new(<PrivateCopy>::uninit()));
    PrivateCopy::move_from(q.as_pointer(), { p.as_pointer() });
    assert!(({ ({ (*q.borrow()).v } == 3) && ({ (*p.borrow()).v } == 0) }));
    ({ PrivateCopyImpl::move_assign(&p.as_pointer(), q.as_pointer()) });
    assert!(({ ({ (*p.borrow()).v } == 3) && ({ (*q.borrow()).v } == 0) }));
    let im: Value<Immovable> = Rc::new(RefCell::new(<Immovable>::uninit()));
    Immovable::new(im.as_pointer());
    (*im.borrow_mut()).v = 4;
    ({ bump_ref_1(im.as_pointer()) });
    let mut pim: Ptr<Immovable> = (im.as_pointer());
    assert!((pim.with(|__s| __s.v) == 5));
    let c: Value<Container> = Rc::new(RefCell::new(<Container>::uninit()));
    {
        let __d = c.as_pointer();
        NoCopy::new(field_ptr!(__d, inner), { 6 });
        field!(__d, tag).write(7);
    };
    let d: Value<Container> = Rc::new(RefCell::new(<Container>::uninit()));
    Container::move_from(d.as_pointer(), { c.as_pointer() });
    assert!(
        ({
            (({ (*d.borrow()).inner.v } == 6) && ({ (*d.borrow()).tag } == 7))
                && ({ (*c.borrow()).inner.v } == 0)
        })
    );
    let h: Value<VecHolder> = Rc::new(RefCell::new(<VecHolder>::default()));
    let mut h2: VecHolder = VecHolder::move_from({ h.as_pointer() });
    assert!((h2.items.len() == 0_usize));
    return 0;
}
pub trait ContainerImpl {
    fn move_assign(&self, _a0: Ptr<Container>) -> Ptr<Container>;
}
impl ContainerImpl for Ptr<Container> {
    fn move_assign(&self, _a0: Ptr<Container>) -> Ptr<Container> {
        ({
            let _o: Ptr<NoCopy> = field_ptr!(_a0, inner);
            NoCopyImpl::move_assign(&field_ptr!((*self), inner), _o)
        });
        field!((*self), tag).write({ { (*_a0.upgrade().deref()).tag } });
        return (*self).clone();
    }
}
pub trait NoCopyImpl {
    fn move_assign(&self, o: Ptr<NoCopy>) -> Ptr<NoCopy>;
}
impl NoCopyImpl for Ptr<NoCopy> {
    fn move_assign(&self, o: Ptr<NoCopy>) -> Ptr<NoCopy> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        field!(o, v).write(0);
        return (*self).clone();
    }
}
pub trait PrivateCopyImpl {
    fn copy_assign(&self, _a0: Ptr<PrivateCopy>) -> Ptr<PrivateCopy> {
        unimplemented!()
    }
    fn move_assign(&self, o: Ptr<PrivateCopy>) -> Ptr<PrivateCopy>;
}
impl PrivateCopyImpl for Ptr<PrivateCopy> {
    fn move_assign(&self, o: Ptr<PrivateCopy>) -> Ptr<PrivateCopy> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        field!(o, v).write(0);
        return (*self).clone();
    }
}
pub trait VecHolderImpl {
    fn move_assign(&self, _a0: Ptr<VecHolder>) -> Ptr<VecHolder>;
}
impl VecHolderImpl for Ptr<VecHolder> {
    fn move_assign(&self, _a0: Ptr<VecHolder>) -> Ptr<VecHolder> {
        field!((*self), items).with_mut(|__b0| {
            (*__b0) = field!(_a0, items).with_mut(std::mem::take);
        });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
