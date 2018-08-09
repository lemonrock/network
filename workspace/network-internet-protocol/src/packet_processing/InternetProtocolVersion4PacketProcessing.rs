// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Implementation of Internet Protocol (IP) version 4 packet processing.
#[derive(Debug)]
pub struct InternetProtocolVersion4PacketProcessing<EINPDO: EthernetIncomingNetworkPacketDropObserver<IPV4INPDR=InternetProtocolVersion4IncomingNetworkPacketDropReason<ICMPV4::DropReason, TCP::DropReason, UDP::DropReason>>, ICMPV4: Layer4PacketProcessing, TCP: Layer4PacketProcessing, UDP: Layer4PacketProcessing>
{
	dropped_packet_reporting: Rc<EINPDO>,
	
	our_valid_internet_protocol_version_4_host_addresses: Rc<OurValidInternetProtocolVersion4HostAddresses>,
	
	/// Our multicast internet protocol (IP) version 4 host addresses valid for this network interface.
	///
	/// No sender packet should be received from this address; if it was, it implies loopback on this interface, which is daft.
	our_valid_internet_protocol_version_4_multicast_addresses: HashSet<InternetProtocolVersion4HostAddress>,
	
	denied_source_internet_protocol_version_4_host_addresses: TreeBitmap<()>,

	internet_control_message_protocol_version_4_processing: ICMPV4,

	transmission_control_protocol_processing: TCP,

	user_datagram_protocol_processing: UDP,
}

impl<EINPDO: EthernetIncomingNetworkPacketDropObserver<IPV4INPDR=InternetProtocolVersion4IncomingNetworkPacketDropReason<ICMPV4::DropReason, TCP::DropReason, UDP::DropReason>>, ICMPV4: Layer4PacketProcessing, TCP: Layer4PacketProcessing, UDP: Layer4PacketProcessing> Layer3PacketProcessing for InternetProtocolVersion4PacketProcessing<EINPDO, ICMPV4, TCP, UDP>
{
	type DropReason = EINPDO::IPV4INPDR;
	
	type CheckSumsValidated = (bool, bool);
	
	#[inline(always)]
	fn process<'lifetime>(&self, now: MonotonicMillisecondTimestamp, packet: impl EthernetIncomingNetworkPacket, layer_3_packet: &'lifetime Layer3Packet, layer_3_length: u16, ethernet_addresses: &'lifetime EthernetAddresses, check_sum_validated_in_hardware: Self::CheckSumsValidated)
	{
		if unlikely!(InternetProtocolVersion4Packet::is_packet_length_too_short(layer_3_length))
		{
			drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::PacketIsTooShort, ethernet_addresses, self, packet)
		}
		
		let internet_protocol_version_4_packet: &'lifetime InternetProtocolVersion4Packet = layer_3_packet.as_type();
		
		internet_protocol_version_4_packet.process(now, packet, self, layer_3_length, ethernet_addresses, check_sum_validated_in_hardware.0, check_sum_validated_in_hardware.1)
	}
}

impl<EINPDO: EthernetIncomingNetworkPacketDropObserver<IPV4INPDR=InternetProtocolVersion4IncomingNetworkPacketDropReason<ICMPV4::DropReason, TCP::DropReason, UDP::DropReason>>, ICMPV4: Layer4PacketProcessing, TCP: Layer4PacketProcessing, UDP: Layer4PacketProcessing> InternetProtocolVersion4PacketProcessing<EINPDO, ICMPV4, TCP, UDP>
{
	#[inline(always)]
	pub(crate) fn drop<'lifetime>(&self, now: MonotonicMillisecondTimestamp, reason: EINPDO::IPV4INPDR, ethernet_addresses: &'lifetime EthernetAddresses, packet: impl EthernetIncomingNetworkPacket)
	{
		let reason = EthernetIncomingNetworkPacketDropReason::ProblematicInternetProtocolVersion4Packet
		{
			now,
			ethernet_addresses,
			reason,
		};
		
		self.dropped_packet_reporting.dropped_packet(reason);
		packet.free_direct_contiguous_packet();
	}
	
	#[inline(always)]
	pub(crate) fn is_internet_protocol_version_4_host_address_one_of_ours(&self, internet_protocol_version_4_host_address: InternetProtocolVersion4HostAddress) -> bool
	{
		self.our_valid_internet_protocol_version_4_host_addresses.is_internet_protocol_version_4_host_address_one_of_ours(internet_protocol_version_4_host_address)
	}
	
	#[inline(always)]
	pub(crate) fn is_internet_protocol_version_4_multicast_address_not_one_of_ours(&self, _internet_protocol_version_4_multicast_address: InternetProtocolVersion4HostAddress) -> bool
	{
		const MulticastIsUnsupportedAtThisTime: bool = false;
		
		MulticastIsUnsupportedAtThisTime
	}
	
	#[inline(always)]
	pub(crate) fn is_internet_protocol_version_4_host_address_not_one_of_ours(&self, internet_protocol_version_4_host_address: InternetProtocolVersion4HostAddress) -> bool
	{
		debug_assert!(internet_protocol_version_4_host_address.is_valid_unicast(), "internet_protocol_version_4_host_address '{:?}' is not valid unicast", internet_protocol_version_4_host_address);
		
		!self.is_internet_protocol_version_4_host_address_one_of_ours(internet_protocol_version_4_host_address)
	}
	
	#[inline(always)]
	pub(crate) fn is_source_internet_protocol_version_4_address_denied(&self, internet_protocol_version_4_host_address: &InternetProtocolVersion4HostAddress) -> bool
	{
		debug_assert!(internet_protocol_version_4_host_address.is_valid_unicast(), "internet_protocol_version_4_host_address '{:?}' is not valid unicast", internet_protocol_version_4_host_address);
		
		let nibbles = internet_protocol_version_4_host_address.nibbles_non_destructively();
		self.denied_source_internet_protocol_version_4_host_addresses.longest_match_present(&nibbles)
	}
	
	#[inline(always)]
	pub(crate) fn process_internet_control_message_protocol_version_4<'lifetime>(&self, now: MonotonicMillisecondTimestamp, packet: impl EthernetIncomingNetworkPacket, layer_4_packet: &'lifetime Layer4Packet, layer_4_length: u16, ethernet_addresses: &'lifetime EthernetAddresses, layer_4_check_sum_validated_in_hardware: bool)
	{
		self.internet_control_message_protocol_version_4_processing.process(now, packet, layer_4_packet, layer_4_length, ethernet_addresses, layer_4_check_sum_validated_in_hardware)
	}
	
	#[inline(always)]
	pub(crate) fn process_transmission_control_protocol<'lifetime>(&self, now: MonotonicMillisecondTimestamp, packet: impl EthernetIncomingNetworkPacket, layer_4_packet: &'lifetime Layer4Packet, layer_4_length: u16, ethernet_addresses: &'lifetime EthernetAddresses, layer_4_check_sum_validated_in_hardware: bool)
	{
		self.transmission_control_protocol_processing.process(now, packet, layer_4_packet, layer_4_length, ethernet_addresses, layer_4_check_sum_validated_in_hardware)
	}
	
	#[inline(always)]
	pub(crate) fn process_user_datagram_protocol<'lifetime>(&self, now: MonotonicMillisecondTimestamp, packet: impl EthernetIncomingNetworkPacket, layer_4_packet: &'lifetime Layer4Packet, layer_4_length: u16, ethernet_addresses: &'lifetime EthernetAddresses, layer_4_check_sum_validated_in_hardware: bool)
	{
		self.user_datagram_protocol_processing.process(now, packet, layer_4_packet, layer_4_length, ethernet_addresses, layer_4_check_sum_validated_in_hardware)
	}
}
