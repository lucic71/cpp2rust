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
#[cpp_from(i32, new_1, value)]
pub struct Tracked {
    pub v: i32,
}
impl Tracked {
    pub unsafe fn new_2() -> Self {
        let mut this = Self { v: 0 };
        this
    }
    pub unsafe fn new_1(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(o: *const Tracked) -> Self {
        let mut this = Self { v: (*o).v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
    pub unsafe fn move_from(o: *mut Tracked) -> Self {
        let mut this = Self { v: (*o).v };
        this
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
        unsafe { Tracked::copy_from(self as *const Tracked) }
    }
}
impl Default for Tracked {
    fn default() -> Self {
        unsafe { Tracked::new_2() }
    }
}
pub unsafe fn operator_lt_1(a: *const Tracked, b: *const Tracked) -> bool {
    return (((*a).v) < ((*b).v));
}
pub unsafe fn greater_2(a: *const Tracked, b: *const Tracked) -> bool {
    return (((*a).v) > ((*b).v));
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
        self.items = std::mem::take(&mut (*_a0).items);
        return &mut (*(self as *mut Bag));
    }
}
impl std::cmp::Ord for Bag {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_3(self as *const Bag, other as *const Bag) {
                std::cmp::Ordering::Less
            } else if operator_lt_3(other as *const Bag, self as *const Bag) {
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
        unsafe { operator_eq_4(self as *const Bag, other as *const Bag) }
    }
}
impl std::cmp::Eq for Bag {}
pub unsafe fn operator_lt_3(a: *const Bag, b: *const Bag) -> bool {
    return (((*a).items.len()) < ((*b).items.len()));
}
pub unsafe fn operator_eq_4(a: *const Bag, b: *const Bag) -> bool {
    return (((*a).items.len()) == ((*b).items.len()));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr: [Tracked; 5] = [
        Tracked::new_1({ 3 }),
        Tracked::new_1({ 1 }),
        Tracked::new_1({ 4 }),
        Tracked::new_1({ 1 }),
        Tracked::new_1({ 5 }),
    ];
    {
        let __first = arr.as_mut_ptr();
        let __n = arr
            .as_mut_ptr()
            .offset((5) as isize)
            .iter_distance(&__first);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| (*__first.iter_at(__x)).cmp(&*__first.iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__first.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__first.iter_at(__i), __v);
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
    if 4_usize as usize > vec_.capacity() as usize {
        let len_0 = vec_.len();
        vec_.reserve_exact(4_usize as usize - len_0 as usize);
    };
    {
        let __init = Tracked::new_1({ 2 });
        vec_.push(__init)
    };
    {
        let __init = Tracked::new_1({ 7 });
        vec_.push(__init)
    };
    {
        let __init = Tracked::new_1({ 1 });
        vec_.push(__init)
    };
    {
        let __init = Tracked::new_1({ 8 });
        vec_.push(__init)
    };
    {
        let __first = vec_.begin();
        let __n = vec_.end().iter_distance(&__first);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| (*__first.iter_at(__x)).cmp(&*__first.iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__first.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__first.iter_at(__i), __v);
        }
    };
    assert!(
        (((((*unimplemented!()).v) == (1)) && (((*unimplemented!()).v) == (2)))
            && (((*unimplemented!()).v) == (7)))
            && (((*unimplemented!()).v) == (8))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    unimplemented!();
    assert!((((*unimplemented!()).v) == (8)) && (((*unimplemented!()).v) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    unimplemented!();
    assert!((((*unimplemented!()).v) == (1)) && (((*unimplemented!()).v) == (8)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    unimplemented!();
    assert!(
        ((((*unimplemented!()).v) == (1)) && (((*unimplemented!()).v) == (8)))
            && (((*unimplemented!()).v) == (2))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    let mut bags: [Bag; 3] = std::array::from_fn::<_, 3, _>(|_| <Bag>::default());
    {
        let __init = Tracked::new_1({ 1 });
        bags[(0) as usize].items.push(__init)
    };
    {
        let __init = Tracked::new_1({ 2 });
        bags[(0) as usize].items.push(__init)
    };
    {
        let __init = Tracked::new_1({ 3 });
        bags[(2) as usize].items.push(__init)
    };
    {
        let __first = bags.as_mut_ptr();
        let __n = bags
            .as_mut_ptr()
            .offset((3) as isize)
            .iter_distance(&__first);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| (*__first.iter_at(__x)).cmp(&*__first.iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__first.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__first.iter_at(__i), __v);
        }
    };
    assert!(
        ((bags[(0) as usize].items.is_empty()) && ((bags[(1) as usize].items.len()) == (1_usize)))
            && ((bags[(2) as usize].items.len()) == (2_usize))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    let mut bag_vec: Vec<Bag> = unimplemented!();
    {
        let __init = Tracked::new_1({ 4 });
        (*unimplemented!()).items.push(__init)
    };
    {
        let __init = Tracked::new_1({ 5 });
        (*unimplemented!()).items.push(__init)
    };
    {
        let __init = Tracked::new_1({ 6 });
        (*unimplemented!()).items.push(__init)
    };
    {
        let __first = bag_vec.begin();
        let __n = bag_vec.end().iter_distance(&__first);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| (*__first.iter_at(__x)).cmp(&*__first.iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__first.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__first.iter_at(__i), __v);
        }
    };
    assert!(
        (((*unimplemented!()).items.is_empty())
            && (((*unimplemented!()).items.len()) == (1_usize)))
            && (((*unimplemented!()).v) == (5))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    assert!(unimplemented!());
    assert!(unimplemented!());
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
    (*bag_map.subscript_move(unimplemented!(), |__k| cpp_move_unsafe!(__k))) = 2;
    (*bag_map.subscript_move(unimplemented!(), |__k| cpp_move_unsafe!(__k))) = 0;
    assert!((((*bag_map.find(&bags[(2) as usize])).second) == (2)));
    assert!(unimplemented!());
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0));
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
}
