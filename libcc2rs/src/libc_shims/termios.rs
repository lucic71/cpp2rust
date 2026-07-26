// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::ByteRepr;

#[derive(Clone)]
pub struct Termios {
    pub c_iflag: u32,
    pub c_oflag: u32,
    pub c_cflag: u32,
    pub c_lflag: u32,
    pub c_line: u8,
    pub c_cc: Box<[u8]>,
    pub c_ispeed: u32,
    pub c_ospeed: u32,
}

impl Default for Termios {
    fn default() -> Self {
        Self {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: vec![0u8; 32].into_boxed_slice(),
            c_ispeed: 0,
            c_ospeed: 0,
        }
    }
}

impl ByteRepr for Termios {}

impl Termios {
    #[allow(clippy::unnecessary_cast)]
    pub fn from_libc(t: &::libc::termios) -> Self {
        let mut s = Self::default();
        s.c_iflag = t.c_iflag as u32;
        s.c_oflag = t.c_oflag as u32;
        s.c_cflag = t.c_cflag as u32;
        s.c_lflag = t.c_lflag as u32;
        #[cfg(target_os = "linux")]
        {
            s.c_line = t.c_line;
        }
        {
            let cc = &mut s.c_cc;
            let n = t.c_cc.len().min(cc.len());
            cc[..n].copy_from_slice(&t.c_cc[..n]);
        }
        s.c_ispeed = t.c_ispeed as u32;
        s.c_ospeed = t.c_ospeed as u32;
        s
    }

    #[cfg(target_os = "linux")]
    pub fn to_libc(&self) -> ::libc::termios {
        ::libc::termios {
            c_iflag: self.c_iflag,
            c_oflag: self.c_oflag,
            c_cflag: self.c_cflag,
            c_lflag: self.c_lflag,
            c_line: self.c_line,
            c_cc: {
                let mut cc = [0u8; 32];
                let src = &self.c_cc;
                let n = src.len().min(cc.len());
                cc[..n].copy_from_slice(&src[..n]);
                cc
            },
            c_ispeed: self.c_ispeed,
            c_ospeed: self.c_ospeed,
        }
    }

    #[cfg(target_os = "macos")]
    pub fn to_libc(&self) -> ::libc::termios {
        ::libc::termios {
            c_iflag: self.c_iflag as u64,
            c_oflag: self.c_oflag as u64,
            c_cflag: self.c_cflag as u64,
            c_lflag: self.c_lflag as u64,
            c_cc: {
                let mut cc = [0u8; 20];
                let src = &self.c_cc;
                let n = src.len().min(cc.len());
                cc[..n].copy_from_slice(&src[..n]);
                cc
            },
            c_ispeed: self.c_ispeed as u64,
            c_ospeed: self.c_ospeed as u64,
        }
    }
}

#[derive(Default, Clone)]
pub struct Winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}

impl Winsize {
    pub fn from_fd(fd: std::os::fd::BorrowedFd<'_>) -> Option<Self> {
        terminal_size::terminal_size_of(fd).map(|(cols, rows)| Self {
            ws_row: rows.0,
            ws_col: cols.0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        })
    }
}


impl ByteRepr for Winsize {}
