// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Implementation of Internet Protocol (IP) version 6 packet processing.
#[derive(Debug)]
pub struct InternetProtocolVersion6PacketProcessing<EINPDO: EthernetIncomingNetworkPacketDropObserver<IPV6INPDR=InternetProtocolVersion6IncomingNetworkPacketDropReason<ICMPV4::DropReason, TCP::DropReason, UDP::DropReason>>, ICMPV6: Layer4PacketProcessing, TCP: Layer4PacketProcessing, UDP: Layer4PacketProcessing>
{
	dropped_packet_reporting: Rc<EINPDO>,
	
	/// No sender packet should be received from this address; if it was, it implies loopback on this interface, which is daft.
	our_valid_internet_protocol_version_6_host_addresses: HashSet<InternetProtocolVersion6HostAddress>,
	
	/// No sender packet should be received from this address; if it was, it implies loopback on this interface, which is daft.
	our_valid_internet_protocol_version_6_multicast_addresses: HashSet<InternetProtocolVersion6HostAddress>,
	
	denied_source_internet_protocol_version_6_host_addresses: TreeBitmap<()>,
	
	internet_control_message_protocol_processing: ICMPV6,
	
	transmission_control_protocol_processing: TCP,
	
	user_datagram_protocol_processing: UDP,
}

impl<EINPDO: EthernetIncomingNetworkPacketDropObserver<IPV6INPDR=InternetProtocolVersion6IncomingNetworkPacketDropReason<ICMPV4::DropReason, TCP::DropReason, UDP::DropReason>>, ICMPV6: Layer4PacketProcessing, TCP: Layer4PacketProcessing, UDP: Layer4PacketProcessing> Layer3PacketProcessing for InternetProtocolVersion6PacketProcessing<EINPDO, ICMPV6, TCP, UDP>
{
	type DropReason = EINPDO::IPV6INPDR;
	
	type CheckSumsValidated = bool;
	
	#[inline(always)]
	fn process<'lifetime>(&self, now: MonotonicMillisecondTimestamp, packet: impl EthernetIncomingNetworkPacket, layer_3_packet: &'lifetime Layer3Packet, layer_3_length: u16, ethernet_addresses: &'lifetime EthernetAddresses, check_sum_validated_in_hardware: Self::CheckSumsValidated)
	{
		if unlikely!(InternetProtocolVersion6Packet::is_packet_length_too_short(layer_3_length))
		{
			drop!(InternetProtocolVersion6IncomingNetworkPacketDropReason::PacketIsTooShort, ethernet_addresses, self, packet)
		}
		
		let internet_protocol_version_6_packet: &'lifetime InternetProtocolVersion6Packet = layer_3_packet.as_type();
		
		internet_protocol_version_6_packet.process(packet, self, layer_3_length, ethernet_addresses, check_sum_validated_in_hardware)
	}
}

impl<EINPDO: EthernetIncomingNetworkPacketDropObserver<IPV6INPDR=InternetProtocolVersion6IncomingNetworkPacketDropReason<ICMPV4::DropReason, TCP::DropReason, UDP::DropReason>>, ICMPV6: Layer4PacketProcessing, TCP: Layer4PacketProcessing, UDP: Layer4PacketProcessing> InternetProtocolVersion6PacketProcessing<EINPDO, ICMPV6, TCP, UDP>
{
	#[inline(always)]
	pub(crate) fn drop<'lifetime>(&self, reason: EINPDO::IPV6INPDR, ethernet_addresses: &'lifetime EthernetAddresses, packet: impl EthernetIncomingNetworkPacket)
	{
		let reason = EthernetIncomingNetworkPacketDropReason::ProblematicInternetProtocolVersion6Packet
		{
			ethernet_addresses,
			reason,
		};
		
		self.dropped_packet_reporting.dropped_packet(reason);
		packet.free_direct_contiguous_packet();
	}
	
	#[inline(always)]
	pub(crate) fn is_internet_protocol_version_6_host_address_not_one_of_our_multicast_addresses(&self, _internet_protocol_version_6_multicast_address: &InternetProtocolVersion6HostAddress) -> bool
	{
		// TODO: solicited node check implicit group membership.
		// TODO: all nodes (FF02::1); equivalent to 224.0.0.1 and 255.255.255.255.
		
		const MulticastIsUnsupportedAtThisTime: bool = false;
		
		MulticastIsUnsupportedAtThisTime
	}
	
	#[inline(always)]
	pub(crate) fn is_internet_protocol_version_6_host_address_one_of_ours(&self, internet_protocol_version_6_host_address: &InternetProtocolVersion6HostAddress) -> bool
	{
		debug_assert!(internet_protocol_version_6_host_address.is_valid_unicast(), "internet_protocol_version_6_host_address '{:?}' is not valid unicast", internet_protocol_version_6_host_address);
		
		self.our_valid_internet_protocol_version_6_host_addresses.contains(&internet_protocol_version_6_host_address)
	}
	
	#[inline(always)]
	pub(crate) fn is_internet_protocol_version_6_host_address_not_one_of_our_unicast_addresses(&self, internet_protocol_version_6_host_address: &InternetProtocolVersion6HostAddress) -> bool
	{
		debug_assert!(internet_protocol_version_6_host_address.is_valid_unicast(), "internet_protocol_version_6_host_address '{:?}' is not valid unicast", internet_protocol_version_6_host_address);
		
		!self.is_internet_protocol_version_6_host_address_one_of_ours(internet_protocol_version_6_host_address)
	}
	
	#[inline(always)]
	pub(crate) fn is_source_internet_protocol_version_6_address_denied(&self, internet_protocol_version_6_host_address: &InternetProtocolVersion6HostAddress) -> bool
	{
		debug_assert!(internet_protocol_version_6_host_address.is_valid_unicast(), "internet_protocol_version_6_host_address '{:?}' is not valid unicast", internet_protocol_version_6_host_address);
		
		let nibbles = internet_protocol_version_6_host_address.nibbles_non_destructively();
		self.denied_source_internet_protocol_version_6_host_addresses.longest_match_present(&nibbles)
	}
}
