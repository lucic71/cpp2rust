# Translation Plugins

Some library calls cannot be expressed as a translation rule. `emplace_back`
perfect-forwards its arguments to a constructor of the element type, so there is
no single signature to write a rule against, and Rust has no perfect forwarding,
so each call must be translated with the constructor call written out. Such
calls are handled by a plugin, a converter method that inspects the call and
prints the translation itself. Plugins are consulted by `TryPluginConvert`
before translation rules, in `VisitCallExpr`.

The plugin infrastructure is minimal: a plugin is a pair of `Converter`
methods, `<name>_plugin_match(CallExpr *)` and `<name>_plugin_convert(CallExpr *)`,
implemented in `cpp2rust/converter/plugins/<name>.cpp`, plus virtual hooks that
`ConverterRefCount` overrides where the two models differ. Adding a plugin means
declaring the pair in `converter.h`, adding a branch to `TryPluginConvert`, and
overriding the hooks in the refcount model. This will become a proper plugin
mechanism once more perfect-forwarding and variadic functions (`make_unique`,
`printf`) need it.

## `emplace_back`

The one plugin. `emplace_back_plugin_match` matches a member call whose source
spelling contains `emplace_back`. `emplace_back_plugin_convert` takes the
element type `T` from the receiver's first template argument, runs clang's
overload resolution (`Sema`, `InitializationSequence`) on `T(args...)` to obtain
the `CXXConstructExpr` the call would build, and converts that like a variable
initializer of type `T`. The call itself becomes `push` of that value. Given

```cpp
struct Point {
  Point(int x, int y);
};
std::vector<Point> v;
v.emplace_back(1, 2);
```

the unsafe model produces

```rust
v.push(Point::Point(1, 2));
```

and the refcount model produces

```rust
(*v.borrow_mut()).push(Point::Point(1, 2));
```

A `std::move(x)` argument becomes `std::mem::take(&mut x)`, since moving out of
a `Vec` element or a field is not otherwise expressible; a POD element type with
no arguments gets its [default value](../declarations/defaults.md), and with one
argument a cast to the element type.
