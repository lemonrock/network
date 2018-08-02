// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Implementation of Internet Protocol (IP) version 4 packet processing.
#[derive(Debug)]
pub struct InternetProtocolVersion4PacketProcessing<EINPDO: EthernetIncomingNetworkPacketDropObserver<IPV4INPDR=InternetProtocolVersion4IncomingNetworkPacketDropReason>>
{
	dropped_packet_reporting: Rc<EINPDO>,
	
	our_valid_internet_protocol_version_4_host_addresses: Rc<OurValidInternetProtocolVersion4HostAddresses>,
	
	/// Our multicast internet protocol (IP) version 4 host addresses valid for this network interface.
	///
	/// No sender packet should be received from this address; if it was, it implies loopback on this interface, which is daft.
	our_valid_internet_protocol_version_4_multicast_addresses: HashSet<InternetProtocolVersion4HostAddress>,
	
	denied_source_internet_protocol_version_4_host_addresses: TreeBitmap<()>,
}

impl<EINPDO: EthernetIncomingNetworkPacketDropObserver<IPV4INPDR=InternetProtocolVersion4IncomingNetworkPacketDropReason>> Layer3PacketProcessing for InternetProtocolVersion4PacketProcessing<EINPDO>
{
	type DropReason = EINPDO::IPV4INPDR;
	
	#[inline(always)]
	fn process<'lifetime>(&self, packet: impl EthernetIncomingNetworkPacket, layer_3_packet: &'lifetime Layer3Packet, layer_3_length: u16, ethernet_addresses: &'lifetime EthernetAddresses)
	{
		if unlikely!(InternetProtocolVersion4Packet::is_packet_length_too_short(layer_3_length))
		{
			drop!(InternetProtocolVersion4IncomingNetworkPacketDropReason::PacketIsTooShort, ethernet_addresses, self, packet)
		}
		
		let internet_protocol_version_4_packet: &'lifetime InternetProtocolVersion4Packet = layer_3_packet.as_type();
		
		internet_protocol_version_4_packet.process(packet, self, layer_3_length, ethernet_addresses)
	}
}

impl<EINPDO: EthernetIncomingNetworkPacketDropObserver<IPV4INPDR=InternetProtocolVersion4IncomingNetworkPacketDropReason>> InternetProtocolVersion4PacketProcessing<EINPDO>
{
	#[inline(always)]
	pub(crate) fn drop<'lifetime>(&self, reason: EINPDO::IPV4INPDR, ethernet_addresses: &'lifetime EthernetAddresses, packet: impl EthernetIncomingNetworkPacket)
	{
		let reason = EthernetIncomingNetworkPacketDropReason::ProblematicInternetProtocolVersion4Packet
		{
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
}
