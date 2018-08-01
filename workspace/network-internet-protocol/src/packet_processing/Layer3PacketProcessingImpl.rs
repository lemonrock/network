// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Implementation of Layer 3 packet processing.
#[derive(Debug)]
pub struct Layer3PacketProcessingImpl<EINPDO: EthernetIncomingNetworkPacketDropObserver>
{
	dropped_packet_reporting: Rc<EINPDO>,
	
	
	
	/// Our unicast internet protocol (IP) version 4 host addresses valid for this network interface.
	///
	/// No sender packet should be received from this address; if it was, it implies loopback on this interface, which is daft.
	our_valid_internet_protocol_version_4_host_addresses: HashSet<InternetProtocolVersion4HostAddress>,
	
	/// Our multicast internet protocol (IP) version 4 host addresses valid for this network interface.
	///
	/// No sender packet should be received from this address; if it was, it implies loopback on this interface, which is daft.
	our_valid_internet_protocol_version_4_multicast_addresses: HashSet<InternetProtocolVersion4HostAddress>,
	
	denied_source_internet_protocol_version_4_host_addresses: TreeBitmap<()>,
	
	
	
	/// No sender packet should be received from this address; if it was, it implies loopback on this interface, which is daft.
	our_valid_internet_protocol_version_6_host_addresses: HashSet<InternetProtocolVersion6HostAddress>,
	
	/// No sender packet should be received from this address; if it was, it implies loopback on this interface, which is daft.
	our_valid_internet_protocol_version_6_multicast_addresses: HashSet<InternetProtocolVersion6HostAddress>,
	
	denied_source_internet_protocol_version_6_host_addresses: TreeBitmap<()>,
}

impl<EINPDO: EthernetIncomingNetworkPacketDropObserver> Layer3PacketProcessing for Layer3PacketProcessingImpl<EINPDO>
{
	#[inline(always)]
	fn process_internet_protocol_version_4<'ethernet_addresses>(&self, packet: impl EthernetIncomingNetworkPacket, layer_3_packet: &'ethernet_addresses Layer3Packet, layer_3_length: u16, ethernet_addresses: &'ethernet_addresses EthernetAddresses)
	{
		unimplemented!()
	}
	
	#[inline(always)]
	fn process_internet_protocol_version_6<'ethernet_addresses>(&self, packet: impl EthernetIncomingNetworkPacket, layer_3_packet: &'ethernet_addresses Layer3Packet, layer_3_length: u16, ethernet_addresses: &'ethernet_addresses EthernetAddresses)
	{
		unimplemented!()
	}
	
	#[inline(always)]
	fn process_address_resolution_protocol<'ethernet_addresses>(&self, packet: impl EthernetIncomingNetworkPacket, layer_3_packet: &'ethernet_addresses Layer3Packet, layer_3_length: u16, ethernet_addresses: &'ethernet_addresses EthernetAddresses)
	{
		// TODO: This is decidely unpleasant to implement, as we do not know about ARP data structures here.
		unimplemented!()
	}
}

impl<EINPDO: EthernetIncomingNetworkPacketDropObserver> Layer3PacketProcessingImpl<EINPDO>
{
	/// In order to observe dropped packets.
	#[inline(always)]
	pub fn dropped_packet<'ethernet_addresses>(&self, reason: EthernetIncomingNetworkPacketDropReason<'ethernet_addresses, EINPDO::IPV4INPDR, EINPDO::IPV6INPDR, EINPDO::ARPINPDR>)
	{
		self.dropped_packet_reporting.dropped_packet(reason)
	}
	
	/// Public because also used by the Address Resolution Protocol (ARP) `network-address-resolution-protocol` crate.
	#[inline(always)]
	pub fn is_internet_protocol_version_4_host_address_one_of_ours(&self, internet_protocol_version_4_host_address: InternetProtocolVersion4HostAddress) -> bool
	{
		debug_assert!(internet_protocol_version_4_host_address.is_valid_unicast(), "internet_protocol_version_4_host_address '{:?}' is not valid unicast", internet_protocol_version_4_host_address);
		
		self.our_valid_internet_protocol_version_4_host_addresses.contains(&internet_protocol_version_4_host_address)
	}
	
	#[inline(always)]
	pub(crate) fn is_internet_protocol_version_4_multicast_address_not_one_of_ours(&self, internet_protocol_version_4_multicast_address: InternetProtocolVersion4HostAddress) -> bool
	{
		const MulticastIsUnsupportedAtThisTime: bool = false;
		
		MulticastIsUnsupportedAtThisTime
	}
	
	#[inline(always)]
	pub(crate) fn is_internet_protocol_version_6_host_address_not_one_of_our_multicast_addresses(&self, internet_protocol_version_6_multicast_address: &InternetProtocolVersion6HostAddress) -> bool
	{
		// TODO: solicited node check implicit group membership.
		// TODO: all nodes (FF02::1); equivalent to 224.0.0.1 and 255.255.255.255.
		
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
	pub(crate) fn is_source_internet_protocol_version_4_address_denied(&self, internet_protocol_version_4_host_address: &InternetProtocolVersion4HostAddress) -> bool
	{
		debug_assert!(internet_protocol_version_4_host_address.is_valid_unicast(), "internet_protocol_version_4_host_address '{:?}' is not valid unicast", internet_protocol_version_4_host_address);
		
		let nibbles = internet_protocol_version_4_host_address.nibbles_non_destructively();
		self.denied_source_internet_protocol_version_4_host_addresses.longest_match_present(&nibbles)
	}
	
	#[inline(always)]
	pub(crate) fn is_source_internet_protocol_version_6_address_denied(&self, internet_protocol_version_6_host_address: &InternetProtocolVersion6HostAddress) -> bool
	{
		debug_assert!(internet_protocol_version_6_host_address.is_valid_unicast(), "internet_protocol_version_6_host_address '{:?}' is not valid unicast", internet_protocol_version_6_host_address);
		
		let nibbles = internet_protocol_version_6_host_address.nibbles_non_destructively();
		self.denied_source_internet_protocol_version_6_host_addresses.longest_match_present(&nibbles)
	}
	
	/// Public because used by the Address Resolution Protocol (ARP) `network-address-resolution-protocol` crate.
	#[inline(always)]
	pub fn reply_to_probe<'ethernet_addresses>(&self, _packet: impl EthernetIncomingNetworkPacket, _ethernet_addresses: &'ethernet_addresses EthernetAddresses)
	{
		// TODO: REPLY
		arp_unsupported!("replies to probes are not supported");
	}
	
	/// Public because used by the Address Resolution Protocol (ARP) `network-address-resolution-protocol` crate.
	#[inline(always)]
	pub fn reply_to_broadcast<'ethernet_addresses>(&self, _packet: impl EthernetIncomingNetworkPacket, _ethernet_addresses: &'ethernet_addresses EthernetAddresses)
	{
		// TODO: REPLY
		arp_unsupported!("replies to broadcasts are not supported");
	}
	
	/// Public because used by the Address Resolution Protocol (ARP) `network-address-resolution-protocol` crate.
	#[inline(always)]
	pub fn add_to_address_resolution_cache(&self, _sender_hardware_address: &MediaAccessControlAddress, _sender_protocol_address: InternetProtocolVersion4HostAddress, packet: impl EthernetIncomingNetworkPacket)
	{
		// TODO: Manage an ARP cache.
		arp_unsupported!("adding to resolution cache");
		packet.free_direct_contiguous_packet();
	}
	
	/// Public because used by the Address Resolution Protocol (ARP) `network-address-resolution-protocol` crate.
	#[inline(always)]
	pub fn internet_protocol_version_4_host_address_conflict<'ethernet_addresses>(&self, _packet: impl EthernetIncomingNetworkPacket, _ethernet_addresses: &'ethernet_addresses EthernetAddresses)
	{
		// TODO: Handle ARP host address conflicts; see AddressResolutionProtocolAddressConflictState.rs.
		arp_unsupported!("host address conflict");
	}
}
