// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Represents a quantity of bytes.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Bytes(usize);

impl Display for Bytes
{
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl From<usize> for Bytes
{
	#[inline(always)]
	fn from(value: usize) -> Self
	{
		Bytes(value)
	}
}

impl From<u32> for Bytes
{
	#[inline(always)]
	fn from(value: u32) -> Self
	{
		Bytes(value as usize)
	}
}

impl Into<usize> for Bytes
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0
	}
}

impl Into<isize> for Bytes
{
	#[inline(always)]
	fn into(self) -> isize
	{
		self.0 as isize
	}
}

impl Into<u64> for Bytes
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0 as u64
	}
}

impl Into<i64> for Bytes
{
	#[inline(always)]
	fn into(self) -> i64
	{
		self.0 as i64
	}
}

impl Add for Bytes
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: Self) -> Self::Output
	{
		Bytes(self.0 + rhs.0)
	}
}

impl AddAssign for Bytes
{
	fn add_assign(&mut self, rhs: Self)
	{
		self.0 += rhs.0
	}
}

impl Sub for Bytes
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output
	{
		Bytes(self.0 - rhs.0)
	}
}

impl SubAssign for Bytes
{
	fn sub_assign(&mut self, rhs: Self)
	{
		self.0 -= rhs.0
	}
}

impl Mul<usize> for Bytes
{
	type Output = Self;
	
	#[inline(always)]
	fn mul(self, rhs: usize) -> Self::Output
	{
		Bytes(self.0 * rhs)
	}
}

impl Bytes
{
	/// Rounds up to a page size multiple.
	#[inline(always)]
	pub fn round_up_to_page_size(self) -> Self
	{
		let page_size = page_size();
		Bytes((self.0 + page_size - 1) / page_size)
	}
}
