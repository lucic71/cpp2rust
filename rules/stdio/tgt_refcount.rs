// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> Ptr<CFile> {
    Ptr::<CFile>::null()
}

fn f1(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<CFile> {
    match CFile::open(&a0.to_rust_string(), &a1.to_rust_string()) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::<CFile>::null(),
    }
}

fn f2(a0: Ptr<CFile>) -> i32 {
    libcc2rs::fclose_refcount(a0.clone())
}

fn f3(a0: Ptr<CFile>) -> i64 {
    a0.with(|__f| __f.tell())
}

fn f4(a0: &mut CFile, a1: i64, a2: i32) -> i32 {
    match a0.seek(a1, a2) {
        -1 => -1,
        _ => 0,
    }
}

fn f5(a0: AnyPtr, a1: usize, a2: usize, a3: Ptr<CFile>) -> usize {
    let __a0 = a0.clone();
    let __a1 = a1;
    let __a2 = a2;
    let __a3 = a3.clone();
    libcc2rs::fread_refcount(__a0, __a1, __a2, __a3)
}

fn f6(a0: AnyPtr, a1: usize, a2: usize, a3: Ptr<CFile>) -> usize {
    let __a0 = a0.clone();
    let __a1 = a1;
    let __a2 = a2;
    let __a3 = a3.clone();
    libcc2rs::fwrite_refcount(__a0, __a1, __a2, __a3)
}

fn f7(a0: Ptr<CFile>) -> i32 {
    0
}

fn f8() -> Ptr<CFile> {
    libcc2rs::c_stdout()
}

fn f9() -> Ptr<CFile> {
    libcc2rs::c_stderr()
}

fn f10() -> Ptr<CFile> {
    libcc2rs::c_stdin()
}

fn f11(a0: i32, a1: Ptr<CFile>) -> i32 {
    let __c = a0 as u8;
    match a1.with_mut(|__f| __f.write(&[__c])) {
        1 => __c as i32,
        _ => -1,
    }
}

fn f12(a0: Ptr<u8>, a1: Ptr<CFile>) -> i32 {
    let __bytes: Vec<u8> = a0.to_c_string_iterator().collect();
    match a1.with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
        true => 0,
        false => -1,
    }
}

fn f13(a0: Ptr<u8>) -> i32 {
    let mut __bytes: Vec<u8> = a0.to_c_string_iterator().collect();
    __bytes.push(b'\n');
    match libcc2rs::c_stdout().with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
        true => 0,
        false => -1,
    }
}

fn f14(a0: Ptr<CFile>) -> i32 {
    a0.with(|__f| __f.fd)
}

fn f15(a0: Ptr<CFile>) -> i32 {
    a0.with(|__f| __f.err) as i32
}

fn f16(a0: Ptr<CFile>) -> i32 {
    a0.with(|__f| __f.eof) as i32
}

fn f17(a0: Ptr<u8>, a1: i32, a2: Ptr<CFile>) -> Ptr<u8> {
    let __buf = a0.clone();
    let __n = a1;
    if __n <= 0 {
        Ptr::<u8>::null()
    } else {
        let __max = (__n - 1) as usize;
        let mut __dst = __buf.clone();
        let mut __count: usize = 0;
        let __failed = a2.with_mut(|__f| {
            while __count < __max {
                let __c = __f.getc();
                if __c < 0 {
                    break;
                }
                __dst.write(__c as u8);
                __dst += 1;
                __count += 1;
                if __c as u8 == b'\n' {
                    break;
                }
            }
            __f.err
        });
        if __failed || __count == 0 {
            Ptr::<u8>::null()
        } else {
            __dst.write(0);
            __buf
        }
    }
}

fn f18(a0: Ptr<u8>, a1: Ptr<u8>, a2: Ptr<CFile>) -> Ptr<CFile> {
    let __stream = a2.clone();
    let __old = __stream.with(|__f| __f.fd);
    match __old {
        0..=2 => {}
        __fd => {
            FdRegistry::close(__fd);
        }
    }
    match CFile::open(&a0.to_rust_string(), &a1.to_rust_string()) {
        Some(__f) => {
            __stream.write(__f);
            __stream
        }
        None => Ptr::<CFile>::null(),
    }
}

fn f19(a0: Ptr<CFile>, a1: i64, a2: i32) -> i32 {
    match a0.with_mut(|__f| __f.seek(a1, a2)) {
        -1 => -1,
        _ => 0,
    }
}

fn f20(a0: i32, a1: Ptr<u8>) -> Ptr<CFile> {
    Ptr::alloc(CFile::new(a0))
}

fn f21(a0: Ptr<u8>, a1: usize, a2: Ptr<u8>, va: &[VaArg]) -> i32 {
    let __fmt: String = a2.to_c_string_iterator().map(|b| b as char).collect();
    let __b: Vec<u8> = libcc2rs::format_c(&__fmt, va)
        .chars()
        .map(|c| c as u32 as u8)
        .collect();
    if a1 > 0 {
        let __n = ::std::cmp::min(__b.len(), a1 - 1);
        a0.with_slice_mut(__n + 1, |__dst| {
            __dst[..__n].copy_from_slice(&__b[..__n]);
            __dst[__n] = 0;
        });
    }
    __b.len() as i32
}

fn f22(a0: Ptr<u8>, a1: Ptr<u8>) -> i32 {
    match ::std::fs::rename(a0.to_rust_string(), a1.to_rust_string()) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EIO));
            -1
        }
    }
}

fn f23(a0: Ptr<CFile>) -> i32 {
    a0.with_mut(|__f| __f.getc())
}

fn f24(a0: Ptr<CFile>, a1: Ptr<u8>, a2: i32, a3: usize) -> i32 {
    0
}

fn f29(a0: Ptr<CFile>, a1: Ptr<u8>, va: &[VaArg]) -> i32 {
    let __fmt: String = a1.to_c_string_iterator().map(|b| b as char).collect();
    let __bytes: Vec<u8> = libcc2rs::format_c(&__fmt, va)
        .chars()
        .map(|c| c as u32 as u8)
        .collect();
    match a0.with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
        true => __bytes.len() as i32,
        false => -1,
    }
}

fn f28(a0: Ptr<CFile>) -> i32 {
    a0.with_mut(|__f| __f.getc())
}

fn f33(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<CFile> {
    libcc2rs::popen_refcount(a0.clone(), a1.clone())
}

fn f34(a0: Ptr<CFile>) -> i32 {
    libcc2rs::pclose_refcount(a0.clone())
}

fn f30(a0: Ptr<CFile>) {
    a0.with_mut(|__f| {
        __f.seek(0, 0);
        __f.err = false;
        __f.eof = false;
    });
}

fn f35(a0: Ptr<CFile>, a1: Ptr<u8>, a2: VaList) -> i32 {
    let __fmt: String = a1.to_c_string_iterator().map(|b| b as char).collect();
    let __bytes: Vec<u8> = libcc2rs::format_c(&__fmt, a2.remaining())
        .chars()
        .map(|c| c as u32 as u8)
        .collect();
    match a0.with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
        true => __bytes.len() as i32,
        false => -1,
    }
}

fn f32(a0: Ptr<u8>, a1: Ptr<u8>, va: &[VaArg]) -> i32 {
    libcc2rs::scan_c(&a0.to_rust_string(), &a1.to_rust_string(), va)
}

fn f36(a0: Ptr<u8>, va: &[VaArg]) -> i32 {
    let __fmt: String = a0.to_c_string_iterator().map(|b| b as char).collect();
    let __bytes: Vec<u8> = libcc2rs::format_c(&__fmt, va)
        .chars()
        .map(|c| c as u32 as u8)
        .collect();
    match libcc2rs::c_stdout().with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
        true => __bytes.len() as i32,
        false => -1,
    }
}

fn f37(a0: i32, a1: Ptr<CFile>) -> i32 {
    let __c = a0 as u8;
    match a1.clone().with_mut(|__f| __f.write(&[__c])) {
        1 => __c as i32,
        _ => -1,
    }
}

fn f38(a0: i32) -> i32 {
    let __c = a0 as u8;
    match libcc2rs::c_stdout().with_mut(|__f| __f.write(&[__c])) {
        1 => __c as i32,
        _ => -1,
    }
}

fn f39(a0: Ptr<u8>, a1: usize, a2: Ptr<u8>, a3: VaList) -> i32 {
    let __fmt: String = a2.to_c_string_iterator().map(|b| b as char).collect();
    let __b: Vec<u8> = libcc2rs::format_c(&__fmt, a3.remaining())
        .chars()
        .map(|c| c as u32 as u8)
        .collect();
    if a1 > 0 {
        let __n = ::std::cmp::min(__b.len(), a1 - 1);
        a0.clone().with_slice_mut(__n + 1, |__dst| {
            __dst[..__n].copy_from_slice(&__b[..__n]);
            __dst[__n] = 0;
        });
    }
    __b.len() as i32
}

fn f40() -> Ptr<CFile> {
    panic!("tmpfile: temporary files are not supported in the refcount model")
}

fn f41(a0: Ptr<CFile>) {
    a0.clone().with_mut(|__f| {
        __f.err = false;
        __f.eof = false;
    });
}

fn f42(a0: Ptr<CFile>) -> i64 {
    a0.clone().with(|__f| __f.tell())
}

fn f43(a0: Ptr<u8>) {
    let __msg = format!(
        "{}: {}\n",
        a0.to_rust_string(),
        nix::errno::Errno::from_raw(libcc2rs::cpp2rust_errno().read()).desc()
    );
    libcc2rs::c_stderr().with_mut(|__f| __f.write(__msg.as_bytes()));
}

fn f44(a0: Ptr<u8>) -> i32 {
    match nix::unistd::unlink(a0.to_rust_string().as_str()) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f31(a0: Ptr<u8>, a1: Ptr<u8>, va: &[VaArg]) -> i32 {
    let __fmt: String = a1.to_c_string_iterator().map(|b| b as char).collect();
    let __b: Vec<u8> = libcc2rs::format_c(&__fmt, va)
        .chars()
        .map(|c| c as u32 as u8)
        .collect();
    a0.clone().with_slice_mut(__b.len() + 1, |__dst| {
        __dst[..__b.len()].copy_from_slice(&__b);
        __dst[__b.len()] = 0;
    });
    __b.len() as i32
}
