extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive(Default)]
pub struct Chunk {
    pub data: i32,
}
impl Clone for Chunk {
    fn clone(&self) -> Self {
        let mut this = Self { data: self.data };
        this
    }
}
impl ByteRepr for Chunk {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.data.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct Writer {
    pub output: Ptr<Vec<Chunk>>,
    pub chunk: Chunk,
}
impl Clone for Writer {
    fn clone(&self) -> Self {
        let mut this = Self {
            output: (self.output).clone(),
            chunk: (self.chunk).clone(),
        };
        this
    }
}
impl ByteRepr for Writer {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.output.to_bytes(&mut buf[0..8]);
        self.chunk.to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            output: <Ptr<Vec<Chunk>>>::from_bytes(&buf[0..8]),
            chunk: <Chunk>::from_bytes(&buf[8..12]),
        }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct JPEGData {
    pub com_data: Vec<Value<Vec<u8>>>,
    pub app_data: Vec<Value<Vec<u8>>>,
}
impl Clone for JPEGData {
    fn clone(&self) -> Self {
        let mut this = Self {
            com_data: self
                .com_data
                .iter()
                .map(|inner_vec| Rc::new(RefCell::new(inner_vec.borrow().clone())))
                .collect(),
            app_data: self
                .app_data
                .iter()
                .map(|inner_vec| Rc::new(RefCell::new(inner_vec.borrow().clone())))
                .collect(),
        };
        this
    }
}
impl ByteRepr for JPEGData {
    fn byte_size() -> usize {
        48
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.com_data.to_bytes(&mut buf[0..24]);
        self.app_data.to_bytes(&mut buf[24..48]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            com_data: <Vec<Value<Vec<u8>>>>::from_bytes(&buf[0..24]),
            app_data: <Vec<Value<Vec<u8>>>>::from_bytes(&buf[24..48]),
        }
    }
}
pub fn push_param_0(dest: Ptr<Vec<Value<Vec<u8>>>>) {
    let dest: Value<Ptr<Vec<Value<Vec<u8>>>>> = Rc::new(RefCell::new(dest));
    ((*dest.borrow()).clone() as Ptr<Vec<Value<Vec<u8>>>>).with_mut(
        |__v: &mut Vec<Value<Vec<u8>>>| __v.push(Rc::new(RefCell::new(Vec::new().clone()))),
    );
}
pub fn push_local_from_field_1(jpg: Ptr<JPEGData>, cond: bool) {
    let jpg: Value<Ptr<JPEGData>> = Rc::new(RefCell::new(jpg));
    let cond: Value<bool> = Rc::new(RefCell::new(cond));
    let head: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8])));
    let dest: Value<Ptr<Vec<Value<Vec<u8>>>>> =
        Rc::new(RefCell::new(Ptr::<Vec<Value<Vec<u8>>>>::null()));
    if (*cond.borrow()) {
        (*dest.borrow_mut()) = ((*jpg.borrow()).field_ptr(
            0,
            |__v: &JPEGData| ::std::slice::from_ref(&__v.com_data),
            |__v: &mut JPEGData| ::std::slice::from_mut(&mut __v.com_data),
        ));
    } else {
        (*dest.borrow_mut()) = ((*jpg.borrow()).field_ptr(
            24,
            |__v: &JPEGData| ::std::slice::from_ref(&__v.app_data),
            |__v: &mut JPEGData| ::std::slice::from_mut(&mut __v.app_data),
        ));
    }
    ((*dest.borrow()).clone() as Ptr<Vec<Value<Vec<u8>>>>).with_mut(
        |__v: &mut Vec<Value<Vec<u8>>>| {
            __v.push(Rc::new(RefCell::new(
                {
                    let __count = (head.as_pointer() as Ptr<u8>)
                        .offset(((3) as isize))
                        .get_offset()
                        - (head.as_pointer() as Ptr<u8>).get_offset();
                    PtrValueIter::new(&(head.as_pointer() as Ptr<u8>), __count)
                        .map(|item| u8::try_from(item).ok().unwrap())
                        .collect::<Vec<_>>()
                }
                .clone(),
            )))
        },
    );
}
pub fn shrink_through_ptr_2(comps: Ptr<Vec<Chunk>>) {
    let comps: Value<Ptr<Vec<Chunk>>> = Rc::new(RefCell::new(comps));
    (*comps.borrow()).with_mut(|__v: &mut Vec<Chunk>| __v.shrink_to_fit());
}
pub fn nested_push_move_3(bw: Ptr<Writer>) {
    let bw: Value<Ptr<Writer>> = Rc::new(RefCell::new(bw));
    {
        let __obj = (*bw.borrow()).with(|__v| ((*__v).output).clone());
        __obj.with_mut(|__v: &mut Vec<Chunk>| {
            __v.push(std::mem::take(
                &mut (*bw.borrow()).with(|__v| (*__v).chunk.clone()),
            ))
        })
    };
}
pub fn emplace_local_from_field_4(jpg: Ptr<JPEGData>, cond: bool) {
    let jpg: Value<Ptr<JPEGData>> = Rc::new(RefCell::new(jpg));
    let cond: Value<bool> = Rc::new(RefCell::new(cond));
    let head: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8])));
    let dest: Value<Ptr<Vec<Value<Vec<u8>>>>> =
        Rc::new(RefCell::new(Ptr::<Vec<Value<Vec<u8>>>>::null()));
    if (*cond.borrow()) {
        (*dest.borrow_mut()) = ((*jpg.borrow()).field_ptr(
            0,
            |__v: &JPEGData| ::std::slice::from_ref(&__v.com_data),
            |__v: &mut JPEGData| ::std::slice::from_mut(&mut __v.com_data),
        ));
    } else {
        (*dest.borrow_mut()) = ((*jpg.borrow()).field_ptr(
            24,
            |__v: &JPEGData| ::std::slice::from_ref(&__v.app_data),
            |__v: &mut JPEGData| ::std::slice::from_mut(&mut __v.app_data),
        ));
    }
    (*dest.borrow())
        .clone()
        .with_mut(|__v: &mut Vec<Value<Vec<u8>>>| {
            __v.push(Rc::new(RefCell::new({
                let __count = (head.as_pointer() as Ptr<u8>)
                    .offset(((3) as isize))
                    .get_offset()
                    - (head.as_pointer() as Ptr<u8>).get_offset();
                PtrValueIter::new(&(head.as_pointer() as Ptr<u8>), __count)
                    .map(|item| u8::try_from(item).ok().unwrap())
                    .collect::<Vec<_>>()
            })))
        });
}
pub fn nested_emplace_move_5(bw: Ptr<Writer>) {
    let bw: Value<Ptr<Writer>> = Rc::new(RefCell::new(bw));
    (*bw.borrow())
        .with(|__v| (*__v).output.clone().clone())
        .with_mut(|__v: &mut Vec<Chunk>| {
            __v.push(std::mem::take(
                &mut (*bw.borrow()).with(|__v| (*__v).chunk.clone()),
            ))
        });
}
pub fn self_ref_push_6(comps: Ptr<Vec<Chunk>>) {
    let comps: Value<Ptr<Vec<Chunk>>> = Rc::new(RefCell::new(comps));
    {
        let a0_clone = (((*comps.borrow()).elems() as Ptr<Chunk>).read()).clone();
        (*comps.borrow()).with_mut(|__v: &mut Vec<Chunk>| __v.push(a0_clone))
    };
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let vecs: Value<Vec<Value<Vec<u8>>>> = Rc::new(RefCell::new(Vec::new()));
    ({ push_param_0((vecs.as_pointer())) });
    assert!(((*vecs.borrow()).len() == 1_usize));
    assert!(
        ((((vecs.as_pointer() as Ptr<Value<Vec<u8>>>)
            .offset(0_usize)
            .read())
        .as_pointer() as Ptr<Vec<u8>>)
            .read())
        .is_empty()
    );
    let jpg: Value<JPEGData> = Rc::new(RefCell::new(<JPEGData>::default()));
    ({ push_local_from_field_1((jpg.as_pointer()), true) });
    assert!(((*jpg.borrow()).com_data.len() == 1_usize));
    assert!(
        (((((jpg.as_pointer().field_ptr(
            0,
            |__v: &JPEGData| &__v.com_data[..],
            |__v: &mut JPEGData| &mut __v.com_data[..]
        ) as Ptr<Value<Vec<u8>>>)
            .offset(0_usize)
            .read())
        .as_pointer() as Ptr<Vec<u8>>)
            .read())
        .len()
            == 3_usize)
    );
    assert!(
        ((((((jpg.as_pointer().field_ptr(
            0,
            |__v: &JPEGData| &__v.com_data[..],
            |__v: &mut JPEGData| &mut __v.com_data[..]
        ) as Ptr<Value<Vec<u8>>>)
            .offset(0_usize)
            .read())
        .as_pointer() as Ptr<u8>)
            .offset(0_usize)
            .read()) as i32)
            == 1)
    );
    assert!(
        ((((((jpg.as_pointer().field_ptr(
            0,
            |__v: &JPEGData| &__v.com_data[..],
            |__v: &mut JPEGData| &mut __v.com_data[..]
        ) as Ptr<Value<Vec<u8>>>)
            .offset(0_usize)
            .read())
        .as_pointer() as Ptr<u8>)
            .offset(1_usize)
            .read()) as i32)
            == 2)
    );
    assert!(
        ((((((jpg.as_pointer().field_ptr(
            0,
            |__v: &JPEGData| &__v.com_data[..],
            |__v: &mut JPEGData| &mut __v.com_data[..]
        ) as Ptr<Value<Vec<u8>>>)
            .offset(0_usize)
            .read())
        .as_pointer() as Ptr<u8>)
            .offset(2_usize)
            .read()) as i32)
            == 3)
    );
    assert!((*jpg.borrow()).app_data.is_empty());
    let chunks: Value<Vec<Chunk>> = Rc::new(RefCell::new(Vec::new()));
    ({ shrink_through_ptr_2((chunks.as_pointer())) });
    assert!((*chunks.borrow()).is_empty());
    let w: Value<Writer> = Rc::new(RefCell::new(<Writer>::default()));
    (*w.borrow_mut()).chunk.data = 42;
    (*w.borrow_mut()).output = (chunks.as_pointer());
    ({ nested_push_move_3((w.as_pointer())) });
    assert!(((*chunks.borrow()).len() == 1_usize));
    assert!(
        ((chunks.as_pointer() as Ptr<Chunk>)
            .offset(0_usize)
            .with(|__v| (*__v).data)
            == 42)
    );
    ({ emplace_local_from_field_4((jpg.as_pointer()), false) });
    assert!(((*jpg.borrow()).app_data.len() == 1_usize));
    assert!(
        (((((jpg.as_pointer().field_ptr(
            24,
            |__v: &JPEGData| &__v.app_data[..],
            |__v: &mut JPEGData| &mut __v.app_data[..]
        ) as Ptr<Value<Vec<u8>>>)
            .offset(0_usize)
            .read())
        .as_pointer() as Ptr<Vec<u8>>)
            .read())
        .len()
            == 3_usize)
    );
    assert!(
        ((((((jpg.as_pointer().field_ptr(
            24,
            |__v: &JPEGData| &__v.app_data[..],
            |__v: &mut JPEGData| &mut __v.app_data[..]
        ) as Ptr<Value<Vec<u8>>>)
            .offset(0_usize)
            .read())
        .as_pointer() as Ptr<u8>)
            .offset(0_usize)
            .read()) as i32)
            == 1)
    );
    assert!(
        ((((((jpg.as_pointer().field_ptr(
            24,
            |__v: &JPEGData| &__v.app_data[..],
            |__v: &mut JPEGData| &mut __v.app_data[..]
        ) as Ptr<Value<Vec<u8>>>)
            .offset(0_usize)
            .read())
        .as_pointer() as Ptr<u8>)
            .offset(2_usize)
            .read()) as i32)
            == 3)
    );
    assert!(((*jpg.borrow()).com_data.len() == 1_usize));
    (*w.borrow_mut()).chunk.data = 99;
    (*w.borrow_mut()).output = (chunks.as_pointer());
    ({ nested_emplace_move_5((w.as_pointer())) });
    assert!(((*chunks.borrow()).len() == 2_usize));
    assert!(
        ((chunks.as_pointer() as Ptr<Chunk>)
            .offset(1_usize)
            .with(|__v| (*__v).data)
            == 99)
    );
    ({ self_ref_push_6((chunks.as_pointer())) });
    assert!(((*chunks.borrow()).len() == 3_usize));
    assert!(
        ((chunks.as_pointer() as Ptr<Chunk>)
            .offset(2_usize)
            .with(|__v| (*__v).data)
            == 42)
    );
    return 0;
}
