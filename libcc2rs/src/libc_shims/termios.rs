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

impl ByteRepr for Termios {
    fn byte_size() -> usize {
        60
    }

    fn to_bytes(&self, buf: &mut [u8]) {
        self.c_iflag.to_bytes(&mut buf[0..4]);
        self.c_oflag.to_bytes(&mut buf[4..8]);
        self.c_cflag.to_bytes(&mut buf[8..12]);
        self.c_lflag.to_bytes(&mut buf[12..16]);
        self.c_line.to_bytes(&mut buf[16..17]);
        buf[17..49].copy_from_slice(&self.c_cc);
        self.c_ispeed.to_bytes(&mut buf[52..56]);
        self.c_ospeed.to_bytes(&mut buf[56..60]);
    }

    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            c_iflag: u32::from_bytes(&buf[0..4]),
            c_oflag: u32::from_bytes(&buf[4..8]),
            c_cflag: u32::from_bytes(&buf[8..12]),
            c_lflag: u32::from_bytes(&buf[12..16]),
            c_line: u8::from_bytes(&buf[16..17]),
            c_cc: buf[17..49].to_vec().into_boxed_slice(),
            c_ispeed: u32::from_bytes(&buf[52..56]),
            c_ospeed: u32::from_bytes(&buf[56..60]),
        }
    }
}

impl Termios {
    #[allow(clippy::unnecessary_cast)]
    pub fn from_libc(t: &::libc::termios) -> Self {
        let mut s = Self {
            c_iflag: t.c_iflag as u32,
            c_oflag: t.c_oflag as u32,
            c_cflag: t.c_cflag as u32,
            c_lflag: t.c_lflag as u32,
            ..Default::default()
        };
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

impl ByteRepr for Winsize {
    fn byte_size() -> usize {
        8
    }

    fn to_bytes(&self, buf: &mut [u8]) {
        self.ws_row.to_bytes(&mut buf[0..2]);
        self.ws_col.to_bytes(&mut buf[2..4]);
        self.ws_xpixel.to_bytes(&mut buf[4..6]);
        self.ws_ypixel.to_bytes(&mut buf[6..8]);
    }

    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            ws_row: u16::from_bytes(&buf[0..2]),
            ws_col: u16::from_bytes(&buf[2..4]),
            ws_xpixel: u16::from_bytes(&buf[4..6]),
            ws_ypixel: u16::from_bytes(&buf[6..8]),
        }
    }
}
