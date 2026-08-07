extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fn_0(v: Vec<u8>) -> Vec<u8> {
    let v: Value<Vec<u8>> = Rc::new(RefCell::new(v));
    return {
        let mut r = (*v.borrow()).clone();
        r.pop();
        r.extend(Ptr::from_string_literal(b" str\0").to_c_string_iterator());
        r.push(0);
        r
    };
}
pub fn fn2_1(v: Ptr<Vec<u8>>) -> Ptr<Vec<u8>> {
    return (v).clone();
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    println!("{}", Ptr::from_string_literal(b"fprintf stdout\0"));
    println!("{} {} {}", 1, 2_u32, 3_i64);
    print!("hello world");
    let in_: Value<Ptr<CFile>> = Rc::new(RefCell::new((libcc2rs::c_stdin()).clone()));
    assert!(!((*in_.borrow()).is_null()));
    println!("{}", Ptr::from_string_literal(b"printf\0"));
    print!("hello world");
    let s: Value<Vec<u8>> = Rc::new(RefCell::new(
        Ptr::from_string_literal(b"a string\0")
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<u8>>(),
    ));
    println!("{}", (s.as_pointer() as Ptr<u8>));
    println!(
        "{}",
        (Rc::new(RefCell::new(
            ({
                fn_0(
                    Ptr::from_string_literal(b"foo\0")
                        .to_c_string_iterator()
                        .chain(std::iter::once(0))
                        .collect::<Vec<u8>>(),
                )
            })
        ))
        .as_pointer() as Ptr<u8>)
    );
    println!(
        "{}",
        (({ fn2_1(s.as_pointer()) }).to_strong().as_pointer() as Ptr<u8>)
    );
    let n: Value<i32> = Rc::new(RefCell::new({
        let __s = libcc2rs::format_c(
            &Ptr::from_string_literal(b"%s\0").to_rust_string(),
            &[(Ptr::from_string_literal(b"1234\0")).into()],
        );
        let __bytes = __s.as_bytes();
        match libcc2rs::c_stdout().with_mut(|__f| __f.write(__bytes)) == __bytes.len() {
            true => __bytes.len() as i32,
            false => -1,
        }
    }));
    assert!(((*n.borrow()) == 4));
    println!("");
    let total: Value<i32> = Rc::new(RefCell::new(0));
    (*total.borrow_mut()) += {
        let __s = libcc2rs::format_c(
            &Ptr::from_string_literal(b"%d\0").to_rust_string(),
            &[(42).into()],
        );
        let __bytes = __s.as_bytes();
        match libcc2rs::c_stdout().with_mut(|__f| __f.write(__bytes)) == __bytes.len() {
            true => __bytes.len() as i32,
            false => -1,
        }
    };
    (*total.borrow_mut()) += {
        let __s = libcc2rs::format_c(
            &Ptr::from_string_literal(b"%c\0").to_rust_string(),
            &[(('x' as u8) as i32).into()],
        );
        let __bytes = __s.as_bytes();
        match libcc2rs::c_stdout().with_mut(|__f| __f.write(__bytes)) == __bytes.len() {
            true => __bytes.len() as i32,
            false => -1,
        }
    };
    println!("");
    assert!(((*total.borrow()) == 3));
    'loop_: while ((*n.borrow_mut()).postfix_inc() < 6) {
        print!(" ");
    }
    assert!(((*n.borrow()) == 7));
    println!("");
    return 0;
}
