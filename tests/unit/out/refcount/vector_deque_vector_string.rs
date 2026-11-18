extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<Deque<Vec<Vec<i8>>>>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __init = Deque::new();
        (*v.borrow_mut()).push(__init)
    };
    {
        let __init = Deque::new();
        (*v.borrow_mut()).push(__init)
    };
    let row: Value<Vec<Vec<i8>>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"a").to_c_bytes();
            __bytes.push(0);
            __bytes
        };
        (*row.borrow_mut()).push(__a1)
    };
    {
        let __a1 = {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"bc").to_c_bytes();
            __bytes.push(0);
            __bytes
        };
        (*row.borrow_mut()).push(__a1)
    };
    (*v.borrow_mut())[0_usize].push_back(({ (*row.borrow()).clone() }));
    {
        let __a1 = {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"def").to_c_bytes();
            __bytes.push(0);
            __bytes
        };
        (*row.borrow_mut()).push(__a1)
    };
    (*v.borrow_mut())[1_usize].push_back(({ (*row.borrow()).clone() }));
    {
        let __init = Vec::new();
        (*v.borrow_mut())[1_usize].push_back(__init)
    };
    match (v.decay().offset(1_usize).back(),) {
        (__tmp_0,) => {
            let __a1 = {
                let mut r = ({
                    (*v.decay()
                        .offset(0_usize)
                        .front()
                        .decay()
                        .offset(1_usize)
                        .upgrade()
                        .deref())
                    .clone()
                });
                r.pop();
                Ptr::<i8>::from_string_literal(b"x").with_c_str(|__s| r.extend_from_slice(__s));
                r.push(0);
                r
            };
            __tmp_0.with_mut(|__v: &mut Vec<Vec<i8>>| __v.push(__a1))
        }
    };
    assert!(({ ((*v.decay().offset(0_usize).front().upgrade().deref()).len() == 2_usize) }));
    assert!(
        ({
            ((*v.decay().offset(1_usize).front().upgrade().deref()).len() == 3_usize)
                && (match (v.decay().offset(1_usize).front().decay().offset(2_usize),) {
                    (__tmp_1,) => Ptr::<i8>::from_string_literal(b"def").with_c_str(|__s| {
                        (*__tmp_1.upgrade().deref())
                            [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                })
        })
    );
    assert!(
        ({
            match (v.decay().offset(1_usize).back().decay().offset(0_usize),) {
                (__tmp_2,) => Ptr::<i8>::from_string_literal(b"bcx").with_c_str(|__s| {
                    (*__tmp_2.upgrade().deref())
                        [..(*__tmp_2.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                }),
            }
        })
    );
    let copy: Value<Vec<Deque<Vec<Vec<i8>>>>> = Rc::new(RefCell::new((*v.borrow()).clone()));
    match (Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"z").to_c_bytes();
        __bytes.push(0);
        __bytes
    }))
    .as_pointer(),)
    {
        (__tmp_3,) => copy
            .decay()
            .offset(0_usize)
            .front()
            .decay()
            .offset(0_usize)
            .write(__tmp_3.with_mut(|__v| std::mem::replace(__v, vec![0]))),
    };
    match (v.decay().offset(0_usize).front().decay().offset(1_usize),) {
        (__tmp_4,) => {
            if 0_usize >= (*__tmp_4.upgrade().deref()).len().saturating_sub(1) {
                panic!("out of bounds access")
            } else {
                __tmp_4.decay().offset(0_usize as isize)
            }
        }
    }
    .write(('B' as i8));
    assert!(
        ({
            (match (v.decay().offset(0_usize).front().decay().offset(0_usize),) {
                (__tmp_5,) => Ptr::<i8>::from_string_literal(b"a").with_c_str(|__s| {
                    (*__tmp_5.upgrade().deref())
                        [..(*__tmp_5.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                }),
            }) && (match (copy.decay().offset(0_usize).front().decay().offset(0_usize),) {
                (__tmp_6,) => Ptr::<i8>::from_string_literal(b"z").with_c_str(|__s| {
                    (*__tmp_6.upgrade().deref())
                        [..(*__tmp_6.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                }),
            })
        })
    );
    assert!(
        ({
            (match (v.decay().offset(0_usize).front().decay().offset(1_usize),) {
                (__tmp_7,) => Ptr::<i8>::from_string_literal(b"Bc").with_c_str(|__s| {
                    (*__tmp_7.upgrade().deref())
                        [..(*__tmp_7.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                }),
            }) && (match (copy.decay().offset(0_usize).front().decay().offset(1_usize),) {
                (__tmp_8,) => Ptr::<i8>::from_string_literal(b"bc").with_c_str(|__s| {
                    (*__tmp_8.upgrade().deref())
                        [..(*__tmp_8.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                }),
            })
        })
    );
    (*v.borrow_mut())[1_usize].pop_front();
    assert!(
        ({
            match (v.decay().offset(1_usize).front().decay().offset(0_usize),) {
                (__tmp_9,) => Ptr::<i8>::from_string_literal(b"bcx").with_c_str(|__s| {
                    (*__tmp_9.upgrade().deref())
                        [..(*__tmp_9.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                }),
            }
        })
    );
    assert!(({ ((*copy.decay().offset(1_usize).front().upgrade().deref()).len() == 3_usize) }));
    assert!(({ ((*row.borrow()).len() == 3_usize) }));
    let d: Value<Deque<i32>> = Rc::new(RefCell::new(Deque::new()));
    let mut first: i32 = 1;
    let mut last: i32 = 2;
    (*d.borrow_mut()).push_back(first);
    (*d.borrow_mut()).push_back(last);
    let r: Ptr<i32> = d.as_pointer().front();
    r.write(5);
    let c: Ptr<i32> = d.as_pointer().back();
    assert!(((d.as_pointer().front().read()) == 5) && ((c.read()) == 2));
    (*d.borrow_mut()).push_front(first);
    assert!((((r.read()) == 5) && ((c.read()) == 2)) && ((d.as_pointer().front().read()) == 1));
    let b: Ptr<i32> = d.as_pointer().back();
    (*d.borrow_mut()).pop_front();
    assert!(
        ({
            ((((b.read()) == 2) && ((*d.borrow()).len() == 2_usize))
                && ((d.as_pointer().get(0_usize).read()) == 5))
                && ((d.as_pointer().at(1_usize).read()) == 2)
        })
    );
    (*d.borrow_mut()).push_back(3);
    (*d.borrow_mut()).pop_back();
    let mut sum: i32 = 0;
    'loop_: for x in IterRange::new(d.as_pointer().begin(), d.as_pointer().end()) {
        let mut x: i32 = (x.iter_deref().read());
        sum += x;
    }
    {
        let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(d.as_pointer().begin()));
        'loop_: while (*it.borrow()) != d.as_pointer().end() {
            sum += ((*it.borrow()).iter_deref().read());
            {
                it.as_pointer().with_mut(|__it| {
                    __it.iter_inc();
                });
                it.as_pointer()
            };
        }
    }
    assert!((sum == 14));
    let m: Ptr<i32> = d.as_pointer().get(1_usize);
    (*d.borrow_mut()).push_front(5);
    (*d.borrow_mut()).push_front(1);
    (*d.borrow_mut()).pop_front();
    (*d.borrow_mut()).push_back(30);
    assert!(({ ((m.read()) == 2) && ((*d.borrow()).len() == 4_usize) }));
    let s: Ptr<i32> = d.as_pointer().get(2_usize);
    (*d.borrow_mut()).pop_back();
    (*d.borrow_mut()).push_front(9);
    s.write(4);
    assert!(
        ((((m.read()) == 4) && ((d.as_pointer().get(3_usize).read()) == 4))
            && ((d.as_pointer().front().read()) == 9))
            && ((d.as_pointer().back().read()) == 4)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
