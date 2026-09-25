// The static-wstring module.

use core::{fmt, mem::MaybeUninit, ops::{Index, IndexMut}, slice::SliceIndex};

use crate::{error::InsertError, vec::StaticVec};

/// The `StaticWString` is a fixed-capacity UTF-16 string object.
#[derive(Default, Debug, Clone)]
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
	pub fn push_char(&mut self,ch:char)->Result<(),InsertError>
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
					self.internal.push(*c)?;
				}
			}
		}
		Ok(())
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
	pub fn push_str(&mut self,s:&str)->Result<(),InsertError>
	{
		for c in s.encode_utf16()
		{
			self.internal.push(c)?;
		}
		Ok(())
	}

	/// Inserts a character to the position specifed by `index`.
	/// 
	/// # Example
	/// ```
	/// use static_collections::ffi::wstring::StaticWString;
	/// use utf16_lit::utf16;
	/// let mut s:StaticWString<32>=StaticWString::from("Hello World!");
	/// s.insert_char(5,',').unwrap();
	/// assert_eq!(s.as_slice(),utf16!("Hello, World!"));
	/// ```
	pub fn insert_char(&mut self,index:usize,ch:char)->Result<(),InsertError>
	{
		if index>self.len()
		{
			return Err(InsertError::NonUtf8Boundary);
		}
		let rsvd_size=ch.len_utf16();
		if self.len()+rsvd_size>self.capacity()
		{
			return Err(InsertError::InsufficientSpace);
		}
		let mut x:MaybeUninit<[u16;2]>=MaybeUninit::uninit();
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
		Ok(())
	}

	/// Inserts a UTF-8-encoded string-slice to the position specified by `index`.
	/// 
	/// # Example
	/// ```
	/// use static_collections::ffi::wstring::StaticWString;
	/// use utf16_lit::utf16;
	/// let mut s:StaticWString<32>=StaticWString::from("123789");
	/// s.insert_str(3,"456").unwrap();
	/// assert_eq!(s.as_slice(),utf16!("123456789"));
	/// ```
	pub fn insert_str(&mut self,index:usize,s:&str)->Result<(),InsertError>
	{
		if index>self.len()
		{
			return Err(InsertError::NonUtf8Boundary);
		}
		// Use `encode_utf16` iterator twice in order to avoid dynamic allocations.
		// To avoid repeated memmoves, we need to count the number of UTF-16 characters.
		let insert_len:usize=s.encode_utf16().count();
		if self.len()+insert_len>self.capacity()
		{
			return Err(InsertError::InsufficientSpace);
		}
		let copy_range=index..self.len();
		unsafe
		{
			self.internal.force_resize(self.len()+insert_len);
		}
		self.internal.copy_within(copy_range,index+insert_len);
		for (i,c) in s.encode_utf16().enumerate()
		{
			self[index+i]=c;
		}
		Ok(())
	}

	/// Replaces all matching substrings with another string.
	/// 
	/// The `replace` method is a two-phase procedure.
	/// 1. It counts all occurences of matching substrings. Then report failure if capacity is insufficient.
	/// 2. Perform `memmove` operation in reverse order to reserve gaps for replacements, then fill in the gaps.
	/// 
	/// Note: In order to avoid dynamic allocations, you should pass a slice of UTF-16 string. \
	/// You may use `StaticWString::from` to craft a string for the sake of minimal stack-allocation.
	/// 
	/// **Caveat:** If `from` is an empty slice, then this method would unconditionally return `Ok(())`.
	/// 
	/// ## Example
	/// Here is an example that replaces all LF with CRLF:
	/// ```
	/// use static_collections::ffi::wstring::StaticWString;
	/// use utf16_lit::utf16;
	/// let mut s:StaticWString<32>=StaticWString::from("Hello\nWorld\n");
	/// s.replace(&[b'\n' as u16],&[b'\r' as u16,b'\n' as u16]).unwrap();
	/// assert_eq!(s.as_slice(),utf16!("Hello\r\nWorld\r\n"));
	/// ```
	/// Here is an example that replaces all CRLF with LF:
	/// ```
	/// use static_collections::ffi::wstring::StaticWString;
	/// use utf16_lit::utf16;
	/// let mut s:StaticWString<32>=StaticWString::from("Hello\r\nWorld\r\n");
	/// s.replace(&utf16!("\r\n"),&utf16!("\n")).unwrap();
	/// assert_eq!(s.as_slice(),utf16!("Hello\nWorld\n"));
	/// ```
	/// Here is an example that replaces all CR with LF:
	/// ```
	/// use static_collections::ffi::wstring::StaticWString;
	/// use utf16_lit::utf16;
	/// let mut s:StaticWString<32>=StaticWString::from("Hello\rWorld\r");
	/// s.replace(&utf16!("\r"),&utf16!("\n")).unwrap();
	/// assert_eq!(s.as_slice(),utf16!("Hello\nWorld\n"));
	/// ```
	pub fn replace(&mut self,from:&[u16],to:&[u16])->Result<(),InsertError>
	{
		// Caveat check.
		if from.is_empty()
		{
			return Ok(());
		}

		// For the special cases where `from` and `to` has equal length, skip the length check.
		if from.len()==to.len()
		{
			let mut i=0;
			while i+from.len()<=self.len()
			{
				if &self[i..i+from.len()]==from
				{
					self[i..i+to.len()].copy_from_slice(to);
					i+=from.len();
				}
				else
				{
					i+=1;
				}
			}
			return Ok(());
		}

		// Count the matching substrings.
		let mut matches:usize=0;
		let mut i:usize=0;
		let src=self.as_slice();
		while i+from.len()<=src.len()
		{
			if &src[i..i+from.len()]==from
			{
				matches+=1;
				i+=from.len();
			}
			else
			{
				i+=1;
			}
		}

		// Predict the new length and report failure if needed.
		let old_len=self.len();
		let new_len = if to.len()>=from.len()
		{
			old_len+matches*(to.len()-from.len())
		}
		else
		{
			old_len-matches*(from.len()-to.len())
		};
		if new_len>self.capacity()
		{
			return Err(InsertError::InsufficientSpace);
		}

		// If the replacement string is smaller, resize should be done after replacment.
		if to.len()<from.len()
		{
			let mut read=0;
			let mut write=0;
			while read<old_len
			{
				if read+from.len()<=old_len && &self[read..read+from.len()]==from
				{
					for &value in to
					{
						self[write]=value;
						write+=1;
					}
					read+=from.len();
				}
				else
				{
					self[write]=self[read];
					read+=1;
					write+=1;
				}
			}
			unsafe
			{
				self.internal.force_resize(new_len);
			}
			return Ok(());
		}

		// If the replacement string is bigger, resize should be done before replacement.
		unsafe
		{
			self.internal.force_resize(new_len);
		}
		let mut read=old_len;
		let mut write=new_len;
		while read>0
		{
			if read>=from.len() && &self[read-from.len()..read]==from
			{
				read-=from.len();
				write-=to.len();
				self[write..write+to.len()].copy_from_slice(to);
			}
			else
			{
				read-=1;
				write-=1;
				self[write]=self[read];
			}
		}
		Ok(())
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
		if let Err(e)=s.push_str(value)
		{
			panic!("{e}");
		}
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

impl<const N:usize> fmt::Display for StaticWString<N>
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		use fmt::Write;
		for c in char::decode_utf16(self.iter())
		{
			match c
			{
				Ok(c)=>f.write_char(c),
				Err(_)=>f.write_char(char::REPLACEMENT_CHARACTER)
			}?;
		}
		Ok(())
	}
}

impl<const N:usize> fmt::Write for StaticWString<N>
{
	fn write_char(&mut self, c: char) -> fmt::Result
	{
		self.push_char(c)?;
		Ok(())
	}

	fn write_str(&mut self, s: &str) -> fmt::Result
	{
		self.push_str(s)?;
		Ok(())
	}
}

impl<'a,const N:usize> StaticWString<N>
{
	// Just to make sure `char::decode_utf16` can work without cloning the whole string.
	fn iter(&'a self)->StaticWIter<'a,N>
	{
		StaticWIter
		{
			internal:StaticWIterator
			{
				index:0,
				source:self
			}
		}
	}
}

struct StaticWIterator<'a,const N:usize>
{
	index:usize,
	source:&'a StaticWString<N>
}

impl<'a,const N:usize> Iterator for StaticWIterator<'a,N>
{
	type Item = u16;

	fn next(&mut self) -> Option<Self::Item>
	{
		let i=self.index;
		if i<self.source.len()
		{
			self.index+=1;
			Some(self.source[i])
		}
		else
		{
			None
		}
	}
}

struct StaticWIter<'a,const N:usize>
{
	internal:StaticWIterator<'a,N>
}

impl<'a,const N:usize> IntoIterator for StaticWIter<'a,N>
{
	type IntoIter = StaticWIterator<'a,N>;
	type Item = u16;

	fn into_iter(self) -> Self::IntoIter
	{
		self.internal
	}
}

#[cfg(test)]
mod test
{
	extern crate std;
	use std::format;
	use super::{StaticWString, InsertError};

	#[test] fn correct_fmt()
	{
		let s:StaticWString<32>=StaticWString::from("abcd魑魅魍魉1234😀🤣😅👍");
		let ss=format!("This is {s}!");
		assert_eq!(ss,"This is abcd魑魅魍魉1234😀🤣😅👍!");
	}

	#[test] fn insert_methods_return_result()
	{
		let mut s:StaticWString<8>=StaticWString::from("abc");
		assert_eq!(s.insert_char(1,'X'),Ok(()));
		assert_eq!(s.as_slice(),['a' as u16,'X' as u16,'b' as u16,'c' as u16]);

		let mut s2:StaticWString<2>=StaticWString::from("ab");
		assert_eq!(s2.insert_char(3,'X'),Err(InsertError::NonUtf8Boundary));
		assert_eq!(s2.as_slice(),['a' as u16,'b' as u16]);

		let mut t:StaticWString<6>=StaticWString::from("ab");
		assert_eq!(t.insert_str(1,"XY"),Ok(()));
		assert_eq!(t.as_slice(),['a' as u16,'X' as u16,'Y' as u16,'b' as u16]);

		let mut t2:StaticWString<4>=StaticWString::from("ab");
		assert_eq!(t2.insert_str(1,"XYZ"),Err(InsertError::InsufficientSpace));
		assert_eq!(t2.as_slice(),['a' as u16,'b' as u16]);
	}
}