// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Destination Unreachable (RFC 4443).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct InternetControlMessageProtocolVersion6CodeDestinationUnreachable(u8);

impl InternetControlMessageProtocolVersion6CodeDestinationUnreachable
{
	/// No route to destination.
	pub const NoRouteToDestination: Self = InternetControlMessageProtocolVersion6CodeDestinationUnreachable(0);
	
	/// Communication with destination administratively prohibited.
	pub const CommunicationWithDestinationAdministrativelyProhibited: Self = InternetControlMessageProtocolVersion6CodeDestinationUnreachable(1);
	
	/// Beyond scope of source address (RFC 4443).
	pub const BeyondScopeOfSourceAddress: Self = InternetControlMessageProtocolVersion6CodeDestinationUnreachable(2);
	
	/// Address Unreachable.
	pub const AddressUnreachable: Self = InternetControlMessageProtocolVersion6CodeDestinationUnreachable(3);
	
	/// Port Unreachable.
	pub const PortUnreachable: Self = InternetControlMessageProtocolVersion6CodeDestinationUnreachable(4);
	
	/// Source address failed ingress or egress policy (RFC 4443).
	pub const SourceAddressFailedIngressOrEgressPolicy: Self = InternetControlMessageProtocolVersion6CodeDestinationUnreachable(5);
	
	/// Reject route to destination (RFC 4443).
	pub const RejectRouteToDestination: Self = InternetControlMessageProtocolVersion6CodeDestinationUnreachable(6);
	
	/// Error in Source Routing Header (RFC 6550 and RFC 65554).
	pub const ErrorInSourceRoutingHeader: Self = InternetControlMessageProtocolVersion6CodeDestinationUnreachable(7);
}

impl TryFrom<u8> for InternetControlMessageProtocolVersion6CodeDestinationUnreachable
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if value <= 7
		{
			Ok(InternetControlMessageProtocolVersion6CodeDestinationUnreachable(value))
		}
		else
		{
			Err(())
		}
	}
}

impl Into<u8> for InternetControlMessageProtocolVersion6CodeDestinationUnreachable
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl Display for InternetControlMessageProtocolVersion6CodeDestinationUnreachable
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}
