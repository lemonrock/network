// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Internet Protocol (IP) version 6 packet processing.
pub trait InternetProtocolVersion6PacketProcessing: Debug
{
	/// Process an internet protocol version 6 packet.
	#[inline(always)]
	fn process<'lifetime>(&self, packet: impl EthernetIncomingNetworkPacket, layer_3_packet: &'lifetime Layer3Packet, layer_3_length: u16, ethernet_addresses: &'lifetime EthernetAddresses);
}
