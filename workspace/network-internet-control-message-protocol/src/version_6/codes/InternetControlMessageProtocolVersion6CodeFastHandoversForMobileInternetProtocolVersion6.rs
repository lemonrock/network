// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Fast handovers for Mobile Internet Protocol (IP) version 6, aka FMIPv6 (RFC 5568).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct InternetControlMessageProtocolVersion6CodeFastHandoversForMobileInternetProtocolVersion6(u8);

impl InternetControlMessageProtocolVersion6CodeFastHandoversForMobileInternetProtocolVersion6
{
	/// Reserved.
	pub const Reserved1: Self = InternetControlMessageProtocolVersion6CodeFastHandoversForMobileInternetProtocolVersion6(0);
	
	/// Reserved.
	pub const Reserved2: Self = InternetControlMessageProtocolVersion6CodeFastHandoversForMobileInternetProtocolVersion6(1);
	
	/// Router Solicitation for Proxy Advertisement (RtSolPr) (RFC 5568).
	pub const RouterSolicitationForProxyAdvertisement: Self = InternetControlMessageProtocolVersion6CodeFastHandoversForMobileInternetProtocolVersion6(2);
	
	/// Proxy Router Advertisement (PrRtAdv) (RFC 5568).
	pub const ProxyRouterAdvertisement: Self = InternetControlMessageProtocolVersion6CodeFastHandoversForMobileInternetProtocolVersion6(3);
	
	/// Handover Initiate (HI) (RFC 5568).
	///
	/// Deprecated and unavailable for reassignment.
	pub const HandoverInitiate: Self = InternetControlMessageProtocolVersion6CodeFastHandoversForMobileInternetProtocolVersion6(4);
	
	/// Handover Acknowledge (HAck) (RFC 5568).
	///
	/// Deprecated and unavailable for reassignment.
	pub const HandoverAcknowledge: Self = InternetControlMessageProtocolVersion6CodeFastHandoversForMobileInternetProtocolVersion6(5);
}

impl TryFrom<u8> for InternetControlMessageProtocolVersion6CodeFastHandoversForMobileInternetProtocolVersion6
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if value <= 5
		{
			Ok(InternetControlMessageProtocolVersion6CodeFastHandoversForMobileInternetProtocolVersion6(value))
		}
		else
		{
			Err(())
		}
	}
}

impl Into<u8> for InternetControlMessageProtocolVersion6CodeFastHandoversForMobileInternetProtocolVersion6
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl Display for InternetControlMessageProtocolVersion6CodeFastHandoversForMobileInternetProtocolVersion6
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}
