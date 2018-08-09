// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Represents an Ether type.
#[repr(C, packed)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct EtherType(NetworkEndianU16);

impl Into<NetworkEndianU16> for EtherType
{
	#[inline(always)]
	fn into(self) -> NetworkEndianU16
	{
		self.0
	}
}

impl Into<u16> for EtherType
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0.to_native_endian()
	}
}

impl TryFrom<u16> for EtherType
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u16) -> Result<Self, Self::Error>
	{
		Self::new(NetworkEndianU16::from_native_endian(value))
	}
}

impl TryFrom<NetworkEndianU16> for EtherType
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: NetworkEndianU16) -> Result<Self, Self::Error>
	{
		Self::new(value)
	}
}

impl Display for EtherType
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", self.0.to_native_endian())
	}
}

impl Debug for EtherType
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", self.0.to_native_endian())
	}
}

impl EtherType
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(value: NetworkEndianU16) -> Result<Self, ()>
	{
		if value.bytes()[0] >= EtherTypeOrLegacyEthernetFrameSize::FirstByteSwitchOverValue
		{
			Ok(EtherType(value))
		}
		else
		{
			Err(())
		}
	}
	
	/// IEEE 802.3x-1997 frame size change over.
	pub const Minimum: NetworkEndianU16 = NetworkEndianU16::from_network_endian([EtherTypeOrLegacyEthernetFrameSize::FirstByteSwitchOverValue, 0x00]);
	
	/// Slow protocols Link Aggregation Control Protocol (LACP) and Marker.
	///
	/// IEEE Std 802.3-2015, Annex 57A.
	pub const Slow: Self = EtherType(NetworkEndianU16::from_network_endian([0x88, 0x09]));
	
	/// Internet protocol (IP) version 4 ether type.
	pub const InternetProtocolVersion4: Self = EtherType(NetworkEndianU16::from_network_endian([0x08, 0x00]));
	
	/// Internet protocol (IP) version 6 ether type.
	pub const InternetProtocolVersion6: Self = EtherType(NetworkEndianU16::from_network_endian([0x86, 0xDD]));
	
	/// Address resolution protocol (ARP) ether type.
	pub const AddressResolutionProtocol: Self = EtherType(NetworkEndianU16::from_network_endian([0x08, 0x06]));
	
	/// Reverse address resolution protocol (RARP) ether type.
	pub const ReverseAddressResolutionProtocol: Self = EtherType(NetworkEndianU16::from_network_endian([0x80, 0x35]));
	
	/// IEEE 802.1Q VLAN tagging.
	pub const VlanTagging: Self = EtherType(NetworkEndianU16::from_network_endian([0x81, 0x00]));
	
	/// IEEE 802.1ad QinQ tagging.
	pub const QinQVlanTagging: Self = EtherType(NetworkEndianU16::from_network_endian([0x88, 0xA8]));
	
	/// IEEE1588 / 802.1AS Precise time protocol (PTP).
	pub const PreciseTimeProtocol: Self = EtherType(NetworkEndianU16::from_network_endian([0x88, 0xF7]));
	
	/// Transparent Ethernet Bridging.
	pub const TransparentEthernetBridging: Self = EtherType(NetworkEndianU16::from_network_endian([0x65, 0x58]));
	
	/// Link local discovery protocol (LLDP).
	pub const LinkLocalDiscoveryProtocol: Self = EtherType(NetworkEndianU16::from_network_endian([0x88, 0xCC]));
	
	/// Is this a valid Ether Type (as opposed to a legacy ethernet frame size).
	#[inline(always)]
	pub fn is_valid_ether_type(self) -> bool
	{
		self.0 >= Self::Minimum
	}
	
	/// Use this to eliminate unwanted ARP traffic.
	#[inline(always)]
	pub fn is_not_internet_protocol_version_4(self) -> bool
	{
		self.0 != Self::InternetProtocolVersion4.0
	}
	
	/// To a network endian value.
	#[inline(always)]
	pub fn to_network_endian(self) -> u16
	{
		self.0.to_network_endian()
	}
}
