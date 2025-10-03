// The static-string module

use core::{fmt::{self, Debug, Display}, ops::{AddAssign, Deref, DerefMut}, str};

use crate::{ffi::c_str::strnlen, vec::StaticVec};

#[derive(Debug)]
pub struct InsertError;

/// The `StaticString` type is a fixed-capacity UTF-8 string object. \
/// To estimate length `N` you need, consider the following UTF-8 facts:
/// - 1-byte: English letters and basic punctuations.
/// - 2-byte: Latin-based, Greek, Cyrillic, Hebrew, Armenian letters and Thai characters.
/// - 3-byte: Chinese, Japanese and Korean characters.
/// - 4-byte: Emoji and rare symbols.
pub struct StaticString<const N:usize>
{
	internal:StaticVec<N,u8>
}

impl<const N:usize> Default for StaticString<N>
{
	fn default() -> Self
	{
		Self::new()
	}
}

impl<const N:usize> StaticString<N>
{
	/// Creates a new empty `StaticString`.
	/// 
	/// Given that the string is empty, the buffer that contains the string isn't initialized.
	/// This means the initial operation is very inexpensive.
	/// 
	/// # Examples
	/// ```
	/// use static_collections::string::StaticString;
	/// let s:StaticString<512>=StaticString::new();
	/// ```
	pub const fn new()->Self
	{
		Self
		{
			internal:StaticVec::new()
		}
	}

	/// Returns a byte slice of this `StaticString`'s contents.
	/// 
	/// # Examples
	/// ```
	/// use static_collections::string::StaticString;
	/// let s:StaticString<64>=StaticString::from("Hello");
	/// assert_eq!(s.as_bytes(),b"Hello");
	/// ```
	#[inline(always)] pub const fn as_bytes(&self)->&[u8]
	{
		self.internal.as_slice()
	}

	/// Returns a mutable byte slice of this `StaticString`'s contents.
	/// 
	/// # Examples
	/// ```
	/// use static_collections::string::StaticString;
	/// let mut s:StaticString<64>=StaticString::from("Hello");
	/// let array=s.as_mut_bytes();
	/// array[0]=b'C';
	/// assert_eq!(s.as_bytes(),b"Cello");
	/// ```
	#[inline(always)] pub const fn as_mut_bytes(&mut self)->&mut [u8]
	{
		self.internal.as_mut_slice()
	}

	/// Returns a string slice of this `StaticString`'s contents.
	/// 
	/// # Examples
	/// ```
	/// use static_collections::string::StaticString;
	/// let s:StaticString<64>=StaticString::from("Hello, World!");
	/// assert_eq!(s.as_str(),"Hello, World!");
	/// ```
	#[inline(always)] pub const fn as_str(&self)->&str
	{
		unsafe
		{
			str::from_utf8_unchecked(self.as_bytes())
		}
	}

	/// Returns a string slice of this `StaticString`'s contents.
	/// 
	/// # Examples
	/// ```
	/// use static_collections::string::StaticString;
	/// let mut s:StaticString<64>=StaticString::from("Hello, World!");
	/// assert_eq!(s.as_mut_str(),"Hello, World!");
	/// ```
	#[inline(always)] pub const fn as_mut_str(&mut self)->&mut str
	{
		unsafe
		{
			str::from_utf8_unchecked_mut(self.as_mut_bytes())
		}
	}

	/// Appends a given `char` to the end of this `StaticString`.
	/// 
	/// # Examples
	/// ```
	/// use static_collections::string::StaticString;
	/// let mut s:StaticString<64>=StaticString::from("Hello");
	/// s.push('!');
	/// assert_eq!(s.as_str(),"Hello!");
	/// ```
	pub fn push(&mut self,ch:char)->Result<(),InsertError>
	{
		let ch_len=ch.len_utf8();
		let insertion_index=self.len();
		if insertion_index+ch_len>N
		{
			Err(InsertError)
		}
		else
		{
			unsafe
			{
				self.internal.force_resize(insertion_index+ch_len);
			}
			ch.encode_utf8(&mut self.internal[insertion_index..]);
			Ok(())
		}
	}

	/// Appends a given string slice to the end of this `StaticString`.
	/// 
	/// # Examples
	/// ```
	/// use static_collections::string::StaticString;
	/// let mut s:StaticString<64>=StaticString::from("Hello");
	/// s.push_str(", World!");
	/// assert_eq!(s.as_str(),"Hello, World!");
	/// ```
	pub fn push_str(&mut self,string:&str)->Result<(),InsertError>
	{
		let str_len=string.len();
		let insertion_index=self.len();
		if insertion_index+str_len>N
		{
			Err(InsertError)
		}
		else
		{
			unsafe
			{
				self.internal.force_resize(insertion_index+str_len);
			}
			let dest_buff=&mut self.internal[insertion_index..];
			dest_buff.copy_from_slice(string.as_bytes());
			Ok(())
		}
	}

	/// Inserts a given `char` to the end of this `StaticString` at specified byte location `index`.
	/// 
	/// # Examples
	/// ```
	/// use static_collections::string::StaticString;
	/// let mut s:StaticString<64>=StaticString::from("Hello World!");
	/// s.insert(5,',');
	/// assert_eq!(s.as_str(),"Hello, World!");
	/// ```
	pub fn insert(&mut self,index:usize,ch:char)->Result<(),InsertError>
	{
		let ch_len=ch.len_utf8();
		let old_end=self.len();
		if old_end+ch_len>N
		{
			Err(InsertError)
		}
		else
		{
			// Move string contents.
			unsafe
			{
				self.internal.force_resize(old_end+ch_len);
			}
			self.internal.copy_within(index..old_end,index+ch_len);
			ch.encode_utf8(&mut self.internal[index..index+ch_len]);
			Ok(())
		}
	}

	/// Inserts a given string slice to the end of this `StaticString` at specified byte location `index`.
	/// 
	/// # Examples
	/// ```
	/// use static_collections::string::StaticString;
	/// let mut s:StaticString<64>=StaticString::from("Hello!");
	/// s.insert_str(5,", World");
	/// assert_eq!(s.as_str(),"Hello, World!");
	/// ```
	pub fn insert_str(&mut self,index:usize,string:&str)->Result<(),InsertError>
	{
		let str_len=string.len();
		let old_end=self.len();
		if old_end+str_len>N
		{
			Err(InsertError)
		}
		else
		{
			// Move string contents.
			unsafe
			{
				self.internal.force_resize(old_end+str_len);
			}
			self.internal.copy_within(index..old_end,index+str_len);
			self.internal[index..index+str_len].copy_from_slice(string.as_bytes());
			Ok(())
		}
	}

	/// Shortens this `StaticString` so that no null terminator is present in the string.
	/// 
	/// # Example
	/// ```
	/// use static_collections::string::StaticString;
	/// let mut s:StaticString<64>=StaticString::from("Hello,\0World!");
	/// s.truncate_to_nul();
	/// assert_eq!(s.as_str(),"Hello,");
	/// ```
	pub fn truncate_to_nul(&mut self)
	{
		unsafe
		{
			let new_size=strnlen(self.as_bytes().as_ptr().cast(),self.len());
			self.internal.force_resize(new_size);
		};
	}

	/// Shortens this `StaticString` to the specified `new_len`.
	/// 
	/// # Examples
	/// ```
	/// use static_collections::string::StaticString;
	/// let mut s:StaticString<64>=StaticString::from("Hello, World!");
	/// s.truncate(5);
	/// assert_eq!(s.as_str(),"Hello");
	/// ```
	pub fn truncate(&mut self,new_len:usize)
	{
		if new_len<=self.len()
		{
			unsafe
			{
				self.internal.force_resize(new_len);
			}
			if str::from_utf8(self.as_bytes()).is_err()
			{
				panic!("The new length {new_len} does not lie on UTF-8 character boundary!");
			}
		}
	}

	/// Removes the last character from this `StaticString` and returns it. \
	/// Returns `None` if this `StaticString` is empty.
	/// 
	/// # Examples
	/// ```
	/// use static_collections::string::StaticString;
	/// let mut s:StaticString<64>=StaticString::from("Hello!");
	/// assert_eq!(s.pop(),Some('!'));
	/// assert_eq!(s.as_str(),"Hello");
	/// ```
	pub fn pop(&mut self)->Option<char>
	{
		let s=self.as_str();
		match s.chars().last()
		{
			Some(c)=>
			{
				let new_size=self.len()-c.len_utf8();
				unsafe
				{
					self.internal.force_resize(new_size);
				}
				Some(c)
			}
			None=>None
		}
	}

	/// Removes the character from this `StaticString` specified at byte location and returns it.
	/// 
	/// # Examples
	/// ```
	/// use static_collections::string::StaticString;
	/// let mut s:StaticString<64>=StaticString::from("Hello, World!");
	/// assert_eq!(s.remove(5),',');
	/// assert_eq!(s.as_str(),"Hello World!");
	/// ```
	pub fn remove(&mut self,index:usize)->char
	{
		match str::from_utf8(&self.internal[index..])
		{
			Ok(s)=>
			{
				let c=s.chars().nth(0).unwrap();
				let ch_len=c.len_utf8();
				self.internal.copy_within(index+ch_len..,index);
				unsafe
				{
					self.internal.force_resize(self.len()-ch_len);
				}
				c
			}
			Err(_)=>panic!("Index {index} does not lie on UTF-8 character boundary!")
		}
	}

	/// Returns the length of this string in bytes.
	/// 
	/// # Examples
	/// ```
	/// use static_collections::string::StaticString;
	/// let mut s:StaticString<64>=StaticString::from("Hello, World!");
	/// assert_eq!(s.len(),13);
	/// ```
	#[inline(always)] pub fn len(&self)->usize
	{
		self.internal.len()
	}

	/// Returns the capacity of this string in bytes.
	/// 
	/// # Examples
	/// ```
	/// use static_collections::string::StaticString;
	/// let s:StaticString<128>=StaticString::new();
	/// assert_eq!(s.capacity(),128);
	/// ```
	#[inline(always)] pub fn capacity(&self)->usize
	{
		N
	}

	/// Checks if this string is empty.
	/// 
	/// # Examples
	/// ```
	/// use static_collections::string::StaticString;
	/// let mut s:StaticString<64>=StaticString::from("Hello, World!");
	/// assert_eq!(s.is_empty(),false);
	/// s=StaticString::new();
	/// assert_eq!(s.is_empty(),true);
	/// ```
	#[inline(always)] pub fn is_empty(&self)->bool
	{
		self.len()==0
	}

	/// Remove all contents of the string.
	/// 
	/// # Examples
	/// ```
	/// use static_collections::string::StaticString;
	/// let mut s:StaticString<64>=StaticString::from("Hello, World!");
	/// assert_eq!(s.is_empty(),false);
	/// s.clear();
	/// assert_eq!(s.is_empty(),true);
	/// ```
	#[inline(always)] pub fn clear(&mut self)
	{
		self.internal.clear();
	}
}

impl<const N:usize> Deref for StaticString<N>
{
	type Target = str;

	fn deref(&self) -> &Self::Target
	{
		self.as_str()
	}
}

impl<const N:usize> DerefMut for StaticString<N>
{
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		self.as_mut_str()
	}
}

impl<const N:usize> From<&str> for StaticString<N>
{
	fn from(value:&str)->Self
	{
		let mut s=Self::default();
		if s.insert_str(0,value).is_err()
		{
			panic!("String is too large!");
		}
		s
	}
}

impl<const N:usize> fmt::Write for StaticString<N>
{
	fn write_str(&mut self, s: &str) -> fmt::Result
	{
		match self.push_str(s)
		{
			Ok(())=>Ok(()),
			Err(_)=>Err(fmt::Error)
		}
	}

	fn write_char(&mut self, c: char) -> fmt::Result
	{
		match self.push(c)
		{
			Ok(_)=>Ok(()),
			Err(_)=>Err(fmt::Error)
		}
	}
}

impl<const N:usize> Display for StaticString<N>
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		f.write_str(self.as_str())
	}
}

impl<const N:usize> Debug for StaticString<N>
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		f.write_str(self.as_str())
	}
}

impl<const N:usize> AddAssign<&str> for StaticString<N>
{
	fn add_assign(&mut self, rhs: &str)
	{
		if self.push_str(rhs).is_err()
		{
			panic!("StaticString buffer Overflow!");
		}
	}
}

impl<const N:usize> PartialEq<&str> for StaticString<N>
{
	fn eq(&self,other:&&str)->bool
	{
		self.as_str().eq(*other)
	}
}

/// This routine is the internal helper function for `format_static` macro. Do not use directly.
pub fn _static_fmt_str<const N:usize>(args:fmt::Arguments)->Result<StaticString<N>,InsertError>
{
	let mut s:StaticString<N>=StaticString::new();
	match fmt::write(&mut s,args)
	{
		Ok(_)=>Ok(s),
		Err(_)=>Err(InsertError)
	}
}

/// The `format_static` macro builds a static string via format.
/// 
/// # Example
/// ```
/// use static_collections::*;
/// let s=format_static!(256,"Hello, {}!","World");
/// assert_eq!(s.unwrap(),"Hello, World!");
/// ```
#[macro_export] macro_rules! format_static
{
	($len:expr,$($arg:tt)*)=>
	{
		$crate::string::_static_fmt_str::<$len>(format_args!($($arg)*))
	};
}
