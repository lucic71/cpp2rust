unsafe fn f1() -> i32 {
    libc::IPPROTO_TCP
}

unsafe fn f2() -> i32 {
    libc::IPPROTO_UDP
}

unsafe fn f3() -> i32 {
    libc::IPPROTO_IP
}

unsafe fn f4() -> i32 {
    libc::IPPROTO_IPV6
}

#[cfg(target_os = "linux")]
unsafe fn f5() -> i32 {
    libc::IPPROTO_MPTCP
}
