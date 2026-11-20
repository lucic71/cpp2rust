extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut made_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut visits_1: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Item {
    pub value: i32,
}
impl Item {
    pub unsafe fn new_1(this: *mut Item) {
        (&raw mut (*this).value).write(0);
    }
    pub unsafe fn new_2(this: *mut Item, counter: *mut i32, mut amount: i32) {
        (&raw mut (*this).value).write({
            (*counter) += amount;
            (*counter)
        });
        (*(*&raw const made_0).get()).prefix_inc();
    }
    pub unsafe fn copy_from(this: *mut Item, _a0: *const Item) {
        (&raw mut (*this).value).write((*_a0).value);
    }
}
impl Default for Item {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
                Item::new_1(&raw mut __v);
                __v
            }
        }
    }
}
pub unsafe fn receiver_2(m: *mut UnsafeMap<i32, (i32, Item)>) -> *mut UnsafeMap<i32, (i32, Item)> {
    (*(*&raw const visits_1).get()).prefix_inc();
    return m;
}
pub unsafe fn place_3(n: *mut i32) -> *mut i32 {
    (*(*&raw const visits_1).get()).prefix_inc();
    return n;
}
pub static mut temporary_calls_4: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Temporary {
    pub n: i32,
    pub data: [i32; 2],
}
impl Temporary {
    pub unsafe fn new(this: *mut Temporary) {
        (&raw mut (*this).n).write(5);
        (&raw mut (*this).data).write([6, 7]);
        (*(*&raw const temporary_calls_4).get()).prefix_inc();
    }
    pub unsafe fn copy_from(this: *mut Temporary, _a0: *const Temporary) {
        (&raw mut (*this).n).write((*_a0).n);
        (&raw mut (*this).data).write(std::array::from_fn::<_, 2, _>(|__i: usize| {
            (*_a0).data[(__i)]
        }));
    }
}
impl Default for Temporary {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: Temporary = ::std::mem::MaybeUninit::uninit().assume_init();
                Temporary::new(&raw mut __v);
                __v
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Nested {
    pub inner: Temporary,
}
impl Nested {
    pub unsafe fn new_1(this: *mut Nested) {
        Temporary::new((&raw mut (*this).inner));
    }
    pub unsafe fn copy_from(this: *mut Nested, _a0: *const Nested) {
        Temporary::copy_from((&raw mut (*this).inner), { &(*_a0).inner });
    }
}
pub unsafe fn make_temporary_5(__ret: *mut Temporary) {
    Temporary::new(__ret.clone());
    return;
}
pub unsafe fn make_nested_6(__ret: *mut Nested) {
    Nested::new_1(__ret.clone());
    return;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct Owner {
    pub ptr: Option<Box<Temporary>>,
}
impl Owner {
    pub unsafe fn new(this: *mut Owner) {
        (&raw mut (*this).ptr).write(Some(Box::new({
            let mut __v: Temporary = ::std::mem::MaybeUninit::uninit().assume_init();
            Temporary::new(&raw mut __v);
            __v
        })));
    }
    pub unsafe fn value(&mut self) -> *const i32 {
        return &(*self.ptr.as_deref().map_or(::std::ptr::null_mut(), |v| {
            v as *const Temporary as *mut Temporary
        }))
        .n;
    }
    pub unsafe fn move_from(this: *mut Owner, _a0: *mut Owner) {
        (&raw mut (*this).ptr).write((*_a0).ptr.take());
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Owner) -> *mut Owner {
        {
            self.ptr = (*_a0).ptr.take();
        };
        return &mut (*(self as *mut Owner));
    }
}
impl Default for Owner {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: Owner = ::std::mem::MaybeUninit::uninit().assume_init();
                Owner::new(&raw mut __v);
                __v
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct Noncopy {
    pub n: i32,
}
impl Noncopy {
    pub unsafe fn new(this: *mut Noncopy) {
        (&raw mut (*this).n).write(5);
    }
}
impl Default for Noncopy {
    fn default() -> Self {
        unsafe { Noncopy { n: 5 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new_1, ptr)]
pub struct Held {
    pub ptr: *const i32,
}
impl Held {
    pub unsafe fn new_2(this: *mut Held) {
        (&raw mut (*this).ptr).write(std::ptr::null());
    }
    pub unsafe fn new_1(this: *mut Held, n: *const i32) {
        (&raw mut (*this).ptr).write((n));
    }
    pub unsafe fn new_3(this: *mut Held, n: *const [i32; 2], mut _a1: i32) {
        (&raw mut (*this).ptr).write((*n).as_ptr());
    }
    pub unsafe fn copy_from(this: *mut Held, _a0: *const Held) {
        (&raw mut (*this).ptr).write((*_a0).ptr);
    }
}
impl Default for Held {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: Held = ::std::mem::MaybeUninit::uninit().assume_init();
                Held::new_2(&raw mut __v);
                __v
            }
        }
    }
}
pub static mut global_values_7: libcc2rs::LazyStorage<UnsafeMap<i32, (i32, i32)>> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(UnsafeMap::new()) });
pub static mut global_key_8: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut conditional_key_9: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(1) });
pub static mut owned_key_10: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(2) });
pub static mut global_a_11: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut global_b_12: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut global_choose_13: libcc2rs::LazyStorage<bool> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(false) });
pub static mut global_temporary_14: libcc2rs::LazyStorage<(
    UnsafeBidirectionalIterator<(i32, i32)>,
    bool,
)> = libcc2rs::LazyStorage::new(|__p| unsafe {
    __p.write({
        let mut __tmp_1: Temporary;
        match ((&mut (*{
            __tmp_1 = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { make_temporary_5(&raw mut __tmp_1) });
            &mut __tmp_1
        })
        .n as *mut i32),)
        {
            (__tmp_0,) => (*(*&raw const global_values_7).get())
                .try_emplace((*(*&raw const global_key_8).get()), || (*__tmp_0)),
        }
    })
});
pub static mut global_conditional_15: libcc2rs::LazyStorage<(
    UnsafeBidirectionalIterator<(i32, i32)>,
    bool,
)> = libcc2rs::LazyStorage::new(|__p| unsafe {
    __p.write(
        match ((if (*(*&raw const global_choose_13).get()) {
            ({
                let __tmp_3 = &mut (*(*&raw const global_a_11).get());
                (*__tmp_3).prefix_inc();
                __tmp_3
            } as *mut i32)
        } else {
            ({
                let __tmp_4 = &mut (*(*&raw const global_b_12).get());
                (*__tmp_4).prefix_inc();
                __tmp_4
            } as *mut i32)
        } as *mut i32),)
        {
            (__tmp_2,) => (*(*&raw const global_values_7).get())
                .try_emplace((*(*&raw const conditional_key_9).get()), || (*__tmp_2)),
        },
    )
});
pub static mut global_owned_16: libcc2rs::LazyStorage<(
    UnsafeBidirectionalIterator<(i32, i32)>,
    bool,
)> = libcc2rs::LazyStorage::new(|__p| unsafe {
    __p.write({
        let mut __tmp_6: Option<Box<Temporary>>;
        match ((&mut (*(*{
            __tmp_6 = Some(Box::new({
                let mut __v: Temporary = ::std::mem::MaybeUninit::uninit().assume_init();
                Temporary::new(&raw mut __v);
                __v
            }));
            &mut __tmp_6
        })
        .as_deref()
        .map_or(::std::ptr::null_mut(), |v| {
            v as *const Temporary as *mut Temporary
        }))
        .n as *mut i32),)
        {
            (__tmp_5,) => (*(*&raw const global_values_7).get())
                .try_emplace((*(*&raw const owned_key_10).get()), || (*__tmp_5)),
        }
    })
});
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((*(*&raw const global_temporary_14).get()).1)
            && (((*(*(*&raw const global_temporary_14).get()).0.iter_deref()).1) == (5))
    );
    assert!(
        ((*(*&raw const global_owned_16).get()).1)
            && (((*(*(*&raw const global_owned_16).get()).0.iter_deref()).1) == (5))
    );
    assert!(
        ((((*(*&raw const global_conditional_15).get()).1)
            && (((*(*(*&raw const global_conditional_15).get()).0.iter_deref()).1) == (1)))
            && ((*(*&raw const global_a_11).get()) == (0)))
            && ((*(*&raw const global_b_12).get()) == (1))
    );
    let mut m: UnsafeMap<i32, (i32, Item)> = UnsafeMap::new();
    let mut key: i32 = 0;
    let mut counter: i32 = 0;
    m.try_emplace(key, || {
        let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
        Item::new_2(&raw mut __v, { &mut counter }, { 3 });
        __v
    });
    (*(*&raw const made_0).get()) = 0;
    let mut hit: (UnsafeBidirectionalIterator<(i32, Item)>, bool) = match (
        ((unsafe { receiver_2(&mut m) }) as *mut UnsafeMap<i32, (i32, Item)>),
        ({
            let __tmp_9 = (unsafe { place_3(&mut counter) });
            (*__tmp_9).prefix_inc();
            __tmp_9
        } as *mut i32),
    ) {
        (__tmp_7, __tmp_8) => (*__tmp_7).try_emplace(key, || {
            let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
            Item::new_2(&raw mut __v, { __tmp_8 }, { 2 });
            __v
        }),
    };
    assert!(
        ((((counter) == (4)) && ((*(*&raw const made_0).get()) == (0)))
            && ((*(*&raw const visits_1).get()) == (2)))
            && (!(hit.1))
    );
    key.prefix_inc();
    let mut inserted: (UnsafeBidirectionalIterator<(i32, Item)>, bool) = match (({
        let __tmp_11 = &mut counter;
        (*__tmp_11).prefix_inc();
        __tmp_11
    } as *mut i32),)
    {
        (__tmp_10,) => m.try_emplace(key, || {
            let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
            Item::new_2(&raw mut __v, { __tmp_10 }, { 2 });
            __v
        }),
    };
    assert!((((counter) == (7)) && ((*(*&raw const made_0).get()) == (1))) && (inserted.1));
    let mut assigned: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
        match ((match ({ 9 }, (&mut counter as *mut i32)) {
            (__tmp_14, __tmp_13) => {
                __tmp_13.write(__tmp_14);
                __tmp_13
            }
        } as *mut i32),)
        {
            (__tmp_12,) => m.try_emplace(key, || {
                let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
                Item::new_2(&raw mut __v, { __tmp_12 }, { 2 });
                __v
            }),
        };
    assert!((((counter) == (9)) && ((*(*&raw const made_0).get()) == (1))) && (!(assigned.1)));
    key.prefix_inc();
    let mut compound: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
        match ((match ({ 4 }, (&mut counter as *mut i32)) {
            (__tmp_17, __tmp_16) => {
                __tmp_16.write((((*__tmp_16) as i32) + (__tmp_17 as i32)) as i32);
                __tmp_16
            }
        } as *mut i32),)
        {
            (__tmp_15,) => m.try_emplace(key, || {
                let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
                Item::new_2(&raw mut __v, { __tmp_15 }, { 2 });
                __v
            }),
        };
    assert!((((counter) == (15)) && ((*(*&raw const made_0).get()) == (2))) && (compound.1));
    assert!(
        (((*inserted.0.iter_deref()).1.value) == (7))
            && (((*compound.0.iter_deref()).1.value) == (15))
    );
    let mut array: [i32; 2] = [0, 0];
    let mut index: i32 = 0;
    key.prefix_inc();
    match (({
        let __tmp_19 = &mut array[(index.postfix_inc()) as usize];
        (*__tmp_19).prefix_inc();
        __tmp_19
    } as *mut i32),)
    {
        (__tmp_18,) => m.try_emplace(key, || {
            let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
            Item::new_2(&raw mut __v, { __tmp_18 }, { 3 });
            __v
        }),
    };
    assert!(
        (((index) == (1)) && ((array[(0) as usize]) == (4)))
            && ((*(*&raw const made_0).get()) == (3))
    );
    key.prefix_inc();
    match ((match (
        { 5 },
        (&mut array[(index.postfix_inc()) as usize] as *mut i32),
    ) {
        (__tmp_22, __tmp_21) => {
            __tmp_21.write(__tmp_22);
            __tmp_21
        }
    } as *mut i32),)
    {
        (__tmp_20,) => m.try_emplace(key, || {
            let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
            Item::new_2(&raw mut __v, { __tmp_20 }, { 3 });
            __v
        }),
    };
    assert!(
        (((index) == (2)) && ((array[(1) as usize]) == (8)))
            && ((*(*&raw const made_0).get()) == (4))
    );
    match ((match ({ ((counter) + (1)) }, (&mut counter as *mut i32)) {
        (__tmp_25, __tmp_24) => {
            __tmp_24.write(__tmp_25);
            __tmp_24
        }
    } as *mut i32),)
    {
        (__tmp_23,) => m.try_emplace(key, || {
            let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
            Item::new_2(&raw mut __v, { __tmp_23 }, { 3 });
            __v
        }),
    };
    assert!(((counter) == (16)) && ((*(*&raw const made_0).get()) == (4)));
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut choose: bool = false;
    match ((if choose {
        ({
            let __tmp_27 = &mut a;
            (*__tmp_27).prefix_inc();
            __tmp_27
        } as *mut i32)
    } else {
        ({
            let __tmp_28 = &mut b;
            (*__tmp_28).prefix_inc();
            __tmp_28
        } as *mut i32)
    } as *mut i32),)
    {
        (__tmp_26,) => m.try_emplace(key, || {
            let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
            Item::new_2(&raw mut __v, { __tmp_26 }, { 3 });
            __v
        }),
    };
    assert!((((a) == (0)) && ((b) == (1))) && ((*(*&raw const made_0).get()) == (4)));
    choose = true;
    match ((if choose {
        ({
            let __tmp_30 = &mut a;
            (*__tmp_30).prefix_inc();
            __tmp_30
        } as *mut i32)
    } else {
        ({
            let __tmp_31 = &mut b;
            (*__tmp_31).prefix_inc();
            __tmp_31
        } as *mut i32)
    } as *mut i32),)
    {
        (__tmp_29,) => m.try_emplace(key, || {
            let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
            Item::new_2(&raw mut __v, { __tmp_29 }, { 3 });
            __v
        }),
    };
    assert!((((a) == (1)) && ((b) == (1))) && ((*(*&raw const made_0).get()) == (4)));
    match (({
        counter.postfix_inc();
        &mut counter
    } as *mut i32),)
    {
        (__tmp_32,) => m.try_emplace(key, || {
            let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
            Item::new_2(&raw mut __v, { __tmp_32 }, { 3 });
            __v
        }),
    };
    assert!(((counter) == (17)) && ((*(*&raw const made_0).get()) == (4)));
    let mut unsigned_map: UnsafeMap<i32, (i32, u32)> = UnsafeMap::new();
    let mut n: u32 = !0_u32;
    let mut wrapped: (UnsafeBidirectionalIterator<(i32, u32)>, bool) =
        match ((match ({ 1_u32 }, (&mut n as *mut u32)) {
            (__tmp_35, __tmp_34) => {
                __tmp_34.write((((*__tmp_34) as u32).wrapping_add((__tmp_35 as u32))) as u32);
                __tmp_34
            }
        } as *mut u32),)
        {
            (__tmp_33,) => unsigned_map.try_emplace(key, || (*__tmp_33)),
        };
    assert!(((n) == (0_u32)) && (((*wrapped.0.iter_deref()).1) == (0_u32)));
    let mut bytes: [u8; 2] = [255_u8, 0_u8];
    index = 0;
    key.prefix_inc();
    let mut narrowed: (UnsafeBidirectionalIterator<(i32, u32)>, bool) = match ((match (
        { 2 },
        (&mut bytes[(index.postfix_inc()) as usize] as *mut u8),
    ) {
        (__tmp_38, __tmp_37) => {
            __tmp_37.write((((*__tmp_37) as i32) + (__tmp_38 as i32)) as u8);
            __tmp_37
        }
    } as *mut u8),)
    {
        (__tmp_36,) => unsigned_map.try_emplace(key, || ((*__tmp_36) as u32)),
    };
    assert!(
        (((index) == (1)) && ((bytes[(0) as usize] as i32) == (1)))
            && (((*narrowed.0.iter_deref()).1) == (1_u32))
    );
    let mut changed: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
    (*changed.subscript(&key, || 0_i32)) = 10;
    let mut effects: i32 = 0;
    let mut erased: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = match (({
        changed.erase_key(&key);
        {
            let __tmp_40 = &mut effects;
            (*__tmp_40).prefix_inc();
            __tmp_40
        }
    } as *mut i32),)
    {
        (__tmp_39,) => changed.try_emplace(key, || (*__tmp_39)),
    };
    assert!((((effects) == (1)) && (erased.1)) && (((*erased.0.iter_deref()).1) == (1)));
    let mut updated: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = match (({
        (*changed.subscript(&key, || 0_i32)).prefix_inc();
        {
            let __tmp_42 = &mut effects;
            (*__tmp_42).prefix_inc();
            __tmp_42
        }
    } as *mut i32),)
    {
        (__tmp_41,) => changed.try_emplace(key, || (*__tmp_41)),
    };
    assert!((((effects) == (2)) && (!(updated.1))) && (((*updated.0.iter_deref()).1) == (2)));
    let mut pointers: UnsafeMap<i32, (i32, *mut i32)> = UnsafeMap::new();
    let mut p: *mut i32 = array.as_mut_ptr();
    let mut pointer: (UnsafeBidirectionalIterator<(i32, *mut i32)>, bool) =
        match ((match ({ 1 }, (&mut p as *mut *mut i32)) {
            (__tmp_45, __tmp_44) => {
                __tmp_44.write((*__tmp_44).offset((__tmp_45 as isize)));
                __tmp_44
            }
        } as *mut *mut i32),)
        {
            (__tmp_43,) => pointers.try_emplace(key, || (*__tmp_43)),
        };
    assert!(
        ((p) == (array.as_mut_ptr().offset((1) as isize)))
            && (((*pointer.0.iter_deref()).1) == (p))
    );
    {
        let mut hit: i32 = 0;
        'loop_: while ((hit) < (2)) {
            let mut values: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
            let keys: [i32; 6] = [0, 1, 2, 3, 4, 5];
            if (hit != 0) {
                'loop_: for k in
                    IterRange::new(keys.as_ptr(), keys.as_ptr().offset((6_i64) as isize))
                {
                    let mut k: i32 = (*k);
                    (*values.subscript(&k, || 0_i32)) = 42;
                }
            }
            let mut moved: i32 = 0;
            let mut forwarded: i32 = 0;
            let mut xvalue: i32 = 0;
            let mut lvalue: i32 = 0;
            let mut assigned: i32 = 0;
            let mut nested: i32 = 0;
            let mut r0: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = match (({
                let __tmp_47 = &mut moved;
                (*__tmp_47).prefix_inc();
                __tmp_47
            }
                as *mut i32),)
            {
                (__tmp_46,) => values.try_emplace(keys[(0) as usize], || (*__tmp_46)),
            };
            let mut r1: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = match (({
                let __tmp_49 = &mut forwarded;
                (*__tmp_49).prefix_inc();
                __tmp_49
            }
                as *mut i32),)
            {
                (__tmp_48,) => values.try_emplace(keys[(1) as usize], || (*__tmp_48)),
            };
            let mut r2: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = match (({
                let __tmp_51 = &mut xvalue;
                (*__tmp_51).prefix_inc();
                __tmp_51
            }
                as *mut i32),)
            {
                (__tmp_50,) => values.try_emplace(keys[(2) as usize], || (*__tmp_50)),
            };
            let mut r3: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = match (({
                let __tmp_53 = &mut lvalue;
                (*__tmp_53).prefix_inc();
                __tmp_53
            }
                as *mut i32),)
            {
                (__tmp_52,) => values.try_emplace(keys[(3) as usize], || (*__tmp_52)),
            };
            let mut r4: (UnsafeBidirectionalIterator<(i32, i32)>, bool) =
                match ((match ({ 5 }, (&mut assigned as *mut i32)) {
                    (__tmp_56, __tmp_55) => {
                        __tmp_55.write(__tmp_56);
                        __tmp_55
                    }
                } as *mut i32),)
                {
                    (__tmp_54,) => values.try_emplace(keys[(4) as usize], || (*__tmp_54)),
                };
            let mut r5: (UnsafeBidirectionalIterator<(i32, i32)>, bool) =
                match ((match ({ 1 }, (&mut nested as *mut i32)) {
                    (__tmp_59, __tmp_58) => {
                        __tmp_58.write((((*__tmp_58) as i32) + (__tmp_59 as i32)) as i32);
                        __tmp_58
                    }
                } as *mut i32),)
                {
                    (__tmp_57,) => values.try_emplace(keys[(5) as usize], || (*__tmp_57)),
                };
            assert!(
                ((((((moved) == (1)) && ((forwarded) == (1))) && ((xvalue) == (1)))
                    && ((lvalue) == (1)))
                    && ((assigned) == (5)))
                    && ((nested) == (1))
            );
            assert!(
                (((((((r0.1) as i32) == (!(hit != 0) as i32))
                    && (((r1.1) as i32) == (!(hit != 0) as i32)))
                    && (((r2.1) as i32) == (!(hit != 0) as i32)))
                    && (((r3.1) as i32) == (!(hit != 0) as i32)))
                    && (((r4.1) as i32) == (!(hit != 0) as i32)))
                    && (((r5.1) as i32) == (!(hit != 0) as i32))
            );
            'loop_: for k in IterRange::new(keys.as_ptr(), keys.as_ptr().offset((6_i64) as isize)) {
                let mut k: i32 = (*k);
                assert!(
                    ((*values.subscript(&k, || 0_i32))
                        == (if (hit != 0) {
                            42
                        } else {
                            if ((k) == (4)) { 5 } else { 1 }
                        }))
                );
            }
            hit.prefix_inc();
        }
    }
    {
        let mut hit: i32 = 0;
        'loop_: while ((hit) < (2)) {
            {
                let mut choose: i32 = 0;
                'loop_: while ((choose) < (2)) {
                    let mut selected: UnsafeMap<i32, (i32, Item)> = UnsafeMap::new();
                    let keys: [i32; 3] = [0, 1, 2];
                    if (hit != 0) {
                        'loop_: for k in
                            IterRange::new(keys.as_ptr(), keys.as_ptr().offset((3_i64) as isize))
                        {
                            let mut k: i32 = (*k);
                            (*selected.subscript(&k, || <Item>::default())).value = 42;
                        }
                    }
                    let mut a: i32 = 0;
                    let mut b: i32 = 0;
                    let mut c: i32 = 0;
                    let mut d: i32 = 0;
                    let mut e: i32 = 0;
                    let mut f: i32 = 0;
                    let mut prefix: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
                        match ((if (choose != 0) {
                            ({
                                let __tmp_61 = &mut a;
                                (*__tmp_61).prefix_inc();
                                __tmp_61
                            } as *mut i32)
                        } else {
                            ({
                                let __tmp_62 = &mut b;
                                (*__tmp_62).prefix_inc();
                                __tmp_62
                            } as *mut i32)
                        } as *mut i32),)
                        {
                            (__tmp_60,) => selected.try_emplace(keys[(0) as usize], || {
                                let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
                                Item::new_2(&raw mut __v, { __tmp_60 }, { 2 });
                                __v
                            }),
                        };
                    let mut assignment: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
                        match ((if (choose != 0) {
                            ((match ({ 5 }, (&mut c as *mut i32)) {
                                (__tmp_65, __tmp_64) => {
                                    __tmp_64.write(__tmp_65);
                                    __tmp_64
                                }
                            }) as *mut i32)
                        } else {
                            ((match ({ 6 }, (&mut d as *mut i32)) {
                                (__tmp_67, __tmp_66) => {
                                    __tmp_66.write(__tmp_67);
                                    __tmp_66
                                }
                            }) as *mut i32)
                        } as *mut i32),)
                        {
                            (__tmp_63,) => selected.try_emplace(keys[(1) as usize], || {
                                let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
                                Item::new_2(&raw mut __v, { __tmp_63 }, { 2 });
                                __v
                            }),
                        };
                    let mut compound: (UnsafeBidirectionalIterator<(i32, Item)>, bool) =
                        match ((if (choose != 0) {
                            ((match ({ 1 }, (&mut e as *mut i32)) {
                                (__tmp_70, __tmp_69) => {
                                    __tmp_69
                                        .write((((*__tmp_69) as i32) + (__tmp_70 as i32)) as i32);
                                    __tmp_69
                                }
                            }) as *mut i32)
                        } else {
                            ((match ({ 2 }, (&mut f as *mut i32)) {
                                (__tmp_72, __tmp_71) => {
                                    __tmp_71
                                        .write((((*__tmp_71) as i32) + (__tmp_72 as i32)) as i32);
                                    __tmp_71
                                }
                            }) as *mut i32)
                        } as *mut i32),)
                        {
                            (__tmp_68,) => selected.try_emplace(keys[(2) as usize], || {
                                let mut __v: Item = ::std::mem::MaybeUninit::uninit().assume_init();
                                Item::new_2(&raw mut __v, { __tmp_68 }, { 2 });
                                __v
                            }),
                        };
                    assert!(
                        ((a) == (if (choose != 0) {
                            (if (hit != 0) { 1 } else { 3 })
                        } else {
                            0
                        })) && ((b)
                            == (if (choose != 0) {
                                0
                            } else {
                                (if (hit != 0) { 1 } else { 3 })
                            }))
                    );
                    assert!(
                        ((c) == (if (choose != 0) {
                            (if (hit != 0) { 5 } else { 7 })
                        } else {
                            0
                        })) && ((d)
                            == (if (choose != 0) {
                                0
                            } else {
                                (if (hit != 0) { 6 } else { 8 })
                            }))
                    );
                    assert!(
                        ((e) == (if (choose != 0) {
                            (if (hit != 0) { 1 } else { 3 })
                        } else {
                            0
                        })) && ((f)
                            == (if (choose != 0) {
                                0
                            } else {
                                (if (hit != 0) { 2 } else { 4 })
                            }))
                    );
                    assert!(
                        ((((prefix.1) as i32) == (!(hit != 0) as i32))
                            && (((assignment.1) as i32) == (!(hit != 0) as i32)))
                            && (((compound.1) as i32) == (!(hit != 0) as i32))
                    );
                    assert!(
                        (((*prefix.0.iter_deref()).1.value) == (if (hit != 0) { 42 } else { 3 }))
                    );
                    assert!(
                        (((*assignment.0.iter_deref()).1.value)
                            == (if (hit != 0) {
                                42
                            } else {
                                if (choose != 0) { 7 } else { 8 }
                            }))
                    );
                    assert!(
                        (((*compound.0.iter_deref()).1.value)
                            == (if (hit != 0) {
                                42
                            } else {
                                if (choose != 0) { 3 } else { 4 }
                            }))
                    );
                    choose.prefix_inc();
                }
            }
            let mut values: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
            let keys: [i32; 3] = [0, 1, 2];
            if (hit != 0) {
                'loop_: for k in
                    IterRange::new(keys.as_ptr(), keys.as_ptr().offset((3_i64) as isize))
                {
                    let mut k: i32 = (*k);
                    (*values.subscript(&k, || 0_i32)) = 42;
                }
            }
            let mut before: i32 = (*(*&raw const temporary_calls_4).get());
            let mut index: i32 = 0;
            let mut member: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = {
                let mut __tmp_74: Temporary;
                match ((&mut (*{
                    __tmp_74 = ::std::mem::MaybeUninit::uninit().assume_init();
                    (unsafe { make_temporary_5(&raw mut __tmp_74) });
                    &mut __tmp_74
                })
                .n as *mut i32),)
                {
                    (__tmp_73,) => values.try_emplace(keys[(0) as usize], || (*__tmp_73)),
                }
            };
            let mut nested: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = {
                let mut __tmp_76: Nested;
                match ((&mut (*&mut (*{
                    __tmp_76 = ::std::mem::MaybeUninit::uninit().assume_init();
                    (unsafe { make_nested_6(&raw mut __tmp_76) });
                    &mut __tmp_76
                })
                .inner)
                    .n as *mut i32),)
                {
                    (__tmp_75,) => values.try_emplace(keys[(1) as usize], || (*__tmp_75)),
                }
            };
            let mut element: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = {
                let mut __tmp_78: Temporary;
                match ((&mut (*{
                    __tmp_78 = ::std::mem::MaybeUninit::uninit().assume_init();
                    (unsafe { make_temporary_5(&raw mut __tmp_78) });
                    &mut __tmp_78
                })
                .data[(index.postfix_inc()) as usize] as *mut i32),)
                {
                    (__tmp_77,) => values.try_emplace(keys[(2) as usize], || (*__tmp_77)),
                }
            };
            assert!(
                ((*(*&raw const temporary_calls_4).get()) == ((before) + (3))) && ((index) == (1))
            );
            assert!(
                ((((member.1) as i32) == (!(hit != 0) as i32))
                    && (((nested.1) as i32) == (!(hit != 0) as i32)))
                    && (((element.1) as i32) == (!(hit != 0) as i32))
            );
            assert!(
                ((((*member.0.iter_deref()).1) == (if (hit != 0) { 42 } else { 5 }))
                    && (((*nested.0.iter_deref()).1) == (if (hit != 0) { 42 } else { 5 })))
                    && (((*element.0.iter_deref()).1) == (if (hit != 0) { 42 } else { 6 }))
            );
            {
                let mut choose: i32 = 0;
                'loop_: while ((choose) < (2)) {
                    let mut references: UnsafeMap<i32, (i32, Held)> = UnsafeMap::new();
                    let mut ordinary: i32 = 9;
                    if (hit != 0) {
                        references.try_emplace(key, || {
                            let mut __v: Held = ::std::mem::MaybeUninit::uninit().assume_init();
                            Held::new_1(&raw mut __v, { &ordinary });
                            __v
                        });
                    }
                    before = (*(*&raw const temporary_calls_4).get());
                    index = 0;
                    let mut value: i32 = 0;
                    {
                        let mut __tmp_80: Nested;
                        let mut __tmp_81: Temporary;
                        {
                            match ((if (choose != 0) {
                                (&mut (*&mut (*{
                                    __tmp_80 = ::std::mem::MaybeUninit::uninit().assume_init();
                                    (unsafe { make_nested_6(&raw mut __tmp_80) });
                                    &mut __tmp_80
                                })
                                .inner)
                                    .n as *mut i32)
                            } else {
                                (&mut (*{
                                    __tmp_81 = ::std::mem::MaybeUninit::uninit().assume_init();
                                    (unsafe { make_temporary_5(&raw mut __tmp_81) });
                                    &mut __tmp_81
                                })
                                .data[(index.postfix_inc()) as usize]
                                    as *mut i32)
                            } as *mut i32),)
                            {
                                (__tmp_79,) => references.try_emplace(key, || {
                                    let mut __v: Held =
                                        ::std::mem::MaybeUninit::uninit().assume_init();
                                    Held::new_1(&raw mut __v, { __tmp_79 });
                                    __v
                                }),
                            };
                            value = (*(*references.subscript(&key, || <Held>::default())).ptr)
                        }
                    };
                    assert!(
                        ((*(*&raw const temporary_calls_4).get()) == ((before) + (1)))
                            && ((index) == (if (choose != 0) { 0 } else { 1 }))
                    );
                    assert!(
                        ((value)
                            == (if (hit != 0) {
                                9
                            } else {
                                if (choose != 0) { 5 } else { 6 }
                            }))
                    );
                    let mut arrays: UnsafeMap<i32, (i32, Held)> = UnsafeMap::new();
                    let ordinary_array: [i32; 2] = [9, 10];
                    if (hit != 0) {
                        arrays.try_emplace(key, || {
                            let mut __v: Held = ::std::mem::MaybeUninit::uninit().assume_init();
                            Held::new_3(&raw mut __v, { &ordinary_array }, { 0 });
                            __v
                        });
                    }
                    before = (*(*&raw const temporary_calls_4).get());
                    {
                        let mut __tmp_83: Nested;
                        let mut __tmp_84: Temporary;
                        {
                            match ((if (choose != 0) {
                                (&mut (*&mut (*{
                                    __tmp_83 = ::std::mem::MaybeUninit::uninit().assume_init();
                                    (unsafe { make_nested_6(&raw mut __tmp_83) });
                                    &mut __tmp_83
                                })
                                .inner)
                                    .data as *mut [i32; 2])
                            } else {
                                (&mut (*{
                                    __tmp_84 = ::std::mem::MaybeUninit::uninit().assume_init();
                                    (unsafe { make_temporary_5(&raw mut __tmp_84) });
                                    &mut __tmp_84
                                })
                                .data as *mut [i32; 2])
                            } as *mut [i32; 2]),)
                            {
                                (__tmp_82,) => arrays.try_emplace(key, || {
                                    let mut __v: Held =
                                        ::std::mem::MaybeUninit::uninit().assume_init();
                                    Held::new_3(&raw mut __v, { __tmp_82 }, { 0 });
                                    __v
                                }),
                            };
                            value = (*(*arrays.subscript(&key, || <Held>::default()))
                                .ptr
                                .offset((1) as isize))
                        }
                    };
                    assert!(
                        ((*(*&raw const temporary_calls_4).get()) == ((before) + (1)))
                            && ((value) == (if (hit != 0) { 10 } else { 7 }))
                    );
                    choose.prefix_inc();
                }
            }
            hit.prefix_inc();
        }
    }
    {
        let mut hit: i32 = 0;
        'loop_: while ((hit) < (2)) {
            let mut values: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
            let keys: [i32; 6] = [0, 1, 2, 3, 4, 5];
            if (hit != 0) {
                'loop_: for k in
                    IterRange::new(keys.as_ptr(), keys.as_ptr().offset((6_i64) as isize))
                {
                    let mut k: i32 = (*k);
                    (*values.subscript(&k, || 0_i32)) = 42;
                }
            }
            let mut before: i32 = (*(*&raw const temporary_calls_4).get());
            let mut index: i32 = 0;
            let mut arrow: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = {
                let mut __tmp_86: Option<Box<Temporary>>;
                match ((&mut (*(*{
                    __tmp_86 = Some(Box::new({
                        let mut __v: Temporary = ::std::mem::MaybeUninit::uninit().assume_init();
                        Temporary::new(&raw mut __v);
                        __v
                    }));
                    &mut __tmp_86
                })
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| {
                    v as *const Temporary as *mut Temporary
                }))
                .n as *mut i32),)
                {
                    (__tmp_85,) => values.try_emplace(keys[(0) as usize], || (*__tmp_85)),
                }
            };
            let mut nested: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = {
                let mut __tmp_88: Option<Box<Nested>>;
                match ((&mut (*&mut (*(*{
                    __tmp_88 = Some(Box::new({
                        let mut __v: Nested = ::std::mem::MaybeUninit::uninit().assume_init();
                        Nested::new_1(&raw mut __v);
                        __v
                    }));
                    &mut __tmp_88
                })
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| {
                    v as *const Nested as *mut Nested
                }))
                .inner)
                    .n as *mut i32),)
                {
                    (__tmp_87,) => values.try_emplace(keys[(1) as usize], || (*__tmp_87)),
                }
            };
            let mut element: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = {
                let mut __tmp_90: Option<Box<Temporary>>;
                match ((&mut (*(*{
                    __tmp_90 = Some(Box::new({
                        let mut __v: Temporary = ::std::mem::MaybeUninit::uninit().assume_init();
                        Temporary::new(&raw mut __v);
                        __v
                    }));
                    &mut __tmp_90
                })
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| {
                    v as *const Temporary as *mut Temporary
                }))
                .data[(index.postfix_inc()) as usize] as *mut i32),)
                {
                    (__tmp_89,) => values.try_emplace(keys[(2) as usize], || (*__tmp_89)),
                }
            };
            let mut get: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = {
                let mut __tmp_92: Option<Box<Temporary>>;
                match ((&mut (*(*{
                    __tmp_92 = Some(Box::new({
                        let mut __v: Temporary = ::std::mem::MaybeUninit::uninit().assume_init();
                        Temporary::new(&raw mut __v);
                        __v
                    }));
                    &mut __tmp_92
                })
                .as_deref_mut()
                .map_or(::std::ptr::null_mut(), |v| v as *mut Temporary))
                .n as *mut i32),)
                {
                    (__tmp_91,) => values.try_emplace(keys[(3) as usize], || (*__tmp_91)),
                }
            };
            let mut deref: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = {
                let mut __tmp_94: Option<Box<Temporary>>;
                match ((&mut (*((*{
                    __tmp_94 = Some(Box::new({
                        let mut __v: Temporary = ::std::mem::MaybeUninit::uninit().assume_init();
                        Temporary::new(&raw mut __v);
                        __v
                    }));
                    &mut __tmp_94
                })
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| {
                    v as *const Temporary as *mut Temporary
                })))
                .n as *mut i32),)
                {
                    (__tmp_93,) => values.try_emplace(keys[(4) as usize], || (*__tmp_93)),
                }
            };
            let mut indirect: (UnsafeBidirectionalIterator<(i32, i32)>, bool) = {
                let mut __tmp_96: Option<Box<Option<Box<Temporary>>>>;
                match ((&mut (*((*(*{
                    __tmp_96 = Some(Box::new(Some(Box::new({
                        let mut __v: Temporary = ::std::mem::MaybeUninit::uninit().assume_init();
                        Temporary::new(&raw mut __v);
                        __v
                    }))));
                    &mut __tmp_96
                })
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| {
                    v as *const Option<Box<Temporary>> as *mut Option<Box<Temporary>>
                }))
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| {
                    v as *const Temporary as *mut Temporary
                })))
                .n as *mut i32),)
                {
                    (__tmp_95,) => values.try_emplace(keys[(5) as usize], || (*__tmp_95)),
                }
            };
            assert!(
                ((*(*&raw const temporary_calls_4).get()) == ((before) + (6))) && ((index) == (1))
            );
            assert!(
                (((((((arrow.1) as i32) == (!(hit != 0) as i32))
                    && (((nested.1) as i32) == (!(hit != 0) as i32)))
                    && (((element.1) as i32) == (!(hit != 0) as i32)))
                    && (((get.1) as i32) == (!(hit != 0) as i32)))
                    && (((deref.1) as i32) == (!(hit != 0) as i32)))
                    && (((indirect.1) as i32) == (!(hit != 0) as i32))
            );
            'loop_: for k in IterRange::new(keys.as_ptr(), keys.as_ptr().offset((6_i64) as isize)) {
                let mut k: i32 = (*k);
                assert!(
                    ((*values.subscript(&k, || 0_i32))
                        == (if (hit != 0) {
                            42
                        } else {
                            if ((k) == (2)) { 6 } else { 5 }
                        }))
                );
            }
            {
                let mut choose: i32 = 0;
                'loop_: while ((choose) < (2)) {
                    let mut references: UnsafeMap<i32, (i32, Held)> = UnsafeMap::new();
                    let mut ordinary: i32 = 9;
                    if (hit != 0) {
                        references.try_emplace(key, || {
                            let mut __v: Held = ::std::mem::MaybeUninit::uninit().assume_init();
                            Held::new_1(&raw mut __v, { &ordinary });
                            __v
                        });
                    }
                    before = (*(*&raw const temporary_calls_4).get());
                    index = 0;
                    let mut value: i32 = 0;
                    {
                        let mut __tmp_98: Option<Box<Temporary>>;
                        let mut __tmp_99: Option<Box<Temporary>>;
                        {
                            match ((if (choose != 0) {
                                (&mut (*(*{
                                    __tmp_98 = Some(Box::new({
                                        let mut __v: Temporary =
                                            ::std::mem::MaybeUninit::uninit().assume_init();
                                        Temporary::new(&raw mut __v);
                                        __v
                                    }));
                                    &mut __tmp_98
                                })
                                .as_deref()
                                .map_or(::std::ptr::null_mut(), |v| {
                                    v as *const Temporary as *mut Temporary
                                }))
                                .n as *mut i32)
                            } else {
                                (&mut (*(*{
                                    __tmp_99 = Some(Box::new({
                                        let mut __v: Temporary =
                                            ::std::mem::MaybeUninit::uninit().assume_init();
                                        Temporary::new(&raw mut __v);
                                        __v
                                    }));
                                    &mut __tmp_99
                                })
                                .as_deref()
                                .map_or(::std::ptr::null_mut(), |v| {
                                    v as *const Temporary as *mut Temporary
                                }))
                                .data[(index.postfix_inc()) as usize]
                                    as *mut i32)
                            } as *mut i32),)
                            {
                                (__tmp_97,) => references.try_emplace(key, || {
                                    let mut __v: Held =
                                        ::std::mem::MaybeUninit::uninit().assume_init();
                                    Held::new_1(&raw mut __v, { __tmp_97 });
                                    __v
                                }),
                            };
                            value = (*(*references.subscript(&key, || <Held>::default())).ptr)
                        }
                    };
                    assert!(
                        ((*(*&raw const temporary_calls_4).get()) == ((before) + (1)))
                            && ((index) == (if (choose != 0) { 0 } else { 1 }))
                    );
                    assert!(
                        ((value)
                            == (if (hit != 0) {
                                9
                            } else {
                                if (choose != 0) { 5 } else { 6 }
                            }))
                    );
                    let mut owners: UnsafeMap<i32, (i32, Held)> = UnsafeMap::new();
                    {
                        match (((unsafe {
                            Owner::value(&mut *(|__p: *mut Owner| {
                                Owner::new(__p);
                                __p
                            })(
                                (&mut ::std::mem::MaybeUninit::<Owner>::uninit()).as_mut_ptr(),
                            ))
                        }) as *const i32),)
                        {
                            (__tmp_100,) => owners.try_emplace(key, || {
                                let mut __v: Held = ::std::mem::MaybeUninit::uninit().assume_init();
                                Held::new_1(&raw mut __v, { __tmp_100 });
                                __v
                            }),
                        };
                        value = (*(*owners.subscript(&key, || <Held>::default())).ptr)
                    };
                    assert!(((value) == (5)));
                    choose.prefix_inc();
                }
            }
            hit.prefix_inc();
        }
    }
    let mut move_only: UnsafeMap<i32, (i32, (Option<Box<Noncopy>>, i32))> = UnsafeMap::new();
    let mut move_new: (
        UnsafeBidirectionalIterator<(i32, (Option<Box<Noncopy>>, i32))>,
        bool,
    ) = {
        let mut __tmp_102: Option<Box<Noncopy>>;
        match (({
            __tmp_102 = Some(Box::new({
                let mut __v: Noncopy = ::std::mem::MaybeUninit::uninit().assume_init();
                Noncopy::new(&raw mut __v);
                __v
            }));
            &mut __tmp_102
        } as *mut Option<Box<Noncopy>>),)
        {
            (__tmp_101,) => move_only.try_emplace(key, || ((*__tmp_101).take(), 7)),
        }
    };
    let mut move_hit: (
        UnsafeBidirectionalIterator<(i32, (Option<Box<Noncopy>>, i32))>,
        bool,
    ) = {
        let mut __tmp_104: Option<Box<Noncopy>>;
        match (({
            __tmp_104 = Some(Box::new({
                let mut __v: Noncopy = ::std::mem::MaybeUninit::uninit().assume_init();
                Noncopy::new(&raw mut __v);
                __v
            }));
            &mut __tmp_104
        } as *mut Option<Box<Noncopy>>),)
        {
            (__tmp_103,) => move_only.try_emplace(key, || ((*__tmp_103).take(), 9)),
        }
    };
    assert!((move_new.1) && (!(move_hit.1)));
    assert!(
        (((*(*move_new.0.iter_deref())
            .1
            .0
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const Noncopy
                as *mut Noncopy))
        .n) == (5))
            && (((*move_hit.0.iter_deref()).1.1) == (7))
    );
    let mut strings: UnsafeMap<i32, (i32, Vec<libc::c_char>)> = UnsafeMap::new();
    let mut text: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"value".as_ptr(),
        (0..)
            .take_while(|&i| *c"value".as_ptr().add(i) != 0)
            .count()
            + 1,
    )
    .to_vec();
    strings.try_emplace(key, || {
        std::slice::from_raw_parts(
            c"original".as_ptr(),
            (0..)
                .take_while(|&i| *c"original".as_ptr().add(i) != 0)
                .count()
                + 1,
        )
        .to_vec()
    });
    let mut string_hit: (UnsafeBidirectionalIterator<(i32, Vec<libc::c_char>)>, bool) = match (({
        (*(*&raw const visits_1).get()).prefix_inc();
        &mut text
    }
        as *mut Vec<libc::c_char>),)
    {
        (__tmp_105,) => strings.try_emplace(key, || std::mem::replace(&mut (*__tmp_105), vec![0])),
    };
    assert!(
        (((*(*&raw const visits_1).get()) == (3)) && (!(string_hit.1)))
            && (text == {
                std::slice::from_raw_parts(
                    c"value".as_ptr(),
                    (0..)
                        .take_while(|&i| *c"value".as_ptr().add(i) != 0)
                        .count()
                        + 1,
                )
                .to_vec()
            })
    );
    key.prefix_inc();
    let mut string_new: (UnsafeBidirectionalIterator<(i32, Vec<libc::c_char>)>, bool) = match (({
        (*(*&raw const visits_1).get()).prefix_inc();
        &mut text
    }
        as *mut Vec<libc::c_char>),)
    {
        (__tmp_106,) => strings.try_emplace(key, || std::mem::replace(&mut (*__tmp_106), vec![0])),
    };
    assert!(
        (((*(*&raw const visits_1).get()) == (4)) && (string_new.1))
            && ((*string_new.0.iter_deref()).1 == {
                std::slice::from_raw_parts(
                    c"value".as_ptr(),
                    (0..)
                        .take_while(|&i| *c"value".as_ptr().add(i) != 0)
                        .count()
                        + 1,
                )
                .to_vec()
            })
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const made_0).get();
    (*&raw const visits_1).get();
    (*&raw const temporary_calls_4).get();
    (*&raw const global_values_7).get();
    (*&raw const global_key_8).get();
    (*&raw const conditional_key_9).get();
    (*&raw const owned_key_10).get();
    (*&raw const global_a_11).get();
    (*&raw const global_b_12).get();
    (*&raw const global_choose_13).get();
    (*&raw const global_temporary_14).get();
    (*&raw const global_conditional_15).get();
    (*&raw const global_owned_16).get();
}
