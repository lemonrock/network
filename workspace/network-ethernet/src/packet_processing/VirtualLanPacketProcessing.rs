// This file is part of of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/of network/master/COPYRIGHT. No part of of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/of network/master/COPYRIGHT.


/// Packet processing configuration by Virtual LAN.
#[derive(Debug)]
pub struct VirtualLanPacketProcessing<EINPDO: EthernetIncomingNetworkPacketDropObserver<ARPINPDR=ARP::DropReason, IPV4INPDR=IPV4::DropReason, IPV6INPDR=IPV6::DropReason>, ARP: Layer3PacketProcessing, IPV4: Layer3PacketProcessing, IPV6: Layer3PacketProcessing>
{
	/// Outer QinQ Virtual LAN.
	pub outer: HashMap<(Option<VirtualLanIdentifier>, Option<VirtualLanIdentifier>), QinQVirtualLanPacketProcessing<EINPDO, ARP, IPV4, IPV6>>,
	
	/// Inner 802.1Q Virtual LAN.
	pub inner: HashMap<VirtualLanIdentifier, EthernetPacketProcessing<EINPDO, ARP, IPV4, IPV6>>,
	
	/// No virtual LANs.
	pub none: EthernetPacketProcessing<EINPDO, ARP, IPV4, IPV6>,
}

impl<EINPDO: EthernetIncomingNetworkPacketDropObserver<ARPINPDR=ARP::DropReason, IPV4INPDR=IPV4::DropReason, IPV6INPDR=IPV6::DropReason>, ARP: Layer3PacketProcessing, IPV4: Layer3PacketProcessing, IPV6: Layer3PacketProcessing> VirtualLanPacketProcessing<EINPDO, ARP, IPV4, IPV6>
{
	#[inline(always)]
	pub(crate) fn dropped_packet<'ethernet_addresses>(&self, reason: EthernetIncomingNetworkPacketDropReason<'ethernet_addresses, EINPDO::ARPINPDR, EINPDO::IPV4INPDR, EINPDO::IPV6INPDR>)
	{
		self.none.dropped_packet(reason)
	}
	
	#[inline(always)]
	pub(crate) fn get_packet_processing_for_outer_virtual_lan(&self, outer_virtual_lan_identifier: Option<VirtualLanIdentifier>, inner_virtual_lan_identifier: Option<VirtualLanIdentifier>) -> Option<&QinQVirtualLanPacketProcessing<EINPDO, ARP, IPV4, IPV6>>
	{
		self.outer.get(&(inner_virtual_lan_identifier, outer_virtual_lan_identifier))
	}
	
	#[inline(always)]
	pub(crate) fn get_packet_processing_for_inner_virtual_lan(&self, inner_virtual_lan_identifier: Option<VirtualLanIdentifier>) -> Option<&EthernetPacketProcessing<EINPDO, ARP, IPV4, IPV6>>
	{
		match inner_virtual_lan_identifier
		{
			None => Some(&self.none),
			Some(ref virtual_lan_identifier) => self.inner.get(virtual_lan_identifier),
		}
	}
}
