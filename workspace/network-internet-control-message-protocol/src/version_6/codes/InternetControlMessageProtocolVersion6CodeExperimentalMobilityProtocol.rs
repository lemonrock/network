// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Utilized by experimental mobility protocols such as Seamoby (RFC 4065).
///
/// Note that `try_from` always succeeds; we implement `TryFrom` rather than `From` to maintain an API complementary to other kinds of `InternetControlMessageProtocolVersion6Code*`.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct InternetControlMessageProtocolVersion6CodeExperimentalMobilityProtocol(u8);

impl InternetControlMessageProtocolVersion6CodeExperimentalMobilityProtocol
{
	/// Only known value.
	pub const Zero: Self = InternetControlMessageProtocolVersion6CodeExperimentalMobilityProtocol(0);
}

impl TryFrom<u8> for InternetControlMessageProtocolVersion6CodeExperimentalMobilityProtocol
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		Ok(InternetControlMessageProtocolVersion6CodeExperimentalMobilityProtocol(value))
	}
}

impl Into<u8> for InternetControlMessageProtocolVersion6CodeExperimentalMobilityProtocol
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl Display for InternetControlMessageProtocolVersion6CodeExperimentalMobilityProtocol
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}
