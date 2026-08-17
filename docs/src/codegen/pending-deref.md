# Pending Dereferences

> TODO: explain `pending_deref_` in `ConverterRefCount`: how an lvalue use of a
> pointer dereference records the pointer expression instead of printing it,
> which consumers take it (`EmitSetOrAssign` emitting `ptr.write(rhs)`,
> `ConvertMappedMethodCall` emitting `ptr.with_mut(...)`), the boxed-pointee
> flag, and the `assert_consumed` check after every statement.
