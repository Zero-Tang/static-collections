// The bitmap module
use core::{ops::{Deref, DerefMut}, ptr};
#[cfg(target_arch="x86_64")]
use core::arch::{asm, x86_64::{_bittest64,_bittestandcomplement64,_bittestandreset64,_bittestandset64}};
#[cfg(target_arch="x86")]
use core::arch::{asm, x86::{_bittest,_bittestandcomplement,_bittestandreset,_bittestandset}};

use crate::error::OutOfBitmapError;

/// The ZST `RefBitmap` reference with `N` bits.
/// 
/// To create a ref-bitmap, use `from_raw_ptr` and `from_raw_mut_ptr`.
pub struct RefBitmap<const N:usize>;

impl<'a,const N:usize> RefBitmap<N>
{
	/// Creates a `RefBitmap` from raw constant pointer.
	/// 
	/// # Safety
	/// You must ensure `ptr` points to a valid buffer which has at least `N` bits!
	pub unsafe fn from_raw_ptr(ptr:*const usize)->&'a Self
	{
		unsafe
		{
			&*ptr.cast()
		}
	}
	
	/// Creates a `RefBitmap` from raw mutable pointer.
	/// 
	/// # Safety
	/// You must ensure `ptr` points to a valid buffer which has at least `N` bits!
	pub unsafe fn from_raw_mut_ptr(ptr:*mut usize)->&'a mut Self
	{
		unsafe
		{
			&mut *ptr.cast()
		}
	}
}

macro_rules! derive_test
{
	($ptr:expr,$position:expr,$limit:expr)=>
	{
		if $position<$limit
		{
			#[cfg(target_arch="x86_64")]
			{
				unsafe
				{
					Ok(_bittest64($ptr.cast(),$position as i64)!=0)
				}
			}
			#[cfg(target_arch="x86")]
			{
				unsafe
				{
					Ok(_bittest($ptr.cast(),$position as i32)!=0)
				}
			}
			#[cfg(all(not(target_arch="x86_64"),not(target_arch="x86")))]
			{
				let bmp:*const u8=$ptr.cast();
				let i=$position>>3;
				let j=$position&7;
				unsafe
				{
					Ok((bmp.add(i).read()&(1<<j))!=0)
				}
			}
		}
		else
		{
			Err(OutOfBitmapError::new($position,$limit))
		}
	};
}

macro_rules! derive_set
{
	($ptr:expr,$position:expr,$limit:expr)=>
	{
		if $position<$limit
		{
			#[cfg(target_arch="x86_64")]
			{
				unsafe
				{
					Ok(_bittestandset64($ptr.cast(),$position as i64)!=0)
				}
			}
			#[cfg(target_arch="x86")]
			{
				unsafe
				{
					Ok(_bittestandset($ptr.cast(),$position as i32)!=0)
				}
			}
			#[cfg(all(not(target_arch="x86_64"),not(target_arch="x86")))]
			{
				let bmp:*mut u8=$ptr.cast();
				let i=$position>>3;
				let j=$position&7;
				let v=1<<j;
				unsafe
				{
					let old=bmp.add(i).read();
					bmp.add(i).write(old|v);
					Ok((old&v)!=0)
				}
			}
		}
		else
		{
			Err(OutOfBitmapError::new($position,$limit))
		}
	};
}

macro_rules! derive_reset
{
	($ptr:expr,$position:expr,$limit:expr)=>
	{
		if $position<$limit
		{
			#[cfg(target_arch="x86_64")]
			{
				unsafe
				{
					Ok(_bittestandreset64($ptr.cast(),$position as i64)!=0)
				}
			}
			#[cfg(target_arch="x86")]
			{
				unsafe
				{
					Ok(_bittestandreset($ptr.cast(),$position as i32)!=0)
				}
			}
			#[cfg(all(not(target_arch="x86_64"),not(target_arch="x86")))]
			{
				let bmp:*mut u8=$ptr.cast();
				let i=$position>>3;
				let j=$position&7;
				let v=1<<j;
				unsafe
				{
					let old=bmp.add(i).read();
					bmp.add(i).write(old&!v);
					Ok((old&v)!=0)
				}
			}
		}
		else
		{
			Err(OutOfBitmapError::new($position,$limit))
		}
	};
}

macro_rules! derive_compl
{
	($ptr:expr,$position:expr,$limit:expr)=>
	{
		if $position<$limit
		{
			#[cfg(target_arch="x86_64")]
			{
				unsafe
				{
					Ok(_bittestandcomplement64($ptr.cast(),$position as i64)!=0)
				}
			}
			#[cfg(target_arch="x86")]
			{
				unsafe
				{
					Ok(_bittestandcomplement($ptr.cast(),$position as i32)!=0)
				}
			}
			#[cfg(all(not(target_arch="x86_64"),not(target_arch="x86")))]
			{
				let bmp:*mut u8=$ptr.cast();
				let i=$position>>3;
				let j=$position&7;
				let v=1<<j;
				unsafe
				{
					let old=bmp.add(i).read();
					bmp.add(i).write(old^v);
					Ok((old&v)!=0)
				}
			}
		}
		else
		{
			Err(OutOfBitmapError::new($position,$limit))
		}
	};
}

macro_rules! derive_assign
{
	($ptr:expr,$position:expr,$value:expr,$limit:expr)=>
	{
		if $position<$limit
		{
			#[cfg(target_arch="x86_64")]
			{
				// In x86-64, use the 64-bit `bts` instruction.
				unsafe
				{
					if $value
					{
						Ok(_bittestandset64($ptr.cast(),$position as i64)!=0)
					}
					else
					{
						Ok(_bittestandreset64($ptr.cast(),$position as i64)!=0)
					}
				}
			}
			#[cfg(target_arch="x86")]
			{
				// In x86, use the 32-bit `bit` instruction.
				unsafe
				{
					if $value
					{
						Ok(_bittestandset($ptr.cast(),$position as i32)!=0)
					}
					else
					{
						Ok(_bittestandreset($ptr.cast(),$position as i32)!=0)
					}
				}
			}
			#[cfg(all(not(target_arch="x86_64"),not(target_arch="x86")))]
			{
				// Unknown CPU architecture. Use the generic method.
				let bmp:*mut u8=$ptr.cast();
				let i=$position>>3;
				let j=$position&7;
				let v=1<<j;
				unsafe
				{
					let old=bmp.add(i).read();
					bmp.add(i).write(if $value {old|v} else {old&!v});
					Ok((old&v)!=0)
				}
			}
		}
		else
		{
			Err(OutOfBitmapError::new($position,$limit))
		}
	};
}

macro_rules! derive_search_set_forward
{
	($self:expr,$limit:expr)=>
	{
		{
			#[cfg(target_arch="x86_64")]
			{
				let bmp:*const u64=(&raw const *$self).cast();
				let lim=($limit>>6)+if ($limit&0x3F)!=0 {1} else {0};
				for i in 0..lim
				{
					let j:u64;
					let b:u8;
					unsafe
					{
						asm!
						(
							"mov {v},qword ptr [{p}]",
							"bsf {r},{v}",
							"setz {zf}",
							p=in(reg) bmp.add(i),
							v=out(reg) _,
							r=out(reg) j,
							zf=out(reg_byte) b
						);
					}
					if b==0
					{
						let pos=(i<<6)+j as usize;
						return if pos<$limit {Some(pos)} else {None};
					}
				}
				None
			}
			#[cfg(target_arch="x86")]
			{
				let bmp:*const u32=(&raw const *$self).cast();
				let lim=($limit>>5)+if ($limit&0x1F)!=0 {1} else {0};
				for i in 0..lim
				{
					let j:u32;
					let b:u8;
					unsafe
					{
						asm!
						(
							"bsf {r},dword ptr [{p}]",
							"setz {zf}",
							p=in(reg) bmp.add(i),
							r=out(reg) j,
							zf=out(reg_byte) b
						);
					}
					if b==0
					{
						let pos=(i<<5)+j as usize;
						return if pos<$limit {Some(pos)} else {None};
					}
				}
				None
			}
			#[cfg(all(not(target_arch="x86_64"),not(target_arch="x86")))]
			{
				for i in 0..$limit
				{
					if $self.test(i)==Ok(true)
					{
						return Some(i);
					}
				}
				None
			}
		}
	};
}

macro_rules! derive_search_set_backward
{
	($self:expr,$limit:expr)=>
	{
		{
			#[cfg(target_arch="x86_64")]
			{
				let bmp:*const u64=(&raw const *$self).cast();
				let lim=($limit>>6)+if ($limit&0x3F)!=0 {1} else {0};
				for i in (0..lim).rev()
				{
					let j:u64;
					let b:u8;
					unsafe
					{
						asm!
						(
							"mov {v},qword ptr [{p}]",
							"bsr {r},{v}",
							"setz {zf}",
							p=in(reg) bmp.add(i),
							v=out(reg) _,
							r=out(reg) j,
							zf=out(reg_byte) b
						);
					}
					if b==0
					{
						let pos=(i<<6)+j as usize;
						return if pos<$limit {Some(pos)} else {None};
					}
				}
				None
			}
			#[cfg(target_arch="x86")]
			{
				let bmp:*const u32=(&raw const *$self).cast();
				let lim=($limit>>5)+if ($limit&0x1F)!=0 {1} else {0};
				for i in (0..lim).rev()
				{
					let j:u32;
					let b:u8;
					unsafe
					{
						asm!
						(
							"bsr {r},dword ptr [{p}]",
							"setz {zf}",
							p=in(reg) bmp.add(i),
							r=out(reg) j,
							zf=out(reg_byte) b
						);
					}
					if b==0
					{
						let pos=(i<<5)+j as usize;
						return if pos<$limit {Some(pos)} else {None};
					}
				}
				None
			}
			#[cfg(all(not(target_arch="x86_64"),not(target_arch="x86")))]
			{
				for i in (0..$limit).rev()
				{
					if $self.test(i)==Ok(true)
					{
						return Some(i);
					}
				}
				None
			}
		}
	};
}

macro_rules! derive_search_cleared_forward
{
	($self:expr,$limit:expr)=>
	{
		{
			#[cfg(target_arch="x86_64")]
			{
				let bmp:*const u64=(&raw const *$self).cast();
				let lim=($limit>>6)+if ($limit&0x3F)!=0 {1} else {0};
				for i in 0..lim
				{
					let j:u64;
					let b:u8;
					unsafe
					{
						asm!
						(
							"mov {v},qword ptr [{p}]",
							"not {v}",
							"bsf {r},{v}",
							"setz {zf}",
							p=in(reg) bmp.add(i),
							v=out(reg) _,
							r=out(reg) j,
							zf=out(reg_byte) b
						);
					}
					if b==0
					{
						let pos=(i<<6)+j as usize;
						return if pos<$limit {Some(pos)} else {None};
					}
				}
				None
			}
			#[cfg(target_arch="x86")]
			{
				let bmp:*const u32=(&raw const *$self).cast();
				let lim=($limit>>5)+if ($limit&0x1F)!=0 {1} else {0};
				for i in 0..lim
				{
					let j:u32;
					let b:u8;
					unsafe
					{
						asm!
						(
							"mov {v},dword ptr [{p}]",
							"not {v}",
							"bsf {r},{v}",
							"setz {zf}",
							p=in(reg) bmp.add(i),
							v=out(reg) _,
							r=out(reg) j,
							zf=out(reg_byte) b
						);
					}
					if b==0
					{
						let pos=(i<<5)+j as usize;
						return if pos<$limit {Some(pos)} else {None};
					}
				}
				None
			}
			#[cfg(all(not(target_arch="x86_64"),not(target_arch="x86")))]
			{
				for i in 0..$limit
				{
					if $self.test(i)==Ok(false)
					{
						return Some(i);
					}
				}
				None
			}
		}
	};
}

macro_rules! derive_search_cleared_backward
{
	($self:expr,$limit:expr)=>
	{
		{
			#[cfg(target_arch="x86_64")]
			{
				let bmp:*const u64=(&raw const *$self).cast();
				let lim=($limit>>6)+if ($limit&0x3F)!=0 {1} else {0};
				for i in (0..lim).rev()
				{
					let j:u64;
					let b:u8;
					unsafe
					{
						asm!
						(
							"mov {v},qword ptr [{p}]",
							"not {v}",
							"bsr {r},{v}",
							"setz {zf}",
							p=in(reg) bmp.add(i),
							v=out(reg) _,
							r=out(reg) j,
							zf=out(reg_byte) b
						);
					}
					if b==0
					{
						let pos=(i<<6)+j as usize;
						return if pos<$limit {Some(pos)} else {None};
					}
				}
				None
			}
			#[cfg(target_arch="x86")]
			{
				let bmp:*const u32=(&raw const *$self).cast();
				let lim=($limit>>5)+if ($limit&0x1F)!=0 {1} else {0};
				for i in (0..lim).rev()
				{
					let j:u32;
					let b:u8;
					unsafe
					{
						asm!
						(
							"mov {v},dword ptr [{p}]",
							"not {v}",
							"bsr {r},{v}",
							"setz {zf}",
							p=in(reg) bmp.add(i),
							v=out(reg) _,
							r=out(reg) j,
							zf=out(reg_byte) b
						);
					}
					if b==0
					{
						let pos=(i<<5)+j as usize;
						return if pos<$limit {Some(pos)} else {None};
					}
				}
				None
			}
			#[cfg(all(not(target_arch="x86_64"),not(target_arch="x86")))]
			{
				for i in (0..$limit).rev()
				{
					if $self.test(i)==Ok(false)
					{
						return Some(i);
					}
				}
				None
			}
		}
	};
}

impl<const N:usize> RefBitmap<N>
{
	/// Tests if a position in the bitmap is set. \
	/// Returns `Ok(bool) if `position<N`. The `bool` specifies whether the bit is set or not. \
	/// Returns `Err(OutOfBitmapError) if `position>=N`.
	/// 
	/// # Example
	/// ```
	/// use static_collections::{bitmap::*,error::OutOfBitmapError};
	/// let bmp_raw:[u64;4]=[0,1,2,4];
	/// let bmp:&RefBitmap<256>=unsafe{RefBitmap::from_raw_ptr(bmp_raw.as_ptr().cast())};
	/// assert_eq!(bmp.test(36),Ok(false));
	/// assert_eq!(bmp.test(64),Ok(true));
	/// assert_eq!(bmp.test(194),Ok(true));
	/// assert_eq!(bmp.test(288),Err(OutOfBitmapError::new(288,256)))
	/// ```
	pub fn test(&self,position:usize)->Result<bool,OutOfBitmapError>
	{
		derive_test!(&raw const *self,position,N)
	}

	/// Tests and also assigns `true` to a position in the bitmap and returns the previous value. \
	/// Returns `Ok(bool) if `position<N`. The `bool` specifies whether the bit is set or not. \
	/// Returns `Err(OutOfBitmapError) if `position>=N`.
	/// 
	/// # Example
	/// ```
	/// use static_collections::bitmap::RefBitmap;
	/// let mut bmp_raw:[u64;4]=[0;4];
	/// let mut bmp:&mut RefBitmap<256>=unsafe{RefBitmap::from_raw_mut_ptr(bmp_raw.as_mut_ptr().cast())};
	/// assert_eq!(bmp.set(36),Ok(false));
	/// assert_eq!(bmp.test(36),Ok(true));
	/// ```
	pub fn set(&mut self,position:usize)->Result<bool,OutOfBitmapError>
	{
		derive_set!(&raw mut *self,position,N)
	}

	/// Tests and also assigns `false` to a position in the bitmap and returns the previous value. \
	/// Returns `Ok(bool) if `position<N`. The `bool` specifies whether the bit is set or not. \
	/// Returns `Err(OutOfBitmapError) if `position>=N`.
	/// 
	/// # Example
	/// ```
	/// use static_collections::bitmap::RefBitmap;
	/// let mut bmp_raw:[u64;4]=[u64::MAX;4];
	/// let mut bmp:&mut RefBitmap<256>=unsafe{RefBitmap::from_raw_mut_ptr(bmp_raw.as_mut_ptr().cast())};
	/// assert_eq!(bmp.reset(236),Ok(true));
	/// assert_eq!(bmp.test(236),Ok(false));
	/// ```
	pub fn reset(&mut self,position:usize)->Result<bool,OutOfBitmapError>
	{
		derive_reset!(&raw mut *self,position,N)
	}

	/// Tests and also complements the bit in the position in the bitmap and returns the previous value. \
	/// Returns `Ok(bool) if `position<N`. The `bool` specifies whether the bit is set or not. \
	/// Returns `Err(OutOfBitmapError) if `position>=N`.
	/// 
	/// # Example
	/// ```
	/// use static_collections::bitmap::RefBitmap;
	/// let mut bmp_raw:[u64;4]=[0;4];
	/// let mut bmp:&mut RefBitmap<256>=unsafe{RefBitmap::from_raw_mut_ptr(bmp_raw.as_mut_ptr().cast())};
	/// assert_eq!(bmp.complement(123),Ok(false));
	/// assert_eq!(bmp.complement(123),Ok(true));
	/// ```
	pub fn complement(&mut self,position:usize)->Result<bool,OutOfBitmapError>
	{
		derive_compl!(&raw mut *self,position,N)
	}

	/// Tests and also assigns `value` to the bit in the position in the bitmap and returns the previous value. \
	/// Returns `Ok(bool) if `position<N`. The `bool` specifies whether the bit is set or not. \
	/// Returns `Err(OutOfBitmapError) if `position>=N`.
	/// 
	/// # Example
	/// ```
	/// use static_collections::bitmap::RefBitmap;
	/// let mut bmp_raw:[u64;4]=[0;4];
	/// let mut bmp:&mut RefBitmap<256>=unsafe{RefBitmap::from_raw_mut_ptr(bmp_raw.as_mut_ptr().cast())};
	/// assert_eq!(bmp.assign(123,true),Ok(false));
	/// assert_eq!(bmp.assign(123,true),Ok(true));
	/// assert_eq!(bmp.assign(123,false),Ok(true));
	/// ```
	pub fn assign(&mut self,position:usize,value:bool)->Result<bool,OutOfBitmapError>
	{
		derive_assign!(&raw mut *self,position,value,N)
	}

	/// Search for a cleared bit in the bitmap in forward direction. \
	/// Returns `Some(usize)` if there is a cleared bit. \
	/// Returns `None` if all bits in bitmap are set.
	/// 
	/// # Example
	/// ```
	/// use static_collections::bitmap::RefBitmap;
	/// let bmp_raw:[u64;4]=[u64::MAX,0x7,0,0];
	/// let bmp:&RefBitmap<256>=unsafe{RefBitmap::from_raw_ptr(bmp_raw.as_ptr().cast())};
	/// assert_eq!(bmp.search_cleared_forward(),Some(67));
	/// ```
	pub fn search_cleared_forward(&self)->Option<usize>
	{
		derive_search_cleared_forward!(self,N)
	}

	/// Search for a set bit in the bitmap in forward direction. \
	/// Returns `Some(usize)` if there is a set bit. \
	/// Returns `None` if all bits in bitmap are cleared.
	/// 
	/// # Example
	/// ```
	/// use static_collections::bitmap::RefBitmap;
	/// let bmp_raw:[u64;4]=[0,0,1,0];
	/// let bmp:&RefBitmap<256>=unsafe{RefBitmap::from_raw_ptr(bmp_raw.as_ptr().cast())};
	/// assert_eq!(bmp.search_set_forward(),Some(128));
	/// ```
	pub fn search_set_forward(&self)->Option<usize>
	{
		derive_search_set_forward!(self,N)
	}

	/// Search for a cleared bit in the bitmap in backward direction. \
	/// Returns `Some(usize)` if there is a cleared bit. \
	/// Returns `None` if all bits in bitmap are set.
	/// 
	/// # Example
	/// ```
	/// use static_collections::bitmap::RefBitmap;
	/// let bmp_raw:[u64;4]=[u64::MAX,0x7,0,u64::MAX];
	/// let bmp:&RefBitmap<256>=unsafe{RefBitmap::from_raw_ptr(bmp_raw.as_ptr().cast())};
	/// assert_eq!(bmp.search_cleared_backward(),Some(191));
	/// ```
	pub fn search_cleared_backward(&self)->Option<usize>
	{
		derive_search_cleared_backward!(self,N)
	}

	/// Search for a set bit in the bitmap in backward direction. \
	/// Returns `Some(usize)` if there is a set bit. \
	/// Returns `None` if all bits in bitmap are cleared.
	/// 
	/// # Example
	/// ```
	/// use static_collections::bitmap::RefBitmap;
	/// let bmp_raw:[u64;4]=[0,0,1,2];
	/// let bmp:&RefBitmap<256>=unsafe{RefBitmap::from_raw_ptr(bmp_raw.as_ptr().cast())};
	/// assert_eq!(bmp.search_set_backward(),Some(193));
	/// ```
	pub fn search_set_backward(&self)->Option<usize>
	{
		derive_search_set_backward!(self,N)
	}
}

pub struct BitmapSlice([u8]);

impl BitmapSlice
{
	/// Creates an immutable slice of bitmap that has `len*8` bits.
	/// 
	/// ## Safety
	/// You must ensure `ptr` points to a valid bitmap that has `len` bytes.
	pub unsafe fn from_raw_parts<'a>(ptr:*const u8,len:usize)->&'a Self
	{
		unsafe
		{
			&*(ptr::slice_from_raw_parts(ptr,len) as *const Self)
		}
	}

	/// Creates a mutable slice of bitmap that has `len*8` bits.
	/// 
	/// ## Safety
	/// You must ensure `ptr` points to a valid mutable bitmap that has `len` bytes.
	pub unsafe fn from_raw_parts_mut<'a>(ptr:*mut u8,len:usize)->&'a mut Self
	{
		unsafe
		{
			&mut *(ptr::slice_from_raw_parts_mut(ptr,len) as *mut Self)
		}
	}

	/// Tests if a position in the bitmap slice is set. \
	/// Returns `Ok(bool)` if `position<self.len()`. The `bool` specifies whether the bit is set or not. \
	/// Returns `Err(OutOfBitmapError)` if `position>=self.len()`.
	/// 
	/// # Example
	/// ```
	/// use static_collections::{bitmap::*,error::OutOfBitmapError};
	/// let raw:[u8;2]=[0b00000001,0b10000000];
	/// let bmp:&BitmapSlice=unsafe{BitmapSlice::from_raw_parts(raw.as_ptr(),raw.len())};
	/// assert_eq!(bmp.test(0),Ok(true));
	/// assert_eq!(bmp.test(7),Ok(false));
	/// assert_eq!(bmp.test(15),Ok(true));
	/// assert_eq!(bmp.test(16),Err(OutOfBitmapError::new(16,16)))
	/// ```
	pub fn test(&self,position:usize)->Result<bool,OutOfBitmapError>
	{
		derive_test!(self.as_ptr(),position,self.len())
	}

	/// Tests and also assigns `true` to a position in the bitmap slice and returns the previous value. \
	/// Returns `Ok(bool)` if `position<self.len()`. The `bool` specifies whether the bit was set or not. \
	/// Returns `Err(OutOfBitmapError)` if `position>=self.len()`.
	/// 
	/// # Example
	/// ```
	/// use static_collections::bitmap::BitmapSlice;
	/// let mut raw:[u8;2]=[0;2];
	/// let mut bmp:&mut BitmapSlice=unsafe{BitmapSlice::from_raw_parts_mut(raw.as_mut_ptr(),raw.len())};
	/// assert_eq!(bmp.set(4),Ok(false));
	/// assert_eq!(bmp.test(4),Ok(true));
	/// ```
	pub fn set(&mut self,position:usize)->Result<bool,OutOfBitmapError>
	{
		derive_set!(self.as_mut_ptr(),position,self.len())
	}

	/// Tests and also assigns `false` to a position in the bitmap slice and returns the previous value. \
	/// Returns `Ok(bool)` if `position<self.len()`. The `bool` specifies whether the bit was set or not. \
	/// Returns `Err(OutOfBitmapError)` if `position>=self.len()`.
	/// 
	/// # Example
	/// ```
	/// use static_collections::bitmap::BitmapSlice;
	/// let mut raw:[u8;2]=[0xFF,0xFF];
	/// let mut bmp:&mut BitmapSlice=unsafe{BitmapSlice::from_raw_parts_mut(raw.as_mut_ptr(),raw.len())};
	/// assert_eq!(bmp.reset(9),Ok(true));
	/// assert_eq!(bmp.test(9),Ok(false));
	/// ```
	pub fn reset(&mut self,position:usize)->Result<bool,OutOfBitmapError>
	{
		derive_reset!(self.as_mut_ptr(),position,self.len())
	}

	/// Tests and also complements the bit in the position in the bitmap slice and returns the previous value. \
	/// Returns `Ok(bool)` if `position<self.len()`. The `bool` specifies whether the bit was set or not. \
	/// Returns `Err(OutOfBitmapError)` if `position>=self.len()`.
	/// 
	/// # Example
	/// ```
	/// use static_collections::bitmap::BitmapSlice;
	/// let mut raw:[u8;2]=[0;2];
	/// let mut bmp:&mut BitmapSlice=unsafe{BitmapSlice::from_raw_parts_mut(raw.as_mut_ptr(),raw.len())};
	/// assert_eq!(bmp.complement(5),Ok(false));
	/// assert_eq!(bmp.complement(5),Ok(true));
	/// ```
	pub fn complement(&mut self,position:usize)->Result<bool,OutOfBitmapError>
	{
		derive_compl!(self.as_mut_ptr(),position,self.len())
	}

	/// Tests and also assigns `value` to the bit in the position in the bitmap slice and returns the previous value. \
	/// Returns `Ok(bool)` if `position<self.len()`. The `bool` specifies whether the bit was set or not. \
	/// Returns `Err(OutOfBitmapError)` if `position>=self.len()`.
	/// 
	/// # Example
	/// ```
	/// use static_collections::bitmap::BitmapSlice;
	/// let mut raw:[u8;2]=[0;2];
	/// let mut bmp:&mut BitmapSlice=unsafe{BitmapSlice::from_raw_parts_mut(raw.as_mut_ptr(),raw.len())};
	/// assert_eq!(bmp.assign(12,true),Ok(false));
	/// assert_eq!(bmp.assign(12,false),Ok(true));
	/// ```
	pub fn assign(&mut self,position:usize,value:bool)->Result<bool,OutOfBitmapError>
	{
		derive_assign!(self.as_mut_ptr(),position,value,self.len())
	}

	/// Search for a cleared bit in the bitmap slice in forward direction. \
	/// Returns `Some(usize)` if there is a cleared bit. \
	/// Returns `None` if all bits in the bitmap slice are set.
	/// 
	/// # Example
	/// ```
	/// use static_collections::bitmap::BitmapSlice;
	/// let raw:[u8;2]=[0xFF,0x7F];
	/// let bmp:&BitmapSlice=unsafe{BitmapSlice::from_raw_parts(raw.as_ptr(),raw.len())};
	/// assert_eq!(bmp.search_cleared_forward(),Some(15));
	/// ```
	pub fn search_cleared_forward(&self)->Option<usize>
	{
		derive_search_cleared_forward!(self,self.len())
	}

	/// Search for a set bit in the bitmap slice in forward direction. \
	/// Returns `Some(usize)` if there is a set bit. \
	/// Returns `None` if all bits in the bitmap slice are cleared.
	/// 
	/// # Example
	/// ```
	/// use static_collections::bitmap::BitmapSlice;
	/// let raw:[u8;8]=[0,0,0,0,0,0,1,0];
	/// let bmp:&BitmapSlice=unsafe{BitmapSlice::from_raw_parts(raw.as_ptr(),raw.len())};
	/// assert_eq!(bmp.search_set_forward(),Some(48));
	/// ```
	pub fn search_set_forward(&self)->Option<usize>
	{
		derive_search_set_forward!(self,self.len())
	}

	/// Search for a cleared bit in the bitmap slice in backward direction. \
	/// Returns `Some(usize)` if there is a cleared bit. \
	/// Returns `None` if all bits in the bitmap slice are set.
	/// 
	/// # Example
	/// ```
	/// use static_collections::bitmap::BitmapSlice;
	/// let raw:[u8;8]=[0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0x00];
	/// let bmp:&BitmapSlice=unsafe{BitmapSlice::from_raw_parts(raw.as_ptr(),raw.len())};
	/// assert_eq!(bmp.search_cleared_backward(),Some(63));
	/// ```
	pub fn search_cleared_backward(&self)->Option<usize>
	{
		derive_search_cleared_backward!(self,self.len())
	}

	/// Search for a set bit in the bitmap slice in backward direction. \
	/// Returns `Some(usize)` if there is a set bit. \
	/// Returns `None` if all bits in the bitmap slice are cleared.
	/// 
	/// # Example
	/// ```
	/// use static_collections::bitmap::BitmapSlice;
	/// let raw:[u8;8]=[0,0,0,0,0,0,0,0b10000000];
	/// let bmp:&BitmapSlice=unsafe{BitmapSlice::from_raw_parts(raw.as_ptr(),raw.len())};
	/// assert_eq!(bmp.search_set_backward(),Some(63));
	/// ```
	pub fn search_set_backward(&self)->Option<usize>
	{
		derive_search_set_backward!(self,self.len())
	}

	pub fn as_ptr(&self)->*const u8
	{
		self.0.as_ptr()
	}

	pub fn as_mut_ptr(&mut self)->*mut u8
	{
		self.0.as_mut_ptr()
	}

	pub fn len(&self)->usize
	{
		self.0.len()<<3
	}

	pub fn is_empty(&self)->bool
	{
		self.0.is_empty()
	}

	pub fn as_slice(&self)->&[u8]
	{
		&self.0
	}

	pub fn as_mut_slice(&mut self)->&mut [u8]
	{
		&mut self.0
	}
}

impl Deref for BitmapSlice
{
	type Target=[u8];

	fn deref(&self)->&Self::Target
	{
		&self.0
	}
}

impl DerefMut for BitmapSlice
{
	fn deref_mut(&mut self)->&mut Self::Target
	{
		&mut self.0
	}
}

#[cfg(test)] mod tests
{
	use super::BitmapSlice;

	#[test] fn bitmap_slice_from_raw_parts_round_trips()
	{
		let words=[1,2,3,4];
		let slice: &BitmapSlice = unsafe { BitmapSlice::from_raw_parts(words.as_ptr(), words.len()) };
		assert_eq!(slice.len(), 4*8);
		assert_eq!(slice.as_slice(), &words);
		assert_eq!(slice.as_ptr(), words.as_ptr());

		let mut words=[1,2,3,4];
		let slice: &mut BitmapSlice = unsafe { BitmapSlice::from_raw_parts_mut(words.as_mut_ptr(), words.len()) };
		slice[0] = 9;
		assert_eq!(slice.as_slice(), &[9,2,3,4]);
		assert_eq!(words, [9,2,3,4]);
	}

	#[test] fn bitmap_slice_searches_use_shared_macros()
	{
		let mut data=[0u8; 8];
		data[0] = 0b00000001;
		data[1] = 0b10000000;
		let slice: &mut BitmapSlice = unsafe { BitmapSlice::from_raw_parts_mut(data.as_mut_ptr(), data.len()) };

		assert_eq!(slice.search_set_forward(), Some(0));
		assert_eq!(slice.search_set_backward(), Some(15));
		assert_eq!(slice.search_cleared_forward(), Some(1));
		assert_eq!(slice.search_cleared_backward(), Some(63));
	}
}