extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive(Default)]
pub struct GraphNode {
    pub dst: u32,
    pub next: Ptr<GraphNode>,
}
impl Clone for GraphNode {
    fn clone(&self) -> Self {
        let mut this = Self {
            dst: self.dst,
            next: (self.next).clone(),
        };
        this
    }
}
impl ByteRepr for GraphNode {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.dst.to_bytes(&mut buf[0..4]);
        self.next.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            dst: <u32>::from_bytes(&buf[0..4]),
            next: <Ptr<GraphNode>>::from_bytes(&buf[8..16]),
        }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct Graph {
    pub V: u32,
    pub adj: Ptr<Ptr<GraphNode>>,
}
pub trait GraphMethods {
    fn push(&self, src: u32, dst: u32);
}
impl GraphMethods for Ptr<Graph> {
    fn push(&self, src: u32, dst: u32) {
        let src: Value<u32> = Rc::new(RefCell::new(src));
        let dst: Value<u32> = Rc::new(RefCell::new(dst));
        {
            let __rhs = Ptr::alloc(GraphNode {
                dst: (*dst.borrow()),
                next: (self
                    .with(|__v| (*__v).adj.offset(((*src.borrow()) as isize)).clone())
                    .read())
                .clone(),
            });
            self.with(|__v| (*__v).adj.offset(((*src.borrow()) as isize)).clone())
                .write(__rhs)
        };
        {
            let __rhs = Ptr::alloc(GraphNode {
                dst: (*src.borrow()),
                next: (self
                    .with(|__v| (*__v).adj.offset(((*dst.borrow()) as isize)).clone())
                    .read())
                .clone(),
            });
            self.with(|__v| (*__v).adj.offset(((*dst.borrow()) as isize)).clone())
                .write(__rhs)
        };
    }
}
impl Clone for Graph {
    fn clone(&self) -> Self {
        let mut this = Self {
            V: self.V,
            adj: (self.adj).clone(),
        };
        this
    }
}
impl ByteRepr for Graph {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.V.to_bytes(&mut buf[0..4]);
        self.adj.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            V: <u32>::from_bytes(&buf[0..4]),
            adj: <Ptr<Ptr<GraphNode>>>::from_bytes(&buf[8..16]),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let g: Value<Graph> = Rc::new(RefCell::new(Graph {
        V: 5_u32,
        adj: Ptr::<Ptr<GraphNode>>::null(),
    }));
    return 0;
}
