// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Layer 3 packet processing
pub trait Layer3PacketProcessing: Debug
{
	/// Process an internet protocol version 4 packet.
	#[inline(always)]
	fn process_internet_protocol_version_4<'ethernet_addresses>(&self, packet: impl IncomingNetworkPacket, layer_3_packet: &'ethernet_addresses mut Layer3Packet, layer_3_length: u16, ethernet_addresses: &'ethernet_addresses EthernetAddresses);
	
	/// Process an internet protocol version 6 packet.
	#[inline(always)]
	fn process_internet_protocol_version_6<'ethernet_addresses>(&self, packet: impl IncomingNetworkPacket, layer_3_packet: &'ethernet_addresses mut Layer3Packet, layer_3_length: u16, ethernet_addresses: &'ethernet_addresses EthernetAddresses);
	
	/// Process an address resolution protocol packet.
	#[inline(always)]
	fn process_address_resolution_protocol<'ethernet_addresses>(&self, packet: impl IncomingNetworkPacket, layer_3_packet: &'ethernet_addresses mut Layer3Packet, layer_3_length: u16, ethernet_addresses: &'ethernet_addresses EthernetAddresses);
}
