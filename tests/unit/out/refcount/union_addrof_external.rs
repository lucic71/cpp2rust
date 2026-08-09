extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive(Clone)]
pub struct record {
    pub code: u16,
    pub lo: u16,
    pub hi: u32,
    pub pad: Box<[u8]>,
}
impl Default for record {
    fn default() -> Self {
        record {
            code: <u16>::default(),
            lo: <u16>::default(),
            hi: <u32>::default(),
            pad: (0..8).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
        }
    }
}
impl ByteRepr for record {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.code.to_bytes(&mut buf[0..2]);
        self.lo.to_bytes(&mut buf[2..4]);
        self.hi.to_bytes(&mut buf[4..8]);
        self.pad.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            code: <u16>::from_bytes(&buf[0..2]),
            lo: <u16>::from_bytes(&buf[2..4]),
            hi: <u32>::from_bytes(&buf[4..8]),
            pad: <Box<[u8]>>::from_bytes(&buf[8..16]),
        }
    }
}
pub struct anon_0 {
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn h(&self) -> Ptr<record> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn raw_(&self) -> Ptr<u8> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Clone for anon_0 {
    fn clone(&self) -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(self.__bytes.borrow().clone())),
        }
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 128]))),
        }
    }
}
impl ByteRepr for anon_0 {
    fn byte_size() -> usize {
        128
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        buf.copy_from_slice(&self.__bytes.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from(buf))),
        }
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Container {
    pub view: anon_0,
}
impl ByteRepr for Container {
    fn byte_size() -> usize {
        128
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.view.to_bytes(&mut buf[0..128]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            view: <anon_0>::from_bytes(&buf[0..128]),
        }
    }
}
pub fn fill_1(out: AnyPtr, cap: usize) {
    let out: Value<AnyPtr> = Rc::new(RefCell::new(out));
    let cap: Value<usize> = Rc::new(RefCell::new(cap));
    let src: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8,
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
    ])));
    (*src.borrow_mut())[(0) as usize] = 2_u8;
    (*src.borrow_mut())[(1) as usize] = 0_u8;
    (*src.borrow_mut())[(2) as usize] = 0_u8;
    (*src.borrow_mut())[(3) as usize] = 80_u8;
    (*src.borrow_mut())[(4) as usize] = 127_u8;
    (*src.borrow_mut())[(5) as usize] = 0_u8;
    (*src.borrow_mut())[(6) as usize] = 0_u8;
    (*src.borrow_mut())[(7) as usize] = 1_u8;
    let n: Value<usize> = Rc::new(RefCell::new(
        (if ((16usize < (*cap.borrow())) as i32) != 0 {
            (16usize as u64)
        } else {
            ((*cap.borrow()) as u64)
        } as usize),
    ));
    {
        (*out.borrow()).memcpy(
            &((src.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
            (*n.borrow()) as usize,
        );
        (*out.borrow()).clone()
    };
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let c: Value<Container> = <Value<Container>>::default();
    {
        ((c.as_pointer()) as Ptr<Container>)
            .to_any()
            .memset((0) as u8, 128usize as usize);
        ((c.as_pointer()) as Ptr<Container>).to_any().clone()
    };
    ({
        let _out: AnyPtr = (c.as_pointer().field_ptr(
            0,
            |__v: &Container| ::std::slice::from_ref(&__v.view),
            |__v: &mut Container| ::std::slice::from_mut(&mut __v.view),
        ))
        .to_any();
        let _cap: usize = 128usize;
        fill_1(_out, _cap)
    });
    assert!(
        ((((c
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<record>() as Ptr<record>)
            .with(|__v| (__v.code as i32) == 2)) as i32)
            != 0)
    );
    assert!(
        ((((((((c
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<record>() as Ptr<record>)
            .field_ptr(
                2,
                |__v: &record| ::std::slice::from_ref(&__v.lo),
                |__v: &mut record| ::std::slice::from_mut(&mut __v.lo)
            ))
        .reinterpret_cast::<u8>())
        .offset(((0) as isize))
        .read()) as i32)
            == 0) as i32)
            != 0)
    );
    assert!(
        ((((((((c
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<record>() as Ptr<record>)
            .field_ptr(
                2,
                |__v: &record| ::std::slice::from_ref(&__v.lo),
                |__v: &mut record| ::std::slice::from_mut(&mut __v.lo)
            ))
        .reinterpret_cast::<u8>())
        .offset(((1) as isize))
        .read()) as i32)
            == 80) as i32)
            != 0)
    );
    assert!(
        (((((((c.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr::<u8>)
            .offset(((0) as isize))
            .read()) as i32)
            == 2) as i32)
            != 0)
    );
    assert!(
        ((((((((c.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr::<u8>)
            .offset(((3) as isize))
            .read()) as u8) as i32)
            == 80) as i32)
            != 0)
    );
    return 0;
}
