// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f45(a0: std::cmp::Ordering, a1: i32) -> bool {
    a0 != std::cmp::Ordering::Less
}

unsafe fn f46(a0: std::cmp::Ordering, a1: i32) -> bool {
    a0 != std::cmp::Ordering::Greater
}

unsafe fn f47(a0: std::cmp::Ordering, a1: i32) -> bool {
    a0 == std::cmp::Ordering::Greater
}

unsafe fn f48(a0: std::cmp::Ordering, a1: i32) -> bool {
    a0 == std::cmp::Ordering::Less
}

unsafe fn f49(a0: std::cmp::Ordering, a1: std::cmp::Ordering) -> bool {
    a0 == a1
}

unsafe fn f50(a0: std::cmp::Ordering, a1: i32) -> bool {
    a0 == std::cmp::Ordering::Equal
}

fn t6() -> std::cmp::Ordering {
    std::cmp::Ordering::Equal
}
