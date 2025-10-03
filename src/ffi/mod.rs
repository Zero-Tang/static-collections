// FFI Interface

/// A null-terminated, growable, but fixed-capacity string.
/// 
/// This module contains the `StaticCString` type.
pub mod c_str;

/// A UTF-16-encoded, growable, but fixed-capacity string.
/// 
/// This module contains the `StaticWString` type. \
/// It should be helpful to pass UTF-16 strings in Windows.
pub mod wstring;