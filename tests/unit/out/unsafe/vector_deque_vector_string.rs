extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<std::collections::VecDeque<Vec<Vec<libc::c_char>>>> = Vec::new();
    {
        let __init = std::collections::VecDeque::new();
        v.push(__init)
    };
    {
        let __init = std::collections::VecDeque::new();
        v.push(__init)
    };
    let mut row: Vec<Vec<libc::c_char>> = Vec::new();
    {
        let __a1 = std::slice::from_raw_parts(
            c"a".as_ptr(),
            (0..).take_while(|&i| *c"a".as_ptr().add(i) != 0).count() + 1,
        )
        .to_vec();
        row.push(__a1)
    };
    {
        let __a1 = std::slice::from_raw_parts(
            c"bc".as_ptr(),
            (0..).take_while(|&i| *c"bc".as_ptr().add(i) != 0).count() + 1,
        )
        .to_vec();
        row.push(__a1)
    };
    v[(0_usize)].push_back((row).clone());
    {
        let __a1 = std::slice::from_raw_parts(
            c"def".as_ptr(),
            (0..).take_while(|&i| *c"def".as_ptr().add(i) != 0).count() + 1,
        )
        .to_vec();
        row.push(__a1)
    };
    v[(1_usize)].push_back((row).clone());
    {
        let __init = Vec::new();
        v[(1_usize)].push_back(__init)
    };
    match (((v[(1_usize)].back_mut().unwrap()) as *mut Vec<Vec<libc::c_char>>),) {
        (__tmp_0,) => {
            let __a1 = {
                let mut __tmp2 = (&mut (*((v[(0_usize)]).front_mut().unwrap())))[(1_usize)].clone();
                __tmp2.pop();
                __tmp2.extend_from_slice(::std::slice::from_raw_parts(
                    c"x".as_ptr(),
                    (0..).position(|i| *c"x".as_ptr().add(i) == 0).unwrap(),
                ));
                __tmp2.push(0);
                __tmp2
            };
            (*__tmp_0).push(__a1)
        }
    };
    assert!((((*((v[(0_usize)]).front_mut().unwrap())).len()) == (2_usize)));
    assert!(
        (((*((v[(1_usize)]).front_mut().unwrap())).len()) == (3_usize))
            && ((&mut (*((v[(1_usize)]).front_mut().unwrap())))[(2_usize)] == {
                std::slice::from_raw_parts(
                    c"def".as_ptr(),
                    (0..).take_while(|&i| *c"def".as_ptr().add(i) != 0).count() + 1,
                )
                .to_vec()
            })
    );
    assert!(
        (&mut (*(v[(1_usize)].back_mut().unwrap())))[(0_usize)] == {
            std::slice::from_raw_parts(
                c"bcx".as_ptr(),
                (0..).take_while(|&i| *c"bcx".as_ptr().add(i) != 0).count() + 1,
            )
            .to_vec()
        }
    );
    let mut copy: Vec<std::collections::VecDeque<Vec<Vec<libc::c_char>>>> = v.clone();
    match ((&mut std::slice::from_raw_parts(
        c"z".as_ptr(),
        (0..).take_while(|&i| *c"z".as_ptr().add(i) != 0).count() + 1,
    )
    .to_vec() as *mut Vec<libc::c_char>),)
    {
        (__tmp_1,) => {
            (&mut (*((copy[(0_usize)]).front_mut().unwrap())))[(0_usize)] =
                std::mem::replace(&mut (*__tmp_1), vec![0])
        }
    };
    (*match (
        (&mut (&mut (*((v[(0_usize)]).front_mut().unwrap())))[(1_usize)] as *mut Vec<libc::c_char>),
    ) {
        (__tmp_2,) => {
            if 0_usize >= (*__tmp_2).len() - 1 {
                panic!("out of bounds access")
            } else {
                (&mut (&mut (*__tmp_2))[0_usize] as *mut libc::c_char)
            }
        }
    }) = ('B' as libc::c_char);
    assert!(
        ((&mut (*((v[(0_usize)]).front_mut().unwrap())))[(0_usize)] == {
            std::slice::from_raw_parts(
                c"a".as_ptr(),
                (0..).take_while(|&i| *c"a".as_ptr().add(i) != 0).count() + 1,
            )
            .to_vec()
        }) && ((&mut (*((copy[(0_usize)]).front_mut().unwrap())))[(0_usize)] == {
            std::slice::from_raw_parts(
                c"z".as_ptr(),
                (0..).take_while(|&i| *c"z".as_ptr().add(i) != 0).count() + 1,
            )
            .to_vec()
        })
    );
    assert!(
        ((&mut (*((v[(0_usize)]).front_mut().unwrap())))[(1_usize)] == {
            std::slice::from_raw_parts(
                c"Bc".as_ptr(),
                (0..).take_while(|&i| *c"Bc".as_ptr().add(i) != 0).count() + 1,
            )
            .to_vec()
        }) && ((&mut (*((copy[(0_usize)]).front_mut().unwrap())))[(1_usize)] == {
            std::slice::from_raw_parts(
                c"bc".as_ptr(),
                (0..).take_while(|&i| *c"bc".as_ptr().add(i) != 0).count() + 1,
            )
            .to_vec()
        })
    );
    v[(1_usize)].pop_front();
    assert!(
        (&mut (*((v[(1_usize)]).front_mut().unwrap())))[(0_usize)] == {
            std::slice::from_raw_parts(
                c"bcx".as_ptr(),
                (0..).take_while(|&i| *c"bcx".as_ptr().add(i) != 0).count() + 1,
            )
            .to_vec()
        }
    );
    assert!((((*((copy[(1_usize)]).front_mut().unwrap())).len()) == (3_usize)));
    assert!(((row.len()) == (3_usize)));
    let mut d: std::collections::VecDeque<i32> = std::collections::VecDeque::new();
    let mut first: i32 = 1;
    let mut last: i32 = 2;
    d.push_back(first);
    d.push_back(last);
    let r: *mut i32 = ((d).front_mut().unwrap());
    (*r) = 5;
    let c: *const i32 = (d.back_mut().unwrap());
    assert!(((*((d).front_mut().unwrap())) == (5)) && ((*c) == (2)));
    d.push_front(first);
    assert!((((*r) == (5)) && ((*c) == (2))) && ((*((d).front_mut().unwrap())) == (1)));
    let b: *mut i32 = (d.back_mut().unwrap());
    d.pop_front();
    assert!(
        ((((*b) == (2)) && ((d.len()) == (2_usize)))
            && ((*(&mut (d)[0_usize] as *mut i32)) == (5)))
            && ((*(d.get_mut(1_usize).expect("out of bounds access") as *mut i32)) == (2))
    );
    d.push_back(3);
    d.pop_back();
    let mut sum: i32 = 0;
    'loop_: for x in IterRange::new(d.begin(), d.end()) {
        let mut x: i32 = (*x.iter_deref());
        sum += x;
    }
    {
        let mut it: UnsafeRandomAccessIterator<i32> = d.begin();
        'loop_: while it != d.end() {
            sum += (*it.iter_deref());
            {
                (*(&mut it as *mut UnsafeRandomAccessIterator<i32>)).iter_inc();
                (&mut it as *mut UnsafeRandomAccessIterator<i32>)
            };
        }
    }
    assert!(((sum) == (14)));
    let m: *mut i32 = &mut (*(&mut (d)[1_usize] as *mut i32));
    d.push_front(5);
    d.push_front(1);
    d.pop_front();
    d.push_back(30);
    assert!(((*m) == (2)) && ((d.len()) == (4_usize)));
    let s: *mut i32 = &mut (*(&mut (d)[2_usize] as *mut i32));
    d.pop_back();
    d.push_front(9);
    (*s) = 4;
    assert!(
        ((((*m) == (4)) && ((*(&mut (d)[3_usize] as *mut i32)) == (4)))
            && ((*((d).front_mut().unwrap())) == (9)))
            && ((*(d.back_mut().unwrap())) == (4))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
