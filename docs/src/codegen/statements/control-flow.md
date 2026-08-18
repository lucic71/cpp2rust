# Control Flow

`if`, `while`, `do`, and `for` map onto Rust's `if` and `while`; every loop is
labeled `'loop_` so that `break` and `continue` can name it. Given

```cpp
int count(int n) {
  int out = 0;
  for (int i = 0; i < n; i++) {
    if (i % 2 == 0)
      continue;
    out++;
  }
  do {
    out--;
  } while (out > 10);
  return out;
}
```

the unsafe model produces

```rust
pub unsafe fn count_0(mut n: i32) -> i32 {
    let mut out: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while i < n {
        if i % 2 == 0 {
            i.postfix_inc();
            continue 'loop_;
        }
        out.postfix_inc();
        i.postfix_inc();
    }
    let mut __do_while = true;
    while __do_while || out > 10 {
        __do_while = false;
        out.postfix_dec();
    }
    return out;
}
```

and the refcount model the same shape with every variable access borrowed
(`*i.borrow() < *n.borrow()`, `(*i.borrow_mut()).postfix_inc()`).

## Conditions

`ConvertCondition` converts the condition as an rvalue. Rust needs a `bool`
there, and C++ conditions already are `bool` in the AST, so they print as is. C
has no `bool`: a condition is an integer or pointer compared against zero, so
`NormalizeToBool` first turns an integer into `x != 0`, a pointer into
`!p.is_null()`, and re-types `!x` on an integer to yield `bool`. Since C
comparisons have type `int`, this is why C conditions come out as
`(a < b) as i32 != 0`.

## `if`

`if cond { then } else { else }`; an `else if` chain is emitted as such rather
than as nested blocks.

## `while` and `for`

`while` becomes `while cond { body }`. `for (init; cond; inc)` becomes
`init; while cond { body; inc; }`, with `while true` when the condition is
empty. Both carry the `'loop_` label, shown in the example above. The increment
is remembered on [`curr_for_inc_`](../internals/state.md) for the duration of
the body, so that `continue` can run it before jumping (see below); `while` and
`do` push a null entry so a `continue` inside them emits no increment.

## `do`

There is no `do` in Rust. `do { body } while (cond)` becomes a `while` guarded
by a flag that forces the first iteration:

```rust
let mut __do_while = true;
while __do_while || cond {
    __do_while = false;
    body
}
```

The flag is redeclared per loop, so nested `do` loops shadow it.

## `break` and `continue`

`break` inside a loop is `break`; inside a `switch` it becomes `break 'switch`
so that it leaves the `match` block and not an enclosing loop (see
[Switch](./switch.md)). `continue` first emits the enclosing `for` loop's
increment, if any, and then `continue 'loop_`; because every loop carries the
same label, `'loop_` always resolves to the innermost one.

## Range-based `for`

`for (T x : range)` is dispatched on the range's type (`GetClassName`):
`std::map`, `std::basic_string`, or anything else, which is treated as a
vector-like container. Only rule-mapped library types are supported as ranges.

What the loop variable becomes depends on whether it is declared by value or by
reference. Given

```cpp
for (auto x : v) sum += x;
for (auto &x : v) x += 10;
```

the unsafe model iterates by index and produces

```rust
for x in 0..(v.len()) {
    let mut x = v[x].clone();
    sum += x;
}
for x in 0..(v.len()) {
    let mut x = v.as_mut_ptr().add(x);
    *x += 10;
}
```

so a by-value variable is a clone of the element and a reference variable is a
raw pointer to it. The refcount model iterates through a `Ptr` over the
container, so the loop variable is already a pointer to each element, and
produces

```rust
for mut x in v.as_pointer() as Ptr<i32> {
    let x: Value<i32> = Rc::new(RefCell::new(x.read().clone()));
    *sum.borrow_mut() += *x.borrow();
}
for mut x in v.as_pointer() as Ptr<i32> {
    let _ptr = x.clone();
    _ptr.write(_ptr.read() + 10);
}
```

so a by-value variable is read out of the pointer into a boxed local, and a
reference variable is the pointer itself, used through `read`/`write` like any
other `Ptr` (a `const` variable drops the `mut` on the binding). Strings iterate
to `len()-1` in the unsafe model, to skip the terminator, and through
`.to_string_iterator() as StringIterator<T>` in the refcount model (see
[Iterators](../../runtime/iterators.md)). Maps use
`UnsafeMapIterator::begin(&m as *const BTreeMap<K, V>)` and
`RefcountMapIter::begin(m.as_pointer())`, and the loop variable is the map
iterator itself (see [Special-cased Library Types](../types/special-types.md)).
