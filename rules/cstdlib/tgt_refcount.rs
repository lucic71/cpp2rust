// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f2(a0: AnyPtr) {
    libcc2rs::free_refcount(a0.clone())
}

fn f3(a0: usize) -> AnyPtr {
    libcc2rs::malloc_refcount(a0)
}

fn f4(a0: AnyPtr, a1: usize) -> AnyPtr {
    libcc2rs::realloc_refcount(a0, a1)
}

fn f5(a0: usize, a1: usize) -> AnyPtr {
    libcc2rs::calloc_refcount(a0, a1)
}

fn f6(a0: Ptr<u8>) -> Ptr<u8> {
    match ::std::env::var(a0.to_rust_string()) {
        Ok(__val) => {
            let mut __bytes = __val.into_bytes();
            __bytes.push(0);
            Ptr::alloc_array(__bytes.into_boxed_slice())
        }
        Err(_) => Ptr::null(),
    }
}

fn f10(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<u8> {
    let __resolved = a1.clone();
    match ::std::fs::canonicalize(a0.to_rust_string()) {
        Ok(__p) => {
            let mut __bytes = __p.into_os_string().into_encoded_bytes();
            __bytes.push(0);
            if __resolved.is_null() {
                Ptr::alloc_array(__bytes.into_boxed_slice())
            } else {
                __resolved.with_slice_mut(__bytes.len(), |__s| __s.copy_from_slice(&__bytes));
                __resolved
            }
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EIO));
            Ptr::null()
        }
    }
}

fn f8(a0: AnyPtr, a1: AnyPtr, a2: usize, a3: usize, a4: fn(AnyPtr, AnyPtr) -> i32) -> AnyPtr {
    let __base = a1.reinterpret_cast::<u8>();
    let mut __lo: isize = 0;
    let mut __hi: isize = a2 as isize - 1;
    let mut __found = AnyPtr::default();
    while __lo <= __hi && __found.is_null() {
        let __mid = __lo + (__hi - __lo) / 2;
        let __elem = __base.offset(__mid as usize * a3);
        let __r = a4(a0.clone(), __elem.to_any());
        if __r == 0 {
            __found = __elem.to_any();
        } else if __r < 0 {
            __hi = __mid - 1;
        } else {
            __lo = __mid + 1;
        }
    }
    __found
}

fn f9(a0: AnyPtr, a1: usize, a2: usize, a3: fn(AnyPtr, AnyPtr) -> i32) {
    let __base = a0.reinterpret_cast::<u8>();
    let __size = a2;
    let mut __x = vec![0u8; __size];
    let mut __y = vec![0u8; __size];
    for __i in 0..a1 {
        let mut __min = __i;
        for __j in (__i + 1)..a1 {
            if a3(
                __base.offset(__j * __size).to_any(),
                __base.offset(__min * __size).to_any(),
            ) < 0
            {
                __min = __j;
            }
        }
        if __min != __i {
            __base
                .offset(__i * __size)
                .with_slice(__size, |__s| __x.copy_from_slice(__s));
            __base
                .offset(__min * __size)
                .with_slice(__size, |__s| __y.copy_from_slice(__s));
            __base
                .offset(__i * __size)
                .with_slice_mut(__size, |__d| __d.copy_from_slice(&__y));
            __base
                .offset(__min * __size)
                .with_slice_mut(__size, |__d| __d.copy_from_slice(&__x));
        }
    }
}

fn f12(a0: Ptr<u8>) -> i32 {
    let __s = a0.to_rust_string();
    let __num: String = __s
        .trim_start()
        .chars()
        .enumerate()
        .take_while(|(__i, __c)| {
            __c.is_ascii_digit() || (*__i == 0 && (*__c == '-' || *__c == '+'))
        })
        .map(|(_, __c)| __c)
        .collect();
    __num
        .parse::<i64>()
        .map_or(0, |__v| __v.clamp(i32::MIN as i64, i32::MAX as i64) as i32)
}

fn f15(a0: FnPtr<fn()>) -> i32 {
    libcc2rs::atexit_refcount(a0.clone())
}

fn f11(a0: i32) {
    libcc2rs::exit_refcount(a0);
}

fn f13(a0: Ptr<u8>, a1: Ptr<Ptr<u8>>) -> f64 {
    libcc2rs::strtod_refcount(a0.clone(), a1.clone())
}

fn f14(a0: Ptr<u8>, a1: Ptr<Ptr<u8>>, a2: i32) -> i64 {
    libcc2rs::strtoll_refcount(a0.clone(), a1.clone(), a2)
}
