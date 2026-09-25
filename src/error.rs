// The error module

use core::{char::DecodeUtf16Error, error::Error, fmt::{self, Display}};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InsertError
{
	InsufficientSpace,
	NonUtf8Boundary,
	Utf16Error(DecodeUtf16Error)
}

impl Error for InsertError {}

impl From<InsertError> for fmt::Error
{
	fn from(_value:InsertError)->Self
	{
		Self
	}
}

impl Display for InsertError
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match self
		{
			Self::InsufficientSpace=>f.write_str("insufficient space"),
			Self::NonUtf8Boundary=>f.write_str("not on utf8 boundary"),
			Self::Utf16Error(x)=>write!(f,"{x}")
		}
	}
}

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

impl Error for OutOfBitmapError {}