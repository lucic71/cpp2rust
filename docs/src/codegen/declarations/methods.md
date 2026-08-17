# Methods and Constructors

Given

```cpp
class Account {
  int balance_;

public:
  Account(int start) : balance_(start) {}
  int balance() const { return balance_; }
  void deposit(int v);
};

void Account::deposit(int v) { balance_ += v; }
```

the unsafe model produces

```rust
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Account {
    balance_: i32,
}
impl Account {
    pub unsafe fn Account(mut start: i32) -> Self {
        let mut this = Self { balance_: start };
        this
    }
    pub unsafe fn balance(&self) -> i32 {
        return self.balance_;
    }
}
impl Account {
    pub unsafe fn deposit(&mut self, mut v: i32) {
        self.balance_ += v;
    }
}
```

and the refcount model produces (`Clone` and `ByteRepr` impls omitted)

```rust
#[derive(Default)]
pub struct Account {
    balance_: Value<i32>,
}
impl Account {
    pub fn Account(start: i32) -> Self {
        let start: Value<i32> = Rc::new(RefCell::new(start));
        let mut this = Self {
            balance_: Rc::new(RefCell::new(*start.borrow())),
        };
        this
    }
    pub fn balance(&self) -> i32 {
        return *self.balance_.borrow();
    }
}
impl Account {
    pub fn deposit(&self, v: i32) {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        *self.balance_.borrow_mut() += *v.borrow();
    }
}
```

## Where methods are emitted

`EmitRustStructOrUnion` prints the class in a fixed order: enums and `static`
data members declared inside the class, which Rust cannot nest in a struct and
are emitted as top-level items (a `static` data member is a
[global variable](./globals.md)), nested records, the struct itself, one
`impl Name` block with the constructors and the non-virtual methods defined
inside the class body, an `impl Base for Name` block with the virtual methods
when the class has a base (see [Inheritance](../types/classes.md#inheritance)),
and finally the trait impls (see [Traits](../types/traits.md)).

A method defined out of line is converted where its definition appears, in its
own `impl Name { ... }` block, which is why `deposit` above sits in a second
block. A constructor defined out of line is instead converted at its in-class
declaration, using the definition's body, so all constructors stay in the first
block. Destructors are never converted as methods; a destructor body becomes
`impl Drop` (see [Drop](../types/traits.md#drop)).

## Signature

`VisitCXXMethodDecl` prints `pub` for public methods and nothing for private or
protected ones. Virtual methods never get `pub`, because they are emitted inside
a trait or a trait impl, where Rust does not allow it. As for
[Functions](./functions.md), the unsafe model emits `unsafe fn` and the refcount
model `fn`.

The receiver is `&self` for a `const` method and `&mut self` otherwise in the
unsafe model, and `&self` always in the refcount model, since fields are
`Value`s and are mutated through `borrow_mut` (`GetSelfMaybeWithMut`). A
`static` method has no receiver and is called as `Name::method(...)`. Inside a
method, `this` is `self`.

Overloaded methods are disambiguated by appending the parameter types and
`_const` (see [Naming](../types/naming.md)); comparison operators are assumed
`const` and are the only overloaded operators supported (see
[Comparison](../types/traits.md#comparison)). Parameters and the preamble are
handled exactly as for free functions.

## Constructors

A constructor becomes an associated function named after the class that returns
`Self`. When the class has more than one converting constructor, the functions
are numbered `Name1`, `Name2`, ... in the order the constructors are first
encountered (`GetCtorIndex`), and a construction `Name(args)` becomes
`Name::Name1(args)`. Copy and move constructors are never emitted; a
user-provided one stops the translation with an assertion (see
[Copy and Clone](../types/traits.md#copy-and-clone)).

`ConvertCXXConstructorBody` runs the parameter preamble, then builds
`let mut this = Self { ... };` with one entry per field in declaration order:
the field's initializer from the member initializer list, converted like a
variable initializer, or its [default value](./defaults.md) when the field is
not in the list. The C++ body follows, with `this` referring to that local, and
the function ends by returning `this`. A base-class initializer at the head of
the list is skipped, since bases carry no data in the supported subset.
