// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// An IEEE 802.1Q virtual LAN tagged packet.
///
/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
#[derive(Debug)]
pub struct VirtualLanPacket
{
	/// Header.
	pub header: VirtualLanPacketHeader,
	
	/// Layer 3 packet.
	pub layer_3_packet: Layer3Packet,
}

impl Display for VirtualLanPacket
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl VirtualLanPacket
{
	/// Tag control information (TCI).
	#[inline(always)]
	pub fn tag_control_information(&self) -> TagControlInformation
	{
		self.header.tag_control_information()
	}
	
	/// Ether type, potentially invalid as it could be a legacy ethernet frame size.
	#[inline(always)]
	pub fn potentially_invalid_ether_type(&self) -> EtherType
	{
		self.header.potentially_invalid_ether_type()
	}
	
	/// Inner layer 3 packet.
	#[inline(always)]
	pub fn layer_3_packet(&self) -> &Layer3Packet
	{
		&self.layer_3_packet
	}
}
