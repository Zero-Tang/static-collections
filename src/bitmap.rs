// The bitmap module
use core::fmt;
#[cfg(target_arch="x86_64")]
use core::arch::{asm, x86_64::{_bittest64,_bittestandcomplement64,_bittestandreset64,_bittestandset64}};

#[derive(PartialEq, Debug)]
pub struct OutOfBitmapError
{
	position:usize,
	limit:usize
}

impl OutOfBitmapError
{
	pub const fn new(position:usize,limit:usize)->Self
	{
		Self
		{
			position,
			limit
		}
	}
}

impl fmt::Display for OutOfBitmapError
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f,"bit {} is out of bitmap's limit {}",self.position,self.limit)
	}
}

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

impl<const N:usize> RefBitmap<N>
{
	/// Tests if a position in the bitmap is set. \
	/// Returns `Ok(bool) if `position<N`. The `bool` specifies whether the bit is set or not. \
	/// Returns `Err(OutOfBitmapError) if `position>=N`.
	/// 
	/// # Example
	/// ```
	/// use static_collections::bitmap::*;
	/// let bmp_raw:[u64;4]=[0,1,2,4];
	/// let bmp:&RefBitmap<256>=unsafe{RefBitmap::from_raw_ptr(bmp_raw.as_ptr().cast())};
	/// assert_eq!(bmp.test(36),Ok(false));
	/// assert_eq!(bmp.test(64),Ok(true));
	/// assert_eq!(bmp.test(194),Ok(true));
	/// assert_eq!(bmp.test(288),Err(OutOfBitmapError::new(288,256)))
	/// ```
	pub fn test(&self,position:usize)->Result<bool,OutOfBitmapError>
	{
		if position<N
		{
			#[cfg(target_arch="x86_64")]
			{
				// In x86-64, use the 64-bit `bt` instruction.
				let bmp:*const i64=(&raw const *self).cast();
				unsafe
				{
					Ok(_bittest64(bmp,position as i64)!=0)
				}
			}
			#[cfg(not(target_arch="x86_64"))]
			{
				// Unknown CPU architecture. Use the generic method.
				let bmp:*const u32=(&raw const *self).cast();
				let i=position>>5;
				let j=position&0x1F;
				unsafe
				{
					Ok((bmp.add(i).read_unaligned()&(1<<j))!=0)
				}
			}
		}
		else
		{
			Err(OutOfBitmapError::new(position,N))
		}
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
		if position<N
		{
			#[cfg(target_arch="x86_64")]
			{
				// In x86-64, use the 64-bit `bts` instruction.
				let bmp:*mut i64=(&raw mut *self).cast();
				unsafe
				{
					Ok(_bittestandset64(bmp,position as i64)!=0)
				}
			}
			#[cfg(not(target_arch="x86_64"))]
			{
				// Unknown CPU architecture. Use the generic method.
				let bmp:*mut u32=(&raw mut *self).cast();
				let i=position>>5;
				let j=position&0x1F;
				let v=1<<j;
				unsafe
				{
					let old=bmp.add(i).read_unaligned();
					bmp.add(i).write_unaligned(old|v);
					Ok((old&v)!=0)
				}
			}
		}
		else
		{
			Err(OutOfBitmapError::new(position,N))
		}
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
		if position<N
		{
			#[cfg(target_arch="x86_64")]
			{
				// In x86-64, use the 64-bit `bts` instruction.
				let bmp:*mut i64=(&raw mut *self).cast();
				unsafe
				{
					Ok(_bittestandreset64(bmp,position as i64)!=0)
				}
			}
			#[cfg(not(target_arch="x86_64"))]
			{
				// Unknown CPU architecture. Use the generic method.
				let bmp:*mut u32=(&raw mut *self).cast();
				let i=position>>5;
				let j=position&0x1F;
				let v=1<<j;
				unsafe
				{
					let old=bmp.add(i).read_unaligned();
					bmp.add(i).write_unaligned(old&!v);
					Ok((old&v)!=0)
				}
			}
		}
		else
		{
			Err(OutOfBitmapError::new(position,N))
		}
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
		if position<N
		{
			#[cfg(target_arch="x86_64")]
			{
				// In x86-64, use the 64-bit `bts` instruction.
				let bmp:*mut i64=(&raw mut *self).cast();
				unsafe
				{
					Ok(_bittestandcomplement64(bmp,position as i64)!=0)
				}
			}
			#[cfg(not(target_arch="x86_64"))]
			{
				// Unknown CPU architecture. Use the generic method.
				let bmp:*mut u32=(&raw mut *self).cast();
				let i=position>>5;
				let j=position&0x1F;
				let v=1<<j;
				unsafe
				{
					let old=bmp.add(i).read_unaligned();
					let r=(old&v)!=0;
					bmp.add(i).write_unaligned(if r {old&!v} else {old|v});
					Ok(r)
				}
			}
		}
		else
		{
			Err(OutOfBitmapError::new(position,N))
		}
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
		if position<N
		{
			#[cfg(target_arch="x86_64")]
			{
				// In x86-64, use the 64-bit `bts` instruction.
				let bmp:*mut i64=(&raw mut *self).cast();
				unsafe
				{
					if value
					{
						Ok(_bittestandset64(bmp,position as i64)!=0)
					}
					else
					{
						Ok(_bittestandreset64(bmp,position as i64)!=0)
					}
				}
			}
			#[cfg(not(target_arch="x86_64"))]
			{
				// Unknown CPU architecture. Use the generic method.
				let bmp:*mut u32=(&raw mut *self).cast();
				let i=position>>5;
				let j=position&0x1F;
				let v=1<<j;
				unsafe
				{
					let old=bmp.add(i).read_unaligned();
					bmp.add(i).write_unaligned(if value {old|v} else {old&!v});
					Ok((old&v)!=0)
				}
			}
		}
		else
		{
			Err(OutOfBitmapError::new(position,N))
		}
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
		#[cfg(target_arch="x86_64")]
		{
			let bmp:*const u64=(&raw const *self).cast();
			let lim=(N>>6)+if (N&0x3F)!=0 {1} else {0};
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
					return if pos<N {Some(pos)} else {None};
				}
			}
			None
		}
		#[cfg(not(target_arch="x86_64"))]
		{
			for i in 0..N
			{
				if self.test(i)==Ok(false)
				{
					return Some(i);
				}
			}
			None
		}
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
		#[cfg(target_arch="x86_64")]
		{
			let bmp:*const u64=(&raw const *self).cast();
			let lim=(N>>6)+if (N&0x3F)!=0 {1} else {0};
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
					return if pos<N {Some(pos)} else {None};
				}
			}
			None
		}
		#[cfg(not(target_arch="x86_64"))]
		{
			for i in 0..N
			{
				if self.test(i)==Ok(true)
				{
					return Some(i);
				}
			}
			None
		}
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
		#[cfg(target_arch="x86_64")]
		{
			let bmp:*const u64=(&raw const *self).cast();
			let lim=(N>>6)+if (N&0x3F)!=0 {1} else {0};
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
					return if pos<N {Some(pos)} else {None};
				}
			}
			None
		}
		#[cfg(not(target_arch="x86_64"))]
		{
			for i in (0..N).rev()
			{
				if self.test(i)==Ok(false)
				{
					return Some(i);
				}
			}
			None
		}
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
		#[cfg(target_arch="x86_64")]
		{
			let bmp:*const u64=(&raw const *self).cast();
			let lim=(N>>6)+if (N&0x3F)!=0 {1} else {0};
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
					return if pos<N {Some(pos)} else {None};
				}
			}
			None
		}
		#[cfg(not(target_arch="x86_64"))]
		{
			for i in (0..N).rev()
			{
				if self.test(i)==Ok(true)
				{
					return Some(i);
				}
			}
			None
		}
	}
}