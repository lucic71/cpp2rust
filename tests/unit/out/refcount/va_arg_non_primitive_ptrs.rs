extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive(Clone, Default)]
pub struct node {
    pub data: i32,
    pub next: Ptr<node>,
}
impl ByteRepr for node {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.data.to_bytes(&mut buf[0..4]);
        self.next.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data: <i32>::from_bytes(&buf[0..4]),
            next: <Ptr<node>>::from_bytes(&buf[8..16]),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum opt {
    #[default]
    OPT_STRING_OUT = 0,
    OPT_FILE = 1,
    OPT_NODE = 2,
    OPT_NODE_OUT = 3,
}
impl From<i32> for opt {
    fn from(n: i32) -> opt {
        match n {
            0 => opt::OPT_STRING_OUT,
            1 => opt::OPT_FILE,
            2 => opt::OPT_NODE,
            3 => opt::OPT_NODE_OUT,
            _ => panic!("invalid opt value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(opt);
impl ByteRepr for opt {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self as i32).to_bytes(buf);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        <opt>::from(i32::from_bytes(buf))
    }
}
pub fn dispatch_0(option: i32, __args: &[VaArg]) -> i32 {
    let option: Value<i32> = Rc::new(RefCell::new(option));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    let result: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*option.borrow());
        match __match_cond {
            __v if __v == (opt::OPT_STRING_OUT as i32) => {
                let out: Value<Ptr<Ptr<u8>>> =
                    Rc::new(RefCell::new((*ap.borrow_mut()).arg::<Ptr<Ptr<u8>>>()));
                (*out.borrow()).write(Ptr::from_string_literal(b"hello"));
                (*result.borrow_mut()) = 1;
                break 'switch;
            }
            __v if __v == (opt::OPT_FILE as i32) => {
                let f: Value<Ptr<CFile>> =
                    Rc::new(RefCell::new((*ap.borrow_mut()).arg::<Ptr<CFile>>()));
                (*result.borrow_mut()) = ((!((*f.borrow()).is_null())) as i32);
                break 'switch;
            }
            __v if __v == (opt::OPT_NODE as i32) => {
                let n: Value<Ptr<node>> =
                    Rc::new(RefCell::new((*ap.borrow_mut()).arg::<Ptr<node>>()));
                (*result.borrow_mut()) = (*n.borrow()).with(|__v| (*__v).data);
                break 'switch;
            }
            __v if __v == (opt::OPT_NODE_OUT as i32) => {
                let out: Value<Ptr<Ptr<node>>> =
                    Rc::new(RefCell::new((*ap.borrow_mut()).arg::<Ptr<Ptr<node>>>()));
                (*out.borrow()).write(Ptr::<node>::null());
                (*result.borrow_mut()) = 2;
                break 'switch;
            }
            _ => {}
        }
    };
    return (*result.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
    assert!(
        (((({ dispatch_0((opt::OPT_STRING_OUT as i32), &[(s.as_pointer()).into(),]) }) == 1)
            as i32)
            != 0)
    );
    assert!((((!((*s.borrow()).is_null())) as i32) != 0));
    assert!(
        (((({
            dispatch_0(
                (opt::OPT_FILE as i32),
                &[((libcc2rs::c_stdout()).clone()).into()],
            )
        }) == 1) as i32)
            != 0)
    );
    assert!(
        (((({
            dispatch_0(
                (opt::OPT_FILE as i32),
                &[((AnyPtr::default()).reinterpret_cast::<CFile>()).into()],
            )
        }) == 0) as i32)
            != 0)
    );
    let head: Value<node> = Rc::new(RefCell::new(node {
        data: 42,
        next: Ptr::<node>::null(),
    }));
    assert!(
        (((({ dispatch_0((opt::OPT_NODE as i32), &[(head.as_pointer()).into(),]) }) == 42) as i32)
            != 0)
    );
    let outp: Value<Ptr<node>> = Rc::new(RefCell::new((head.as_pointer())));
    assert!(
        (((({ dispatch_0((opt::OPT_NODE_OUT as i32), &[(outp.as_pointer()).into(),]) }) == 2)
            as i32)
            != 0)
    );
    assert!(((((*outp.borrow()).is_null()) as i32) != 0));
    return 0;
}
