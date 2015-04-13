This module defines a trait `UpCastAs<T>` which allows one to upcast (as in only types which make sense
and can fit it another are allowed) between primitive types. These follow a simple hierarchy:

```rust
f64 > f32 > u64 > u32 > u16 > u8
f64 > f32 > i64 > i32 > i16 > i8
```

Signed and unsigned types don't mix well. You can see these as implication rules, as in a type
which is `UpCastAs<u64>` implies it can be cast from `u32` since `u64 > u32`. And in this
scheme, `UpCastAs<f64>` means it can be cast from a `f64`, which would mean it can be up cast
from any number type.

# Examples

Examples of `cast`:

```rust
fn example<T: UpCastAs<u32>>() {
    let _: T = cast(10u8);
    let _ = cast::<u8, T>(10u8); // Alternate syntax, uglier.
    let _: T = cast(10u16);
    let _: T = cast(10u32);
    let _: T = cast(10u64); // Error, u64 > u32
    let _: T = cast(10f32); // Error, f32 > u32
    let _: T = cast(10f64); // Error, f32 > u32
}
```

`cast` is just a thin wrapper around `UpCastAs::from`:

```rust
fn example<T: UpCastAs<u32>>() {
    let _: T = UpCastAs::from(10u8);
    let _: T = UpCastAs::from(10u16);
    // ...
}
```

You can also call from directly from `T`, *but it will not follow the implication rules*, it'll
only recognize casting from `V` if `T: UpCastAs<V>`, so this is *not recommended*:

```rust
fn example<T: UpCastAs<u32>>() {
    let _ = T::from(10u16); // Error
    let _ = T::from(10u32);
    let _ = T::from(10u64); // Error.
}
```
