extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, Uninit, FnPtrArg, MoveCtor, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct p11_defaulted_move_constructor_is_trivial_MoveOnly {
    #[offset(0)]
    pub value: i32,
}
impl p11_defaulted_move_constructor_is_trivial_MoveOnly {
    pub fn new(this: Ptr<p11_defaulted_move_constructor_is_trivial_MoveOnly>, mut value: i32) {
        field!(this, value).write(value);
    }
    pub fn move_from(
        this: Ptr<p11_defaulted_move_constructor_is_trivial_MoveOnly>,
        _a0: Ptr<p11_defaulted_move_constructor_is_trivial_MoveOnly>,
    ) {
        field!(this, value).write({ (*_a0.upgrade().deref()).value });
    }
}
pub fn pass_0(
    __ret: Ptr<p11_defaulted_move_constructor_is_trivial_MoveOnly>,
    object: Value<p11_defaulted_move_constructor_is_trivial_MoveOnly>,
) {
    p11_defaulted_move_constructor_is_trivial_MoveOnly::move_from(__ret.clone(), {
        object.as_pointer()
    });
    return;
}
pub fn test_1() {
    let result: Value<p11_defaulted_move_constructor_is_trivial_MoveOnly> = Rc::new(RefCell::new(
        <p11_defaulted_move_constructor_is_trivial_MoveOnly>::uninit(),
    ));
    ({
        pass_0(result.as_pointer(), {
            let __v: Value<p11_defaulted_move_constructor_is_trivial_MoveOnly> = Rc::new(
                RefCell::new(<p11_defaulted_move_constructor_is_trivial_MoveOnly>::uninit()),
            );
            p11_defaulted_move_constructor_is_trivial_MoveOnly::new(__v.as_pointer(), { 7 });
            __v
        })
    });
    assert!(({ ({ (*result.borrow()).value } == 7) }));
}
pub fn read_by_value_2(it: RandomAccessIterator<i32>) -> i32 {
    let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(it));
    return (({ (*it.borrow()).iter_deref() }).read());
}
pub fn test_3() {
    let source: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3])));
    let moved: Value<Vec<i32>> = Rc::new(RefCell::new((*source.borrow_mut()).clone()));
    assert!(({ ((*moved.borrow()).len() == 3_usize) }));
    assert!(({ ((*source.borrow()).len() == 3_usize) }));
    let values: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([7, 8])));
    let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(values.decay().begin()));
    assert!(({ (({ read_by_value_2((*it.borrow()).clone(),) }) == 7) }));
    assert!(((({ (*it.borrow()).iter_deref() }).read()) == 7));
    let moved_it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new((*it.borrow()).clone()));
    assert!(((({ (*moved_it.borrow()).iter_deref() }).read()) == 7));
    assert!(((({ (*it.borrow()).iter_deref() }).read()) == 7));
}
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct p14_memberwise_copy_and_move_in_declaration_order_Logged {
    #[offset(0)]
    pub id: i32,
}
impl p14_memberwise_copy_and_move_in_declaration_order_Logged {
    pub fn new(this: Ptr<p14_memberwise_copy_and_move_in_declaration_order_Logged>, mut id: i32) {
        field!(this, id).write(id);
    }
    pub fn copy_from(
        this: Ptr<p14_memberwise_copy_and_move_in_declaration_order_Logged>,
        other: Ptr<p14_memberwise_copy_and_move_in_declaration_order_Logged>,
    ) {
        field!(this, id).write(other.with(|__s| __s.id));
        printf!(b"copy %d\n", this.with(|__s| __s.id));
    }
    pub fn move_from(
        this: Ptr<p14_memberwise_copy_and_move_in_declaration_order_Logged>,
        other: Ptr<p14_memberwise_copy_and_move_in_declaration_order_Logged>,
    ) {
        field!(this, id).write(other.with(|__s| __s.id));
        printf!(b"move %d\n", this.with(|__s| __s.id));
        field!(other, id).write(-1_i32);
    }
}
impl Clone for p14_memberwise_copy_and_move_in_declaration_order_Logged {
    fn clone(&self) -> Self {
        let __src: Value<p14_memberwise_copy_and_move_in_declaration_order_Logged> = Rc::new(
            RefCell::new(p14_memberwise_copy_and_move_in_declaration_order_Logged {
                id: self.id.clone(),
            }),
        );
        let __dst: Value<p14_memberwise_copy_and_move_in_declaration_order_Logged> = Rc::new(
            RefCell::new(<p14_memberwise_copy_and_move_in_declaration_order_Logged>::uninit()),
        );
        p14_memberwise_copy_and_move_in_declaration_order_Logged::copy_from(
            __dst.as_pointer(),
            __src.as_pointer(),
        );
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor)]
#[byte_size(20)]
pub struct p14_memberwise_copy_and_move_in_declaration_order_Holder {
    #[offset(0)]
    #[byte_size(4)]
    pub first: p14_memberwise_copy_and_move_in_declaration_order_Logged,
    #[offset(4)]
    pub number: i32,
    #[offset(8)]
    #[byte_size(8)]
    #[array_len(2)]
    pub elements: Box<[p14_memberwise_copy_and_move_in_declaration_order_Logged]>,
    #[offset(16)]
    #[byte_size(4)]
    pub last: p14_memberwise_copy_and_move_in_declaration_order_Logged,
}
impl p14_memberwise_copy_and_move_in_declaration_order_Holder {
    pub fn copy_from(
        this: Ptr<p14_memberwise_copy_and_move_in_declaration_order_Holder>,
        _a0: Ptr<p14_memberwise_copy_and_move_in_declaration_order_Holder>,
    ) {
        p14_memberwise_copy_and_move_in_declaration_order_Logged::copy_from(
            field_ptr!(this, first),
            { field_ptr!(_a0, first) },
        );
        field!(this, number).write(_a0.with(|__s| __s.number));
        {
            let __d = array_field_ptr!(this, elements);
            for __i in 0..2_usize {
                p14_memberwise_copy_and_move_in_declaration_order_Logged::copy_from(
                    __d.offset(__i as isize),
                    { array_field_ptr!(_a0, elements).offset(__i) },
                );
            }
        };
        p14_memberwise_copy_and_move_in_declaration_order_Logged::copy_from(
            field_ptr!(this, last),
            { field_ptr!(_a0, last) },
        );
    }
    pub fn move_from(
        this: Ptr<p14_memberwise_copy_and_move_in_declaration_order_Holder>,
        _a0: Ptr<p14_memberwise_copy_and_move_in_declaration_order_Holder>,
    ) {
        p14_memberwise_copy_and_move_in_declaration_order_Logged::move_from(
            field_ptr!(this, first),
            { field_ptr!(_a0, first) },
        );
        field!(this, number).write({ (*_a0.upgrade().deref()).number });
        {
            let __d = array_field_ptr!(this, elements);
            for __i in 0..2_usize {
                p14_memberwise_copy_and_move_in_declaration_order_Logged::move_from(
                    __d.offset(__i as isize),
                    { array_field_ptr!(_a0, elements).offset(__i) },
                );
            }
        };
        p14_memberwise_copy_and_move_in_declaration_order_Logged::move_from(
            field_ptr!(this, last),
            { field_ptr!(_a0, last) },
        );
    }
}
impl Default for p14_memberwise_copy_and_move_in_declaration_order_Holder {
    fn default() -> Self {
        {
            p14_memberwise_copy_and_move_in_declaration_order_Holder {
                first: <p14_memberwise_copy_and_move_in_declaration_order_Logged>::default(),
                number: 0_i32,
                elements: (0..2)
                    .map(|_| <p14_memberwise_copy_and_move_in_declaration_order_Logged>::default())
                    .collect::<Box<[p14_memberwise_copy_and_move_in_declaration_order_Logged]>>(),
                last: <p14_memberwise_copy_and_move_in_declaration_order_Logged>::default(),
            }
        }
    }
}
pub fn test_4() {
    let source: Value<p14_memberwise_copy_and_move_in_declaration_order_Holder> = Rc::new(
        RefCell::new(<p14_memberwise_copy_and_move_in_declaration_order_Holder>::uninit()),
    );
    {
        let __d = source.as_pointer();
        p14_memberwise_copy_and_move_in_declaration_order_Logged::new(field_ptr!(__d, first), {
            1
        });
        field!(__d, number).write(5);
        {
            let __d = array_field_ptr!(__d, elements);
            p14_memberwise_copy_and_move_in_declaration_order_Logged::new(__d.offset(0), { 2 });
            p14_memberwise_copy_and_move_in_declaration_order_Logged::new(__d.offset(1), { 3 });
        };
        p14_memberwise_copy_and_move_in_declaration_order_Logged::new(field_ptr!(__d, last), { 4 });
    };
    let copy: Value<p14_memberwise_copy_and_move_in_declaration_order_Holder> = Rc::new(
        RefCell::new(<p14_memberwise_copy_and_move_in_declaration_order_Holder>::uninit()),
    );
    p14_memberwise_copy_and_move_in_declaration_order_Holder::copy_from(copy.as_pointer(), {
        source.as_pointer()
    });
    assert!(
        ({
            (({ (*copy.borrow()).first.id } == 1) && ({ (*copy.borrow()).number } == 5))
                && ({ (*copy.borrow()).last.id } == 4)
        })
    );
    assert!(
        (array_elem!(copy.as_pointer(), elements, 0).with(|__s| __s.id) == 2)
            && (array_elem!(copy.as_pointer(), elements, 1).with(|__s| __s.id) == 3)
    );
    assert!(({ ({ (*source.borrow()).first.id } == 1) && ({ (*source.borrow()).last.id } == 4) }));
    let moved: Value<p14_memberwise_copy_and_move_in_declaration_order_Holder> = Rc::new(
        RefCell::new(<p14_memberwise_copy_and_move_in_declaration_order_Holder>::uninit()),
    );
    p14_memberwise_copy_and_move_in_declaration_order_Holder::move_from(moved.as_pointer(), {
        source.as_pointer()
    });
    assert!(
        ({
            (({ (*moved.borrow()).first.id } == 1) && ({ (*moved.borrow()).number } == 5))
                && ({ (*moved.borrow()).last.id } == 4)
        })
    );
    assert!(
        (array_elem!(moved.as_pointer(), elements, 0).with(|__s| __s.id) == 2)
            && (array_elem!(moved.as_pointer(), elements, 1).with(|__s| __s.id) == 3)
    );
    assert!(
        ({
            ({ (*source.borrow()).first.id } == -1_i32)
                && (array_elem!(source.as_pointer(), elements, 1).with(|__s| __s.id) == -1_i32)
        })
    );
    assert!(
        ({ ({ (*source.borrow()).last.id } == -1_i32) && ({ (*source.borrow()).number } == 5) })
    );
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_4() });
    ({ test_1() });
    ({ test_3() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
