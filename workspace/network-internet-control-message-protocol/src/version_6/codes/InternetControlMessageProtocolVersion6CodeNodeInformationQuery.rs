// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Node Information Query (RFC 4620).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct InternetControlMessageProtocolVersion6CodeNodeInformationQuery(u8);

impl InternetControlMessageProtocolVersion6CodeNodeInformationQuery
{
	/// The Data field contains an Internet Protocol (IP) version 6 address which is the Subject of this query.
	pub const DataFieldContainsInternetProtocolVersion6Address: Self = InternetControlMessageProtocolVersion6CodeNodeInformationQuery(0);
	
	/// The Data field contains a name which is the Subject of this query, or is empty, as in the case of a 'no operation' (NOOP).
	pub const DataFieldContainsNameOrIsEmpty: Self = InternetControlMessageProtocolVersion6CodeNodeInformationQuery(1);
	
	/// The Data field contains an Internet Protocol (IP) version 4 address which is the Subject of this Query.
	pub const DataFieldContainsInternetProtocolVersion4Address: Self = InternetControlMessageProtocolVersion6CodeNodeInformationQuery(2);
}

impl TryFrom<u8> for InternetControlMessageProtocolVersion6CodeNodeInformationQuery
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if value <= 2
		{
			Ok(InternetControlMessageProtocolVersion6CodeNodeInformationQuery(value))
		}
		else
		{
			Err(())
		}
	}
}

impl Into<u8> for InternetControlMessageProtocolVersion6CodeNodeInformationQuery
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl Display for InternetControlMessageProtocolVersion6CodeNodeInformationQuery
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}
