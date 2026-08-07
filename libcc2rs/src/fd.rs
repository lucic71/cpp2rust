// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::RefCell;
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd};

pub struct FdRegistry {
    fds: Vec<Option<OwnedFd>>,
}

thread_local! {
    static FD_REGISTRY: RefCell<FdRegistry> = const { RefCell::new(FdRegistry { fds: Vec::new() }) };
}

impl FdRegistry {
    pub fn register(fd: OwnedFd) -> i32 {
        let raw = fd.as_raw_fd();
        FD_REGISTRY.with(|r| {
            let fds = &mut r.borrow_mut().fds;
            let idx = raw as usize;
            if fds.len() <= idx {
                fds.resize_with(idx + 1, || None);
            }
            assert!(fds[idx].is_none(), "ub: fd registry collision on fd {raw}");
            fds[idx] = Some(fd);
        });
        raw
    }

    fn slot(&self, fd: i32) -> Option<BorrowedFd<'_>> {
        self.fds
            .get(fd as usize)
            .and_then(|slot| slot.as_ref())
            .map(AsFd::as_fd)
    }

    pub fn with_fd<R>(fd: i32, f: impl FnOnce(BorrowedFd<'_>) -> R) -> R {
        FD_REGISTRY.with(|r| {
            let reg = r.borrow();
            match reg.slot(fd) {
                Some(b) => f(b),
                None => match fd {
                    0 => f(std::io::stdin().as_fd()),
                    1 => f(std::io::stdout().as_fd()),
                    2 => f(std::io::stderr().as_fd()),
                    _ => panic!("ub: bad fd {fd}"),
                },
            }
        })
    }

    pub fn with_fds<R>(fds: &[i32], f: impl FnOnce(&[BorrowedFd<'_>]) -> R) -> R {
        FD_REGISTRY.with(|r| {
            let reg = r.borrow();
            let stdio = (std::io::stdin(), std::io::stdout(), std::io::stderr());
            let borrowed: Vec<BorrowedFd<'_>> = fds
                .iter()
                .map(|&fd| {
                    reg.slot(fd).unwrap_or_else(|| match fd {
                        0 => stdio.0.as_fd(),
                        1 => stdio.1.as_fd(),
                        2 => stdio.2.as_fd(),
                        _ => panic!("ub: bad fd {fd}"),
                    })
                })
                .collect();
            f(&borrowed)
        })
    }

    pub fn dup2(oldfd: i32, newfd: i32) -> i32 {
        FD_REGISTRY.with(|r| {
            let fds = &mut r.borrow_mut().fds;
            let old_registered = fds.get(oldfd as usize).is_some_and(|s| s.is_some());
            assert!(
                old_registered || (0..=2).contains(&oldfd),
                "ub: bad fd {oldfd}"
            );
            if oldfd == newfd {
                return newfd;
            }
            let stdio = (std::io::stdin(), std::io::stdout(), std::io::stderr());
            let idx = newfd as usize;
            if fds.len() <= idx {
                fds.resize_with(idx + 1, || None);
            }
            let target = fds[idx].take();
            let result = {
                let old = match (old_registered, oldfd) {
                    (true, _) => fds[oldfd as usize].as_ref().unwrap().as_fd(),
                    (false, 0) => stdio.0.as_fd(),
                    (false, 1) => stdio.1.as_fd(),
                    (false, 2) => stdio.2.as_fd(),
                    _ => unreachable!(),
                };
                match (newfd, target) {
                    (0, _) => nix::unistd::dup2_stdin(old).map(|()| None),
                    (1, _) => nix::unistd::dup2_stdout(old).map(|()| None),
                    (2, _) => nix::unistd::dup2_stderr(old).map(|()| None),
                    (_, Some(mut owned)) => {
                        nix::unistd::dup2(old, &mut owned).map(|()| Some(owned))
                    }
                    (_, None) => {
                        let mut spares = Vec::new();
                        loop {
                            match nix::unistd::dup(old) {
                                Ok(d) => {
                                    assert!(
                                        d.as_raw_fd() <= newfd,
                                        "fd registry invariant violation"
                                    );
                                    if d.as_raw_fd() == newfd {
                                        break Ok(Some(d));
                                    }
                                    spares.push(d);
                                }
                                Err(e) => break Err(e),
                            }
                        }
                    }
                }
            };
            match result {
                Ok(slot) => {
                    fds[idx] = slot;
                    newfd
                }
                Err(e) => {
                    crate::cpp2rust_errno().write(e as i32);
                    -1
                }
            }
        })
    }

    pub fn close(fd: i32) -> i32 {
        FD_REGISTRY.with(|r| {
            r.borrow_mut()
                .fds
                .get_mut(fd as usize)
                .and_then(|slot| slot.take())
                .unwrap_or_else(|| panic!("ub: bad fd {fd}"))
        });
        0
    }
}
