# Translation Unit

Traversal starts at `VisitTranslationUnitDecl`, which walks the top-level
declarations of the unit in source order and converts the ones that pass two
filters:

- `IsUserDefinedDecl`: the declaration has a location, is not implicit, and
  comes neither from a system header nor from a system macro. Everything from
  the standard library and libc is skipped here; those APIs reach the output
  only through [translation rules](../../rules/overview.md).
- Declarations outside the main file (user headers) are converted the first time
  they are seen and skipped afterwards, keyed by `GetID` in the static set
  [`decl_ids_`](../internals/state.md). Declarations in the main file are always
  converted. In `--dir` mode this is what keeps a header included by several
  files from being emitted more than once, since every unit appends to the same
  output.

`VisitNamespaceDecl` applies the second filter to its children and converts them
in place; namespaces themselves leave no trace in the output beyond the `ns_`
prefix on names (see [Naming](../types/naming.md)).

Function definitions are converted where they appear; a function that is
declared but never defined in the input is not emitted (see
[Functions](../declarations/functions.md)). Records referenced but never defined
are collected in [`record_decls_`](../internals/state.md) and emitted as opaque
`pub struct Name;` after the last unit
([The Translation Pipeline](../pipeline.md)).

Nothing is reordered: the Rust file follows the C++ order unit by unit, which is
fine for Rust items but is why local declarations sometimes need hoisting (see
[Goto and Hoisting](./goto.md)).
