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
pub struct registry {
    pub slot: AnyPtr,
    pub level: i64,
}
impl ByteRepr for registry {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.slot.to_bytes(&mut buf[0..8]);
        self.level.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            slot: <AnyPtr>::from_bytes(&buf[0..8]),
            level: <i64>::from_bytes(&buf[8..16]),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[repr(u32)]
pub enum field {
    #[default]
    FIELD_SLOT = 0,
    FIELD_LEVEL = 1,
}
impl From<i32> for field {
    fn from(n: i32) -> field {
        match n {
            0 => field::FIELD_SLOT,
            1 => field::FIELD_LEVEL,
            _ => panic!("invalid field value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(field);
impl ByteRepr for field {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self as i32).to_bytes(buf);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        <field>::from(i32::from_bytes(buf))
    }
}
pub fn registry_update_0(r: Ptr<registry>, field: field, __args: &[VaArg]) -> i32 {
    let r: Value<Ptr<registry>> = Rc::new(RefCell::new(r));
    let field: Value<field> = Rc::new(RefCell::new(field));
    let result: Value<i32> = Rc::new(RefCell::new(0));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    'switch: {
        let __match_cond = ((*field.borrow()) as u32);
        match __match_cond {
            __v if __v == ((field::FIELD_SLOT as i32) as u32) => {
                (*r.borrow()).with_mut(|__v| __v.slot = (*ap.borrow_mut()).arg::<AnyPtr>());
                break 'switch;
            }
            __v if __v == ((field::FIELD_LEVEL as i32) as u32) => {
                (*r.borrow()).with_mut(|__v| __v.level = (*ap.borrow_mut()).arg::<i64>());
                break 'switch;
            }
            _ => {
                (*result.borrow_mut()) = 1;
                break 'switch;
            }
        }
    };
    return (*result.borrow());
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let r: Value<registry> = Rc::new(RefCell::new(registry {
        slot: AnyPtr::default(),
        level: 0_i64,
    }));
    let payload: Value<i32> = Rc::new(RefCell::new(7));
    assert!(
        (((({
            registry_update_0(
                (r.as_pointer()),
                field::FIELD_SLOT,
                &[(payload.as_pointer()).into()],
            )
        }) == 0) as i32)
            != 0)
    );
    assert!(
        (((({ registry_update_0((r.as_pointer()), field::FIELD_LEVEL, &[(5_i64).into(),]) }) == 0)
            as i32)
            != 0)
    );
    assert!(
        ((({
            let _lhs = ((*r.borrow()).slot).clone();
            _lhs == (payload.as_pointer()).to_any()
        }) as i32)
            != 0)
    );
    assert!((((((*r.borrow()).slot.reinterpret_cast::<i32>().read()) == 7) as i32) != 0));
    assert!(((((*r.borrow()).level == 5_i64) as i32) != 0));
    return 0;
}
