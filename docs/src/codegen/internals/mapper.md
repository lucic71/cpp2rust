# The Mapper Interface

`Mapper` (`cpp2rust/converter/mapper.h`) is the converter's only view of the
translation rules. It is a namespace with process-wide state: the expression and
type rule tables filled by `LoadTranslationRules`, the model, and the
`ASTContext` of the unit being converted. How the tables are built and searched
is described in [Loading and Matching](../../rules/loading.md) and
[The Matching Engine](../../rules/matching.md); this page is about what the
converter asks the mapper for.

## Printing a construct as a rule key

Rules are keyed by the text the rule preprocessor prints for a C++ signature or
type, so the converter has to print the construct it meets in exactly the same
form before it can look it up. `ToString` is that printer, described
[below](#tostring). `ToRustName` turns such a printed type name into a Rust
identifier by replacing `<`, `>`, `,`, spaces, and `::` with `_`; it is how
records, enums, and template instantiations get their Rust names (see
[Naming](../types/naming.md)).

## Asking whether a rule applies

`Contains(QualType)` and `Contains(Expr *)` answer whether a rule matches. Every
visitor that may apply a rule asks this first and takes its ordinary path when
the answer is no, which is why the converter never has to know what the rule
tables hold. `GetExprRule` returns the matched expression rule itself when the
converter needs more than a yes: `ConvertIRFragment` walks its body fragments,
and the cast visitor compares its return type with the cast target to drop a
redundant `as`.

## Mapping types

`Map(QualType)` gives the Rust type for a C++ type, instantiating `T1`..`T9`
recursively through the same table, so `std::vector<Item>` becomes `Vec<Item>`
once `Item` is known to the mapper (see [Type Mappings](../types/mappings.md)).
`MapInitializer` gives the type rule's initializer expression, used for a
variable declared without one (see
[Default Values](../declarations/defaults.md)).

The mapper also answers a few questions about a mapped type that the converter
cannot read off the Rust text: `MapsToPointer` and `MapsToRefcountPointer` say
whether the rule's Rust type is a raw pointer or a `Ptr`, which is how an
iterator mapped to `Ptr` is recognized as a contiguous pointer; `MappedDerives`
and `SetDerives` record the `#[derive(...)]` list the converter printed for a
record, so that a later question about a field or element of that type
(`RecordDerivesCopy`) is answered from the table rather than by re-deriving it.

## User types as rules

The rule tables only know library types. `AddRuleForUserDefinedType` registers
each user record and enum, and its pointer form, as type rules when its
declaration is converted; without it `Map` could not instantiate
`std::vector<Item>`, because `T1 = Item` has to go through the same table (see
[Type Mappings](../types/mappings.md#user-defined-types-as-rules)).
`GetTypeForDecl` builds the `QualType` to register from the declaration.

## Applying an expression rule

When a call has a rule, `ConvertPlaceholder` needs to know how the rule declares
each parameter to decide how to print the argument (see
[Applying Rules](../expressions/rules.md)): `GetParamType` gives the
instantiated Rust type of parameter `i`, `ParamIsPointer` and `ReturnsPointer`
whether the parameter or the return type is a pointer, and `InstantiateTemplate`
the type bound to `Tn` at this call site for a `generic` fragment.
`IsLibcPassthrough` singles out the rules that are bare `extern` declarations,
whose call is printed as `libc::name(...)` instead of a rule body (see
[Calls](../expressions/calls.md)). `MapFunctionName` covers the case where a
function with a rule is not called but used as a value: the rule body cannot be
substituted, so the name of the runtime's function, `libcc2rs::fread_unsafe` or
`libcc2rs::fread_refcount`, is printed instead.

## Loading and context

`LoadTranslationRules` registers the built-in type mappings and loads every
`ir_<model>.json` under the rules directory. It runs once per process; the
following translation units of a `--dir` build only update the `ASTContext` the
mapper prints with. `PushASTContext` swaps that context for a scope; it is used
by `cpp-rule-preprocessor`, which runs the mapper over the rule sources
themselves.

## `ToString`

`ToString` prints a `QualType`, a `NamedDecl`, or an `Expr` in the form the rule
preprocessor writes into the Rules IR; the two sides share the printer, which is
what keeps them comparable (see
[Loading and Matching](../../rules/loading.md#matching)). Beyond the plain
cases, functions, types, and qualified names, it has a few special forms that
exist only so that a rule can be written for the construct:

- A lambda type prints as the signature of its call operator, so a rule can
  match a callable by its parameter list.
- An anonymous struct, union, or enum prints as the typedef that names it, or as
  its generated `anon_N` name (see [Naming](../types/naming.md)).
- A member access through an overloaded `operator->` prints as
  `<pointer type>-><member>`, and a field access on the loop variable of a
  range-`for` over a `std::map` prints as `<iterator type>-><member>` (see
  [Iterators](../types/special-types.md#iterators)).
- A unary operator prints as `++x`, `x++`, `-x`, and so on around the printed
  operand.
- An integer literal expanded from a macro prints as the macro name, so
  `O_RDONLY` can have a rule.
- A type is desugared by default; with `ScalarSugar::kPreserve` a typedef such
  as `size_t` is kept, so a rule written against `size_t` is tried before the
  one for `unsigned long`.
