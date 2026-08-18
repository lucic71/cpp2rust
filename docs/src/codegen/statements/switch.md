# Switch

A `switch` is translated in two steps: the body is analyzed into arms, and the
arms are emitted either as a plain `match` or through the runtime's
[`switch!`](../../runtime/control-flow.md#switch) macro when C fallthrough
semantics are needed. Given

```cpp
int pick(int x) {
  int r = 0;
  switch (x) {
  case 0:
    r = 10;
    break;
  case 1:
  case 2:
    r = 20;
    break;
  default:
    r = 40;
    break;
  }
  return r;
}
```

both models produce (with borrows in the refcount model; the generated names
carry a `__` prefix in the real output, `__match_cond` and `__v`, so they cannot
clash with the program's variables)

```rust
'switch: {
    let match_cond = x;
    match match_cond {
        v if v == 0 => {
            r = 10;
            break 'switch;
        }
        v if v == 1 || v == 2 => {
            r = 20;
            break 'switch;
        }
        _ => {
            r = 40;
            break 'switch;
        }
    }
}
```

## Analysis

`AnalyzeSwitchArms` (`converter_lib`) walks the statements of the switch body
and groups them into `SwitchArm`s: each `case`/`default` label starts an arm,
stacked labels (`case 1: case 2:`) form one arm, and every following statement
up to the next label belongs to that arm. For each arm it records:

- `head`, the first `SwitchCase` of the chain, from which the condition is
  printed as `v == a || v == b`;
- `is_default_case`, whether a `default` is anywhere in the chain;
- `label`, a `goto` label sitting on the case, if any;
- `has_fallthrough`, whether control can reach the end of the arm: true when the
  arm is empty or its last statement is not a `break`, `continue`, `return`, or
  `goto` (looking into the last nested block).

## Emission

If no arm falls through and no arm carries a label, the switch is a labeled
block: `'switch: { let match_cond = cond; match match_cond { ... } }`.

The condition is evaluated once into `match_cond` rather than written inline: a
`match *x.borrow() { ... }` would keep the borrow of `x` alive for the whole
`match`, so any arm that writes `x` would trap at run time.

Arms are written as guards, `v if v == 1`, not as patterns, because a C `case`
accepts any integer expression: a `switch` on an `int` can mix enumerators and
literals, or use a `const` variable, none of which is a valid Rust pattern,
while an `==` comparison works for all of them.

Each C `break` becomes `break 'switch`, which leaves the block, and a missing
`default` gets an empty `_ => {}` arm so the `match` is exhaustive.

### Fallthrough

When some arm falls through or carries a label, the arms go into
`switch!(match cond { ... })` instead, and
[`break_target_`](../internals/state.md) is set to `FallthroughSwitch` so that a
`break` inside is emitted as a plain `break` for the macro to handle. The macro
turns the arms into a state machine in which an arm that does not end in `break`
continues into the next one, which is how
`case 1: r += 10; case 2: r += 20; break;` keeps its C meaning:

```rust
switch!(match x {
    v if v == 1 => {
        r += 10;
    }
    v if v == 2 => {
        r += 20;
        break;
    }
    _ => {
        r = -1;
        break;
    }
});
```

The `default` arm is always emitted last, whatever its position in the source,
since a `match` tries arms in order and `_` would otherwise shadow the cases
after it. With `switch!` this reorders the arms, so a `default` in the middle of
a fallthrough chain does not keep C's fallthrough into or out of it.

`continue` inside a switch that is inside a loop still targets the loop, since
it always emits `continue 'loop_`.

Declarations inside a case belong to that arm's block. A variable declared in
one case and used in another compiles only when the switch sits in a function
that also uses `goto`, since then all locals are hoisted (see
[Goto and Hoisting](./goto.md)).
