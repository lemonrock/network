// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
pub union EthernetPacketPayload
{
	/// An IEEE 802.1Q virtual lan tagged packet.
	///
	/// Note that some poll-mode drivers will have stripped this.
	pub virtual_lan_packet: VirtualLanPacket,
	
	/// An IEEE 802.1ad QinQ virtual lan tagged packet.
	///
	/// Note that some poll-mode drivers will have stripped this.
	///
	/// There can be an (infinite) nest of these terminated by an IEEE 802.1Q terminator, although conventionally there is just one followed by an IEEE 802.1Q terminator.
	pub qinq_virtual_lan_packet: QinQVirtualLanPacket,
	
	/// A layer 3 packet.
	pub layer_3_packet: Layer3Packet,
}

impl Display for EthernetPacketPayload
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Debug for EthernetPacketPayload
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "(payload)")
	}
}

impl EthernetPacketPayload
{
	#[inline(always)]
	pub(crate) fn layer_3_packet(&self) -> &Layer3Packet
	{
		unsafe { &self.layer_3_packet }
	}
	
	#[inline(always)]
	pub(crate) fn virtual_lan_packet(&self) -> &VirtualLanPacket
	{
		unsafe { &self.virtual_lan_packet }
	}
	
	#[inline(always)]
	pub(crate) fn qinq_virtual_lan_packet(&self) -> &QinQVirtualLanPacket
	{
		unsafe { &self.qinq_virtual_lan_packet }
	}
}
