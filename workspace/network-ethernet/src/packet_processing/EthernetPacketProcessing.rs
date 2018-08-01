// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Packet processing configuration for a particular combination of Outer Virtual LAN tag, Inner Virtual LAN tag and (our valid unicast) Ethernet Address.
#[derive(Debug)]
pub struct EthernetPacketProcessing<EINPDO: EthernetIncomingNetworkPacketDropObserver, L3PP: Layer3PacketProcessing>
{
	dropped_packet_reporting: Rc<EINPDO>,
	
	/// Inner 802.1Q Virtual LAN honour drop eligible.
	inner_honour_drop_eligible_indicator: bool,
	
	/// Inner 802.1Q Virtual LAN permitted classes of service.
	inner_permitted_classes_of_service: PermittedClassesOfService,
	
	/// Our unicast ethernet addresses valid for this network interface.
	///
	/// No sender packet should be received from this address; if it was, it implies loopback on this interface, which is daft.
	our_valid_unicast_ethernet_address: MediaAccessControlAddress,
	
	/// Blacklist or whitelist of ethernet addresses.
	source_ethernet_address_blacklist_or_whitelist: MediaAccessControlAddressList,

	/// Layer 3 packet processing.
	layer_3_packet_processing: L3PP,
}

impl<EINPDO: EthernetIncomingNetworkPacketDropObserver, L3PP: Layer3PacketProcessing> Layer3PacketProcessing for EthernetPacketProcessing<EINPDO, L3PP>
{
	#[inline(always)]
	fn process_internet_protocol_version_4<'ethernet_addresses>(&self, packet: impl IncomingNetworkPacket, layer_3_packet: &'ethernet_addresses Layer3Packet, layer_3_length: u16, ethernet_addresses: &'ethernet_addresses EthernetAddresses)
	{
		self.layer_3_packet_processing.process_internet_protocol_version_4(packet, layer_3_packet, layer_3_length, ethernet_addresses)
	}
	
	#[inline(always)]
	fn process_internet_protocol_version_6<'ethernet_addresses>(&self, packet: impl IncomingNetworkPacket, layer_3_packet: &'ethernet_addresses Layer3Packet, layer_3_length: u16, ethernet_addresses: &'ethernet_addresses EthernetAddresses)
	{
		self.layer_3_packet_processing.process_internet_protocol_version_6(packet, layer_3_packet, layer_3_length, ethernet_addresses)
	}
	
	#[inline(always)]
	fn process_address_resolution_protocol<'ethernet_addresses>(&self, packet: impl IncomingNetworkPacket, layer_3_packet: &'ethernet_addresses Layer3Packet, layer_3_length: u16, ethernet_addresses: &'ethernet_addresses EthernetAddresses)
	{
		self.layer_3_packet_processing.process_address_resolution_protocol(packet, layer_3_packet, layer_3_length, ethernet_addresses)
	}
}

impl<EINPDO: EthernetIncomingNetworkPacketDropObserver, L3PP: Layer3PacketProcessing> EthernetPacketProcessing<EINPDO, L3PP>
{
	#[inline(always)]
	pub(crate) fn dropped_packet<'ethernet_addresses>(&self, reason: EthernetIncomingNetworkPacketDropReason<'ethernet_addresses, EINPDO::IPV4INPDR, EINPDO::IPV6INPDR, EINPDO::ARPINPDR>)
	{
		self.dropped_packet_reporting.dropped_packet(reason)
	}
	
	#[inline(always)]
	pub(crate) fn honour_drop_eligible_indicator(&self, drop_eligible_indicator: bool) -> bool
	{
		drop_eligible_indicator && self.inner_honour_drop_eligible_indicator
	}
	
	#[inline(always)]
	pub(crate) fn drop_packets_of_class_of_service(&self, class_of_service: ClassOfService) -> bool
	{
		self.inner_permitted_classes_of_service.is_denied(class_of_service)
	}
	
	#[inline(always)]
	pub(crate) fn is_ethernet_address_our_valid_unicast_ethernet_address(&self, destination_ethernet_address: &MediaAccessControlAddress) -> bool
	{
		debug_assert!(destination_ethernet_address.is_valid_unicast(), "ethernet_address '{:?}' is not valid unicast", destination_ethernet_address);
		
		&self.our_valid_unicast_ethernet_address == destination_ethernet_address
	}
	
	#[inline(always)]
	pub(crate) fn is_ethernet_address_not_our_valid_unicast_ethernet_address(&self, destination_ethernet_address: &MediaAccessControlAddress) -> bool
	{
		debug_assert!(destination_ethernet_address.is_valid_unicast(), "destination_ethernet_address '{:?}' is not valid unicast", destination_ethernet_address);
		
		&self.our_valid_unicast_ethernet_address != destination_ethernet_address
	}
	
	#[inline(always)]
	pub(crate) fn is_denied_source_ethernet_address(&self, source_ethernet_address: &MediaAccessControlAddress) -> bool
	{
		debug_assert!(source_ethernet_address.is_valid_unicast(), "source_ethernet_address '{:?}' is not valid unicast", source_ethernet_address);
		
		self.source_ethernet_address_blacklist_or_whitelist.is_denied(&source_ethernet_address)
	}
}
