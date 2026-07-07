// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1(a0: AnyPtr, a1: AnyPtr, a2: usize) -> AnyPtr {
    a0.memcpy(&a1, a2 as usize);
    a0.clone()
}

fn f2(a0: AnyPtr, a1: u8, a2: usize) -> AnyPtr {
    a0.memset((a1) as u8, a2 as usize);
    a0.clone()
}

fn f3(a0: AnyPtr, a1: AnyPtr, a2: usize) -> i32 {
    a0.memcmp(&a1, a2)
}

fn f4(a0: AnyPtr, a1: AnyPtr, a2: usize) -> AnyPtr {
    let __tmp: Vec<u8> = (0..a2)
        .map(|__i| a1.reinterpret_cast::<u8>().offset(__i).read())
        .collect();
    for __i in 0..a2 {
        a0.reinterpret_cast::<u8>().offset(__i).write(__tmp[__i]);
    }
    a0.clone()
}

fn f5(a0: Ptr<u8>, a1: i32) -> Ptr<u8> {
    let mut __i: usize = 0;
    loop {
        let __c = a0.offset(__i).read();
        if __c == a1 as u8 {
            break a0.offset(__i);
        }
        if __c == 0 {
            break Ptr::null();
        }
        __i += 1;
    }
}

unsafe fn f7(a0: Ptr<u8>) -> usize {
    let mut __i: usize = 0;
    while a0.offset(__i).read() != 0 {
        __i += 1;
    }
    __i
}

fn f8(a0: Ptr<u8>, a1: Ptr<u8>) -> i32 {
    let mut __i: usize = 0;
    loop {
        let __c1 = a0.offset(__i).read();
        let __c2 = a1.offset(__i).read();
        if __c1 != __c2 {
            break (__c1 as i32) - (__c2 as i32);
        }
        if __c1 == 0 {
            break 0;
        }
        __i += 1;
    }
}

fn f9(a0: Ptr<u8>, a1: Ptr<u8>, a2: usize) -> i32 {
    let mut __i: usize = 0;
    loop {
        if __i == a2 {
            break 0;
        }
        let __c1 = a0.offset(__i).read();
        let __c2 = a1.offset(__i).read();
        if __c1 != __c2 {
            break (__c1 as i32) - (__c2 as i32);
        }
        if __c1 == 0 {
            break 0;
        }
        __i += 1;
    }
}

fn f10(a0: AnyPtr, a1: i32, a2: usize) -> AnyPtr {
    match (0..a2).find(|&__i| a0.reinterpret_cast::<u8>().offset(__i).read() == a1 as u8) {
        Some(__i) => a0.reinterpret_cast::<u8>().offset(__i).to_any(),
        None => Ptr::<u8>::null().to_any(),
    }
}

fn f11(a0: Ptr<u8>, a1: i32) -> Ptr<u8> {
    let mut __i: usize = 0;
    let mut __found = Ptr::null();
    loop {
        let __c = a0.offset(__i).read();
        if __c == a1 as u8 {
            __found = a0.offset(__i);
        }
        if __c == 0 {
            break __found;
        }
        __i += 1;
    }
}

fn f15(a0: Ptr<u8>) -> Ptr<u8> {
    libcc2rs::strdup_refcount(a0.clone())
}

fn f16(a0: Ptr<u8>, a1: Ptr<u8>) -> usize {
    let mut __i: usize = 0;
    loop {
        let __c = a0.offset(__i).read();
        if __c == 0 {
            break __i;
        }
        let mut __j: usize = 0;
        let __hit = loop {
            let __r = a1.offset(__j).read();
            if __r == 0 {
                break false;
            }
            if __r == __c {
                break true;
            }
            __j += 1;
        };
        if __hit {
            break __i;
        }
        __i += 1;
    }
}

fn f17(a0: Ptr<u8>, a1: Ptr<u8>) -> usize {
    let mut __i: usize = 0;
    loop {
        let __c = a0.offset(__i).read();
        if __c == 0 {
            break __i;
        }
        let mut __j: usize = 0;
        let __hit = loop {
            let __r = a1.offset(__j).read();
            if __r == 0 {
                break false;
            }
            if __r == __c {
                break true;
            }
            __j += 1;
        };
        if !__hit {
            break __i;
        }
        __i += 1;
    }
}

fn f18(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<u8> {
    let mut __s: usize = 0;
    loop {
        let mut __i: usize = 0;
        let __matched = loop {
            let __n = a1.offset(__i).read();
            if __n == 0 {
                break true;
            }
            if a0.offset(__s + __i).read() != __n {
                break false;
            }
            __i += 1;
        };
        if __matched {
            break a0.offset(__s);
        }
        if a0.offset(__s).read() == 0 {
            break Ptr::null();
        }
        __s += 1;
    }
}

fn f21(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<u8> {
    let mut __i: usize = 0;
    loop {
        let __c = a0.offset(__i).read();
        if __c == 0 {
            break Ptr::null();
        }
        let mut __j: usize = 0;
        let __hit = loop {
            let __r = a1.offset(__j).read();
            if __r == 0 {
                break false;
            }
            if __r == __c {
                break true;
            }
            __j += 1;
        };
        if __hit {
            break a0.offset(__i);
        }
        __i += 1;
    }
}

#[cfg(target_os = "linux")]
fn f24(a0: AnyPtr, a1: i32, a2: usize) -> AnyPtr {
    match (0..a2)
        .rev()
        .find(|&__i| a0.reinterpret_cast::<u8>().offset(__i).read() == a1 as u8)
    {
        Some(__i) => a0.reinterpret_cast::<u8>().offset(__i).to_any(),
        None => Ptr::<u8>::null().to_any(),
    }
}

fn f27(a0: Ptr<u8>, a1: Ptr<u8>) -> i32 {
    let mut __i: usize = 0;
    loop {
        let __c1 = a0.offset(__i).read().to_ascii_lowercase();
        let __c2 = a1.offset(__i).read().to_ascii_lowercase();
        if __c1 != __c2 {
            break (__c1 as i32) - (__c2 as i32);
        }
        if __c1 == 0 {
            break 0;
        }
        __i += 1;
    }
}

#[cfg(target_os = "linux")]
fn f28(a0: i32, a1: Ptr<u8>, a2: usize) -> Ptr<u8> {
    let __msg = std::io::Error::from_raw_os_error(a0).to_string();
    let __len = __msg.len().min(a2.saturating_sub(1));
    for __i in 0..__len {
        a1.offset(__i).write(__msg.as_bytes()[__i]);
    }
    if a2 > 0 {
        a1.offset(__len).write(0);
    }
    a1
}

#[cfg(target_os = "macos")]
fn f28(a0: i32, a1: Ptr<u8>, a2: usize) -> i32 {
    let __msg = std::io::Error::from_raw_os_error(a0).to_string();
    let __len = __msg.len().min(a2.saturating_sub(1));
    for __i in 0..__len {
        a1.offset(__i).write(__msg.as_bytes()[__i]);
    }
    if a2 > 0 {
        a1.offset(__len).write(0);
    }
    0
}
