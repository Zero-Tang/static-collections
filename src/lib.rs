#![no_std]

/// A UTF-8-encoded, growable but fixed-capacity string.
/// 
/// This module contains the `StaticString` type.
pub mod string;

/// A ZST type that can be used to reference a bitmap object.
/// 
/// The module contains the `RefBitmap` type.
pub mod bitmap;