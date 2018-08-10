// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Multicast Router Advertisement (RFC 4286) interval value.
///
/// Note that `try_from` always succeeds; we implement `TryFrom` rather than `From` to maintain an API complementary to other kinds of `InternetControlMessageProtocolVersion6Code*`.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct InternetControlMessageProtocolVersion6CodeMulticastRouterAdvertisement(u8);

impl InternetControlMessageProtocolVersion6CodeMulticastRouterAdvertisement
{
}

impl TryFrom<u8> for InternetControlMessageProtocolVersion6CodeMulticastRouterAdvertisement
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		Ok(InternetControlMessageProtocolVersion6CodeMulticastRouterAdvertisement(value))
	}
}

impl Into<u8> for InternetControlMessageProtocolVersion6CodeMulticastRouterAdvertisement
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl Display for InternetControlMessageProtocolVersion6CodeMulticastRouterAdvertisement
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl InternetControlMessageProtocolVersion6CodeMulticastRouterAdvertisement
{
	/// Inverval.
	#[inline(always)]
	pub fn interval(self) -> u8
	{
		self.0
	}
}
