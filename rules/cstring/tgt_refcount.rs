// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f2001(a0: Ptr<i8>, a1: i32) -> Ptr<i8> {
    let __s = a0;
    let __t = a1 as i8;
    match __s.to_c_string_iterator().position(|__c| __c == __t) {
        Some(__i) => __s.offset(__i),
        None => {
            if __t == 0 {
                __s.offset(__s.to_c_string_iterator().count())
            } else {
                Ptr::null()
            }
        }
    }
}

fn f2003(a0: Ptr<i8>, a1: i32) -> Ptr<i8> {
    let __s = a0;
    let __t = a1 as i8;
    match __s
        .to_c_string_iterator()
        .enumerate()
        .filter(|__e| __e.1 == __t)
        .last()
    {
        Some((__i, _)) => __s.offset(__i),
        None => {
            if __t == 0 {
                __s.offset(__s.to_c_string_iterator().count())
            } else {
                Ptr::null()
            }
        }
    }
}

fn f2004(a0: Ptr<i8>, a1: i32) -> Ptr<i8> {
    let __s = a0;
    let __t = a1 as i8;
    match __s
        .to_c_string_iterator()
        .enumerate()
        .filter(|__e| __e.1 == __t)
        .last()
    {
        Some((__i, _)) => __s.offset(__i),
        None => {
            if __t == 0 {
                __s.offset(__s.to_c_string_iterator().count())
            } else {
                Ptr::null()
            }
        }
    }
}

fn f2005(a0: Ptr<i8>, a1: Ptr<i8>) -> Ptr<i8> {
    let __s = a0;
    let __set = a1;
    match __s
        .to_c_string_iterator()
        .position(|__c| __set.to_c_string_iterator().any(|__r| __r == __c))
    {
        Some(__i) => __s.offset(__i),
        None => Ptr::null(),
    }
}

fn f2006(a0: Ptr<i8>, a1: Ptr<i8>) -> Ptr<i8> {
    let __s = a0;
    let __set = a1;
    match __s
        .to_c_string_iterator()
        .position(|__c| __set.to_c_string_iterator().any(|__r| __r == __c))
    {
        Some(__i) => __s.offset(__i),
        None => Ptr::null(),
    }
}

fn f2007(a0: Ptr<i8>, a1: Ptr<i8>) -> Ptr<i8> {
    let __needle = a1;
    let mut __p = a0;
    loop {
        let mut __h = __p.to_c_string_iterator();
        if __needle
            .to_c_string_iterator()
            .all(|__c| __h.next() == Some(__c))
        {
            break __p;
        }
        if __p.read() == 0 {
            break Ptr::null();
        }
        __p += 1;
    }
}

fn f2008(a0: Ptr<i8>, a1: Ptr<i8>) -> Ptr<i8> {
    let __needle = a1;
    let mut __p = a0;
    loop {
        let mut __h = __p.to_c_string_iterator();
        if __needle
            .to_c_string_iterator()
            .all(|__c| __h.next() == Some(__c))
        {
            break __p;
        }
        if __p.read() == 0 {
            break Ptr::null();
        }
        __p += 1;
    }
}

fn f2009(a0: AnyPtr, a1: i32, a2: usize) -> AnyPtr {
    let mut __p = a0.reinterpret_cast::<u8>();
    let mut __i: usize = 0;
    loop {
        if __i == a2 {
            break Ptr::<u8>::null().to_any();
        }
        if __p.read() == a1 as u8 {
            break __p.to_any();
        }
        __p += 1;
        __i += 1;
    }
}
