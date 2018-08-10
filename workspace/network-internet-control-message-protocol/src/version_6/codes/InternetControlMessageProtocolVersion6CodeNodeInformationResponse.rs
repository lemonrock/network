// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Node Information Response (RFC 4620).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct InternetControlMessageProtocolVersion6CodeNodeInformationResponse(u8);

impl InternetControlMessageProtocolVersion6CodeNodeInformationResponse
{
	/// A successful reply.
	///
	/// The Reply Data field may or may not be empty.
	pub const Success: Self = InternetControlMessageProtocolVersion6CodeNodeInformationResponse(0);
	
	/// The Responder refuses to supply the answer.
	///
	/// The Reply Data field will be empty.
	pub const Refused: Self = InternetControlMessageProtocolVersion6CodeNodeInformationResponse(1);
	
	///The query type (Qtype) of the Query is unknown to the Responder.
	///
	/// The Reply Data field will be empty.
	pub const QueryTypeIsUnknown: Self = InternetControlMessageProtocolVersion6CodeNodeInformationResponse(2);
}

impl TryFrom<u8> for InternetControlMessageProtocolVersion6CodeNodeInformationResponse
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if value <= 2
		{
			Ok(InternetControlMessageProtocolVersion6CodeNodeInformationResponse(value))
		}
		else
		{
			Err(())
		}
	}
}

impl Into<u8> for InternetControlMessageProtocolVersion6CodeNodeInformationResponse
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl Display for InternetControlMessageProtocolVersion6CodeNodeInformationResponse
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}
