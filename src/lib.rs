//! This module defines a trait `UpCastAs<T>` which allows one to upcast (as in only types which make sense
//! and can fit it another are allowed) between primitive types. These follow a simple hierarchy:
//!
//! ```
//! f64 > f32 > u64 > u32 > u16 > u8
//! f64 > f32 > i64 > i32 > i16 > i8
//! ```
//!
//! Signed and unsigned types don't mix well. You can see these as implication rules, as in a type
//! which is `UpCastAs<u64>` implies it can be cast from `u32` since `u64 > u32`. And in this
//! scheme, `UpCastAs<f64>` means it can be cast from a `f64`, which would mean it can be up cast
//! from any number type.
//!
//! # Examples
//!
//! Examples of `cast`:
//!
//! ```
//! fn example<T: UpCastAs<u32>>() {
//!     let _: T = cast(10u8);
//!     let _ = cast::<u8, T>(10u8); // Alternate syntax, uglier.
//!     let _: T = cast(10u16);
//!     let _: T = cast(10u32);
//!     let _: T = cast(10u64); // Error, u64 > u32
//!     let _: T = cast(10f32); // Error, f32 > u32
//!     let _: T = cast(10f64); // Error, f32 > u32
//! }
//! ```
//!
//! `cast` is just a thin wrapper around `UpCastAs::from`:
//!
//! ```
//! fn example<T: UpCastAs<u32>>() {
//!     let _: T = UpCastAs::from(10u8);
//!     let _: T = UpCastAs::from(10u16);
//!     // ...
//! }
//! ```
//! 
//! You can also call from directly from `T`, **but it will not follow the implication rules**, it'll
//! only recognize casting from `V` if `T: UpCastAs<V>`, so this is **not recommended**:
//! 
//! ```
//! fn example<T: UpCastAs<u32>>() {
//!     let _ = T::from(10u16); // Error
//!     let _ = T::from(10u32);
//!     let _ = T::from(10u64); // Error.
//! }
//! ```
macro_rules! from_to {
    ($tr:ident, $f:ident, $t:ident) => {
        impl $tr<$f> for $t {
            fn from(x: $f) -> $t { x as $t }
        }
    }
}

pub trait UpCastAs<T> {
    fn from(T) -> Self;
}

macro_rules! cast_rule {
    ($b:ident as $a:ident) => (
        impl UpCastAs<$a> for $b {
            #[inline(always)]
            fn from(t: $a) -> $b { t as $b }
        }
    );
    ($a:ident => $b:ident) => (
        impl<U: UpCastAs<$a>> UpCastAs<$b> for U {
            #[inline(always)]
            fn from(t: $b) -> U { U::from(t as $a) }
        }
    );
    (self $a:ident) => (
        impl UpCastAs<$a> for $a {
            #[inline(always)]
            fn from(t: $a) -> $a { t }
        }
    )
}

cast_rule!(self u8);
cast_rule!(self u16);
cast_rule!(self u32);
cast_rule!(self u64);

cast_rule!(self i8);
cast_rule!(self i16);
cast_rule!(self i32);
cast_rule!(self i64);

cast_rule!(self f32);
cast_rule!(self f64);

// cast_rule!(u8 as u16);
// cast_rule!(u8 as u32);
// cast_rule!(u8 as u64);
// cast_rule!(u8 as f32);
// cast_rule!(u8 as f64);

// Implications. Pyramid.
cast_rule!(i16 => i8);
cast_rule!(i32 => i16);
cast_rule!(i64 => i32);

cast_rule!(u16 => u8);
cast_rule!(u32 => u16);
cast_rule!(u64 => u32);

cast_rule!(f32 => i64);
cast_rule!(f32 => u64);
cast_rule!(f64 => f32);

#[inline(always)]
pub fn cast<V, T: UpCastAs<V>>(v: V) -> T {
    UpCastAs::from(v)
}

#[cfg(test)]
fn doit<T: UpCastAs<u64>>() {
    let _ = T::from(10u64);
    // let y = T::from(10u8); // Error.
    let _: T = cast(10u16); // Works for all types upscalable up to `B` where `T: UpCastAs<B>`
    // let _: T = cast(10f32); // Error
    let _ = cast::<u16, T>(10u16); // Alternate syntax.
    let _: T = UpCastAs::from(10u8); // Works for all types as well.
}
