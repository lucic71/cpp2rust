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
    libcc2rs::realloc_refcount(a0.clone(), a1)
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
        Err(_) => Ptr::<u8>::null(),
    }
}

fn f7(a0: Ptr<u8>, a1: Ptr<u8>, a2: i32) -> i32 {
    match a2 != 0 || ::std::env::var_os(a0.to_rust_string()).is_none() {
        true => {
            unsafe { ::std::env::set_var(a0.to_rust_string(), a1.to_rust_string()) };
            0
        }
        false => 0,
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
            Ptr::<u8>::null()
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
    let __nptr = a0.clone();
    let __endptr = a1.clone();
    libcc2rs::strtod_refcount(__nptr, __endptr)
}

fn f14(a0: Ptr<u8>, a1: Ptr<Ptr<u8>>, a2: i32) -> i64 {
    let __nptr = a0.clone();
    let __endptr = a1.clone();
    let __base = a2;
    libcc2rs::strtoll_refcount(__nptr, __endptr, __base)
}

fn f37(a0: Ptr<u8>, a1: Ptr<Ptr<u8>>, a2: i32) -> i64 {
    let __nptr = a0.clone();
    let __endptr = a1.clone();
    let __base = a2;
    libcc2rs::strtoll_refcount(__nptr, __endptr, __base)
}

fn f38(a0: Ptr<u8>, a1: Ptr<Ptr<u8>>, a2: i32) -> u64 {
    let __nptr = a0.clone();
    let __endptr = a1.clone();
    let __base = a2;
    libcc2rs::strtoul_refcount(__nptr, __endptr, __base)
}

fn f39(a0: Ptr<u8>) -> i32 {
    let __tmpl = a0.clone();
    let mut __name = __tmpl.to_rust_string();
    match __name.ends_with("XXXXXX") {
        false => {
            libcc2rs::cpp2rust_errno().write(::libc::EINVAL);
            -1
        }
        true => {
            let __base = __name.len() - 6;
            let mut __seed = ::std::time::SystemTime::now()
                .duration_since(::std::time::UNIX_EPOCH)
                .map(|__d| __d.as_nanos() as u64)
                .unwrap_or(0)
                ^ ((::std::process::id() as u64) << 32);
            let mut __attempt = 0;
            let mut __fd = -1;
            while __attempt < 100 && __fd < 0 {
                let mut __n = __seed;
                __name.truncate(__base);
                let mut __i = 0;
                while __i < 6 {
                    __name.push(char::from_digit((__n % 36) as u32, 36).unwrap());
                    __n /= 36;
                    __i += 1;
                }
                match nix::fcntl::open(
                    __name.as_str(),
                    nix::fcntl::OFlag::O_CREAT
                        | nix::fcntl::OFlag::O_EXCL
                        | nix::fcntl::OFlag::O_RDWR,
                    nix::sys::stat::Mode::S_IRUSR | nix::sys::stat::Mode::S_IWUSR,
                ) {
                    Ok(__ofd) => {
                        __tmpl.with_slice_mut(__name.len(), |__s| {
                            __s.copy_from_slice(__name.as_bytes())
                        });
                        __fd = FdRegistry::register(__ofd);
                    }
                    Err(__e) => match __e == nix::errno::Errno::EEXIST {
                        true => {
                            __seed = __seed
                                .wrapping_mul(6364136223846793005)
                                .wrapping_add(__attempt + 1);
                        }
                        false => {
                            libcc2rs::cpp2rust_errno().write(__e as i32);
                            __attempt = 100;
                        }
                    },
                }
                __attempt += 1;
            }
            match __fd < 0 && __attempt < 101 {
                true => {
                    libcc2rs::cpp2rust_errno().write(::libc::EEXIST);
                    -1
                }
                false => __fd,
            }
        }
    }
}

fn f40(a0: Ptr<u8>) -> Ptr<u8> {
    let __tmpl = a0.clone();
    let mut __name = __tmpl.to_rust_string();
    match __name.ends_with("XXXXXX") {
        false => {
            libcc2rs::cpp2rust_errno().write(::libc::EINVAL);
            Ptr::<u8>::null()
        }
        true => {
            let __base = __name.len() - 6;
            let mut __seed = ::std::time::SystemTime::now()
                .duration_since(::std::time::UNIX_EPOCH)
                .map(|__d| __d.as_nanos() as u64)
                .unwrap_or(0)
                ^ ((::std::process::id() as u64) << 32);
            let mut __attempt = 0;
            let mut __ret = Ptr::<u8>::null();
            while __attempt < 100 && __ret.is_null() {
                let mut __n = __seed;
                __name.truncate(__base);
                let mut __i = 0;
                while __i < 6 {
                    __name.push(char::from_digit((__n % 36) as u32, 36).unwrap());
                    __n /= 36;
                    __i += 1;
                }
                match ::std::fs::create_dir(&__name) {
                    Ok(()) => {
                        __tmpl.with_slice_mut(__name.len(), |__s| {
                            __s.copy_from_slice(__name.as_bytes())
                        });
                        __ret = __tmpl.clone();
                    }
                    Err(__e) => match __e.kind() == ::std::io::ErrorKind::AlreadyExists {
                        true => {
                            __seed = __seed
                                .wrapping_mul(6364136223846793005)
                                .wrapping_add(__attempt + 1);
                        }
                        false => {
                            libcc2rs::cpp2rust_errno()
                                .write(__e.raw_os_error().unwrap_or(::libc::EIO));
                            __attempt = 100;
                        }
                    },
                }
                __attempt += 1;
            }
            match __ret.is_null() && __attempt < 101 {
                true => {
                    libcc2rs::cpp2rust_errno().write(::libc::EEXIST);
                    Ptr::<u8>::null()
                }
                false => __ret,
            }
        }
    }
}

fn f41(a0: Ptr<u8>) -> i32 {
    unsafe { ::std::env::remove_var(a0.to_rust_string()) };
    0
}
