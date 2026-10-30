// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1() -> Ptr<::std::fs::File> {
    libcc2rs::cerr()
}

fn f4() -> Ptr<::std::fs::File> {
    libcc2rs::cout()
}
