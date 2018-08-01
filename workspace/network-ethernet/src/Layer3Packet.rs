// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
pub struct Layer3Packet
{
	/// Layer 3 packet.
	pub other: PhantomData<u8>,
}

impl Display for Layer3Packet
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Debug for Layer3Packet
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "(layer 3 packet)")
	}
}

impl Layer3Packet
{
	/// Horrible method to cast to a specific implementation of a layer 3 packet.
	#[inline(always)]
	pub fn as_type<P>(&mut self) -> &mut P
	{
		unsafe { transmute(&mut self.other) }
	}
	
//	#[inline(always)]
//	pub(crate) fn process_internet_protocol_version_4<'ethernet_addresses>(&'ethernet_addresses mut self, packet: impl IncomingNetworkPacket, packet_processing: &PacketProcessing<impl IncomingNetworkPacketProcessingDropObserver>, layer_3_length: u16, ethernet_addresses: &'ethernet_addresses EthernetAddresses)
//	{
//		if unlikely!(InternetProtocolVersion4Packet::is_packet_length_too_short(layer_3_length))
//		{
//			drop!(InternetProtocolVersion4PacketIsTooShort { ethernet_addresses }, packet_processing, packet)
//		}
//
//		let internet_protocol_version_4_packet: &'ethernet_addresses mut InternetProtocolVersion4Packet = self.as_type();
//
//		internet_protocol_version_4_packet.process(packet, packet_processing, layer_3_length, ethernet_addresses)
//	}
//
//	#[inline(always)]
//	pub(crate) fn process_internet_protocol_version_6<'ethernet_addresses>(&'ethernet_addresses mut self, packet: impl IncomingNetworkPacket, packet_processing: &PacketProcessing<impl IncomingNetworkPacketProcessingDropObserver>, layer_3_length: u16, ethernet_addresses: &'ethernet_addresses EthernetAddresses)
//	{
//		if unlikely!(InternetProtocolVersion6Packet::is_packet_length_too_short(layer_3_length))
//		{
//			drop!(InternetProtocolVersion6PacketIsTooShort { ethernet_addresses }, packet_processing, packet)
//		}
//
//		let internet_protocol_version_6_packet: &'ethernet_addresses mut InternetProtocolVersion6Packet = self.as_type();
//
//		internet_protocol_version_6_packet.process(packet, packet_processing, layer_3_length, ethernet_addresses)
//	}
//
//	#[inline(always)]
//	pub(crate) fn process_address_resolution_protocol<'ethernet_addresses>(&'ethernet_addresses mut self, packet: impl IncomingNetworkPacket, packet_processing: &PacketProcessing<impl IncomingNetworkPacketProcessingDropObserver>, layer_3_length: u16, ethernet_addresses: &'ethernet_addresses EthernetAddresses)
//	{
//		if unlikely!(AddressResolutionProtocolPacket::is_packet_length_too_short(layer_3_length))
//		{
//			drop!(AddressResolutionProtocolPacketIsTooShort { ethernet_addresses }, packet_processing, packet)
//		}
//
//		let address_resolution_protocol_packet: &'ethernet_addresses mut AddressResolutionProtocolPacket = self.as_type();
//
//		if unlikely!(address_resolution_protocol_packet.is_invalid_for_internet_protocol_version_4(layer_3_length))
//		{
//			drop!(AddressResolutionProtocolNotSupportedForAnythingOtherThanInternetProtocolVersion4 { ethernet_addresses }, packet_processing, packet)
//		}
//
//		address_resolution_protocol_packet.process(packet, packet_processing, ethernet_addresses)
//	}
}
