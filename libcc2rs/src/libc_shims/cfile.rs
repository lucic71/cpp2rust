// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::FdRegistry;
use std::cell::RefCell;
use std::collections::BTreeMap;

thread_local! {
    static POPEN_CHILDREN: RefCell<BTreeMap<i32, std::process::Child>> =
        RefCell::new(BTreeMap::new());
}

pub struct CFile {
    pub fd: i32,
    pub eof: bool,
    pub err: bool,
}

impl CFile {
    pub fn new(fd: i32) -> Self {
        CFile {
            fd,
            eof: false,
            err: false,
        }
    }

    pub fn open(path: &str, mode: &str) -> Option<CFile> {
        use nix::fcntl::OFlag;
        let mut chars = mode.chars();
        let mut flags = match chars.next() {
            Some('r') => OFlag::O_RDONLY,
            Some('w') => OFlag::O_WRONLY.union(OFlag::O_CREAT).union(OFlag::O_TRUNC),
            Some('a') => OFlag::O_WRONLY.union(OFlag::O_CREAT).union(OFlag::O_APPEND),
            _ => panic!("fopen: unsupported mode {:?}", mode),
        };
        for c in chars {
            match c {
                'b' => {}
                '+' => {
                    flags.remove(OFlag::O_WRONLY);
                    flags.insert(OFlag::O_RDWR);
                }
                'x' => flags.insert(OFlag::O_EXCL),
                'e' => flags.insert(OFlag::O_CLOEXEC),
                _ => panic!("fopen: unsupported mode {:?}", mode),
            }
        }
        match nix::fcntl::open(path, flags, nix::sys::stat::Mode::from_bits_truncate(0o666)) {
            Ok(ofd) => Some(CFile::new(FdRegistry::register(ofd))),
            Err(e) => {
                crate::cpp2rust_errno().write(e as i32);
                None
            }
        }
    }

    pub fn tmpfile() -> Option<CFile> {
        use nix::errno::Errno;
        use nix::fcntl::OFlag;
        use nix::sys::stat::Mode;

        let mode = Mode::S_IRUSR | Mode::S_IWUSR;

        match nix::fcntl::open(
            "/tmp",
            OFlag::O_TMPFILE | OFlag::O_RDWR | OFlag::O_EXCL,
            mode,
        ) {
            Ok(ofd) => return Some(CFile::new(FdRegistry::register(ofd))),
            Err(Errno::EOPNOTSUPP) | Err(Errno::EISDIR) => {}
            Err(e) => {
                crate::cpp2rust_errno().write(e as i32);
                return None;
            }
        }

        let pid = std::process::id();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.subsec_nanos());
        for seq in 0..32 {
            let path = format!("/tmp/tmpf{pid:x}.{nanos:x}.{seq:x}");
            match nix::fcntl::open(
                path.as_str(),
                OFlag::O_RDWR | OFlag::O_CREAT | OFlag::O_EXCL,
                mode,
            ) {
                Ok(ofd) => {
                    let _ = nix::unistd::unlink(path.as_str());
                    return Some(CFile::new(FdRegistry::register(ofd)));
                }
                Err(Errno::EEXIST) => {}
                Err(e) => {
                    crate::cpp2rust_errno().write(e as i32);
                    return None;
                }
            }
        }
        crate::cpp2rust_errno().write(Errno::EEXIST as i32);
        None
    }

    pub fn popen(command: &str, mode: &str) -> Option<CFile> {
        use std::process::{Command, Stdio};

        let reading = match mode.chars().next() {
            Some('r') => true,
            Some('w') => false,
            _ => panic!("popen: unsupported mode {:?}", mode),
        };

        let mut cmd = Command::new("/bin/sh");
        cmd.arg("-c").arg(command);
        if reading {
            cmd.stdout(Stdio::piped());
        } else {
            cmd.stdin(Stdio::piped());
        }

        let mut child = match cmd.spawn() {
            Ok(child) => child,
            Err(e) => {
                let errno = e.raw_os_error().unwrap_or(nix::errno::Errno::ENOENT as i32);
                crate::cpp2rust_errno().write(errno);
                return None;
            }
        };

        let pipe: std::os::fd::OwnedFd = if reading {
            child.stdout.take()?.into()
        } else {
            child.stdin.take()?.into()
        };

        let fd = FdRegistry::register(pipe);
        POPEN_CHILDREN.with(|children| children.borrow_mut().insert(fd, child));
        Some(CFile::new(fd))
    }

    pub fn pclose(&self) -> i32 {
        use std::os::unix::process::ExitStatusExt;

        let child = POPEN_CHILDREN.with(|children| children.borrow_mut().remove(&self.fd));
        FdRegistry::close(self.fd);

        let Some(mut child) = child else {
            crate::cpp2rust_errno().write(nix::errno::Errno::ECHILD as i32);
            return -1;
        };
        match child.wait() {
            Ok(status) => status.into_raw(),
            Err(e) => {
                let errno = e.raw_os_error().unwrap_or(nix::errno::Errno::ECHILD as i32);
                crate::cpp2rust_errno().write(errno);
                -1
            }
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let mut n = 0;
        while n < buf.len() {
            match FdRegistry::with_fd(self.fd, |b| nix::unistd::read(b, &mut buf[n..])) {
                Ok(0) => {
                    self.eof = true;
                    break;
                }
                Ok(k) => n += k,
                Err(nix::errno::Errno::EINTR) => {}
                Err(e) => {
                    self.err = true;
                    crate::cpp2rust_errno().write(e as i32);
                    break;
                }
            }
        }
        n
    }

    pub fn write(&mut self, buf: &[u8]) -> usize {
        let mut n = 0;
        while n < buf.len() {
            match FdRegistry::with_fd(self.fd, |b| nix::unistd::write(b, &buf[n..])) {
                Ok(0) => {
                    self.err = true;
                    break;
                }
                Ok(k) => n += k,
                Err(nix::errno::Errno::EINTR) => {}
                Err(e) => {
                    self.err = true;
                    crate::cpp2rust_errno().write(e as i32);
                    break;
                }
            }
        }
        n
    }

    pub fn seek(&mut self, offset: i64, whence: i32) -> i64 {
        let w = match whence {
            0 => nix::unistd::Whence::SeekSet,
            1 => nix::unistd::Whence::SeekCur,
            2 => nix::unistd::Whence::SeekEnd,
            other => panic!("fseek: unsupported whence {}", other),
        };
        match FdRegistry::with_fd(self.fd, |b| nix::unistd::lseek(b, offset, w)) {
            Ok(off) => {
                self.eof = false;
                off
            }
            Err(e) => {
                crate::cpp2rust_errno().write(e as i32);
                -1
            }
        }
    }

    pub fn tell(&self) -> i64 {
        match FdRegistry::with_fd(self.fd, |b| {
            nix::unistd::lseek(b, 0, nix::unistd::Whence::SeekCur)
        }) {
            Ok(off) => off,
            Err(e) => {
                crate::cpp2rust_errno().write(e as i32);
                -1
            }
        }
    }

    pub fn getc(&mut self) -> i32 {
        let mut b = [0u8; 1];
        match self.read(&mut b) {
            1 => b[0] as i32,
            _ => -1,
        }
    }

    pub fn close(&self) -> i32 {
        match self.fd {
            0..=2 => 0,
            fd => FdRegistry::close(fd),
        }
    }
}

impl crate::ByteRepr for CFile {
    fn byte_size() -> usize {
        0
    }
}
