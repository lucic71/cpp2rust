extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Entry {
    #[offset(0)]
    pub bits: u8,
    #[offset(2)]
    pub value: u16,
}
impl Entry {
    pub fn copy_from(this: Ptr<Entry>, _a0: Ptr<Entry>) {
        field!(this, bits).write(_a0.with(|__s| __s.bits));
        field!(this, value).write(_a0.with(|__s| __s.value));
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let table: Value<Box<[Entry]>> = Rc::new(RefCell::new(
        (0..8).map(|_| <Entry>::uninit()).collect::<Box<[_]>>(),
    ));
    {
        let __d = table.decay();
        {
            let __d = __d.offset(0);
            field!(__d, bits).write(1_u8);
            field!(__d, value).write(4369_u16);
        };
        {
            let __d = __d.offset(1);
            field!(__d, bits).write(2_u8);
            field!(__d, value).write(8738_u16);
        };
        {
            let __d = __d.offset(2);
            field!(__d, bits).write(3_u8);
            field!(__d, value).write(13107_u16);
        };
        {
            let __d = __d.offset(3);
            field!(__d, bits).write(4_u8);
            field!(__d, value).write(17476_u16);
        };
        {
            let __d = __d.offset(4);
            field!(__d, bits).write(0_u8);
            field!(__d, value).write(0_u16);
        };
        {
            let __d = __d.offset(5);
            field!(__d, bits).write(0_u8);
            field!(__d, value).write(0_u16);
        };
        {
            let __d = __d.offset(6);
            field!(__d, bits).write(0_u8);
            field!(__d, value).write(0_u16);
        };
        {
            let __d = __d.offset(7);
            field!(__d, bits).write(0_u8);
            field!(__d, value).write(0_u16);
        };
    };
    let mut table_size: usize = 4_usize;
    {
        let mut __tmp_0 = { (table.decay().offset(table_size)).to_any() };
        __tmp_0.memcpy(
            &(table.decay().offset(0)).to_any(),
            ((table_size as u64).wrapping_mul((4usize as u64)) as usize),
        );
        (__tmp_0).clone()
    };
    assert!(
        ({
            (({ (*table.borrow())[(4) as usize].bits } as i32) == 1)
                && (({ (*table.borrow())[(4) as usize].value } as i32) == 4369)
        })
    );
    assert!(
        ({
            (({ (*table.borrow())[(5) as usize].bits } as i32) == 2)
                && (({ (*table.borrow())[(5) as usize].value } as i32) == 8738)
        })
    );
    assert!(
        ({
            (({ (*table.borrow())[(6) as usize].bits } as i32) == 3)
                && (({ (*table.borrow())[(6) as usize].value } as i32) == 13107)
        })
    );
    assert!(
        ({
            (({ (*table.borrow())[(7) as usize].bits } as i32) == 4)
                && (({ (*table.borrow())[(7) as usize].value } as i32) == 17476)
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
