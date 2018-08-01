// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Implementation of Layer 3 packet processing.
#[derive(Debug)]
pub struct Layer3PacketProcessingImpl<IPV4INPDR, IPV6INPDR, ARPINPDR>
{
	marker: (IPV4INPDR, IPV6INPDR, ARPINPDR),
}

impl<IPV4INPDR, IPV6INPDR, ARPINPDR> Layer3PacketProcessingImpl<IPV4INPDR, IPV6INPDR, ARPINPDR>
{
	/// In order to observe dropped packets.
	#[inline(always)]
	pub fn dropped_packet<'ethernet_addresses>(&self, _reason: EthernetIncomingNetworkPacketDropReason<'ethernet_addresses, IPV4INPDR, IPV6INPDR, ARPINPDR>)
	{
		panic!();
	}
	
	/// Also used to manage Address Resolution Protocol (ARP).
	#[inline(always)]
	pub fn is_internet_protocol_version_4_host_address_one_of_ours(&self, _internet_protocol_version_4_host_address: InternetProtocolVersion4HostAddress) -> bool
	{
		panic!();
	}
	
	/// Used to manage Address Resolution Protocol (ARP).
	#[inline(always)]
	pub fn reply_to_probe<'ethernet_addresses>(&self, _packet: impl EthernetIncomingNetworkPacket, _ethernet_addresses: &'ethernet_addresses EthernetAddresses)
	{
		// TODO: REPLY
		arp_unsupported!("replies to probes are not supported");
	}
	
	/// Used to manage Address Resolution Protocol (ARP).
	#[inline(always)]
	pub fn reply_to_broadcast<'ethernet_addresses>(&self, _packet: impl EthernetIncomingNetworkPacket, _ethernet_addresses: &'ethernet_addresses EthernetAddresses)
	{
		// TODO: REPLY
		arp_unsupported!("replies to broadcasts are not supported");
	}
	
	/// Used to manage Address Resolution Protocol (ARP).
	#[inline(always)]
	pub fn add_to_address_resolution_cache(&self, _sender_hardware_address: &MediaAccessControlAddress, _sender_protocol_address: InternetProtocolVersion4HostAddress, packet: impl EthernetIncomingNetworkPacket)
	{
		// TODO: Manage an ARP cache.
		arp_unsupported!("adding to resolution cache");
		packet.free_direct_contiguous_packet();
	}
	
	/// Used to manage Address Resolution Protocol (ARP).
	#[inline(always)]
	pub fn internet_protocol_version_4_host_address_conflict<'ethernet_addresses>(&self, _packet: impl EthernetIncomingNetworkPacket, _ethernet_addresses: &'ethernet_addresses EthernetAddresses)
	{
		// TODO: Handle ARP host address conflicts; see AddressResolutionProtocolAddressConflictState.rs.
		arp_unsupported!("host address conflict");
	}
}
