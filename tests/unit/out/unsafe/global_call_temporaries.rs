extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn read_0(value: *const i32) -> i32 {
    return (*value);
}
pub static mut selected_1: libcc2rs::LazyStorage<i32> = libcc2rs::LazyStorage::new(|__p| unsafe {
    __p.write({
        let mut __tmp_0: i32;
        let mut __tmp_1: i32;
        (unsafe {
            read_0({
                __tmp_0 = 0;
                __tmp_1 = 0;
                {
                    let __a: *const _ = &mut __tmp_0;
                    let __b: *const _ = &mut __tmp_1;
                    if *__b < *__a { __b } else { __a }
                }
            })
        })
    })
});
pub static mut x_2: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut y_3: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut assigned_4: libcc2rs::LazyStorage<i32> = libcc2rs::LazyStorage::new(|__p| unsafe {
    __p.write({
        let mut __tmp_2: i32;
        let mut __tmp_3: i32;
        ({
            (*(*&raw const x_2).get()) = {
                __tmp_2 = 5;
                __tmp_3 = 6;
                (*{
                    let __a: *const _ = &mut __tmp_2;
                    let __b: *const _ = &mut __tmp_3;
                    if *__b < *__a { __b } else { __a }
                })
            };
            (*(*&raw const x_2).get())
        })
    })
});
pub static mut accumulated_5: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        __p.write({
            let mut __tmp_4: i32;
            let mut __tmp_5: i32;
            ({
                (*(*&raw const y_3).get()) += {
                    __tmp_4 = 1;
                    __tmp_5 = 2;
                    (*{
                        let __a: *const _ = &mut __tmp_4;
                        let __b: *const _ = &mut __tmp_5;
                        if *__a < *__b { __b } else { __a }
                    })
                };
                (*(*&raw const y_3).get())
            })
        })
    });
pub static mut call_assigned_6: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        __p.write({
            let mut __tmp_6: i32;
            ({
                (*(*&raw const x_2).get()) = (unsafe {
                    __tmp_6 = 5;
                    read_0(&mut __tmp_6)
                });
                (*(*&raw const x_2).get())
            })
        })
    });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Item {
    pub value: i32,
}
impl Item {
    pub unsafe fn copy_from(this: *mut Item, _a0: *const Item) {
        (&raw mut (*this).value).write((*_a0).value);
    }
}
pub unsafe fn read_array_7(value: *const [i32; 1]) -> i32 {
    return (*value)[(0) as usize];
}
pub unsafe fn read_record_8(value: *const Item) -> i32 {
    return (*value).value;
}
pub static mut aggregate_selected_9: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        __p.write({
            let mut __tmp_8: i32;
            let mut __tmp_9: i32;
            let mut __tmp_7: [i32; 1];
            let mut __tmp_11: i32;
            let mut __tmp_12: i32;
            let mut __tmp_10: Item;
            ((unsafe {
                __tmp_7 = [{
                    __tmp_8 = 3;
                    __tmp_9 = 4;
                    (*{
                        let __a: *const _ = &mut __tmp_8;
                        let __b: *const _ = &mut __tmp_9;
                        if *__b < *__a { __b } else { __a }
                    })
                }];
                read_array_7(&mut __tmp_7)
            }) + (unsafe {
                __tmp_10 = ::std::mem::MaybeUninit::uninit().assume_init();
                {
                    let __d = &raw mut __tmp_10;
                    (&raw mut (*__d).value).write({
                        __tmp_11 = 5;
                        __tmp_12 = 6;
                        (*{
                            let __a: *const _ = &mut __tmp_11;
                            let __b: *const _ = &mut __tmp_12;
                            if *__a < *__b { __b } else { __a }
                        })
                    });
                };
                read_record_8(&mut __tmp_10)
            }))
        })
    });
pub static mut left_10: libcc2rs::LazyStorage<u64> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(3_u64) });
pub static mut right_11: libcc2rs::LazyStorage<usize> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(7_usize) });
pub static mut mapped_selected_12: libcc2rs::LazyStorage<usize> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        __p.write({
            let mut __tmp_13: u64;
            (({
                __tmp_13 = ((*(*&raw const right_11).get()) as u64);
                (*{
                    let __a: *const _ = &mut (*(*&raw const left_10).get());
                    let __b: *const _ = &mut __tmp_13;
                    if *__b < *__a { __b } else { __a }
                })
            }) as usize)
        })
    });
pub unsafe fn get_13() -> usize {
    return (*(*&raw const value_14).get());
}
static mut value_14: libcc2rs::LazyStorage<usize> = libcc2rs::LazyStorage::new(|__p| unsafe {
    __p.write({
        let mut __tmp_14: u64;
        (({
            __tmp_14 = ((*(*&raw const right_11).get()) as u64);
            (*{
                let __a: *const _ = &mut (*(*&raw const left_10).get());
                let __b: *const _ = &mut __tmp_14;
                if *__a < *__b { __b } else { __a }
            })
        }) as usize)
    })
});
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((*(*&raw const selected_1).get()) == (0)));
    assert!(((*(*&raw const assigned_4).get()) == (5)));
    assert!(((*(*&raw const accumulated_5).get()) == (2)));
    assert!(((*(*&raw const call_assigned_6).get()) == (5)));
    assert!(((*(*&raw const aggregate_selected_9).get()) == (9)));
    assert!(((*(*&raw const aggregate_static_15).get()) == (17)));
    assert!(((*(*&raw const mapped_selected_12).get()) == (3_usize)));
    assert!(((unsafe { get_13() }) == (7_usize)));
    return 0;
}
static mut aggregate_static_15: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        __p.write({
            let mut __tmp_16: i32;
            let mut __tmp_17: i32;
            let mut __tmp_15: [i32; 1];
            let mut __tmp_19: i32;
            let mut __tmp_20: i32;
            let mut __tmp_18: Item;
            ((unsafe {
                __tmp_15 = [{
                    __tmp_16 = 7;
                    __tmp_17 = 8;
                    (*{
                        let __a: *const _ = &mut __tmp_16;
                        let __b: *const _ = &mut __tmp_17;
                        if *__a < *__b { __b } else { __a }
                    })
                }];
                read_array_7(&mut __tmp_15)
            }) + (unsafe {
                __tmp_18 = ::std::mem::MaybeUninit::uninit().assume_init();
                {
                    let __d = &raw mut __tmp_18;
                    (&raw mut (*__d).value).write({
                        __tmp_19 = 9;
                        __tmp_20 = 10;
                        (*{
                            let __a: *const _ = &mut __tmp_19;
                            let __b: *const _ = &mut __tmp_20;
                            if *__b < *__a { __b } else { __a }
                        })
                    });
                };
                read_record_8(&mut __tmp_18)
            }))
        })
    });
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const selected_1).get();
    (*&raw const x_2).get();
    (*&raw const y_3).get();
    (*&raw const assigned_4).get();
    (*&raw const accumulated_5).get();
    (*&raw const call_assigned_6).get();
    (*&raw const aggregate_selected_9).get();
    (*&raw const left_10).get();
    (*&raw const right_11).get();
    (*&raw const mapped_selected_12).get();
}
