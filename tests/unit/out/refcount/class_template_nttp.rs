extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Step_0_ {
    pub value: Value<i32>,
}
impl Clone for Step_0_ {
    fn clone(&self) -> Self {
        let __this: Value<Step_0_> = Rc::new(RefCell::new(Self {
            value: Rc::new(RefCell::new((*self.value.borrow()))),
        }));
        let this: Ptr<Step_0_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Step_0_ {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.value.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Step_2_ {
    pub value: Value<i32>,
}
impl Clone for Step_2_ {
    fn clone(&self) -> Self {
        let __this: Value<Step_2_> = Rc::new(RefCell::new(Self {
            value: Rc::new(RefCell::new((*self.value.borrow()))),
        }));
        let this: Ptr<Step_2_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Step_2_ {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.value.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Step_3_ {
    pub value: Value<i32>,
}
impl Clone for Step_3_ {
    fn clone(&self) -> Self {
        let __this: Value<Step_3_> = Rc::new(RefCell::new(Self {
            value: Rc::new(RefCell::new((*self.value.borrow()))),
        }));
        let this: Ptr<Step_3_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Step_3_ {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.value.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Step_neg1_ {
    pub value: Value<i32>,
}
impl Clone for Step_neg1_ {
    fn clone(&self) -> Self {
        let __this: Value<Step_neg1_> = Rc::new(RefCell::new(Self {
            value: Rc::new(RefCell::new((*self.value.borrow()))),
        }));
        let this: Ptr<Step_neg1_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Step_neg1_ {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.value.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Clone, ByteRepr, Default)]
pub struct Choice_true_ {}
#[derive(Clone, ByteRepr, Default)]
pub struct Choice_false_ {}
#[derive()]
pub struct Buffer_int__2_ {
    pub data: Value<Box<[i32]>>,
}
impl Clone for Buffer_int__2_ {
    fn clone(&self) -> Self {
        let __this: Value<Buffer_int__2_> = Rc::new(RefCell::new(Self {
            data: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 2, _>(
                |__i: usize| (*self.data.borrow())[(__i) as usize],
            )))),
        }));
        let this: Ptr<Buffer_int__2_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Buffer_int__2_ {
    fn default() -> Self {
        Buffer_int__2_ {
            data: Rc::new(RefCell::new((0..2).map(|_| 0_i32).collect::<Box<[i32]>>())),
        }
    }
}
impl ByteRepr for Buffer_int__2_ {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.data.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data: Rc::new(RefCell::new(<Box<[i32]>>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive()]
pub struct Buffer_int__3_ {
    pub data: Value<Box<[i32]>>,
}
impl Clone for Buffer_int__3_ {
    fn clone(&self) -> Self {
        let __this: Value<Buffer_int__3_> = Rc::new(RefCell::new(Self {
            data: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 3, _>(
                |__i: usize| (*self.data.borrow())[(__i) as usize],
            )))),
        }));
        let this: Ptr<Buffer_int__3_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Buffer_int__3_ {
    fn default() -> Self {
        Buffer_int__3_ {
            data: Rc::new(RefCell::new((0..3).map(|_| 0_i32).collect::<Box<[i32]>>())),
        }
    }
}
impl ByteRepr for Buffer_int__3_ {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.data.borrow()).to_bytes(&mut buf[0..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data: Rc::new(RefCell::new(<Box<[i32]>>::from_bytes(&buf[0..12]))),
        }
    }
}
#[derive(Clone, ByteRepr, Default)]
pub struct Mark__a___true_ {}
#[derive(Clone, ByteRepr, Default)]
pub struct Mark__a___false_ {}
#[derive(Clone, ByteRepr, Default)]
pub struct Mark__b___true_ {}
#[derive(Clone, ByteRepr, Default)]
pub struct Range_0__10_ {}
#[derive(Clone, ByteRepr, Default)]
pub struct Range_5__6_ {}
#[derive(Clone, ByteRepr, Default)]
pub struct Range_neg3__0_ {}
#[derive(Clone, ByteRepr, Default)]
pub struct Mix___xc8___neg3__10000000000_ {}
#[derive(Clone, ByteRepr, Default)]
pub struct Mix___x01___1__1_ {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s2: Value<Step_2_> = Rc::new(RefCell::new(Step_2_ {
        value: Rc::new(RefCell::new(1)),
    }));
    let s3: Value<Step_3_> = Rc::new(RefCell::new(Step_3_ {
        value: Rc::new(RefCell::new(1)),
    }));
    ({ Step_2_Impl::advance(&s2.as_pointer()) });
    ({ Step_3_Impl::advance(&s3.as_pointer()) });
    assert!(((*(*s2.borrow()).value.borrow()) == 3));
    assert!(((*(*s3.borrow()).value.borrow()) == 4));
    assert!((({ Step_2_Impl::scaled(&s2.as_pointer(),) }) == 6));
    assert!((({ Step_3_Impl::scaled(&s3.as_pointer(),) }) == 12));
    let sn: Value<Step_neg1_> = Rc::new(RefCell::new(Step_neg1_ {
        value: Rc::new(RefCell::new(5)),
    }));
    ({ Step_neg1_Impl::advance(&sn.as_pointer()) });
    assert!(((*(*sn.borrow()).value.borrow()) == 4));
    assert!((({ Step_neg1_Impl::scaled(&sn.as_pointer(),) }) == -4_i32));
    let s0: Value<Step_0_> = Rc::new(RefCell::new(Step_0_ {
        value: Rc::new(RefCell::new(5)),
    }));
    ({ Step_0_Impl::advance(&s0.as_pointer()) });
    assert!(((*(*s0.borrow()).value.borrow()) == -1_i32));
    assert!((({ Step_0_Impl::scaled(&s0.as_pointer(),) }) == 0));
    let yes: Value<Choice_true_> = Rc::new(RefCell::new(<Choice_true_>::default()));
    let no: Value<Choice_false_> = Rc::new(RefCell::new(<Choice_false_>::default()));
    assert!((({ Choice_true_Impl::pick(&yes.as_pointer(), 1, 2,) }) == 1));
    assert!((({ Choice_false_Impl::pick(&no.as_pointer(), 1, 2,) }) == 2));
    let b2: Value<Buffer_int__2_> = Rc::new(RefCell::new(Buffer_int__2_ {
        data: Rc::new(RefCell::new(Box::new([1, 2]))),
    }));
    let b3: Value<Buffer_int__3_> = Rc::new(RefCell::new(Buffer_int__3_ {
        data: Rc::new(RefCell::new(Box::new([1, 2, 3]))),
    }));
    assert!(
        (({ Buffer_int__2_Impl::size(&b2.as_pointer(),) }) == 2)
            && (({ Buffer_int__2_Impl::sum(&b2.as_pointer(),) }) == 3)
    );
    assert!(
        (({ Buffer_int__3_Impl::size(&b3.as_pointer(),) }) == 3)
            && (({ Buffer_int__3_Impl::sum(&b3.as_pointer(),) }) == 6)
    );
    let v2: Value<Vec<Step_2_>> = Rc::new(RefCell::new(Vec::new()));
    let v3: Value<Vec<Step_3_>> = Rc::new(RefCell::new(Vec::new()));
    (*v2.borrow_mut()).push(Step_2_ {
        value: Rc::new(RefCell::new(0)),
    });
    (*v3.borrow_mut()).push(Step_3_ {
        value: Rc::new(RefCell::new(0)),
    });
    ({ Step_2_Impl::advance(&(v2.as_pointer() as Ptr<Step_2_>).offset(0_usize)) });
    ({ Step_3_Impl::advance(&(v3.as_pointer() as Ptr<Step_3_>).offset(0_usize)) });
    assert!(
        ((*v2.borrow()).len() == 1_usize)
            && ((*(*(v2.as_pointer() as Ptr<Step_2_>)
                .offset(0_usize)
                .upgrade()
                .deref())
            .value
            .borrow())
                == 2)
    );
    assert!(
        ((*v3.borrow()).len() == 1_usize)
            && ((*(*(v3.as_pointer() as Ptr<Step_3_>)
                .offset(0_usize)
                .upgrade()
                .deref())
            .value
            .borrow())
                == 3)
    );
    let a2: Value<Vec<Step_2_>> = Rc::new(RefCell::new(vec![
        Step_2_ {
            value: Rc::new(RefCell::new(1)),
        },
        Step_2_ {
            value: Rc::new(RefCell::new(2)),
        },
    ]));
    let a3: Value<Vec<Step_3_>> = Rc::new(RefCell::new(vec![
        Step_3_ {
            value: Rc::new(RefCell::new(1)),
        },
        Step_3_ {
            value: Rc::new(RefCell::new(2)),
        },
        Step_3_ {
            value: Rc::new(RefCell::new(3)),
        },
    ]));
    'loop_: for mut s in a2.as_pointer() as Ptr<Step_2_> {
        ({ Step_2_Impl::advance(&s) });
    }
    'loop_: for mut s in a3.as_pointer() as Ptr<Step_3_> {
        ({ Step_3_Impl::advance(&s) });
    }
    assert!(
        ((*a2.borrow()).len() == 2_usize)
            && (({ Step_2_Impl::scaled(&(a2.as_pointer() as Ptr<Step_2_>).offset(1_usize),) })
                == 8)
    );
    assert!(
        ((*a3.borrow()).len() == 3_usize)
            && (({ Step_3_Impl::scaled(&(a3.as_pointer() as Ptr<Step_3_>).offset(2_usize),) })
                == 18)
    );
    let vb2: Value<Vec<Buffer_int__2_>> = Rc::new(RefCell::new(Vec::new()));
    let vb3: Value<Vec<Buffer_int__3_>> = Rc::new(RefCell::new(Vec::new()));
    {
        let a0_clone = (*b2.borrow()).clone();
        (*vb2.borrow_mut()).push(a0_clone)
    };
    {
        let a0_clone = (*b3.borrow()).clone();
        (*vb3.borrow_mut()).push(a0_clone)
    };
    assert!(
        (({
            Buffer_int__2_Impl::size(&(vb2.as_pointer() as Ptr<Buffer_int__2_>).offset(0_usize))
        }) == 2)
            && (({
                Buffer_int__2_Impl::sum(&(vb2.as_pointer() as Ptr<Buffer_int__2_>).offset(0_usize))
            }) == 3)
    );
    assert!(
        (({
            Buffer_int__3_Impl::size(&(vb3.as_pointer() as Ptr<Buffer_int__3_>).offset(0_usize))
        }) == 3)
            && (({
                Buffer_int__3_Impl::sum(&(vb3.as_pointer() as Ptr<Buffer_int__3_>).offset(0_usize))
            }) == 6)
    );
    let ayes: Value<Vec<Choice_true_>> = Rc::new(RefCell::new(vec![<Choice_true_>::default()]));
    let ano: Value<Vec<Choice_false_>> = Rc::new(RefCell::new(vec![<Choice_false_>::default()]));
    assert!(
        (({
            Choice_true_Impl::pick(
                &(ayes.as_pointer() as Ptr<Choice_true_>).offset(0_usize),
                1,
                2,
            )
        }) == 1)
    );
    assert!(
        (({
            Choice_false_Impl::pick(
                &(ano.as_pointer() as Ptr<Choice_false_>).offset(0_usize),
                1,
                2,
            )
        }) == 2)
    );
    let all_a: Value<Mark__a___true_> = Rc::new(RefCell::new(<Mark__a___true_>::default()));
    let first_a: Value<Mark__a___false_> = Rc::new(RefCell::new(<Mark__a___false_>::default()));
    let all_b: Value<Mark__b___true_> = Rc::new(RefCell::new(<Mark__b___true_>::default()));
    assert!(
        (({
            Mark__a___true_Impl::count(
                &all_a.as_pointer(),
                Ptr::<u8>::from_string_literal(b"banana"),
            )
        }) == 3)
    );
    assert!(
        (({
            Mark__a___false_Impl::count(
                &first_a.as_pointer(),
                Ptr::<u8>::from_string_literal(b"banana"),
            )
        }) == 1)
    );
    assert!(
        (({
            Mark__b___true_Impl::count(
                &all_b.as_pointer(),
                Ptr::<u8>::from_string_literal(b"banana"),
            )
        }) == 1)
    );
    let wide: Value<Range_0__10_> = Rc::new(RefCell::new(<Range_0__10_>::default()));
    let narrow: Value<Range_5__6_> = Rc::new(RefCell::new(<Range_5__6_>::default()));
    let neg: Value<Range_neg3__0_> = Rc::new(RefCell::new(<Range_neg3__0_>::default()));
    assert!(
        ({ Range_0__10_Impl::contains(&wide.as_pointer(), 7,) })
            && (!({ Range_5__6_Impl::contains(&narrow.as_pointer(), 7,) }))
    );
    assert!(
        (({ Range_0__10_Impl::width(&wide.as_pointer(),) }) == 10)
            && (({ Range_5__6_Impl::width(&narrow.as_pointer(),) }) == 1)
    );
    assert!(
        ({ Range_neg3__0_Impl::contains(&neg.as_pointer(), -2_i32,) })
            && (({ Range_neg3__0_Impl::width(&neg.as_pointer(),) }) == 3)
    );
    let big: Value<Mix___xc8___neg3__10000000000_> =
        Rc::new(RefCell::new(<Mix___xc8___neg3__10000000000_>::default()));
    let small: Value<Mix___x01___1__1_> = Rc::new(RefCell::new(<Mix___x01___1__1_>::default()));
    assert!(
        (({ Mix___xc8___neg3__10000000000_Impl::total(&big.as_pointer(),) }) == 10000000197_i64)
    );
    assert!((({ Mix___x01___1__1_Impl::total(&small.as_pointer(),) }) == 3_i64));
    let vwide: Value<Vec<Range_0__10_>> = Rc::new(RefCell::new(vec![(*wide.borrow()).clone()]));
    let vnarrow: Value<Vec<Range_5__6_>> = Rc::new(RefCell::new(vec![(*narrow.borrow()).clone()]));
    assert!(
        ({
            Range_0__10_Impl::contains(
                &(vwide.as_pointer() as Ptr<Range_0__10_>).offset(0_usize),
                7,
            )
        }) && (!({
            Range_5__6_Impl::contains(
                &(vnarrow.as_pointer() as Ptr<Range_5__6_>).offset(0_usize),
                7,
            )
        }))
    );
    let amarks: Value<Vec<Mark__a___true_>> = Rc::new(RefCell::new(vec![
        <Mark__a___true_>::default(),
        <Mark__a___true_>::default(),
    ]));
    let afirst: Value<Vec<Mark__a___false_>> = Rc::new(RefCell::new(vec![
        <Mark__a___false_>::default(),
        <Mark__a___false_>::default(),
    ]));
    assert!(
        (({
            Mark__a___true_Impl::count(
                &(amarks.as_pointer() as Ptr<Mark__a___true_>).offset(1_usize),
                Ptr::<u8>::from_string_literal(b"aaa"),
            )
        }) == 3)
    );
    assert!(
        (({
            Mark__a___false_Impl::count(
                &(afirst.as_pointer() as Ptr<Mark__a___false_>).offset(1_usize),
                Ptr::<u8>::from_string_literal(b"aaa"),
            )
        }) == 1)
    );
    let vbig: Value<Vec<Mix___xc8___neg3__10000000000_>> = Rc::new(RefCell::new(vec![
        (*big.borrow()).clone(),
        (*big.borrow()).clone(),
    ]));
    assert!(
        ((*vbig.borrow()).len() == 2_usize)
            && (({
                Mix___xc8___neg3__10000000000_Impl::total(
                    &(vbig.as_pointer() as Ptr<Mix___xc8___neg3__10000000000_>).offset(1_usize),
                )
            }) == 10000000197_i64)
    );
    return 0;
}
pub trait Buffer_int__2_Impl {
    fn size(&self) -> i32;
    fn sum(&self) -> i32;
}
impl Buffer_int__2_Impl for Ptr<Buffer_int__2_> {
    fn size(&self) -> i32 {
        return 2;
    }
    fn sum(&self) -> i32 {
        let total: Value<i32> = Rc::new(RefCell::new(0));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 2) {
            (*total.borrow_mut()) +=
                (*(*(*self).upgrade().deref()).data.borrow())[(*i.borrow()) as usize];
            (*i.borrow_mut()).prefix_inc();
        }
        return (*total.borrow());
    }
}
pub trait Buffer_int__3_Impl {
    fn size(&self) -> i32;
    fn sum(&self) -> i32;
}
impl Buffer_int__3_Impl for Ptr<Buffer_int__3_> {
    fn size(&self) -> i32 {
        return 3;
    }
    fn sum(&self) -> i32 {
        let total: Value<i32> = Rc::new(RefCell::new(0));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 3) {
            (*total.borrow_mut()) +=
                (*(*(*self).upgrade().deref()).data.borrow())[(*i.borrow()) as usize];
            (*i.borrow_mut()).prefix_inc();
        }
        return (*total.borrow());
    }
}
pub trait Choice_false_Impl {
    fn pick(&self, a: i32, b: i32) -> i32;
}
impl Choice_false_Impl for Ptr<Choice_false_> {
    fn pick(&self, a: i32, b: i32) -> i32 {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        let b: Value<i32> = Rc::new(RefCell::new(b));
        return if false { (*a.borrow()) } else { (*b.borrow()) };
    }
}
pub trait Choice_true_Impl {
    fn pick(&self, a: i32, b: i32) -> i32;
}
impl Choice_true_Impl for Ptr<Choice_true_> {
    fn pick(&self, a: i32, b: i32) -> i32 {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        let b: Value<i32> = Rc::new(RefCell::new(b));
        return if true { (*a.borrow()) } else { (*b.borrow()) };
    }
}
pub trait Mark__a___false_Impl {
    fn count(&self, s: Ptr<u8>) -> i32;
}
impl Mark__a___false_Impl for Ptr<Mark__a___false_> {
    fn count(&self, s: Ptr<u8>) -> i32 {
        let s: Value<Ptr<u8>> = Rc::new(RefCell::new(s));
        let n: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while (((*s.borrow()).read()) != 0) {
            if ((((*s.borrow()).read()) as i32) == (('a' as u8) as i32)) {
                (*n.borrow_mut()).prefix_inc();
                if !(false) {
                    break;
                }
            }
            (*s.borrow_mut()).prefix_inc();
        }
        return (*n.borrow());
    }
}
pub trait Mark__a___true_Impl {
    fn count(&self, s: Ptr<u8>) -> i32;
}
impl Mark__a___true_Impl for Ptr<Mark__a___true_> {
    fn count(&self, s: Ptr<u8>) -> i32 {
        let s: Value<Ptr<u8>> = Rc::new(RefCell::new(s));
        let n: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while (((*s.borrow()).read()) != 0) {
            if ((((*s.borrow()).read()) as i32) == (('a' as u8) as i32)) {
                (*n.borrow_mut()).prefix_inc();
                if !(true) {
                    break;
                }
            }
            (*s.borrow_mut()).prefix_inc();
        }
        return (*n.borrow());
    }
}
pub trait Mark__b___true_Impl {
    fn count(&self, s: Ptr<u8>) -> i32;
}
impl Mark__b___true_Impl for Ptr<Mark__b___true_> {
    fn count(&self, s: Ptr<u8>) -> i32 {
        let s: Value<Ptr<u8>> = Rc::new(RefCell::new(s));
        let n: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while (((*s.borrow()).read()) != 0) {
            if ((((*s.borrow()).read()) as i32) == (('b' as u8) as i32)) {
                (*n.borrow_mut()).prefix_inc();
                if !(true) {
                    break;
                }
            }
            (*s.borrow_mut()).prefix_inc();
        }
        return (*n.borrow());
    }
}
pub trait Mix___x01___1__1_Impl {
    fn total(&self) -> i64;
}
impl Mix___x01___1__1_Impl for Ptr<Mix___x01___1__1_> {
    fn total(&self) -> i64 {
        return ((((('\x01' as u8) as i32) + (1_i16 as i32)) as i64) + 1_i64);
    }
}
pub trait Mix___xc8___neg3__10000000000_Impl {
    fn total(&self) -> i64;
}
impl Mix___xc8___neg3__10000000000_Impl for Ptr<Mix___xc8___neg3__10000000000_> {
    fn total(&self) -> i64 {
        return (((((b'\xc8' as u8) as i32) + (65533_i16 as i32)) as i64) + 10000000000_i64);
    }
}
pub trait Range_0__10_Impl {
    fn contains(&self, x: i32) -> bool;
    fn width(&self) -> i32;
}
impl Range_0__10_Impl for Ptr<Range_0__10_> {
    fn contains(&self, x: i32) -> bool {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) >= 0) && ((*x.borrow()) <= 10);
    }
    fn width(&self) -> i32 {
        return (10 - 0);
    }
}
pub trait Range_5__6_Impl {
    fn contains(&self, x: i32) -> bool;
    fn width(&self) -> i32;
}
impl Range_5__6_Impl for Ptr<Range_5__6_> {
    fn contains(&self, x: i32) -> bool {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) >= 5) && ((*x.borrow()) <= 6);
    }
    fn width(&self) -> i32 {
        return (6 - 5);
    }
}
pub trait Range_neg3__0_Impl {
    fn contains(&self, x: i32) -> bool;
    fn width(&self) -> i32;
}
impl Range_neg3__0_Impl for Ptr<Range_neg3__0_> {
    fn contains(&self, x: i32) -> bool {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) >= 4294967293) && ((*x.borrow()) <= 0);
    }
    fn width(&self) -> i32 {
        return (0 - 4294967293);
    }
}
pub trait Step_0_Impl {
    fn advance(&self);
    fn scaled(&self) -> i32;
}
impl Step_0_Impl for Ptr<Step_0_> {
    fn advance(&self) {
        (*(*(*self).upgrade().deref()).value.borrow_mut()) = -1_i32;
    }
    fn scaled(&self) -> i32 {
        return 0;
    }
}
pub trait Step_2_Impl {
    fn advance(&self);
    fn scaled(&self) -> i32;
}
impl Step_2_Impl for Ptr<Step_2_> {
    fn advance(&self) {
        (*(*(*self).upgrade().deref()).value.borrow_mut()) += 2;
    }
    fn scaled(&self) -> i32 {
        return ((*(*(*self).upgrade().deref()).value.borrow()) * 2);
    }
}
pub trait Step_3_Impl {
    fn advance(&self);
    fn scaled(&self) -> i32;
}
impl Step_3_Impl for Ptr<Step_3_> {
    fn advance(&self) {
        (*(*(*self).upgrade().deref()).value.borrow_mut()) += 3;
    }
    fn scaled(&self) -> i32 {
        return ((*(*(*self).upgrade().deref()).value.borrow()) * 3);
    }
}
pub trait Step_neg1_Impl {
    fn advance(&self);
    fn scaled(&self) -> i32;
}
impl Step_neg1_Impl for Ptr<Step_neg1_> {
    fn advance(&self) {
        (*(*(*self).upgrade().deref()).value.borrow_mut()) += 4294967295;
    }
    fn scaled(&self) -> i32 {
        return ((*(*(*self).upgrade().deref()).value.borrow()) * 4294967295);
    }
}
pub fn __cpp2rust_init_globals() {}
