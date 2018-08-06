// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Layer 4 (eg Transmission Control Protocol (TCP)) packet processing.
pub trait Layer4PacketProcessing
{
	/// Drop reason.
	type DropReason: IncomingNetworkPacketProcessingDropReason;
	
	/// `layer_4_length` is NOT the same as the Internet Protocol (IP) version 6 payload size; in this case, it is the IPv6 payload size ***less*** the extensions headers size.
	///
	/// RFC 2675 IPv6 jumbograms are not supported.
	#[inline(always)]
	fn process<'lifetime>(&self, now: MonotonicMillisecondTimestamp, packet: impl EthernetIncomingNetworkPacket, layer_4_packet: &'lifetime Layer4Packet, layer_4_length: u16, ethernet_addresses: &'lifetime EthernetAddresses, layer_4_check_sum_validated_in_hardware: bool);
}
