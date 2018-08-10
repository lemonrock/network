// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Router renumbering.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct InternetControlMessageProtocolVersion6CodeRouterRenumbering(u8);

impl InternetControlMessageProtocolVersion6CodeRouterRenumbering
{
	/// Command.
	pub const Command: Self = InternetControlMessageProtocolVersion6CodeRouterRenumbering(0);
	
	/// Result.
	pub const Result: Self = InternetControlMessageProtocolVersion6CodeRouterRenumbering(1);
	
	/// Sequence number reset.
	pub const SequenceNumberReset: Self = InternetControlMessageProtocolVersion6CodeRouterRenumbering(255);
}

impl TryFrom<u8> for InternetControlMessageProtocolVersion6CodeRouterRenumbering
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		match value
		{
			0 | 1 | 255 => Ok(InternetControlMessageProtocolVersion6CodeRouterRenumbering(value)),
			_ => Err(()),
		}
	}
}

impl Into<u8> for InternetControlMessageProtocolVersion6CodeRouterRenumbering
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl Display for InternetControlMessageProtocolVersion6CodeRouterRenumbering
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}
