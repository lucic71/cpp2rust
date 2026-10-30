extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestAllocator_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestAllocator_double_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestAllocator_unsigned_long_ {}
pub fn copy_0(mut copy_vector: ()) {}
pub fn fn_1(v: Ptr<()>, v3: ()) {
    let v3: Value<()> = Rc::new(RefCell::new(v3));
    {
        let __a1 = 20_();
        v.with_mut(|__v: &mut Vec<()>| __v.push(__a1))
    };
    let mut x: () = unimplemented!();
    let mut v4: Ptr<()> = (v3.as_pointer());
    let mut v2: () = Vec::new();
    {
        let __a1 = 0_();
        v2.push(__a1)
    };
    {
        let __a1 = 1_();
        v2.push(__a1)
    };
    {
        let __a1 = 3_();
        v2.push(__a1)
    };
    x = (unimplemented!().read());
    unimplemented!().write(1_());
    unimplemented!().write(7_());
    unimplemented!().write(13_());
    ({ cpp2rust_assert_fail_2((x == 6_())) });
    ({ cpp2rust_assert_fail_2(((v.read()) == 4_())) });
    ({ cpp2rust_assert_fail_2(((unimplemented!().read()) == 5_())) });
    ({ cpp2rust_assert_fail_2(((unimplemented!().read()) == 6_())) });
    ({ cpp2rust_assert_fail_2(((v.to_last().read()) == 20_())) });
    ({ cpp2rust_assert_fail_2(((unimplemented!().read()) == 7_())) });
    ({ cpp2rust_assert_fail_2(((unimplemented!().read()) == 13_())) });
    {
        let __a1 = 20_();
        v.with_mut(|__v: &mut Vec<()>| __v.push(__a1))
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut v1: () = Vec::new();
    ({ cpp2rust_assert_fail_2((v1.len() == 0_())) });
    ({ cpp2rust_assert_fail_2(v1.is_empty()) });
    {
        let __a1 = 1_();
        v1.push(__a1)
    };
    ({ cpp2rust_assert_fail_2(!(v1.is_empty())) });
    v1.pop();
    ({ cpp2rust_assert_fail_2(v1.is_empty()) });
    let mut s1: () = v1.len();
    {
        let __a0 = 100_() as usize;
        v1.resize_with(__a0, || unimplemented!())
    };
    ({ cpp2rust_assert_fail_2((v1.len() == 100_())) });
    ({ cpp2rust_assert_fail_2(((unimplemented!().read()) == 0_())) });
    unimplemented!().write(40_());
    unimplemented!().write(50_());
    ({ cpp2rust_assert_fail_2(((unimplemented!().read()) == 40_())) });
    ({ cpp2rust_assert_fail_2(((unimplemented!().read()) == 50_())) });
    let v2: Value<()> = Rc::new(RefCell::new(Vec::new()));
    ({ cpp2rust_assert_fail_2(((*v2.borrow()).len() == 0_())) });
    {
        let __a1 = 1_();
        (*v2.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 2_();
        (*v2.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 3_();
        (*v2.borrow_mut()).push(__a1)
    };
    ({ cpp2rust_assert_fail_2(((*v2.borrow()).len() == 3_())) });
    {
        let __it = unimplemented!();
        let __off = __it.index();
        v2.as_pointer()
            .with_mut(|__v: &mut Vec<()>| __v.remove(__off));
        __it
    };
    ({ cpp2rust_assert_fail_2(((*v2.borrow()).len() == 2_())) });
    ({ cpp2rust_assert_fail_2(((unimplemented!().read()) == 2_())) });
    ({ cpp2rust_assert_fail_2(((unimplemented!().read()) == 3_())) });
    {
        let __it = unimplemented!();
        let __off = __it.index();
        (*v2.borrow_mut()).insert(__off, 100_());
        __it
    };
    ({ copy_0((*v2.borrow()).clone()) });
    ({ cpp2rust_assert_fail_2(((*v2.borrow()).len() == 3_())) });
    ({ cpp2rust_assert_fail_2(((unimplemented!().read()) == 100_())) });
    ({ cpp2rust_assert_fail_2(((unimplemented!().read()) == 2_())) });
    ({ cpp2rust_assert_fail_2(((unimplemented!().read()) == 3_())) });
    let mut s2: () = (*v2.borrow()).len();
    let v3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ cpp2rust_assert_fail_2(((*v3.borrow()).len() == 100_())) });
    let mut i: () = 0_();
    'loop_: while (i < 100_()) {
        ({ cpp2rust_assert_fail_2(((unimplemented!().read()) == 1_())) });
        i.prefix_inc();
    }
    let v6: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ cpp2rust_assert_fail_2(((*v6.borrow()).len() == s2)) });
    let mut i: () = 0_();
    'loop_: while (i < s2) {
        ({ cpp2rust_assert_fail_2(((unimplemented!().read()) == 2_f64)) });
        i.prefix_inc();
    }
    let mut p1: Ptr<()> = v6.as_pointer();
    ({ cpp2rust_assert_fail_2(((p1.read()) == 2_f64)) });
    let mut p2: Ptr<()> = v3.as_pointer();
    ({ cpp2rust_assert_fail_2(((p2.read()) == 1_())) });
    ({ cpp2rust_assert_fail_2(((unimplemented!().read()) == 1_())) });
    ({ cpp2rust_assert_fail_2(((unimplemented!().read()) == 1_())) });
    p2.write(99_f64);
    ({ cpp2rust_assert_fail_2(((p2.read()) == 99_())) });
    ({ cpp2rust_assert_fail_2(((unimplemented!().read()) == 99_())) });
    ({ cpp2rust_assert_fail_2(((unimplemented!().read()) == 1_())) });
    p2.prefix_inc();
    p2.write(98_());
    ({ cpp2rust_assert_fail_2(((unimplemented!().read()) == 99_())) });
    ({ cpp2rust_assert_fail_2(((unimplemented!().read()) == 98_())) });
    ({ cpp2rust_assert_fail_2(((*v3.borrow()).capacity() == 100_())) });
    ({ cpp2rust_assert_fail_2(((*v3.borrow()).len() == 100_())) });
    if 200_() as usize > (*v3.borrow()).capacity() as usize {
        let len_0 = (*v3.borrow()).len();
        (*v3.borrow_mut()).reserve_exact(200_() as usize - len_0 as usize);
    };
    ({ cpp2rust_assert_fail_2(((*v3.borrow()).capacity() == 200_())) });
    ({ cpp2rust_assert_fail_2(((*v3.borrow()).len() == 100_())) });
    if 50_() as usize > (*v3.borrow()).capacity() as usize {
        let len_0 = (*v3.borrow()).len();
        (*v3.borrow_mut()).reserve_exact(50_() as usize - len_0 as usize);
    };
    ({ cpp2rust_assert_fail_2(((*v3.borrow()).capacity() == 200_())) });
    ({ cpp2rust_assert_fail_2(((*v3.borrow()).len() == 100_())) });
    if 200_() as usize > (*v3.borrow()).capacity() as usize {
        let len_0 = (*v3.borrow()).len();
        (*v3.borrow_mut()).reserve_exact(200_() as usize - len_0 as usize);
    };
    ({ cpp2rust_assert_fail_2(((*v3.borrow()).capacity() == 200_())) });
    ({ cpp2rust_assert_fail_2(((*v3.borrow()).len() == 100_())) });
    if 201_() as usize > (*v3.borrow()).capacity() as usize {
        let len_0 = (*v3.borrow()).len();
        (*v3.borrow_mut()).reserve_exact(201_() as usize - len_0 as usize);
    };
    ({ cpp2rust_assert_fail_2(((*v3.borrow()).capacity() == 201_())) });
    ({ cpp2rust_assert_fail_2(((*v3.borrow()).len() == 100_())) });
    ({ cpp2rust_assert_fail_2(((v2.as_pointer().to_last().read()) == 3_())) });
    ({ cpp2rust_assert_fail_2(((v3.as_pointer().to_last().read()) == 1_())) });
    ({ cpp2rust_assert_fail_2(((v6.as_pointer().to_last().read()) == 2_f64)) });
    let ref0: Ptr<()> = v6.as_pointer().to_last();
    ref0.write(5_f64);
    ({ cpp2rust_assert_fail_2(((v6.as_pointer().to_last().read()) == 5_f64)) });
    let mut x0: () = (v6.as_pointer().to_last().read());
    ({ cpp2rust_assert_fail_2((x0 == 5_f64)) });
    x0 = 6_f64;
    ({ cpp2rust_assert_fail_2(((v6.as_pointer().to_last().read()) == 5_f64)) });
    let mut idx: () = 0_();
    ({ cpp2rust_assert_fail_2(((v6.as_pointer().offset(idx as isize).read()) == 2_f64)) });
    ({
        cpp2rust_assert_fail_2(
            ((v6.as_pointer()
                .offset((s2).wrapping_sub(1_()) as isize)
                .read())
                == 5_f64),
        )
    });
    let ref1: Ptr<()> = v6.as_pointer().offset((s2).wrapping_sub(1_()) as isize);
    {
        ref1.with_mut(|__v| *__v = *__v + 1.5_f64)
    };
    ({
        cpp2rust_assert_fail_2(
            ((v6.as_pointer()
                .offset((s2).wrapping_sub(1_()) as isize)
                .read())
                == 6.5_f64),
        )
    });
    let mut x1: () = (v6
        .as_pointer()
        .offset((s2).wrapping_sub(1_()) as isize)
        .read());
    ({ cpp2rust_assert_fail_2((x1 == 6.5_f64)) });
    x1 -= 1.5_f64;
    ({
        cpp2rust_assert_fail_2(
            ((v6.as_pointer()
                .offset((s2).wrapping_sub(1_()) as isize)
                .read())
                == 6.5_f64),
        )
    });
    let v7: Value<()> = Rc::new(RefCell::new(Vec::new()));
    let mut v8: () = Vec::new();
    {
        let __a1 = 4_();
        (*v7.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 5_();
        (*v7.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 6_();
        (*v7.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 8_();
        v8.push(__a1)
    };
    {
        let __a1 = 9_();
        v8.push(__a1)
    };
    ({ fn_1(v7.as_pointer(), v8.clone()) });
    let src: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    let mut v9: () = unimplemented!();
    ({ cpp2rust_assert_fail_2((v9.len() == 3_())) });
    ({
        cpp2rust_assert_fail_2(
            (((unimplemented!().read()) == 1_()) && ((unimplemented!().read()) == 2_()))
                && ((unimplemented!().read()) == 3_()),
        )
    });
    let mut v10: () = unimplemented!();
    ({ cpp2rust_assert_fail_2((v10.len() == 3_())) });
    ({
        cpp2rust_assert_fail_2(
            (((unimplemented!().read()) == 1_()) && ((unimplemented!().read()) == 2_()))
                && ((unimplemented!().read()) == 3_()),
        )
    });
    let mut v11: () = unimplemented!();
    ({ cpp2rust_assert_fail_2((v11.len() == 3_())) });
    ({
        cpp2rust_assert_fail_2(
            (((unimplemented!().read()) == 1_()) && ((unimplemented!().read()) == 2_()))
                && ((unimplemented!().read()) == 3_()),
        )
    });
    let src1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    let mut v12: () = unimplemented!();
    ({ cpp2rust_assert_fail_2((v12.len() == 3_())) });
    ({
        cpp2rust_assert_fail_2(
            (((unimplemented!().read()) == 1_()) && ((unimplemented!().read()) == 2_()))
                && ((unimplemented!().read()) == 3_()),
        )
    });
    let buf: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([10_(), 20_(), 30_(), 40_(), 50_()])));
    let mut start: Ptr<()> = (buf.as_pointer() as Ptr<()>);
    let mut len: () = 5_();
    let mut v13: () = unimplemented!();
    ({ cpp2rust_assert_fail_2((v13.len() == 5_())) });
    ({
        cpp2rust_assert_fail_2(
            ((unimplemented!().read()) == 10_()) && ((unimplemented!().read()) == 50_()),
        )
    });
    ({
        cpp2rust_assert_fail_2(
            (((s1).wrapping_add(s2))
                .wrapping_add(((v2.as_pointer().offset(0_() as isize).read()) as ()))
                == 103_()),
        )
    });
    return 0_();
}
pub trait TestAllocator_double_Impl {
    fn allocate(&self, n: usize) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, _a1: usize);
}
impl TestAllocator_double_Impl for Ptr<TestAllocator_double_> {
    fn allocate(&self, mut n: usize) -> Ptr<()> {
        return Ptr::alloc_array((0..n).map(|_| unimplemented!()).collect::<Box<[()]>>());
    }
    fn deallocate(&self, mut p: Ptr<()>, mut _a1: usize) {
        p.delete();
    }
}
pub trait TestAllocator_int_Impl {
    fn allocate(&self, n: usize) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, _a1: usize);
}
impl TestAllocator_int_Impl for Ptr<TestAllocator_int_> {
    fn allocate(&self, mut n: usize) -> Ptr<()> {
        return Ptr::alloc_array((0..n).map(|_| unimplemented!()).collect::<Box<[()]>>());
    }
    fn deallocate(&self, mut p: Ptr<()>, mut _a1: usize) {
        p.delete();
    }
}
pub trait TestAllocator_unsigned_long_Impl {
    fn allocate(&self, mut n: usize) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut _a1: usize) {
        unimplemented!()
    }
}
impl TestAllocator_unsigned_long_Impl for Ptr<TestAllocator_unsigned_long_> {}
pub fn __cpp2rust_init_globals() {}
