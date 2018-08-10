// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Multicast Listener Query (RFC 3810).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct InternetControlMessageProtocolVersion6CodeMulticastListenerQuery(u8);

impl InternetControlMessageProtocolVersion6CodeMulticastListenerQuery
{
	/// Only known value.
	pub const Zero: Self = InternetControlMessageProtocolVersion6CodeMulticastListenerQuery(0);
}

impl TryFrom<u8> for InternetControlMessageProtocolVersion6CodeMulticastListenerQuery
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if value == 0
		{
			Ok(InternetControlMessageProtocolVersion6CodeMulticastListenerQuery(value))
		}
		else
		{
			Err(())
		}
	}
}

impl Into<u8> for InternetControlMessageProtocolVersion6CodeMulticastListenerQuery
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl Display for InternetControlMessageProtocolVersion6CodeMulticastListenerQuery
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}
