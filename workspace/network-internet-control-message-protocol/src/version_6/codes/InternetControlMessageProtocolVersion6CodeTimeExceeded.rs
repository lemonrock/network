// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Time Exceeded (RFC 4443).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct InternetControlMessageProtocolVersion6CodeTimeExceeded(u8);

impl InternetControlMessageProtocolVersion6CodeTimeExceeded
{
	/// Hop limit exceeded in transit.
	pub const HopLimitExceededInTransit: Self = InternetControlMessageProtocolVersion6CodeTimeExceeded(0);
	
	/// Fragment reassembly time exceeded.
	pub const FragmentReassemblyTimeExceeded: Self = InternetControlMessageProtocolVersion6CodeTimeExceeded(1);
}

impl TryFrom<u8> for InternetControlMessageProtocolVersion6CodeTimeExceeded
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if value <= 2
		{
			Ok(InternetControlMessageProtocolVersion6CodeTimeExceeded(value))
		}
		else
		{
			Err(())
		}
	}
}

impl Into<u8> for InternetControlMessageProtocolVersion6CodeTimeExceeded
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl Display for InternetControlMessageProtocolVersion6CodeTimeExceeded
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}
