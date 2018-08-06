// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Internet Protocol (IP) version 4 packet processing.
pub trait Layer3PacketProcessing: Debug
{
	/// Drop reason.
	type DropReason: IncomingNetworkPacketProcessingDropReason;
	
	/// Which check sums are validated for this layer 3 type?
	type CheckSumsValidated;
	
	/// Process an internet protocol version 4 packet.
	///
	/// `internet_protocol_version_4_and_layer_4_check_sums_validated`
	#[inline(always)]
	fn process<'lifetime>(&self, packet: impl EthernetIncomingNetworkPacket, layer_3_packet: &'lifetime Layer3Packet, layer_3_length: u16, ethernet_addresses: &'lifetime EthernetAddresses, check_sum_validated_in_hardware: Self::CheckSumsValidated);
}
