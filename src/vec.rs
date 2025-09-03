// The static-vector module

use core::{ptr, mem::MaybeUninit, slice};

pub struct StaticVec<const N:usize,T:Sized>
{
	length:usize,
	buff:MaybeUninit<[T;N]>
}

impl<const N:usize,T:Sized> Default for StaticVec<N,T>
{
	fn default() -> Self
	{
		Self::new()
	}
}

impl<const N:usize,T:Sized> StaticVec<N,T>
{
	/// Constructs a new, empty StaticVec<N,T>.
	/// 
	/// The `new` method will not zero the buffer, so the initial operation is very inexpensive.
	/// 
	/// # Example
	/// ```
	/// use static_collections::vec::StaticVec;
	/// let mut v:StaticVec<8,u64>=StaticVec::new();
	/// ```
	pub const fn new()->Self
	{
		Self
		{
			length:0,
			buff:MaybeUninit::uninit()
		}
	}

	pub const fn as_slice(&self)->&[T]
	{
		unsafe
		{
			slice::from_raw_parts(self.as_ptr(),self.length)
		}
	}

	pub const fn as_mut_slice(&mut self)->&mut [T]
	{
		unsafe
		{
			slice::from_raw_parts_mut(self.as_mut_ptr(),self.length)
		}
	}

	pub const fn as_ptr(&self)->*const T
	{
		unsafe
		{
			self.buff.assume_init_ref().as_ptr()
		}
	}

	pub const fn as_mut_ptr(&mut self)->*mut T
	{
		unsafe
		{
			self.buff.assume_init_mut().as_mut_ptr()
		}
	}

	/// Put value `v` to the end of static vector.
	/// 
	/// # Example
	/// ```
	/// use static_collections::vec::StaticVec;
	/// let mut v:StaticVec<8,u64>=StaticVec::new();
	/// v.push(1234);
	/// assert_eq!(v.as_slice(),&[1234]);
	/// v.push(4567);
	/// assert_eq!(v.as_slice(),&[1234,4567]);
	/// ```
	pub fn push(&mut self,v:T)
	{
		if self.length<N
		{
			unsafe
			{
				self.buff.assume_init_mut()[self.length]=v;
			}
			self.length+=1;
		}
	}

	/// Read and remove the value at the end of the static vector.
	/// 
	/// # Example
	/// ```
	/// use static_collections::vec::StaticVec;
	/// let mut v:StaticVec<8,u64>=StaticVec::new();
	/// v.push(1234);
	/// assert_eq!(v.pop(),1234);
	/// ```
	pub fn pop(&mut self)->T
	{
		if self.length>0
		{
			self.length-=1;
			// Use unsafe codes to avoid `Copy` trait.
			unsafe
			{
				ptr::read(self.as_ptr().add(self.length))
			}
		}
		else
		{
			panic!("popping a value from an empty static vector!");
		}
	}

	/// Insert value `v` to a specific location of static vector.
	/// 
	/// # Example
	/// ```
	/// use static_collections::vec::StaticVec;
	/// let mut v:StaticVec<8,u64>=StaticVec::new();
	/// v.push(1234);
	/// v.push(4567);
	/// v.insert(1,2333);
	/// assert_eq!(v.as_slice(),&[1234,2333,4567]);
	/// ```
	pub fn insert(&mut self,index:usize,v:T)
	{
		if self.length<N && index<=self.length
		{
			// Use unsafe codes to avoid `Copy` trait.
			unsafe
			{
				let p=self.as_mut_ptr().add(index);
				ptr::copy(p,p.add(1),self.length-index);
				ptr::write(p,v);
			}
			self.length+=1;
		}
	}

	/// Read and remove the value at location `index` of the static vector.
	/// 
	/// # Example
	/// ```
	/// use static_collections::vec::StaticVec;
	/// let mut v:StaticVec<8,u64>=StaticVec::new();
	/// v.push(1234);
	/// v.push(4567);
	/// v.push(7890);
	/// assert_eq!(v.remove(1),4567);
	/// assert_eq!(v.as_slice(),&[1234,7890]);
	/// ```
	pub fn remove(&mut self,index:usize)->T
	{
		if self.length>index
		{
			// Use unsafe codes to avoid `Copy` trait.
			unsafe
			{
				let p=self.as_mut_ptr().add(index);
				let v=ptr::read(self.as_ptr().add(index));
				ptr::copy(p.add(1),p,self.length-index-1);
				self.length-=1;
				v
			}
		}
		else
		{
			panic!("removal index ({index}) is out of bound ({})!",self.length);
		}
	}

	/// Shortens this static-vector to the specified `new_len`.
	/// 
	/// # Examples
	/// ```
	/// use static_collections::vec::StaticVec;
	/// let mut v:StaticVec<8,u64>=StaticVec::new();
	/// v.push(0);
	/// v.push(1);
	/// v.push(2);
	/// v.push(3);
	/// v.push(4);
	/// v.push(5);
	/// v.truncate(2);
	/// assert_eq!(v.as_slice(),&[0,1]);
	/// ```
	pub fn truncate(&mut self,new_len:usize)
	{
		if new_len<self.length
		{
			self.length=new_len;
		}
	}

	/// Removes all values from the static-vector.
	/// 
	/// # Example
	/// ```
	/// use static_collections::vec::StaticVec;
	/// let mut v:StaticVec<8,u64>=StaticVec::new();
	/// v.push(12345);
	/// v.push(67890);
	/// v.clear();
	/// assert_eq!(v.as_slice(),&[]);
	/// ```
	pub fn clear(&mut self)
	{
		self.length=0;
	}

	/// Checks if the static-vector is empty.
	/// 
	/// # Example
	/// ```
	/// use static_collections::vec::StaticVec;
	/// let mut v:StaticVec<8,u64>=StaticVec::new();
	/// v.push(12);
	/// v.push(34);
	/// assert!(!v.is_empty());
	/// v.clear();
	/// assert!(v.is_empty());
	/// ```
	pub fn is_empty(&self)->bool
	{
		self.len()==0
	}

	/// Returns the number of values in the static-vector.
	/// 
	/// # Example
	/// ```
	/// use static_collections::vec::StaticVec;
	/// let mut v:StaticVec<8,u64>=StaticVec::new();
	/// assert_eq!(v.len(),0);
	/// v.push(1);
	/// assert_eq!(v.len(),1);
	/// ```
	pub fn len(&self)->usize
	{
		self.length
	}

	/// Returns the capacity of the static-vector.
	/// 
	/// # Example
	/// ```
	/// use static_collections::vec::StaticVec;
	/// let v:StaticVec<12,u64>=StaticVec::new();
	/// assert_eq!(v.capacity(),12);
	/// ```
	pub fn capacity(&self)->usize
	{
		N
	}
}