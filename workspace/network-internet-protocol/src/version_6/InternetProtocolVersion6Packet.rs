// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
#[derive(Debug)]
pub struct InternetProtocolVersion6Packet
{
	/// Header.
	pub header: InternetProtocolVersion6PacketHeader,
}

impl Display for InternetProtocolVersion6Packet
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl InternetProtocolVersion6Packet
{
	/// Use this to eliminate invalid traffic.
	#[inline(always)]
	pub fn is_packet_length_too_short(layer_3_length: u16) -> bool
	{
		layer_3_length < InternetProtocolVersion6PacketHeader::HeaderSizeU16
	}
	
	#[inline(always)]
	pub(crate) fn process<'lifetime, EINPDO: EthernetIncomingNetworkPacketDropObserver<IPV6INPDR=InternetProtocolVersion6IncomingNetworkPacketDropReason>>(&'lifetime self, packet: impl EthernetIncomingNetworkPacket, packet_processing: &InternetProtocolVersion6PacketProcessing<EINPDO>, layer_3_length: u16, ethernet_addresses: &'lifetime EthernetAddresses, layer_4_check_sum_validated_in_hardware: bool)
	{
		unimplemented!();
	}
}
