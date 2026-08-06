// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::ByteRepr;
use crate::libc_shims::time::Timeval;

#[derive(Default, Clone)]
pub struct Rusage {
    pub ru_utime: Timeval,
    pub ru_stime: Timeval,
    pub ru_maxrss: i64,
    pub ru_ixrss: i64,
    pub ru_idrss: i64,
    pub ru_isrss: i64,
    pub ru_minflt: i64,
    pub ru_majflt: i64,
    pub ru_nswap: i64,
    pub ru_inblock: i64,
    pub ru_oublock: i64,
    pub ru_msgsnd: i64,
    pub ru_msgrcv: i64,
    pub ru_nsignals: i64,
    pub ru_nvcsw: i64,
    pub ru_nivcsw: i64,
}

impl Rusage {
    pub fn from_libc(r: &::libc::rusage) -> Self {
        Self {
            ru_utime: Timeval {
                tv_sec: r.ru_utime.tv_sec,
                tv_usec: r.ru_utime.tv_usec,
            },
            ru_stime: Timeval {
                tv_sec: r.ru_stime.tv_sec,
                tv_usec: r.ru_stime.tv_usec,
            },
            ru_maxrss: r.ru_maxrss,
            ru_ixrss: r.ru_ixrss,
            ru_idrss: r.ru_idrss,
            ru_isrss: r.ru_isrss,
            ru_minflt: r.ru_minflt,
            ru_majflt: r.ru_majflt,
            ru_nswap: r.ru_nswap,
            ru_inblock: r.ru_inblock,
            ru_oublock: r.ru_oublock,
            ru_msgsnd: r.ru_msgsnd,
            ru_msgrcv: r.ru_msgrcv,
            ru_nsignals: r.ru_nsignals,
            ru_nvcsw: r.ru_nvcsw,
            ru_nivcsw: r.ru_nivcsw,
        }
    }
}

impl ByteRepr for Rusage {
    fn byte_size() -> usize {
        144
    }

    fn to_bytes(&self, buf: &mut [u8]) {
        self.ru_utime.to_bytes(&mut buf[0..16]);
        self.ru_stime.to_bytes(&mut buf[16..32]);
        self.ru_maxrss.to_bytes(&mut buf[32..40]);
        self.ru_ixrss.to_bytes(&mut buf[40..48]);
        self.ru_idrss.to_bytes(&mut buf[48..56]);
        self.ru_isrss.to_bytes(&mut buf[56..64]);
        self.ru_minflt.to_bytes(&mut buf[64..72]);
        self.ru_majflt.to_bytes(&mut buf[72..80]);
        self.ru_nswap.to_bytes(&mut buf[80..88]);
        self.ru_inblock.to_bytes(&mut buf[88..96]);
        self.ru_oublock.to_bytes(&mut buf[96..104]);
        self.ru_msgsnd.to_bytes(&mut buf[104..112]);
        self.ru_msgrcv.to_bytes(&mut buf[112..120]);
        self.ru_nsignals.to_bytes(&mut buf[120..128]);
        self.ru_nvcsw.to_bytes(&mut buf[128..136]);
        self.ru_nivcsw.to_bytes(&mut buf[136..144]);
    }

    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            ru_utime: Timeval::from_bytes(&buf[0..16]),
            ru_stime: Timeval::from_bytes(&buf[16..32]),
            ru_maxrss: i64::from_bytes(&buf[32..40]),
            ru_ixrss: i64::from_bytes(&buf[40..48]),
            ru_idrss: i64::from_bytes(&buf[48..56]),
            ru_isrss: i64::from_bytes(&buf[56..64]),
            ru_minflt: i64::from_bytes(&buf[64..72]),
            ru_majflt: i64::from_bytes(&buf[72..80]),
            ru_nswap: i64::from_bytes(&buf[80..88]),
            ru_inblock: i64::from_bytes(&buf[88..96]),
            ru_oublock: i64::from_bytes(&buf[96..104]),
            ru_msgsnd: i64::from_bytes(&buf[104..112]),
            ru_msgrcv: i64::from_bytes(&buf[112..120]),
            ru_nsignals: i64::from_bytes(&buf[120..128]),
            ru_nvcsw: i64::from_bytes(&buf[128..136]),
            ru_nivcsw: i64::from_bytes(&buf[136..144]),
        }
    }
}

impl ByteRepr for ::libc::rusage {}
