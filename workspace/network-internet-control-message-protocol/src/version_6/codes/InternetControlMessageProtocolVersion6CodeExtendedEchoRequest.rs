// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Extended Echo Request ('ping') (RFC 8335).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct InternetControlMessageProtocolVersion6CodeExtendedEchoRequest(u8);

impl InternetControlMessageProtocolVersion6CodeExtendedEchoRequest
{
	/// No error.
	pub const NoError: Self = InternetControlMessageProtocolVersion6CodeExtendedEchoRequest(0);
	
	/// Malformed query.
	pub const MalformedQuery: Self = InternetControlMessageProtocolVersion6CodeExtendedEchoRequest(1);
	
	/// No such interface.
	pub const NoSuchInterface: Self = InternetControlMessageProtocolVersion6CodeExtendedEchoRequest(2);
	
	/// No such table entry.
	pub const NoSuchTableEntry: Self = InternetControlMessageProtocolVersion6CodeExtendedEchoRequest(3);
	
	/// Multiple interfaces satisfy query.
	pub const MultipleInterfacesSatisfyQuery: Self = InternetControlMessageProtocolVersion6CodeExtendedEchoRequest(4);
}

impl TryFrom<u8> for InternetControlMessageProtocolVersion6CodeExtendedEchoRequest
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if value <= 4
		{
			Ok(InternetControlMessageProtocolVersion6CodeExtendedEchoRequest(value))
		}
		else
		{
			Err(())
		}
	}
}

impl Into<u8> for InternetControlMessageProtocolVersion6CodeExtendedEchoRequest
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl Display for InternetControlMessageProtocolVersion6CodeExtendedEchoRequest
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}
