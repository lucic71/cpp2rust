# Goto and Hoisting

The runtime's [`goto_block!` and `goto!`](../../runtime/control-flow.md) macros
implement the jumps; this page covers what the converter has to do so that the
code inside the block is valid. Given

```c
static int retry(int n) {
  int count = 0;
  int acc = 0;
again:
  count += 1;
  acc += n;
  if (count < 3) {
    goto again;
  }
  return acc;
}
```

the unsafe model produces

```rust
pub unsafe fn retry_0(mut n: i32) -> i32 {
    let mut count: i32 = 0_i32;
    let mut acc: i32 = 0_i32;
    goto_block!({
        '__entry: {
            count = 0;
            acc = 0;
        }
        'again: {
            count += 1;
            acc += n;
            if (count < 3) as i32 != 0 {
                goto!('again);
            }
            return acc;
        }
    });
    panic!("ub: non-void function does not return a value")
}
```

and the refcount model the same with `Value` locals
(`let count: Value<i32> = <Value<i32>>::default();`,
`*count.borrow_mut() = 0;`).

## Detection

Labels are recognized only at the top level of the function, that is, as one of
the statements written directly between the function body's braces.
`CompoundHasTopLevelLabel` checks a block for such a label.

`ConvertFunctionBody` tests the function body with it and hands the body to
`ConvertGotoBlock` when it has one. `VisitCompoundStmt` does the same for a
nested block, so a labeled block inside a loop or an `if` also becomes a
`goto_block!`.

> [!WARNING]
>
> A label that is not at the top level of the function, for example the body of
> an `if` or a `while` written without braces (`if (c) label: x = 1;`), is not
> detected: no `goto_block!` is opened for it and the `goto` that targets it
> does not compile. This is a current limitation.

`VisitGotoStmt` only prints `goto!('label)`. Whether the label lives in an
enclosing block is left to the macro.

A label attached to a `case` is handled by the switch analysis instead. The
label goes into the `SwitchArm` and is emitted as `v if v == 0 => 'target: {`
inside the `switch!`, which the macro accepts as a jump target (see
[Switch](./switch.md)).

## Splitting into arms

`ConvertGotoBlock` walks the block's statements once and opens a new arm at
every label. Everything before the first label goes into `'__entry`; the label's
own statement and everything after it, up to the next label, go into `'label`.

Arms are braces pushed and popped as the walk proceeds. A label in the middle of
a `case` chain, a loop, or an `if` inside the block cannot be split this way,
which is why only top-level labels are supported.

A non-`void` function gets
`panic!("ub: non-void function does not return a value")` after the block, since
Rust cannot see that every path inside the macro returns.

## Hoisting

Each arm is a Rust block, so a `let` in one arm is not visible in the next,
while in C every local of the function is visible after any label.

Before the `goto_block!`, `EmitHoistedDecls` walks the declarations written at
the top level of the block. For every local variable it emits a declaration with
the type's default value, `let mut count: i32 = 0_i32;`
(`<Value<i32>>::default()` in the refcount model), and records the `VarDecl` in
`hoisted_decls_`. Static locals and globals are left alone.

Inside the arms the traversal then meets the original declarations.
`ConvertVarDecl` checks `hoisted_decls_` first and, for a hoisted variable,
emits only the initializer as an assignment: `count = 0;`
(`*count.borrow_mut() = 0;` in the refcount model), or nothing when the
declaration had no initializer. This is `EmitHoistedInArmAssignment`, and it is
what the `'__entry` arm above consists of.

A hoisted variable is declared `mut` even when it was `const` in C, since its
value is only assigned later in the arm.

`hoisted_decls_` is saved and cleared by `PushHoistedDecls` for the duration of
each `ConvertGotoBlock`, so a nested labeled block hoists its own locals to its
own start and the outer block's set is restored afterwards.

Declarations inside nested blocks are not hoisted; they cannot span a label,
since labels are top-level only.
