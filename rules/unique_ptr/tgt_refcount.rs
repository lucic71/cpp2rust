// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn f2<T1>(init: T1) -> Option<Value<T1>> {
    Some(Rc::new(RefCell::new(init)))
}

fn f23<T1>(a0: &mut Option<Value<Box<[T1]>>>) -> Option<Value<Box<[T1]>>> {
    a0.take()
}

fn f24<T1: ByteRepr>(a0: Ptr<Option<Value<Box<[T1]>>>>, a1: &mut Option<Value<Box<[T1]>>>) {
    a0.write(a1.take())
}

fn f26<T1>(a0: Value<T1>, a1: usize) -> Ptr<T1> {
    (a0.as_pointer() as Ptr<T1>).offset(a1 as isize)
}

fn f27<T1>(a0: Value<T1>) -> Ptr<T1> {
    a0.as_pointer()
}

fn f34<T1>() -> Option<Value<Box<[T1]>>> {
    None
}

fn f35<T1>(a0: Ptr<T1>) -> Option<Value<T1>> {
    a0.to_owned_opt()
}

fn f41<T1: ByteRepr>(a0: Ptr<Option<Value<Box<[T1]>>>>, a1: Ptr<T1>) {
    a0.write(a1.to_owned_opt())
}

fn f42<T1>(a0: &mut Option<Value<T1>>) -> Option<Value<T1>> {
    a0.take()
}

fn f43<T1: ByteRepr>(a0: Ptr<Option<Value<T1>>>, a1: &mut Option<Value<T1>>) {
    a0.write(a1.take())
}

fn f45<T1>(a0: Value<T1>) -> Ptr<T1> {
    a0.as_pointer()
}

fn f46<T1>(a0: Value<T1>) -> Ptr<T1> {
    a0.as_pointer()
}

fn f47<T1>(a0: Value<T1>) -> Ptr<T1> {
    a0.as_pointer()
}

fn f52<T1: ByteRepr>(a0: Ptr<Option<Value<T1>>>, a1: Ptr<T1>) {
    let _p: Ptr<_> = a1;
    a0.write(_p.to_owned_opt())
}

fn f54<T1>() -> Option<Value<T1>> {
    None
}

fn f55<T1>(a0: Ptr<T1>) -> Option<Value<T1>> {
    a0.to_owned_opt()
}

fn t1<T1>() -> Option<Value<Box<[T1]>>> {
    None
}

fn t2<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t5<T1>() -> Option<Value<T1>> {
    None
}

fn t6<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn f14<T1, T2>(a0: Option<Value<T1>>, a1: AnyPtr) -> bool {
    a0.is_none()
}
