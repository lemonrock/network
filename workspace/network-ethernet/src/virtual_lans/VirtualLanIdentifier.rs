// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// A virtual Lan identifier 1 - 4094 inclusive.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct VirtualLanIdentifier(u16);

impl Display for VirtualLanIdentifier
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", self.0)
	}
}

impl Into<u16> for VirtualLanIdentifier
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0
	}
}

impl TryFrom<u16> for VirtualLanIdentifier
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u16) -> Result<Self, Self::Error>
	{
		Self::new(value)
	}
}

impl VirtualLanIdentifier
{
	/// Parse.
	///
	/// Returns an error if zero or > 4094.
	#[inline(always)]
	pub fn new(value: u16) -> Result<Self, ()>
	{
		if value >= 1 && value <= 4094
		{
			Ok(VirtualLanIdentifier(value))
		}
		else
		{
			Err(())
		}
	}
}
