// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Internet Protocol (IP) version 6 Routing Protocol for Low-Power and Lossy Networks Control (RPL) (RFC 6550).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct InternetControlMessageProtocolVersion6CodeRoutingProtocolForLowPowerAndLossyNetworksControl(u8);

impl InternetControlMessageProtocolVersion6CodeRoutingProtocolForLowPowerAndLossyNetworksControl
{
	/// Destination-Oriented Directed Acyclic Graph (DODAG) Information Solicitation (RFC 6550).
	pub const DestinationOrientedDirectedAcyclicGraphInformationSolicitation: Self = InternetControlMessageProtocolVersion6CodeRoutingProtocolForLowPowerAndLossyNetworksControl(0x00);
	
	/// Destination-Oriented Directed Acyclic Graph (DODAG) Information Object (RFC 6550).
	pub const DestinationOrientedDirectedAcyclicGraphInformationObject: Self = InternetControlMessageProtocolVersion6CodeRoutingProtocolForLowPowerAndLossyNetworksControl(0x01);
	
	/// Destination Advertisement Object (RFC 6550).
	pub const DestinationAdvertisementObject: Self = InternetControlMessageProtocolVersion6CodeRoutingProtocolForLowPowerAndLossyNetworksControl(0x02);
	
	/// Destination Advertisement Object Acknowledgment (RFC 6550).
	pub const DestinationAdvertisementObjectAcknowledgment: Self = InternetControlMessageProtocolVersion6CodeRoutingProtocolForLowPowerAndLossyNetworksControl(0x03);
	
	/// Secure Destination-Oriented Directed Acyclic Graph (DODAG) Information Solicitation (RFC 6550).
	pub const SecureDestinationOrientedDirectedAcyclicGraphInformationSolicitation: Self = InternetControlMessageProtocolVersion6CodeRoutingProtocolForLowPowerAndLossyNetworksControl(0x80);
	
	/// Secure Destination-Oriented Directed Acyclic Graph (DODAG) Information Object (RFC 6550).
	pub const SecureDestinationOrientedDirectedAcyclicGraphInformationObject: Self = InternetControlMessageProtocolVersion6CodeRoutingProtocolForLowPowerAndLossyNetworksControl(0x81);
	
	/// Secure Destination Advertisement Object (RFC 6550).
	pub const SecureDestinationAdvertisementObject: Self = InternetControlMessageProtocolVersion6CodeRoutingProtocolForLowPowerAndLossyNetworksControl(0x82);
	
	/// Secure Destination Advertisement Object Acknowledgment (RFC 6550).
	pub const SecureDestinationAdvertisementObjectAcknowledgment: Self = InternetControlMessageProtocolVersion6CodeRoutingProtocolForLowPowerAndLossyNetworksControl(0x83);
	
	/// Consistency Check (RFC 6550).
	pub const ConsistencyCheck: Self = InternetControlMessageProtocolVersion6CodeRoutingProtocolForLowPowerAndLossyNetworksControl(0x8A);
}

impl TryFrom<u8> for InternetControlMessageProtocolVersion6CodeRoutingProtocolForLowPowerAndLossyNetworksControl
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		match value
		{
			0x00 ... 0x03 => Ok(InternetControlMessageProtocolVersion6CodeRoutingProtocolForLowPowerAndLossyNetworksControl(value)),
			0x80 ... 0x83 => Ok(InternetControlMessageProtocolVersion6CodeRoutingProtocolForLowPowerAndLossyNetworksControl(value)),
			0x8A => Ok(InternetControlMessageProtocolVersion6CodeRoutingProtocolForLowPowerAndLossyNetworksControl(value)),
			_ => Err(()),
		}
	}
}

impl Into<u8> for InternetControlMessageProtocolVersion6CodeRoutingProtocolForLowPowerAndLossyNetworksControl
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl Display for InternetControlMessageProtocolVersion6CodeRoutingProtocolForLowPowerAndLossyNetworksControl
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}
