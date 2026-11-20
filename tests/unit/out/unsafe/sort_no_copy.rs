extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, CopyAssignUnsafe, MoveAssignUnsafe)]
#[placement]
#[cpp_from(i32, new_1, value)]
pub struct Tracked {
    pub v: i32,
}
impl Tracked {
    pub unsafe fn new_2(this: *mut Tracked) {
        (&raw mut (*this).v).write(0);
    }
    pub unsafe fn new_1(this: *mut Tracked, mut v: i32) {
        (&raw mut (*this).v).write(v);
    }
    pub unsafe fn copy_from(this: *mut Tracked, o: *const Tracked) {
        (&raw mut (*this).v).write((*o).v);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
    }
    pub unsafe fn move_from(this: *mut Tracked, o: *mut Tracked) {
        (&raw mut (*this).v).write((*o).v);
    }
    pub unsafe fn copy_assign(&mut self, o: *const Tracked) -> *mut Tracked {
        self.v = (*o).v;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        return &mut (*(self as *mut Tracked));
    }
    pub unsafe fn move_assign(&mut self, o: *mut Tracked) -> *mut Tracked {
        self.v = (*o).v;
        return &mut (*(self as *mut Tracked));
    }
}
impl std::cmp::Ord for Tracked {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_1(self as *const Tracked, other as *const Tracked) {
                std::cmp::Ordering::Less
            } else if operator_lt_1(other as *const Tracked, self as *const Tracked) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Tracked {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Tracked {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_1(self as *const Tracked, other as *const Tracked))
                && !(operator_lt_1(other as *const Tracked, self as *const Tracked))
        }
    }
}
impl std::cmp::Eq for Tracked {}
impl Clone for Tracked {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
            Tracked::copy_from(&raw mut __v, self as *const Tracked);
            __v
        }
    }
}
impl Default for Tracked {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
                Tracked::new_2(&raw mut __v);
                __v
            }
        }
    }
}
pub unsafe fn operator_lt_1(a: *const Tracked, b: *const Tracked) -> bool {
    return (((*a).v) < ((*b).v));
}
pub unsafe fn greater_2(a: *const Tracked, b: *const Tracked) -> bool {
    return (((*a).v) > ((*b).v));
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe, Default)]
#[placement]
#[cpp_from(i32, new, value)]
pub struct MoveOnly {
    pub v: i32,
}
impl MoveOnly {
    pub unsafe fn new(this: *mut MoveOnly, mut v: i32) {
        (&raw mut (*this).v).write(v);
    }
    pub unsafe fn move_from(this: *mut MoveOnly, _a0: *mut MoveOnly) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut MoveOnly) -> *mut MoveOnly {
        self.v = (*_a0).v;
        return &mut (*(self as *mut MoveOnly));
    }
}
impl std::cmp::Ord for MoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_3(self as *const MoveOnly, other as *const MoveOnly) {
                std::cmp::Ordering::Less
            } else if operator_lt_3(other as *const MoveOnly, self as *const MoveOnly) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_3(self as *const MoveOnly, other as *const MoveOnly))
                && !(operator_lt_3(other as *const MoveOnly, self as *const MoveOnly))
        }
    }
}
impl std::cmp::Eq for MoveOnly {}
pub unsafe fn operator_lt_3(a: *const MoveOnly, b: *const MoveOnly) -> bool {
    return (((*a).v) < ((*b).v));
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Bag {
    pub items: Vec<Tracked>,
}
impl Bag {
    pub unsafe fn move_from(_a0: *mut Bag) -> Self {
        let mut this = Self {
            items: std::mem::take(&mut (*_a0).items),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Bag) -> *mut Bag {
        {
            self.items = std::mem::take(&mut (*_a0).items);
        };
        return &mut (*(self as *mut Bag));
    }
}
impl std::cmp::Ord for Bag {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_4(self as *const Bag, other as *const Bag) {
                std::cmp::Ordering::Less
            } else if operator_lt_4(other as *const Bag, self as *const Bag) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Bag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_5(self as *const Bag, other as *const Bag) }
    }
}
impl std::cmp::Eq for Bag {}
pub unsafe fn operator_lt_4(a: *const Bag, b: *const Bag) -> bool {
    return (((*a).items.len()) < ((*b).items.len()));
}
pub unsafe fn operator_eq_5(a: *const Bag, b: *const Bag) -> bool {
    return (((*a).items.len()) == ((*b).items.len()));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr: [Tracked; 5] = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = (&raw mut arr).cast::<Tracked>();
        Tracked::new_1(__d.offset(0), { 3 });
        Tracked::new_1(__d.offset(1), { 1 });
        Tracked::new_1(__d.offset(2), { 4 });
        Tracked::new_1(__d.offset(3), { 1 });
        Tracked::new_1(__d.offset(4), { 5 });
    };
    {
        let mut __tmp_0 = { arr.as_mut_ptr() };
        let __n = arr
            .as_mut_ptr()
            .offset((5) as isize)
            .iter_distance(&__tmp_0);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| (*__tmp_0.iter_at(__x)).cmp(&*__tmp_0.iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__tmp_0.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__tmp_0.iter_at(__i), __v);
        }
    };
    assert!(
        (((((arr[(0) as usize].v) == (1)) && ((arr[(1) as usize].v) == (1)))
            && ((arr[(2) as usize].v) == (3)))
            && ((arr[(3) as usize].v) == (4)))
            && ((arr[(4) as usize].v) == (5))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    let mut vec_: Vec<Tracked> = Vec::new();
    if 4_usize > vec_.capacity() as usize {
        let len_0 = vec_.len();
        vec_.reserve_exact(4_usize - len_0 as usize);
    };
    vec_.push_mut({
        let mut __v: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
        Tracked::new_1(&raw mut __v, { 2 });
        __v
    });
    vec_.push_mut({
        let mut __v: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
        Tracked::new_1(&raw mut __v, { 7 });
        __v
    });
    vec_.push_mut({
        let mut __v: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
        Tracked::new_1(&raw mut __v, { 1 });
        __v
    });
    vec_.push_mut({
        let mut __v: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
        Tracked::new_1(&raw mut __v, { 8 });
        __v
    });
    {
        let mut __tmp_1 = { vec_.begin() };
        let __n = vec_.end().iter_distance(&__tmp_1);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| (*__tmp_1.iter_at(__x)).cmp(&*__tmp_1.iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__tmp_1.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__tmp_1.iter_at(__i), __v);
        }
    };
    assert!(
        ((((vec_[(0_usize)].v) == (1)) && ((vec_[(1_usize)].v) == (2)))
            && ((vec_[(2_usize)].v) == (7)))
            && ((vec_[(3_usize)].v) == (8))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    {
        let mut __tmp_2 = { vec_.begin() };
        let __n = vec_.end().iter_distance(&__tmp_2);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            let __a = __tmp_2.iter_at(__x) as *const _;
            let __b = __tmp_2.iter_at(__y) as *const _;
            if (greater_2 as unsafe fn(*const Tracked, *const Tracked) -> bool).call(__a, __b) {
                std::cmp::Ordering::Less
            } else if (greater_2 as unsafe fn(*const Tracked, *const Tracked) -> bool)
                .call(__b, __a)
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__tmp_2.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__tmp_2.iter_at(__i), __v);
        }
    };
    assert!(((vec_[(0_usize)].v) == (8)) && ((vec_[(3_usize)].v) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    {
        let mut __tmp_3 = { vec_.begin() };
        let __n = vec_.end().iter_distance(&__tmp_3);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            let __a = __tmp_3.iter_at(__x) as *const _;
            let __b = __tmp_3.iter_at(__y) as *const _;
            if FnPtr::<fn(*const Tracked, *const Tracked) -> bool>::new(
                |a: *const Tracked, b: *const Tracked| -> bool {
                    unsafe {
                        return (((*a).v) < ((*b).v));
                    }
                },
            )
            .call(__a, __b)
            {
                std::cmp::Ordering::Less
            } else if FnPtr::<fn(*const Tracked, *const Tracked) -> bool>::new(
                |a: *const Tracked, b: *const Tracked| -> bool {
                    unsafe {
                        return (((*a).v) < ((*b).v));
                    }
                },
            )
            .call(__b, __a)
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__tmp_3.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__tmp_3.iter_at(__i), __v);
        }
    };
    assert!(((vec_[(0_usize)].v) == (1)) && ((vec_[(3_usize)].v) == (8)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    {
        let mut __tmp_4 = { vec_.begin().iter_add(1_isize) };
        let __n = vec_.end().iter_distance(&__tmp_4);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            let __a = __tmp_4.iter_at(__x) as *const _;
            let __b = __tmp_4.iter_at(__y) as *const _;
            if (greater_2 as unsafe fn(*const Tracked, *const Tracked) -> bool).call(__a, __b) {
                std::cmp::Ordering::Less
            } else if (greater_2 as unsafe fn(*const Tracked, *const Tracked) -> bool)
                .call(__b, __a)
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__tmp_4.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__tmp_4.iter_at(__i), __v);
        }
    };
    assert!(
        (((vec_[(0_usize)].v) == (1)) && ((vec_[(1_usize)].v) == (8)))
            && ((vec_[(3_usize)].v) == (2))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    let mut bags: [Bag; 3] = std::array::from_fn::<_, 3, _>(|_| <Bag>::default());
    bags[(0) as usize].items.push_mut({
        let mut __v: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
        Tracked::new_1(&raw mut __v, { 1 });
        __v
    });
    bags[(0) as usize].items.push_mut({
        let mut __v: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
        Tracked::new_1(&raw mut __v, { 2 });
        __v
    });
    bags[(2) as usize].items.push_mut({
        let mut __v: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
        Tracked::new_1(&raw mut __v, { 3 });
        __v
    });
    {
        let mut __tmp_5 = { bags.as_mut_ptr() };
        let __n = bags
            .as_mut_ptr()
            .offset((3) as isize)
            .iter_distance(&__tmp_5);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| (*__tmp_5.iter_at(__x)).cmp(&*__tmp_5.iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__tmp_5.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__tmp_5.iter_at(__i), __v);
        }
    };
    assert!(
        ((bags[(0) as usize].items.is_empty()) && ((bags[(1) as usize].items.len()) == (1_usize)))
            && ((bags[(2) as usize].items.len()) == (2_usize))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    let mut bag_vec: Vec<Bag> = Vec::from_fn(3_usize, |_| <Bag>::default());
    bag_vec[(1_usize)].items.push_mut({
        let mut __v: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
        Tracked::new_1(&raw mut __v, { 4 });
        __v
    });
    bag_vec[(1_usize)].items.push_mut({
        let mut __v: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
        Tracked::new_1(&raw mut __v, { 5 });
        __v
    });
    bag_vec[(2_usize)].items.push_mut({
        let mut __v: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
        Tracked::new_1(&raw mut __v, { 6 });
        __v
    });
    {
        let mut __tmp_6 = { bag_vec.begin() };
        let __n = bag_vec.end().iter_distance(&__tmp_6);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| (*__tmp_6.iter_at(__x)).cmp(&*__tmp_6.iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__tmp_6.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__tmp_6.iter_at(__i), __v);
        }
    };
    assert!(
        ((bag_vec[(0_usize)].items.is_empty()) && ((bag_vec[(1_usize)].items.len()) == (1_usize)))
            && ((bag_vec[(2_usize)].items[(1_usize)].v) == (5))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    assert!(
        {
            let mut __tmp_7 = { bag_vec.begin() };
            let mut __tmp_8 = { bag_vec.end() };
            let mut __it = (__tmp_7).clone();
            while __it != __tmp_8 && *__it.iter_deref() != bags[(1) as usize] {
                __it.iter_inc();
            }
            __it
        } == bag_vec.begin().iter_add(1_isize)
    );
    assert!(
        {
            let mut __tmp_9 = { bag_vec.begin() };
            let mut __tmp_10 = { bag_vec.end() };
            let mut __best = __tmp_9.clone();
            if __tmp_9 != __tmp_10 {
                let mut __it = (__tmp_9).clone();
                __it.iter_inc();
                while __it != __tmp_10 {
                    if *__best.iter_deref() < *__it.iter_deref() {
                        __best = __it.clone();
                    }
                    __it.iter_inc();
                }
            }
            __best
        } == bag_vec.begin().iter_add(2_isize)
    );
    assert!(
        (({
            let __a: *const _ = &mut bags[(0) as usize];
            let __b: *const _ = &mut bags[(2) as usize];
            if *__b < *__a { __b } else { __a }
        }) == ((&mut bags[(0) as usize] as *mut Bag).cast_const()))
    );
    assert!(
        (({
            let __a: *const _ = &mut bags[(0) as usize];
            let __b: *const _ = &mut bags[(2) as usize];
            if *__a < *__b { __b } else { __a }
        }) == ((&mut bags[(2) as usize] as *mut Bag).cast_const()))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    let mut bag_map: UnsafeMap<Bag, (Bag, i32)> = UnsafeMap::new();
    (*bag_map.subscript_move(
        &mut bag_vec[(2_usize)],
        |__k| cpp_move_unsafe!(__k),
        || 0_i32,
    )) = 2;
    (*bag_map.subscript_move(
        &mut bag_vec[(0_usize)],
        |__k| cpp_move_unsafe!(__k),
        || 0_i32,
    )) = 0;
    assert!((((*bag_map.find(&bags[(2) as usize]).iter_deref()).1) == (2)));
    assert!(bag_map.find(&bags[(1) as usize]) == bag_map.end());
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    let mut moves: Vec<MoveOnly> = Vec::new();
    moves.push_mut({
        let mut __v: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
        MoveOnly::new(&raw mut __v, { 3 });
        __v
    });
    moves.push_mut({
        let mut __v: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
        MoveOnly::new(&raw mut __v, { 1 });
        __v
    });
    moves.push_mut({
        let mut __v: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
        MoveOnly::new(&raw mut __v, { 2 });
        __v
    });
    {
        let mut __tmp_11 = { moves.begin() };
        let __n = moves.end().iter_distance(&__tmp_11);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| (*__tmp_11.iter_at(__x)).cmp(&*__tmp_11.iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__tmp_11.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__tmp_11.iter_at(__i), __v);
        }
    };
    assert!(
        (((moves[(0_usize)].v) == (1)) && ((moves[(1_usize)].v) == (2)))
            && ((moves[(2_usize)].v) == (3))
    );
    {
        let mut __tmp_12 = { moves.begin() };
        let __n = moves.end().iter_distance(&__tmp_12);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            let __a = __tmp_12.iter_at(__x) as *const _;
            let __b = __tmp_12.iter_at(__y) as *const _;
            if FnPtr::<fn(*const MoveOnly, *const MoveOnly) -> bool>::new(
                |a: *const MoveOnly, b: *const MoveOnly| -> bool {
                    unsafe {
                        return (((*a).v) > ((*b).v));
                    }
                },
            )
            .call(__a, __b)
            {
                std::cmp::Ordering::Less
            } else if FnPtr::<fn(*const MoveOnly, *const MoveOnly) -> bool>::new(
                |a: *const MoveOnly, b: *const MoveOnly| -> bool {
                    unsafe {
                        return (((*a).v) > ((*b).v));
                    }
                },
            )
            .call(__b, __a)
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__tmp_12.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__tmp_12.iter_at(__i), __v);
        }
    };
    assert!(
        (((moves[(0_usize)].v) == (3)) && ((moves[(1_usize)].v) == (2)))
            && ((moves[(2_usize)].v) == (1))
    );
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0));
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
}
