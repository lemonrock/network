// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
#[derive(Debug)]
pub struct EthernetPacket
{
	/// Header.
	pub header: EthernetPacketHeader,

	/// Payload
	pub payload: EthernetPacketPayload,
}

impl Display for EthernetPacket
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl EthernetPacket
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn ethernet_addresses(&self) -> &EthernetAddresses
	{
		self.header.ethernet_addresses()
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn potentially_invalid_ether_type(&self) -> EtherType
	{
		self.header.potentially_invalid_ether_type()
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn layer_3_packet(&mut self) -> &mut Layer3Packet
	{
		self.payload.layer_3_packet()
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn virtual_lan_packet(&mut self) -> &mut VirtualLanPacket
	{
		self.payload.virtual_lan_packet()
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn qinq_virtual_lan_packet(&mut self) -> &mut QinQVirtualLanPacket
	{
		self.payload.qinq_virtual_lan_packet()
	}
}
