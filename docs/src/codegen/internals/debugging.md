# Debugging the Generator

## Verbose logging

`cpp2rust --verbose` turns on the generator's log, `log()` in
`cpp2rust/logging.h`, which otherwise goes to `llvm::nulls()`. With it on,
`stderr` shows, for every construct converted:

- the clang AST dump of each function, method, constructor, and record as it is
  entered (`decl->dump(log())`), which is the quickest way to see what the
  visitor is looking at without a separate `clang -ast-dump`;
- every `StrCat` call with the function and line that made it and the text it
  appended, so a piece of output can be traced back to the visitor that printed
  it;
- every push of an [expression kind](../expressions/kinds.md)
  (`PushExprKind <file>:<line> isRValue: ..., isAddrOf: ...` followed by the
  kind stack `[LValue, RValue, ...]` on the next line), so a wrong
  `borrow()`/`borrow_mut()` or a missing dereference can be followed to the
  caller that set the kind;
- every rule lookup, `search expr ...` / `search type ...` with the matched rule
  dumped, or `None` (see [Loading and Matching](../../rules/loading.md)).

The log is large; redirect it to a file and search for the construct.

A few heavier dumps are compiled out and can be switched on by flipping an
`#if 0` in the source, for example the dump of every loaded expression and type
rule at the end of `Mapper::LoadTranslationRules`
(`cpp2rust/converter/mapper.cpp`).

## Assertions

The generator does not try to recover from a construct it cannot translate: it
stops with an assertion, whose message names the construct (see
[Miscellaneous](../expressions/misc.md#not-handled)). Assertions are only
compiled in a `Debug` build, which is CMake's default for this project (see
[Building](../../project/building.md)); a `Release` build silently produces
wrong output instead. `ENSURE(x)` (`cpp2rust/compiler.h`) is an assertion whose
argument is still evaluated in release builds, for checks with side effects.

## Reading the output

The driver writes the output file and runs `rustfmt` on it only after every
translation unit has been converted, so an assertion leaves no output file at
all; the text produced up to that point is only visible in the `--verbose` log,
through the `StrCat` lines.

## Tests

The unit tests in `tests/unit` translate each `.cpp`/`.c` file with both models,
compare the result with `tests/unit/out/<model>/<name>.rs`, compile it, and run
it against the C++ binary (see [Test Suite](../../project/test-suite.md)). The
generated crate of a test is built in `/dev/shm/cpp2rust-tests/<name>-<model>`
(or `/tmp/cpp2rust-tests`) and removed when the test finishes, so a `rustc`
error has to be read from the lit output. To run one test, invoke `lit` on it
directly from the build directory, which is where the runner looks for
`./cpp2rust/cpp2rust`:

```bash
cd build
python3 ../tests/lit/lit.py -v ../tests/unit/foo.cpp
```
