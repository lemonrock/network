// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Implementation of Address Resolution Protocol (ARP) packet processing.
#[derive(Debug)]
pub struct AddressResolutionPacketProcessing<EINPDO: EthernetIncomingNetworkPacketDropObserver>
{
	dropped_packet_reporting: Rc<EINPDO>,
	
	our_valid_internet_protocol_version_4_host_addresses: Rc<OurValidInternetProtocolVersion4HostAddresses>,
}

impl<EINPDO: EthernetIncomingNetworkPacketDropObserver<ARPINPDR=AddressResolutionProtocolIncomingNetworkPacketDropReason>> Layer3PacketProcessing for AddressResolutionPacketProcessing<EINPDO>
{
	type DropReason = AddressResolutionProtocolIncomingNetworkPacketDropReason;
	
	type CheckSumsValidated = ();
	
	#[inline(always)]
	fn process<'lifetime>(&self, packet: impl EthernetIncomingNetworkPacket, layer_3_packet: &'lifetime Layer3Packet, layer_3_length: u16, ethernet_addresses: &'lifetime EthernetAddresses, check_sum_validated_in_hardware: Self::CheckSumsValidated)
	{
		if unlikely!(AddressResolutionProtocolPacket::is_packet_length_too_short(layer_3_length))
		{
			drop!(PacketIsTooShort, ethernet_addresses, self, packet)
		}
		
		let address_resolution_protocol_packet: &'lifetime AddressResolutionProtocolPacket = layer_3_packet.as_type();
		
		address_resolution_protocol_packet.process(packet, self, layer_3_length, ethernet_addresses)
	}
}

impl<EINPDO: EthernetIncomingNetworkPacketDropObserver<ARPINPDR=AddressResolutionProtocolIncomingNetworkPacketDropReason>> AddressResolutionPacketProcessing<EINPDO>
{
	/// In order to observe dropped packets.
	#[inline(always)]
	pub(crate) fn drop<'lifetime>(&self, reason: EINPDO::ARPINPDR, ethernet_addresses: &'lifetime EthernetAddresses, packet: impl EthernetIncomingNetworkPacket)
	{
		let reason = EthernetIncomingNetworkPacketDropReason::ProblematicAddressResolutionProtocolPacket
		{
			ethernet_addresses,
			reason,
		};
		
		self.dropped_packet_reporting.dropped_packet(reason);
		packet.free_direct_contiguous_packet();
	}
}

impl<EINPDO: EthernetIncomingNetworkPacketDropObserver> AddressResolutionPacketProcessing<EINPDO>
{
	#[inline(always)]
	pub(crate) fn is_internet_protocol_version_4_host_address_one_of_ours(&self, internet_protocol_version_4_host_address: InternetProtocolVersion4HostAddress) -> bool
	{
		self.our_valid_internet_protocol_version_4_host_addresses.is_internet_protocol_version_4_host_address_one_of_ours(internet_protocol_version_4_host_address)
	}
	
	#[inline(always)]
	pub(crate) fn reply_to_probe<'ethernet_addresses>(&self, _packet: impl EthernetIncomingNetworkPacket, _ethernet_addresses: &'ethernet_addresses EthernetAddresses)
	{
		// TODO: REPLY
		arp_unsupported!("replies to probes are not supported");
	}
	
	#[inline(always)]
	pub(crate) fn reply_to_broadcast<'ethernet_addresses>(&self, _packet: impl EthernetIncomingNetworkPacket, _ethernet_addresses: &'ethernet_addresses EthernetAddresses)
	{
		// TODO: REPLY
		arp_unsupported!("replies to broadcasts are not supported");
	}
	
	#[inline(always)]
	pub(crate) fn add_to_address_resolution_cache(&self, _sender_hardware_address: &MediaAccessControlAddress, _sender_protocol_address: InternetProtocolVersion4HostAddress, packet: impl EthernetIncomingNetworkPacket)
	{
		// TODO: Manage an ARP cache.
		arp_unsupported!("adding to resolution cache");
		packet.free_direct_contiguous_packet();
	}
	
	#[inline(always)]
	pub(crate) fn internet_protocol_version_4_host_address_conflict<'ethernet_addresses>(&self, _packet: impl EthernetIncomingNetworkPacket, _ethernet_addresses: &'ethernet_addresses EthernetAddresses)
	{
		// TODO: Handle ARP host address conflicts; see AddressResolutionProtocolAddressConflictState.rs.
		arp_unsupported!("host address conflict");
	}
}
