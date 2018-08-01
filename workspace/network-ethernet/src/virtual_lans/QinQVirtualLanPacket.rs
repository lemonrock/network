// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// An IEEE 802.1ad QinQ virtual LAN tagged packet.
///
/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
#[derive(Debug)]
pub struct QinQVirtualLanPacket
{
	/// Header.
	pub header: VirtualLanPacketHeader,
	
	/// IEEE 802.1Q virtual LAN tagged packet.
	///
	/// Strictly speaking, it is possible to nest another QinQVirtualLanPacket inside this packet, almost infinitely.
	///
	/// In practice, this is extremely rare.
	pub virtual_lan_packet: VirtualLanPacket,
}

impl Display for QinQVirtualLanPacket
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl QinQVirtualLanPacket
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
		unsafe { self.header.ether_type_or_legacy_ethernet_frame_size.ether_type }
	}
	
	/// Inner virtual LAN packet.
	#[inline(always)]
	pub fn virtual_lan_packet(&mut self) -> &mut VirtualLanPacket
	{
		&mut self.virtual_lan_packet
	}
}
