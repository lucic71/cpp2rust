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
pub struct Bar {
    #[offset(0)]
    pub w: i32,
}
impl Bar {
    pub fn copy_from(this: Ptr<Bar>, _a0: Ptr<Bar>) {
        field!(this, w).write(_a0.with(|__s| __s.w));
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct Foo {
    #[offset(0)]
    pub x: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub y: Ptr<i32>,
    #[offset(16)]
    #[byte_size(8)]
    pub z: Ptr<i32>,
    #[offset(24)]
    #[byte_size(12)]
    #[array_len(3)]
    pub a: Box<[i32]>,
    #[offset(36)]
    #[byte_size(4)]
    pub bar: Bar,
}
impl Foo {
    pub fn copy_from(this: Ptr<Foo>, _a0: Ptr<Foo>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
        field!(this, y).write((_a0.with(|__s| __s.y.clone())).clone());
        field!(this, z).write(_a0.with(|__s| __s.z.clone()));
        field!(this, a).write(Box::new(std::array::from_fn::<_, 3, _>(|__i: usize| {
            (*_a0.upgrade().deref()).a[(__i) as usize]
        })));
        Bar::copy_from(field_ptr!(this, bar), { field_ptr!(_a0, bar) });
    }
}
impl Default for Foo {
    fn default() -> Self {
        {
            Foo {
                x: 0_i32,
                y: <Ptr<i32>>::default(),
                z: Ptr::<i32>::null(),
                a: (0..3).map(|_| 0_i32).collect::<Box<[i32]>>(),
                bar: <Bar>::default(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Refs {
    #[offset(0)]
    #[byte_size(8)]
    pub a: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub b: Ptr<i32>,
}
impl Refs {
    pub fn copy_from(this: Ptr<Refs>, _a0: Ptr<Refs>) {
        field!(this, a).write((_a0.with(|__s| __s.a.clone())).clone());
        field!(this, b).write((_a0.with(|__s| __s.b.clone())).clone());
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let x1: Value<i32> = Rc::new(RefCell::new(1));
    let mut x2: i32 = (*x1.borrow());
    x2.prefix_inc();
    assert!(({ ((*x1.borrow()) == 1) }));
    assert!((x2 == 2));
    let mut x3: f64 = 3_f64;
    let mut x4: f64 = x3;
    x4.prefix_inc();
    assert!((x3 == 3_f64));
    assert!((x4 == 4_f64));
    let reference: Ptr<i32> = x1.as_pointer();
    let mut x5: i32 = (reference.read());
    x5.prefix_inc();
    assert!(((reference.read()) == 1));
    assert!((x5 == 2));
    let mut pointer: Ptr<i32> = (x1.as_pointer());
    let mut x6: i32 = (pointer.read());
    x6.prefix_inc();
    assert!(((pointer.read()) == 1));
    assert!((x6 == 2));
    let mut other_pointer: Ptr<i32> = (pointer).clone();
    assert!(({ (other_pointer).clone() } == { (pointer).clone() }));
    other_pointer.with_mut(|__v| {
        __v.prefix_inc();
    });
    assert!(({ (other_pointer.read()) } == { (pointer.read()) }));
    let f1: Value<Foo> = Rc::new(RefCell::new(<Foo>::uninit()));
    {
        let __d = f1.as_pointer();
        field!(__d, x).write(1);
        field!(__d, y).write(x1.as_pointer());
        field!(__d, z).write((x1.as_pointer()));
        field!(__d, a).write(Box::new([0, 1, 2]));
        {
            let __d = field_ptr!(__d, bar);
            field!(__d, w).write(10);
        };
    };
    assert!(({ ({ (*f1.borrow()).x } == 1) }));
    assert!(({ (({ (*f1.borrow()).y.clone() }.read()) == 2) }));
    assert!(({ ({ { (*f1.borrow()).z.clone() } } == { (x1.as_pointer()) }) }));
    assert!(({ (({ (*f1.borrow()).z.clone() }.read()) == 2) }));
    let f2: Value<Foo> = Rc::new(RefCell::new(<Foo>::uninit()));
    Foo::copy_from(f2.as_pointer(), { f1.as_pointer() });
    (*f2.borrow_mut()).x.prefix_inc();
    { (*f2.borrow()).y.clone() }.with_mut(|__v| {
        __v.prefix_inc();
    });
    assert!(({ ({ (*f2.borrow()).x } == 2) }));
    assert!(({ (({ (*f2.borrow()).y.clone() }.read()) == 3) }));
    assert!(({ ({ (*f1.borrow()).x } == 1) }));
    assert!(({ (({ (*f1.borrow()).y.clone() }.read()) == 3) }));
    { (*f2.borrow()).z.clone() }.with_mut(|__v| {
        __v.prefix_inc();
    });
    assert!(({ (({ (*f2.borrow()).y.clone() }.read()) == 4) }));
    assert!(({ ({ { (*f2.borrow()).z.clone() } } == { (x1.as_pointer()) }) }));
    assert!(({ (({ (*f2.borrow()).z.clone() }.read()) == 4) }));
    assert!(({ (({ (*f1.borrow()).y.clone() }.read()) == 4) }));
    assert!(({ ({ { (*f1.borrow()).z.clone() } } == { (x1.as_pointer()) }) }));
    assert!(({ (({ (*f1.borrow()).z.clone() }.read()) == 4) }));
    array_elem!(f2.as_pointer(), a, 0).with_mut(|__v| {
        __v.prefix_inc();
    });
    array_elem!(f2.as_pointer(), a, 1).with_mut(|__v| {
        __v.prefix_inc();
    });
    array_elem!(f2.as_pointer(), a, 2).with_mut(|__v| {
        __v.prefix_inc();
    });
    assert!(((array_elem!(f2.as_pointer(), a, 0).read()) == 1));
    assert!(((array_elem!(f2.as_pointer(), a, 1).read()) == 2));
    assert!(((array_elem!(f2.as_pointer(), a, 2).read()) == 3));
    assert!(((array_elem!(f1.as_pointer(), a, 0).read()) == 0));
    assert!(((array_elem!(f1.as_pointer(), a, 1).read()) == 1));
    assert!(((array_elem!(f1.as_pointer(), a, 2).read()) == 2));
    (*f2.borrow_mut()).bar.w = 20;
    assert!(({ ({ (*f2.borrow()).bar.w } == 20) }));
    assert!(({ ({ (*f1.borrow()).bar.w } == 10) }));
    let mut N: i32 = 5;
    let mut v1: Vec<i32> = Vec::new();
    {
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            {
                let a0_clone = i.clone();
                v1.push(a0_clone)
            };
            i.prefix_inc();
        }
    }
    let mut v2: Vec<i32> = v1.clone();
    {
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            assert!((v2[(i as usize)] == i));
            i.prefix_inc();
        }
    }
    {
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            v2[(i as usize)].prefix_inc();
            i.prefix_inc();
        }
    }
    {
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            assert!((v2[(i as usize)] == (i + 1)));
            assert!((v1[(i as usize)] == i));
            i.prefix_inc();
        }
    }
    let m1: Value<Vec<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
    {
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            {
                let __a1 = Vec::from_fn(10_usize, |_| 0_i32);
                (*m1.borrow_mut()).push(__a1)
            };
            i.prefix_inc();
        }
    }
    let m2: Value<Vec<Vec<i32>>> = Rc::new(RefCell::new((*m1.borrow()).clone()));
    {
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            assert!(({ ((*m1.decay().offset((i as usize)).upgrade().deref()).len() == 10_usize) }));
            assert!(({ ((*m2.decay().offset((i as usize)).upgrade().deref()).len() == 10_usize) }));
            {
                let mut j: i32 = 0;
                'loop_: while (j < 10) {
                    assert!(((index!(elem!(m1.decay(), (i as usize)), (j as usize)).read()) == 0));
                    assert!(((index!(elem!(m2.decay(), (i as usize)), (j as usize)).read()) == 0));
                    j.prefix_inc();
                }
            }
            i.prefix_inc();
        }
    }
    {
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            {
                let mut j: i32 = 0;
                'loop_: while (j < 10) {
                    index!(elem!(m2.decay(), (i as usize)), (j as usize))
                        .with_mut(|__v| __v.postfix_inc());
                    j.prefix_inc();
                }
            }
            i.prefix_inc();
        }
    }
    {
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            assert!(({ ((*m1.decay().offset((i as usize)).upgrade().deref()).len() == 10_usize) }));
            assert!(({ ((*m2.decay().offset((i as usize)).upgrade().deref()).len() == 10_usize) }));
            {
                let mut j: i32 = 0;
                'loop_: while (j < 10) {
                    assert!(((index!(elem!(m1.decay(), (i as usize)), (j as usize)).read()) == 0));
                    assert!(((index!(elem!(m2.decay(), (i as usize)), (j as usize)).read()) == 1));
                    j.prefix_inc();
                }
            }
            i.prefix_inc();
        }
    }
    let map1: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < N) {
            let __rhs = (*i.borrow());
            ({ (*map1.borrow_mut()).subscript(i.as_pointer(), || 0_i32) }).write(__rhs);
            (*i.borrow_mut()).prefix_inc();
        }
    }
    let map2: Value<Map<i32, (Value<i32>, Value<i32>)>> =
        Rc::new(RefCell::new((*map1.borrow()).clone()));
    {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < N) {
            assert!(
                ({
                    ((({ (*map2.borrow_mut()).subscript(i.as_pointer(), || 0_i32) }).read())
                        == (*i.borrow()))
                })
            );
            ({ (*map2.borrow_mut()).subscript(i.as_pointer(), || 0_i32) }).with_mut(|__v| {
                __v.prefix_inc();
            });
            (*i.borrow_mut()).prefix_inc();
        }
    }
    {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < N) {
            assert!(
                ({
                    ((({ (*map1.borrow_mut()).subscript(i.as_pointer(), || 0_i32) }).read())
                        == (*i.borrow()))
                })
            );
            assert!(
                ({
                    ((({ (*map2.borrow_mut()).subscript(i.as_pointer(), || 0_i32) }).read())
                        == ((*i.borrow()) + 1))
                })
            );
            (*i.borrow_mut()).prefix_inc();
        }
    }
    let pair1: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(1)),
        Rc::new(RefCell::new(2)),
    )));
    let pair2: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new((*pair1.borrow()).0.borrow().clone())),
        Rc::new(RefCell::new((*pair1.borrow()).1.borrow().clone())),
    )));
    (*(*pair2.borrow()).0.borrow_mut()) = { ((*(*pair2.borrow()).0.borrow()) * 10) };
    (*(*pair2.borrow()).1.borrow_mut()) = { ((*(*pair2.borrow()).1.borrow()) * 10) };
    assert!(({ ((*(*pair2.borrow()).0.borrow()) == 10) }));
    assert!(({ ((*(*pair2.borrow()).1.borrow()) == 20) }));
    assert!(({ ((*(*pair1.borrow()).0.borrow()) == 1) }));
    assert!(({ ((*(*pair1.borrow()).1.borrow()) == 2) }));
    let pair3: Value<(Value<Vec<i32>>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(Vec::from_fn(0_usize, |_| 0_i32))),
        Rc::new(RefCell::new(0)),
    )));
    let pair4: Value<(Value<Vec<i32>>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new((*pair3.borrow()).0.borrow().clone())),
        Rc::new(RefCell::new((*pair3.borrow()).1.borrow().clone())),
    )));
    {
        let __a1 = 1;
        (*(*pair4.borrow()).0.borrow_mut()).push(__a1)
    };
    (*(*pair4.borrow()).1.borrow_mut()) = 1;
    assert!(({ ((*(*pair4.borrow()).0.borrow()).len() == 1_usize) }));
    assert!(({ ((*(*pair4.borrow()).1.borrow()) == 1) }));
    assert!(({ ((*(*pair3.borrow()).0.borrow()).len() == 0_usize) }));
    assert!(({ ((*(*pair3.borrow()).1.borrow()) == 0) }));
    let mut s1: Vec<i8> = vec![('a' as i8); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0))
        .collect();
    let mut s2: Vec<i8> = (s1).clone();
    s2[0_usize] = ('b' as i8);
    s2[1_usize] = ('b' as i8);
    s2[2_usize] = ('b' as i8);
    assert!(((s2[0_usize] as i32) == (('b' as i8) as i32)));
    assert!(((s2[1_usize] as i32) == (('b' as i8) as i32)));
    assert!(((s2[2_usize] as i32) == (('b' as i8) as i32)));
    assert!(((s1[0_usize] as i32) == (('a' as i8) as i32)));
    assert!(((s1[1_usize] as i32) == (('a' as i8) as i32)));
    assert!(((s1[2_usize] as i32) == (('a' as i8) as i32)));
    let b1: Value<Bar> = Rc::new(RefCell::new(<Bar>::uninit()));
    {
        let __d = b1.as_pointer();
        field!(__d, w).write(1);
    };
    let b2: Value<Bar> = Rc::new(RefCell::new(<Bar>::uninit()));
    {
        let __d = b2.as_pointer();
        field!(__d, w).write(2);
    };
    (*b2.borrow_mut()) = (*b1.borrow());
    (*b2.borrow_mut()).w.postfix_inc();
    assert!(({ ({ (*b1.borrow()).w } == 1) }));
    assert!(({ ({ (*b2.borrow()).w } == 2) }));
    let mut v4: Vec<i32> = Vec::new();
    {
        v4 = (v2).clone();
    };
    {
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            assert!((v4[(i as usize)] == (i + 1)));
            v4[(i as usize)].prefix_inc();
            i.prefix_inc();
        }
    }
    {
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            assert!((v4[(i as usize)] == (i + 2)));
            assert!((v2[(i as usize)] == (i + 1)));
            i.prefix_inc();
        }
    }
    let ra: Value<i32> = Rc::new(RefCell::new(1));
    let rb: Value<i32> = Rc::new(RefCell::new(2));
    let r1: Value<Refs> = Rc::new(RefCell::new(<Refs>::uninit()));
    {
        let __d = r1.as_pointer();
        field!(__d, a).write(ra.as_pointer());
        field!(__d, b).write(rb.as_pointer());
    };
    let r2: Value<Refs> = Rc::new(RefCell::new(<Refs>::uninit()));
    Refs::copy_from(r2.as_pointer(), { r1.as_pointer() });
    { (*r2.borrow()).a.clone() }.write(10);
    { (*r2.borrow()).b.clone() }.with_mut(|__v| {
        __v.prefix_inc();
    });
    assert!(({ ((*ra.borrow()) == 10) }));
    assert!(({ ((*rb.borrow()) == 3) }));
    assert!(({ (({ (*r1.borrow()).a.clone() }.read()) == 10) }));
    assert!(({ (({ (*r1.borrow()).b.clone() }.read()) == 3) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
