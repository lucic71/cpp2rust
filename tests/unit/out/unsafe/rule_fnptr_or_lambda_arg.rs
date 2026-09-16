extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Item {
    pub key: i32,
    pub value: i32,
}
pub unsafe fn CompareItem_0(a: *const Item, b: *const Item) -> bool {
    return (((*a).key) < ((*b).key));
}
pub unsafe fn CompareInt_1(mut a: i32, mut b: i32) -> bool {
    return ((a) > (b));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<Item> = Vec::new();
    v.push(Item { key: 3, value: 30 });
    v.push(Item { key: 1, value: 10 });
    v.push(Item { key: 2, value: 20 });
    {
        let len = v.as_mut_ptr().add(v.len()).offset_from(v.as_mut_ptr()) as usize;
        ::std::slice::from_raw_parts_mut(v.as_mut_ptr(), len).sort_by(|x, y| {
            if (CompareItem_0 as unsafe fn(*const Item, *const Item) -> bool)
                .call(x as *const _, y as *const _)
            {
                std::cmp::Ordering::Less
            } else if (CompareItem_0 as unsafe fn(*const Item, *const Item) -> bool)
                .call(y as *const _, x as *const _)
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        })
    };
    assert!(((v[(0_usize)].key) == (1)));
    {
        let len = v.as_mut_ptr().add(v.len()).offset_from(v.as_mut_ptr()) as usize;
        ::std::slice::from_raw_parts_mut(v.as_mut_ptr(), len).sort_by(|x, y| {
            if (|a: *const Item, b: *const Item| {
                return (((*a).key) > ((*b).key));
            })
            .call(x as *const _, y as *const _)
            {
                std::cmp::Ordering::Less
            } else if (|a: *const Item, b: *const Item| {
                return (((*a).key) > ((*b).key));
            })
            .call(y as *const _, x as *const _)
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        })
    };
    assert!(((v[(0_usize)].key) == (3)));
    let mut arr: [i32; 5] = [5, 2, 8, 1, 3];
    {
        let len = arr
            .as_mut_ptr()
            .offset((5) as isize)
            .offset_from(arr.as_mut_ptr()) as usize;
        ::std::slice::from_raw_parts_mut(arr.as_mut_ptr(), len).sort_by(|x, y| {
            if (CompareInt_1 as unsafe fn(i32, i32) -> bool).call(*x, *y) {
                std::cmp::Ordering::Less
            } else if (CompareInt_1 as unsafe fn(i32, i32) -> bool).call(*y, *x) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        })
    };
    assert!(((arr[(0) as usize]) == (8)));
    {
        let len = arr
            .as_mut_ptr()
            .offset((5) as isize)
            .offset_from(arr.as_mut_ptr()) as usize;
        ::std::slice::from_raw_parts_mut(arr.as_mut_ptr(), len).sort_by(|x, y| {
            if (|x: i32, y: i32| {
                return ((x) < (y));
            })
            .call(*x, *y)
            {
                std::cmp::Ordering::Less
            } else if (|x: i32, y: i32| {
                return ((x) < (y));
            })
            .call(*y, *x)
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        })
    };
    assert!(((arr[(0) as usize]) == (1)));
    return 0;
}
