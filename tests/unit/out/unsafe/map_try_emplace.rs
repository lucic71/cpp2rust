extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut calls_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut made_1: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut copies_2: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut moves_3: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct Item {
    pub value: i32,
}
impl Item {
    pub unsafe fn new_2(this: *mut Item) {
        (&raw mut (*this).value).write(0);
    }
    pub unsafe fn new_1(this: *mut Item, mut value: i32) {
        (&raw mut (*this).value).write(value);
        (*(*&raw const made_1).get()).prefix_inc();
    }
    pub unsafe fn copy_from(this: *mut Item, other: *const Item) {
        (&raw mut (*this).value).write((*other).value);
        (*(*&raw const copies_2).get()).prefix_inc();
    }
    pub unsafe fn move_from(this: *mut Item, other: *mut Item) {
        (&raw mut (*this).value).write((*other).value);
        (*(*&raw const moves_3).get()).prefix_inc();
        (*other).value = -1_i32;
    }
}
impl Clone for Item {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
            Item::copy_from(&raw mut __v, self as *const Item);
            __v
        }
    }
}
impl Default for Item {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
                Item::new_2(&raw mut __v);
                __v
            }
        }
    }
}
pub unsafe fn argument_4(item: *mut Item) -> *mut Item {
    (*(*&raw const calls_0).get()).prefix_inc();
    return item;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct RefItem {
    pub value: i32,
}
impl RefItem {
    pub unsafe fn new_1(this: *mut RefItem) {
        (&raw mut (*this).value).write(0);
    }
    pub unsafe fn new_2(this: *mut RefItem, counter: *mut i32, mut amount: i32) {
        (&raw mut (*this).value).write({
            (*counter) += amount;
            (*counter)
        });
        (*(*&raw const made_1).get()).prefix_inc();
    }
    pub unsafe fn copy_from(this: *mut RefItem, _a0: *const RefItem) {
        (&raw mut (*this).value).write((*_a0).value);
    }
}
impl Default for RefItem {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: RefItem = ::std::mem::MaybeUninit::uninit().assume_init();
                RefItem::new_1(&raw mut __v);
                __v
            }
        }
    }
}
pub unsafe fn reference_5(counter: *mut i32) -> *mut i32 {
    (*(*&raw const calls_0).get()).prefix_inc();
    return counter;
}
pub static mut global_map_6: libcc2rs::LazyStorage<UnsafeMap<i32, (i32, i32)>> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(UnsafeMap::new()) });
pub static mut global_key_7: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut global_counter_8: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut seed_9: libcc2rs::LazyStorage<i32> = libcc2rs::LazyStorage::new(|__p| unsafe {
    __p.write({
        (*(*(*&raw const global_map_6).get())
            .subscript(&(*(*&raw const global_key_7).get()), || 0_i32)) = 10;
        0
    })
});
pub static mut global_hit_10: libcc2rs::LazyStorage<(
    UnsafeBidirectionalIterator<(i32, i32)>,
    bool,
)> = libcc2rs::LazyStorage::new(|__p| unsafe {
    __p.write(
        match (({
            let __tmp_1 = &mut (*(*&raw const global_counter_8).get());
            (*__tmp_1).prefix_inc();
            __tmp_1
        } as *mut i32),)
        {
            (__tmp_0,) => (*(*&raw const global_map_6).get())
                .try_emplace((*(*&raw const global_key_7).get()), || (*__tmp_0)),
        },
    )
});
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut ok: bool = (((*(*&raw const global_counter_8).get()) == (1))
        && (!((*(*&raw const global_hit_10).get()).1)))
        && (((*(*(*&raw const global_hit_10).get()).0.iter_deref()).1) == (10));
    let mut rows: i32 = 0;
    {
        let mut subset: i32 = 0;
        'loop_: while ((subset) < (8)) {
            {
                let mut key: i32 = 0;
                'loop_: while ((key) < (3)) {
                    {
                        let mut initial: i32 = 0;
                        'loop_: while ((initial) < (3)) {
                            {
                                let mut post: i32 = 0;
                                'loop_: while ((post) < (2)) {
                                    let mut m: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
                                    let mut size: i32 = 0;
                                    {
                                        let mut k: i32 = 0;
                                        'loop_: while ((k) < (3)) {
                                            if (((subset) & ((1) << (k))) != 0) {
                                                (*m.subscript(&k, || 0_i32)) = ((10) + (k));
                                                size.prefix_inc();
                                            }
                                            k.prefix_inc();
                                        }
                                    }
                                    let mut counter: i32 = initial;
                                    let mut result: (
                                        UnsafeBidirectionalIterator<(i32, i32)>,
                                        bool,
                                    ) = {
                                        let mut __tmp_3: i32;
                                        if (post != 0) {
                                            match (({
                                                __tmp_3 = counter.postfix_inc();
                                                &mut __tmp_3
                                            }
                                                as *mut i32),)
                                            {
                                                (__tmp_2,) => m.try_emplace(key, || (*__tmp_2)),
                                            }
                                        } else {
                                            match (({
                                                let __tmp_5 = &mut counter;
                                                (*__tmp_5).prefix_inc();
                                                __tmp_5
                                            }
                                                as *mut i32),)
                                            {
                                                (__tmp_4,) => m.try_emplace(key, || (*__tmp_4)),
                                            }
                                        }
                                    };
                                    let mut hit: bool = (((subset) & ((1) << (key))) != (0));
                                    ok = ((ok as i32) & (((counter) == ((initial) + (1))) as i32))
                                        != 0;
                                    ok = ((ok as i32)
                                        & ((((result.1) as i32) == (!(hit) as i32)) as i32))
                                        != 0;
                                    let mut found: bool = result.0 == m.find(&key);
                                    ok = ((ok as i32) & (found as i32)) != 0;
                                    ok = ((ok as i32)
                                        & ((((*result.0.iter_deref()).0) == (key)) as i32))
                                        != 0;
                                    ok = ((ok as i32)
                                        & ((((*result.0.iter_deref()).1)
                                            == (if hit {
                                                ((10) + (key))
                                            } else {
                                                ((initial) + (!(post != 0) as i32))
                                            })) as i32))
                                        != 0;
                                    ok = ((ok as i32)
                                        & (((m.len()) == (((size) + (!(hit) as i32)) as usize))
                                            as i32))
                                        != 0;
                                    rows.prefix_inc();
                                    post.prefix_inc();
                                }
                            }
                            initial.prefix_inc();
                        }
                    }
                    key.prefix_inc();
                }
            }
            subset.prefix_inc();
        }
    }
    let mut items: UnsafeMap<i32, (i32, Item)> = UnsafeMap::new();
    let mut key: i32 = 0;
    items.try_emplace(key, || {
        let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
        Item::new_1(&raw mut __v, { 10 });
        __v
    });
    let mut source: Item = ::std::mem::MaybeUninit::uninit().assume_init();
    Item::new_1(&raw mut source, { 20 });
    (*(*&raw const made_1).get()) = {
        (*(*&raw const copies_2).get()) = {
            (*(*&raw const moves_3).get()) = 0;
            (*(*&raw const moves_3).get())
        };
        (*(*&raw const copies_2).get())
    };
    let mut hit: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
        match (((unsafe { argument_4(&mut source) }) as *mut Item),) {
            (__tmp_6,) => items.try_emplace(key, || {
                let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
                Item::copy_from(&raw mut __v, { __tmp_6 });
                __v
            }),
        };
    ok = ((ok as i32)
        & ((((((*(*&raw const calls_0).get()) == (1)) && ((*(*&raw const made_1).get()) == (0)))
            && ((*(*&raw const copies_2).get()) == (0)))
            && ((*(*&raw const moves_3).get()) == (0))) as i32))
        != 0;
    ok = ((ok as i32) & (((!(hit.1)) && (((*hit.0.iter_deref()).1.value) == (10))) as i32)) != 0;
    let mut move_hit: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
        match (((unsafe { argument_4(&mut source) }) as *mut Item),) {
            (__tmp_7,) => items.try_emplace(key, || {
                let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
                Item::move_from(&raw mut __v, { __tmp_7 });
                __v
            }),
        };
    ok = ((ok as i32)
        & ((((((*(*&raw const calls_0).get()) == (2)) && ((*(*&raw const made_1).get()) == (0)))
            && ((*(*&raw const copies_2).get()) == (0)))
            && ((*(*&raw const moves_3).get()) == (0))) as i32))
        != 0;
    ok = ((ok as i32) & (((!(move_hit.1)) && ((source.value) == (20))) as i32)) != 0;
    key.prefix_inc();
    let mut copied: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
        match (((unsafe { argument_4(&mut source) }) as *mut Item),) {
            (__tmp_8,) => items.try_emplace(key, || {
                let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
                Item::copy_from(&raw mut __v, { __tmp_8 });
                __v
            }),
        };
    ok = ((ok as i32)
        & ((((((*(*&raw const calls_0).get()) == (3)) && ((*(*&raw const made_1).get()) == (0)))
            && ((*(*&raw const copies_2).get()) == (1)))
            && ((*(*&raw const moves_3).get()) == (0))) as i32))
        != 0;
    ok = ((ok as i32) & (((copied.1) && (((*copied.0.iter_deref()).1.value) == (20))) as i32)) != 0;
    key.prefix_inc();
    let mut moved: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
        match (((unsafe { argument_4(&mut source) }) as *mut Item),) {
            (__tmp_9,) => items.try_emplace(key, || {
                let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
                Item::move_from(&raw mut __v, { __tmp_9 });
                __v
            }),
        };
    ok = ((ok as i32)
        & ((((((*(*&raw const calls_0).get()) == (4)) && ((*(*&raw const made_1).get()) == (0)))
            && ((*(*&raw const copies_2).get()) == (1)))
            && ((*(*&raw const moves_3).get()) == (1))) as i32))
        != 0;
    ok = ((ok as i32)
        & ((((moved.1) && (((*moved.0.iter_deref()).1.value) == (20)))
            && ((source.value) == (-1_i32))) as i32))
        != 0;
    let mut refs: UnsafeMap<i32, (i32, RefItem)> = UnsafeMap::new();
    key = 0;
    let mut counter: i32 = 0;
    refs.try_emplace(key, || {
        let mut __v: RefItem = ::std::mem::MaybeUninit::uninit().assume_init();
        RefItem::new_2(&raw mut __v, { &mut counter }, { 5 });
        __v
    });
    (*(*&raw const made_1).get()) = 0;
    let mut ref_hit: (UnsafeBidirectionalIterator<(i32, RefItem)>, bool) =
        match (((unsafe { reference_5(&mut counter) }) as *mut i32),) {
            (__tmp_10,) => refs.try_emplace(key, || {
                let mut __v: RefItem = ::std::mem::MaybeUninit::uninit().assume_init();
                RefItem::new_2(&raw mut __v, { __tmp_10 }, { 2 });
                __v
            }),
        };
    ok = ((ok as i32)
        & ((((((*(*&raw const calls_0).get()) == (5)) && ((counter) == (5)))
            && ((*(*&raw const made_1).get()) == (0)))
            && (!(ref_hit.1))) as i32))
        != 0;
    key.prefix_inc();
    let mut ref_new: (UnsafeBidirectionalIterator<(i32, RefItem)>, bool) =
        match (((unsafe { reference_5(&mut counter) }) as *mut i32),) {
            (__tmp_11,) => refs.try_emplace(key, || {
                let mut __v: RefItem = ::std::mem::MaybeUninit::uninit().assume_init();
                RefItem::new_2(&raw mut __v, { __tmp_11 }, { 2 });
                __v
            }),
        };
    ok = ((ok as i32)
        & ((((((*(*&raw const calls_0).get()) == (6)) && ((counter) == (7)))
            && ((*(*&raw const made_1).get()) == (1)))
            && (ref_new.1)) as i32))
        != 0;
    ok = ((ok as i32)
        & (((((*ref_hit.0.iter_deref()).1.value) == (5))
            && (((*ref_new.0.iter_deref()).1.value) == (7))) as i32))
        != 0;
    key = 0;
    (*(*&raw const made_1).get()) = {
        (*(*&raw const copies_2).get()) = {
            (*(*&raw const moves_3).get()) = 0;
            (*(*&raw const moves_3).get())
        };
        (*(*&raw const copies_2).get())
    };
    let mut temporary_hit: (UnsafeBidirectionalIterator<(i32, Item)>, bool) = {
        let mut __tmp_13: Item;
        match (({
            __tmp_13 = ::std::mem::MaybeUninit::uninit().assume_init();
            Item::new_1(&raw mut __tmp_13, { 30 });
            &mut __tmp_13
        } as *mut Item),)
        {
            (__tmp_12,) => items.try_emplace(key, || {
                let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
                Item::move_from(&raw mut __v, { __tmp_12 });
                __v
            }),
        }
    };
    ok = ((ok as i32)
        & ((((((*(*&raw const made_1).get()) == (1)) && ((*(*&raw const copies_2).get()) == (0)))
            && ((*(*&raw const moves_3).get()) == (0)))
            && (!(temporary_hit.1))) as i32))
        != 0;
    key.prefix_inc();
    key.prefix_inc();
    key.prefix_inc();
    let mut temporary_new: (UnsafeBidirectionalIterator<(i32, Item)>, bool) = {
        let mut __tmp_15: Item;
        match (({
            __tmp_15 = ::std::mem::MaybeUninit::uninit().assume_init();
            Item::new_1(&raw mut __tmp_15, { 40 });
            &mut __tmp_15
        } as *mut Item),)
        {
            (__tmp_14,) => items.try_emplace(key, || {
                let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
                Item::move_from(&raw mut __v, { __tmp_14 });
                __v
            }),
        }
    };
    ok = ((ok as i32)
        & ((((((*(*&raw const made_1).get()) == (2)) && ((*(*&raw const copies_2).get()) == (0)))
            && ((*(*&raw const moves_3).get()) == (1)))
            && (temporary_new.1)) as i32))
        != 0;
    ok = ((ok as i32) & ((((*temporary_new.0.iter_deref()).1.value) == (40)) as i32)) != 0;
    (unsafe {
        libc::printf(
            c"%d %d %d %d %d\n".as_ptr() as *const libc::c_char,
            (rows),
            (*(*&raw const calls_0).get()),
            (*(*&raw const made_1).get()),
            (*(*&raw const copies_2).get()),
            (*(*&raw const moves_3).get()),
        )
    });
    return if ok { 0 } else { 1 };
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const calls_0).get();
    (*&raw const made_1).get();
    (*&raw const copies_2).get();
    (*&raw const moves_3).get();
    (*&raw const global_map_6).get();
    (*&raw const global_key_7).get();
    (*&raw const global_counter_8).get();
    (*&raw const seed_9).get();
    (*&raw const global_hit_10).get();
}
