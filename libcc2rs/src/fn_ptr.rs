// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

use crate::rc::{AnyPtr, ErasedPtr, Ptr};
use crate::reinterpret::ByteRepr;

thread_local! {
    static ADAPTER_REGISTRY: RefCell<HashMap<(usize, TypeId), Rc<dyn ErasedFn>>> =
        RefCell::new(HashMap::new());
}

fn register_adapter(orig_addr: usize, target: TypeId, adapter: Rc<dyn ErasedFn>) {
    ADAPTER_REGISTRY.with(|r| {
        r.borrow_mut().insert((orig_addr, target), adapter);
    });
}

fn lookup_adapter(orig_addr: usize, target: TypeId) -> Option<Rc<dyn ErasedFn>> {
    ADAPTER_REGISTRY.with(|r| r.borrow().get(&(orig_addr, target)).cloned())
}

pub trait FnAddr {
    fn fn_addr(&self) -> usize;
}

macro_rules! impl_fn_addr {
    () => {
        impl_fn_addr!(@gen A B C D E F G H I J K L M N O P);
    };
    (@gen $($a:ident)*) => {
        impl<R $(, $a)*> FnAddr for fn($($a,)*) -> R {
            #[inline]
            fn fn_addr(&self) -> usize { *self as *const () as usize }
        }
        impl_fn_addr!(@peel $($a)*);
    };
    (@peel) => {};
    (@peel $head:ident $($tail:ident)*) => {
        impl_fn_addr!(@gen $($tail)*);
    };
}
impl_fn_addr!();

pub(crate) trait ErasedFn: Any {
    fn addr(&self) -> usize;
}

impl<T: FnAddr + Any> ErasedFn for T {
    fn addr(&self) -> usize {
        self.fn_addr()
    }
}

#[derive(Clone)]
enum FnPtrKind {
    Null,
    Fn {
        original: Rc<dyn ErasedFn>,
        current_cast: Option<Rc<dyn ErasedFn>>,
    },
    Dangling(usize),
}

impl FnPtrKind {
    fn addr(&self) -> usize {
        match self {
            FnPtrKind::Null => 0,
            FnPtrKind::Fn { original, .. } => original.addr(),
            FnPtrKind::Dangling(value) => *value,
        }
    }
}

pub struct FnPtr<T> {
    kind: FnPtrKind,
    // FnPtr does not use T, hence wrap in PhantomData
    _marker: PhantomData<T>,
}

impl<T> FnPtr<T> {
    #[inline]
    pub fn null() -> Self {
        FnPtr {
            kind: FnPtrKind::Null,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        matches!(self.kind, FnPtrKind::Null)
    }

    pub(crate) fn dangling_value(&self) -> Option<usize> {
        match self.kind {
            FnPtrKind::Dangling(value) => Some(value),
            _ => None,
        }
    }

    pub fn from_int(value: usize) -> Self {
        if value == 0 {
            return Self::null();
        }
        FnPtr {
            kind: FnPtrKind::Dangling(value),
            _marker: PhantomData,
        }
    }
}

impl<T: FnAddr + 'static> FnPtr<T> {
    pub fn new(f: T) -> Self {
        let rc: Rc<dyn ErasedFn> = Rc::new(f);
        FnPtr {
            kind: FnPtrKind::Fn {
                original: rc.clone(),
                current_cast: Some(rc),
            },
            _marker: PhantomData,
        }
    }
}

impl<T: 'static> FnPtr<T> {
    pub fn cast<U: FnAddr + 'static>(&self, adapter: Option<U>) -> FnPtr<U> {
        let (original, current_cast) = match &self.kind {
            FnPtrKind::Null => panic!("ub: null fn pointer cast"),
            FnPtrKind::Dangling(value) => {
                return FnPtr {
                    kind: FnPtrKind::Dangling(*value),
                    _marker: PhantomData,
                };
            }
            FnPtrKind::Fn {
                original,
                current_cast,
            } => (original, current_cast),
        };

        let current_cast = if current_cast
            .as_ref()
            .is_some_and(|rc| Any::type_id(&**rc) == TypeId::of::<U>())
        {
            current_cast.clone()
        } else if Any::type_id(&**original) == TypeId::of::<U>() {
            Some(original.clone())
        } else {
            adapter.map(|a| {
                let rc: Rc<dyn ErasedFn> = Rc::new(a);
                register_adapter(original.addr(), TypeId::of::<U>(), rc.clone());
                rc
            })
        };

        FnPtr {
            kind: FnPtrKind::Fn {
                original: original.clone(),
                current_cast,
            },
            _marker: PhantomData,
        }
    }
}

impl<T: 'static> Deref for FnPtr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        let current_cast = match &self.kind {
            FnPtrKind::Null => panic!("ub: null fn pointer call"),
            FnPtrKind::Dangling(value) => {
                panic!("ub: called dangling fn pointer 0x{value:x}")
            }
            FnPtrKind::Fn { current_cast, .. } => current_cast,
        };
        let rc = current_cast
            .as_ref()
            .expect("ub: calling through incompatible fn pointer type");
        let any: &dyn Any = &**rc;
        any.downcast_ref::<T>()
            .expect("ub: fn pointer type mismatch")
    }
}

impl<T> Clone for FnPtr<T> {
    fn clone(&self) -> Self {
        FnPtr {
            kind: self.kind.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T> Default for FnPtr<T> {
    fn default() -> Self {
        Self::null()
    }
}

impl<T> PartialEq for FnPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.kind.addr() == other.kind.addr()
    }
}

impl<T> Eq for FnPtr<T> {}

impl<T: 'static> ByteRepr for FnPtr<T> {
    fn byte_size() -> usize {
        std::mem::size_of::<usize>()
    }

    fn to_bytes(&self, buf: &mut [u8]) {
        match &self.kind {
            FnPtrKind::Null => 0usize.to_bytes(buf),
            FnPtrKind::Dangling(value) => value.to_bytes(buf),
            FnPtrKind::Fn { original, .. } => crate::rc::register_ptr(
                original.addr(),
                1,
                crate::rc::Registered::Fn(original.clone()),
            )
            .to_bytes(buf),
        }
    }

    fn from_bytes(buf: &[u8]) -> Self {
        let addr = usize::from_bytes(buf);
        if addr == 0 {
            return Self::null();
        }
        let Some((base, entry, _)) = crate::rc::lookup_ptr(addr) else {
            return FnPtr {
                kind: FnPtrKind::Dangling(addr),
                _marker: PhantomData,
            };
        };
        let crate::rc::Registered::Fn(original) = entry else {
            panic!("ub: cast of data address 0x{addr:x} to fn pointer");
        };
        if base != addr {
            panic!("ub: cast of interior address 0x{addr:x} to fn pointer");
        }
        let current_cast = if Any::type_id(&*original) == TypeId::of::<T>() {
            Some(original.clone())
        } else {
            lookup_adapter(original.addr(), TypeId::of::<T>())
        };
        FnPtr {
            kind: FnPtrKind::Fn {
                original,
                current_cast,
            },
            _marker: PhantomData,
        }
    }
}

impl<T: 'static> ErasedPtr for FnPtr<T> {
    fn as_bytes(&self) -> Ptr<u8> {
        panic!("byte view not supported on fn pointer");
    }
    fn write_address(&self, buf: &mut [u8]) {
        ByteRepr::to_bytes(self, buf);
    }
    fn is_derived(&self) -> bool {
        false
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals(&self, other: &dyn ErasedPtr) -> bool {
        other.as_any().downcast_ref::<FnPtr<T>>() == Some(self)
    }
    fn is_null(&self) -> bool {
        FnPtr::is_null(self)
    }
    fn is_dangling(&self) -> bool {
        matches!(self.kind, FnPtrKind::Dangling(_))
    }
}

impl<T: 'static> FnPtr<T> {
    pub fn to_any(&self) -> AnyPtr {
        AnyPtr {
            ptr: Rc::new(self.clone()),
        }
    }
}

impl AnyPtr {
    pub fn cast_fn<T: 'static>(&self) -> Option<FnPtr<T>> {
        self.ptr.as_any().downcast_ref::<FnPtr<T>>().cloned()
    }
}
