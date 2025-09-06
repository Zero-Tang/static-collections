// C-String

use core::{cmp::Ordering, ffi::CStr, fmt, mem::MaybeUninit, ops::AddAssign, slice};

/// This error is used to indicate the string is not null-terminated.
#[derive(Debug)]
pub struct NotNullTerminatedError;

// Usually, CRT routines are seriously optimized by target vendor.
unsafe extern "C"
{
	fn strncat(dest:*mut i8,src:*const i8,cch:usize)->*mut i8;
	fn strncmp(s1:*const i8,s2:*const i8,cch:usize)->isize;
	fn strncpy(dest:*mut i8,src:*const i8,cch:usize)->*mut i8;
	fn strnlen(str:*const i8,cch:usize)->usize;
}

/// A C-compatible, growable but fixed-capacity string. \
/// The exact encoding of the string depends on the target platform. \
/// The `StaticCString` guarantees a null-terminator at the end, so the maximum length is 1 less than capacity.
/// # Examples
pub struct StaticCString<const N:usize>
{
	buffer:MaybeUninit<[i8;N]>
}

impl<const N:usize> StaticCString<N>
{
	/// Creates a new empty `StaticCString`.
	/// 
	/// Given that the string is empty, the buffer that contains the string isn't initialized.
	/// This means the initial operation is very inexpensive.
	/// 
	/// # Example
	/// ```
	/// use static_collections::ffi::c_str::StaticCString;
	/// let s:StaticCString<32>=StaticCString::from(c"Hello, World!");
	/// assert_eq!(s.len(),13);
	/// ```
	pub const fn new()->Self
	{
		let mut s=Self{buffer:MaybeUninit::uninit()};
		unsafe
		{
			// Just ensure the first byte is null-character.
			s.buffer.assume_init_mut()[0]=0;
		}
		s
	}

	/// Returns the length of the static-string by using `strnlen`.
	/// 
	/// # Example
	/// ```
	/// use static_collections::ffi::c_str::StaticCString;
	/// let s:StaticCString<32>=StaticCString::from(c"Hello, World!");
	/// assert_eq!(s.len(),13);
	/// ```
	#[inline(always)] pub fn len(&self)->usize
	{
		unsafe
		{
			strnlen(self.buffer.assume_init_ref().as_ptr(),N)
		}
	}

	#[inline(always)] pub fn is_empty(&self)->bool
	{
		self.len()==0
	}

	/// Returns the capacity of the static-string.
	/// 
	/// # Example
	/// ```
	/// use static_collections::ffi::c_str::StaticCString;
	/// let s:StaticCString<32>=StaticCString::from(c"Hello, World!");
	/// assert_eq!(s.capacity(),32);
	/// ```
	#[inline(always)] pub const fn capacity(&self)->usize
	{
		N
	}

	/// Returns the immutable pointer to the first character.
	/// 
	/// The returned pointer can be passed to C-FFI routines that accepts raw pointers.
	#[inline(always)] pub const fn as_ptr(&self)->*const i8
	{
		unsafe
		{
			self.buffer.assume_init_ref().as_ptr()
		}
	}

	/// Returns the mutable pointer to the first character.
	/// 
	/// The returned pointer can be passed to C-FFI routines that accepts raw pointers.
	#[inline(always)] pub const fn as_mut_ptr(&mut self)->*mut i8
	{
		unsafe
		{
			self.buffer.assume_init_mut().as_mut_ptr()
		}
	}

	/// Returns the contents of this `StaticCString` as a slice of bytes.
	/// 
	/// The returned slice does **not** contain the trailing null-terminator, and it is
	/// guaranteed to not contain any interior null bytes. If you need this null-terminator,
	/// use `StaticCString::as_bytes_with_nul` instead.
	#[inline(always)] pub fn as_bytes(&self)->&[u8]
	{
		let l=self.len();
		unsafe
		{
			slice::from_raw_parts(self.as_ptr().cast(),l)
		}
	}

	/// Returns the contents of this `StaticCString` as a mutable slice of bytes.
	/// 
	/// The returned slice does **not** contain the trailing null-terminator, and it is
	/// guaranteed to not contain any interior null bytes. If you need this null-terminator,
	/// use `StaticCString::as_mut_bytes_with_nul` instead.
	#[inline(always)] pub fn as_mut_bytes(&mut self)->&mut [u8]
	{
		let l=self.len();
		unsafe
		{
			slice::from_raw_parts_mut(self.as_mut_ptr().cast(),l)
		}
	}

	/// Returns the contents of this `StaticCString` as a slice of bytes, including the trailing null-terminator.
	#[inline(always)] pub fn as_bytes_with_nul(&self)->&[u8]
	{
		let l=self.len();
		unsafe
		{
			slice::from_raw_parts(self.as_ptr().cast(),l+1)
		}
	}

	/// Returns the contents of this `StaticCString` as a slice of bytes, including the trailing null-terminator.
	/// 
	/// # Safety
	/// You must ensure the slice contains a null-terminator throughout the lifetime of the slice.
	#[inline(always)] pub unsafe fn as_mut_bytes_with_nul(&mut self)->&mut [u8]
	{
		let l=self.len();
		unsafe
		{
			slice::from_raw_parts_mut(self.as_mut_ptr().cast(),l+1)
		}
	}

	#[inline(always)] pub fn as_c_str(&self)->&CStr
	{
		unsafe
		{
			CStr::from_ptr(self.as_ptr())
		}
	}
}

impl<'a,const N:usize> StaticCString<N>
{
	#[inline(always)] pub fn from_raw_ptr(ptr:*const i8)->Result<&'a Self,NotNullTerminatedError>
	{
		let r:&Self=unsafe{&*ptr.cast()};
		if r.len()>=N
		{
			Err(NotNullTerminatedError)
		}
		else
		{
			Ok(r)
		}
	}
	
	#[inline(always)] pub fn from_raw_mut_ptr(ptr:*mut i8)->Result<&'a mut Self,NotNullTerminatedError>
	{
		let r:&mut Self=unsafe{&mut *ptr.cast()};
		if r.len()>=N
		{
			Err(NotNullTerminatedError)
		}
		else
		{
			Ok(r)
		}
	}
}

impl<const N:usize> From<&CStr> for StaticCString<N>
{
	fn from(value: &CStr) -> Self
	{
		let mut s=Self::new();
		unsafe
		{
			let p=value.as_ptr();
			let q=s.buffer.assume_init_mut().as_mut_ptr();
			strncpy(q,p,N);
			// Force the final byte to null-character.
			q.add(N-1).write(0);
		}
		s
	}
}

impl<const M:usize,const N:usize> PartialEq<StaticCString<M>> for StaticCString<N>
{
	fn eq(&self, other: &StaticCString<M>) -> bool
	{
		unsafe
		{
			let p=self.buffer.assume_init_ref().as_ptr();
			let q=other.buffer.assume_init_ref().as_ptr();
			strncmp(p,q,if M<N {M} else {N})==0
		}
	}
}

impl<const M:usize,const N:usize> PartialOrd<StaticCString<M>> for StaticCString<N>
{
	fn partial_cmp(&self, other: &StaticCString<M>) -> Option<Ordering>
	{
		let r=unsafe
		{
			let p=self.buffer.assume_init_ref().as_ptr();
			let q=other.buffer.assume_init_ref().as_ptr();
			strncmp(p,q,if M<N {M} else {N})
		};
		match r
		{
			..0=>Some(Ordering::Less),
			0=>Some(Ordering::Equal),
			1.. =>Some(Ordering::Greater)
		}
	}
}

impl<const M:usize,const N:usize> AddAssign<StaticCString<M>> for StaticCString<N>
{
	fn add_assign(&mut self, rhs: StaticCString<M>)
	{
		unsafe
		{
			let p=rhs.buffer.assume_init_ref().as_ptr();
			let q=self.buffer.assume_init_mut().as_mut_ptr();
			strncat(q,p,if M<N {M} else {N});
			// Force the final byte to null-character.
			q.add(N-1).write(0);
		}
	}
}

// The CString type does not implement Display trait. So won't we.
// Only Debug trait will be implemented.
impl<const N:usize> fmt::Debug for StaticCString<N>
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		self.as_c_str().fmt(f)
	}
}

impl<const N:usize> Default for StaticCString<N>
{
	fn default() -> Self
	{
		Self::new()
	}
}

unsafe impl<const N:usize> Send for StaticCString<N> {}
unsafe impl<const N:usize> Sync for StaticCString<N> {}
