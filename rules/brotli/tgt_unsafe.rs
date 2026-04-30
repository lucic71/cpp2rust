// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn types() {
    let t1: ::brotli_sys::BrotliDecoderResult = ::brotli_sys::BROTLI_DECODER_RESULT_ERROR;
    let t2: ::brotli_sys::BrotliEncoderMode = ::brotli_sys::BROTLI_MODE_GENERIC;
    let t3: *mut ::brotli_sys::BrotliDecoderState = std::ptr::null_mut();
    let t4: *const ::brotli_sys::BrotliDecoderState = std::ptr::null();
    let t5: ::brotli_sys::BrotliDecoderErrorCode = ::brotli_sys::BROTLI_DECODER_NO_ERROR;
}

unsafe fn f1() -> ::brotli_sys::BrotliEncoderMode {
    ::brotli_sys::BROTLI_MODE_FONT
}
unsafe fn f2(
    a0: libc::c_int,
    a1: libc::c_int,
    a2: brotli_sys::BrotliEncoderMode,
    a3: usize,
    a4: *mut u8,
    a5: *mut u64,
    a6: *mut u8,
) -> libc::c_int {
    ::brotli_sys::BrotliEncoderCompress(a0, a1, a2, a3 as usize, a4, a5 as *mut usize, a6)
}
unsafe fn f3() -> ::brotli_sys::BrotliEncoderMode {
    ::brotli_sys::BROTLI_MODE_TEXT
}
unsafe fn f4() -> ::brotli_sys::BrotliDecoderResult {
    ::brotli_sys::BROTLI_DECODER_RESULT_SUCCESS
}
unsafe fn f5(
    a0: usize,
    a1: *mut u8,
    a2: *mut u64,
    a3: *mut u8,
) -> ::brotli_sys::BrotliDecoderResult {
    ::brotli_sys::BrotliDecoderDecompress(a0 as usize, a1, a2 as *mut usize, a3)
}

unsafe fn f6() -> *mut ::brotli_sys::BrotliDecoderState {
    ::brotli_sys::BrotliDecoderCreateInstance(None, None, std::ptr::null_mut())
}

unsafe fn f7(a0: *mut ::brotli_sys::BrotliDecoderState) {
    ::brotli_sys::BrotliDecoderDestroyInstance(a0)
}

unsafe fn f8(
    a0: *mut ::brotli_sys::BrotliDecoderState,
    a1: *const u64,
    a2: *mut *const u8,
    a3: *const u64,
    a4: *const *const u8,
    a5: *const u64,
) -> ::brotli_sys::BrotliDecoderResult {
    ::brotli_sys::BrotliDecoderDecompressStream(
        a0,
        a1 as *mut usize,
        a2,
        a3 as *mut usize,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    )
}

unsafe fn f9(a0: *mut ::brotli_sys::BrotliDecoderState, a1: *mut u64) -> *const u8 {
    ::brotli_sys::BrotliDecoderTakeOutput(a0, a1 as *mut usize)
}

unsafe fn f10() -> ::brotli_sys::BrotliDecoderResult {
    ::brotli_sys::BROTLI_DECODER_RESULT_ERROR
}

unsafe fn f11() -> ::brotli_sys::BrotliDecoderResult {
    ::brotli_sys::BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT
}

unsafe fn f12() -> ::brotli_sys::BrotliEncoderMode {
    ::brotli_sys::BROTLI_MODE_GENERIC
}
