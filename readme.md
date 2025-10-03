# static-collections
The `static-collections` crate aims to implement static collection types which do not require dynamic allocations. \
It should be useful to create dynamic data structures on stack or global variables.

## `StaticString<N>` type
The `StaticString<N>` type can be used to put a dynamic-length string on stack and/or global variable with maximum size of `N` bytes.

## `StaticCString<N>` type
The `StaticCString<N>` type can be used to put a dynamic-length null-terminated string on stack and/or global variable with maximum size of `N` bytes.

## `StaticWString<N>` type
The `StaticWString<N>` type can be used to put a dynamic-length UTF-16-encoded string on stack and/or global variable with maximum size of `N` UTF-16 characters.

## `StaticVec<N,T>` type
The `StaticVec<N,T>` type can be used to put a dynamic-length array on stack and/or global variable with maximum size of `N` elements.

## `RefBitmap<N>` type
The `RefBitmap<N>` is a ZST type that can be used to reference a bitmap with `N` bits. \
For x86 (including 32-bit and 64-bit) targets, bitmap operations are accelerated by special bit instructions (e.g.: `bt` instruction).

## Other types
I am no algorithm-expert. Useful data-structures in [`alloc::collections`](https://doc.rust-lang.org/alloc/collections/index.html) (e.g.: `BTreeMap`) module will not be implemented here for now. \
However, feel free to contribute.

## Compatibility
This repository supports `no_std` and does not require dynamic allocations. \
Hence, it could probably be useful for resource-limited environments (e.g.: embedded devices, windows-kernel with high IRQL, etc.)

## License
This crate is licensed under the [MIT license](./license.txt)