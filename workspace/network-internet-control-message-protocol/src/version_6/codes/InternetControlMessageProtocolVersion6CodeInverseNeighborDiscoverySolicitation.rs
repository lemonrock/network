// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Inverse Neighbor Discovery Solicitation (RFC 3122).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct InternetControlMessageProtocolVersion6CodeInverseNeighborDiscoverySolicitation(u8);

impl InternetControlMessageProtocolVersion6CodeInverseNeighborDiscoverySolicitation
{
	/// Only known value.
	pub const Zero: Self = InternetControlMessageProtocolVersion6CodeInverseNeighborDiscoverySolicitation(0);
}

impl TryFrom<u8> for InternetControlMessageProtocolVersion6CodeInverseNeighborDiscoverySolicitation
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if value == 0
		{
			Ok(InternetControlMessageProtocolVersion6CodeInverseNeighborDiscoverySolicitation(value))
		}
		else
		{
			Err(())
		}
	}
}

impl Into<u8> for InternetControlMessageProtocolVersion6CodeInverseNeighborDiscoverySolicitation
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl Display for InternetControlMessageProtocolVersion6CodeInverseNeighborDiscoverySolicitation
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}
