// The static-wstring module.

use core::{fmt, mem::MaybeUninit, ops::{Index, IndexMut}, slice::SliceIndex};

use crate::vec::StaticVec;

/// The `StaticWString` is a fixed-capacity UTF-16 string object.
#[derive(Default, Debug)]
pub struct StaticWString<const N:usize>
{
	internal:StaticVec<N,u16>
}

impl<const N:usize> StaticWString<N>
{
	/// Creates a new empty `StaticWString`.
	/// 
	/// Given that the string is empty, the buffer that contains the string isn't initialized.
	/// This means the initial operation is very inexpensive.
	pub const fn new()->Self
	{
		Self
		{
			internal:StaticVec::new()
		}
	}


	/// Obtains the length of this string, in number of UTF-16 characters. \
	/// If a character cannot fit in a single UTF-16 range (e.g.: emoji), it will be counted as 2 characters.
	/// 
	/// # Example
	/// ```
	/// use static_collections::ffi::wstring::StaticWString;
	/// let mut s:StaticWString<32>=StaticWString::new();
	/// s.push_char('a');
	/// assert_eq!(s.len(),1);
	/// s.push_char('😀');
	/// assert_eq!(s.len(),3);
	/// ```
	pub const fn len(&self)->usize
	{
		self.internal.len()
	}

	/// Obtains the capacity of this string, in number of UTF-16 characters.
	/// 
	/// # Example
	/// ```
	/// use static_collections::ffi::wstring::StaticWString;
	/// let s:StaticWString<32>=StaticWString::new();
	/// assert_eq!(s.capacity(),32);
	/// ```
	pub const fn capacity(&self)->usize
	{
		N
	}

	/// Checks if this string is empty.
	/// 
	/// # Example
	/// ```
	/// use static_collections::ffi::wstring::StaticWString;
	/// let mut s:StaticWString<32>=StaticWString::new();
	/// assert!(s.is_empty());
	/// s.push_char('a');
	/// s.push_char('😀');
	/// assert_eq!(s.is_empty(),false);
	/// ```
	pub const fn is_empty(&self)->bool
	{
		self.len()==0
	}

	/// Returns an immutable slice of this string in `&[u16]` form.
	/// 
	/// # Example
	/// ```
	/// use static_collections::ffi::wstring::StaticWString;
	/// let mut s:StaticWString<32>=StaticWString::new();
	/// s.push_char('😀');
	/// assert_eq!(s.as_slice(),[0xD83D,0xDE00]);
	/// ```
	pub const fn as_slice(&self)->&[u16]
	{
		self.internal.as_slice()
	}


	/// Returns a mutable slice of this string in `&mut [u16]` form.
	/// 
	/// # Example
	/// ```
	/// use static_collections::ffi::wstring::StaticWString;
	/// use utf16_lit::utf16;
	/// let mut s:StaticWString<32>=StaticWString::new();
	/// s.push_char('😀');
	/// let x=s.as_mut_slice();
	/// assert_eq!(x,[0xD83D,0xDE00]);
	/// x[0]=b'1' as u16;
	/// x[1]=b'0' as u16;
	/// assert_eq!(s,utf16!("10"));
	/// ```
	pub const fn as_mut_slice(&mut self)->&mut [u16]
	{
		self.internal.as_mut_slice()
	}

	/// Returns an immutable pointer to the first character of this string.
	pub const fn as_ptr(&self)->*const u16
	{
		self.internal.as_ptr()
	}

	/// Returns a mutable pointer to the first character of this string.
	pub const fn as_mut_ptr(&mut self)->*mut u16
	{
		self.internal.as_mut_ptr()
	}

	/// Inserts a character to the end of the string.
	/// 
	/// # Example
	/// ```
	/// use static_collections::ffi::wstring::StaticWString;
	/// let mut s:StaticWString<32>=StaticWString::new();
	/// s.push_char('a');
	/// assert_eq!(s.len(),1);
	/// assert_eq!(s.as_slice(),[b'a' as u16]);
	/// ```
	pub fn push_char(&mut self,ch:char)
	{
		let rsvd_size=ch.len_utf16();
		if self.capacity()-self.len()>rsvd_size
		{
			unsafe
			{
				let mut x:MaybeUninit<[u16;2]>=MaybeUninit::uninit();
				let u=ch.encode_utf16(x.assume_init_mut());
				for c in u
				{
					self.internal.push(*c);
				}
			}
		}
	}

	/// Inserts a UTF-8 encoded string-slice to the end of the string.
	/// 
	/// # Example
	/// ```
	/// use static_collections::ffi::wstring::StaticWString;
	/// use utf16_lit::utf16;
	/// let mut s:StaticWString<32>=StaticWString::new();
	/// s.push_str("Hello, World!");
	/// assert_eq!(s.as_slice(),utf16!("Hello, World!"));
	/// ```
	pub fn push_str(&mut self,s:&str)
	{
		for c in s.encode_utf16()
		{
			self.internal.push(c)
		}
	}

	/// Inserts a character to the position specifed by `index`.
	/// 
	/// # Example
	/// ```
	/// use static_collections::ffi::wstring::StaticWString;
	/// use utf16_lit::utf16;
	/// let mut s:StaticWString<32>=StaticWString::from("Hello World!");
	/// s.insert_char(5,',');
	/// assert_eq!(s.as_slice(),utf16!("Hello, World!"));
	/// ```
	pub fn insert_char(&mut self,index:usize,ch:char)
	{
		let mut x:MaybeUninit<[u16;2]>=MaybeUninit::uninit();
		let rsvd_size=ch.len_utf16();
		if self.capacity()-self.len()>rsvd_size
		{
			let copy_range=index..self.len();
			let u=unsafe
			{
				self.internal.force_resize(self.len()+rsvd_size);
				ch.encode_utf16(x.assume_init_mut())
			};
			self.internal.copy_within(copy_range,index+rsvd_size);
			for (i,c) in u.iter().enumerate()
			{
				self[index+i]= *c;
			}
		}
	}
	/// Inserts a UTF-8-encoded string-slice to the position specified by `index`.
	/// 
	/// # Example
	/// ```
	/// use static_collections::ffi::wstring::StaticWString;
	/// use utf16_lit::utf16;
	/// let mut s:StaticWString<32>=StaticWString::from("123789");
	/// s.insert_str(3,"456");
	/// assert_eq!(s.as_slice(),utf16!("123456789"));
	/// ```
	pub fn insert_str(&mut self,index:usize,s:&str)
	{
		// Use `encode_utf16` iterator twice in order to avoid dynamic allocations.
		// To avoid repeated memmoves, we need to count the number of UTF-16 characters.
		let insert_len:usize=s.encode_utf16().count();
		let copy_range=index..self.len();
		// May-Panic: The `force_resize` will panic if overflow.
		unsafe
		{
			self.internal.force_resize(self.len()+insert_len);
		}
		self.internal.copy_within(copy_range,index+insert_len);
		for (i,c) in s.encode_utf16().enumerate()
		{
			self[index+i]=c;
		}
	}
}

impl<I:SliceIndex<[u16]>,const N:usize> Index<I> for StaticWString<N>
{
	type Output = I::Output;

	fn index(&self, index: I) -> &Self::Output
	{
		&self.internal[index]
	}
}

impl<I:SliceIndex<[u16]>,const N:usize> IndexMut<I> for StaticWString<N>
{
	fn index_mut(&mut self, index: I) -> &mut Self::Output
	{
		&mut self.internal[index]
	}
}

impl<const N:usize> From<&str> for StaticWString<N>
{
	fn from(value: &str) -> Self
	{
		let mut s=Self::new();
		s.push_str(value);
		s
	}
}

impl<const N:usize> PartialEq<[u16]> for StaticWString<N>
{
	fn eq(&self, other: &[u16]) -> bool
	{
		self.as_slice()==other
	}
}

impl<const M:usize,const N:usize> PartialEq<[u16;M]> for StaticWString<N>
{
	fn eq(&self, other: &[u16;M]) -> bool
	{
		self.as_slice()==other
	}
}

impl<const N:usize> PartialOrd<[u16]> for StaticWString<N>
{
	fn partial_cmp(&self, other: &[u16]) -> Option<core::cmp::Ordering>
	{
		self.as_slice().partial_cmp(other)
	}
}

impl<const M:usize,const N:usize> PartialOrd<[u16;M]> for StaticWString<N>
{
	fn partial_cmp(&self, other: &[u16;M]) -> Option<core::cmp::Ordering>
	{
		self.as_slice().partial_cmp(other)
	}
}

impl<const N:usize> fmt::Write for StaticWString<N>
{
	fn write_char(&mut self, c: char) -> fmt::Result
	{
		self.push_char(c);
		Ok(())
	}

	fn write_str(&mut self, s: &str) -> fmt::Result
	{
		self.push_str(s);
		Ok(())
	}
}