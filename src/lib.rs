#![no_std]

/// A UTF-8-encoded, growable but fixed-capacity string.
/// 
/// This module contains the `StaticString` type.
pub mod string;

/// A ZST type that can be used to reference a bitmap object.
/// 
/// The module contains the `RefBitmap` type.
pub mod bitmap;

/// A growable but fixed-capacity array type, written as
/// `StaticVec<T>`, short for "static vector".
/// 
/// This module contains the `StaticVec` type.
pub mod vec;

/// Utilities related to FFI bindings.
/// 
/// This module contains utilities to handle data across non-Rust interfaces,
/// like other programming languages and the underlying operating system. It is
/// mainly of use for FFI (Foreign Function Interface) bindings and code that
/// needs to exchange C-like strings with other languages.
pub mod ffi;