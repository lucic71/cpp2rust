unsafe fn f1() -> i32 {
    libc::MSG_NOSIGNAL
}

unsafe fn f2() -> i32 {
    libc::SOCK_STREAM
}

unsafe fn f3() -> i32 {
    libc::SOCK_DGRAM
}

unsafe fn f4() -> i32 {
    libc::SOCK_RAW
}

unsafe fn f5() -> i32 {
    libc::SOCK_RDM
}

unsafe fn f6() -> i32 {
    libc::SOCK_SEQPACKET
}

unsafe fn f7() -> i32 {
    libc::SOCK_DCCP
}

unsafe fn f8() -> i32 {
    libc::SOCK_PACKET
}

unsafe fn f9() -> i32 {
    libc::SOCK_CLOEXEC
}

unsafe fn f10() -> i32 {
    libc::SOCK_NONBLOCK
}
