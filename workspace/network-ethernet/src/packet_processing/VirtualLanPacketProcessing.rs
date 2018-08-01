// This file is part of of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/of network/master/COPYRIGHT. No part of of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/of network/master/COPYRIGHT.


/// Packet processing configuration by Virtual LAN.
#[derive(Debug)]
pub struct VirtualLanPacketProcessing<'ethernet_addresses, INPDO: IncomingNetworkPacketDropObserver<DropReason=EthernetIncomingNetworkPacketDropReason<'ethernet_addresses>>>
{
	/// Outer QinQ Virtual LAN.
	pub outer: HashMap<(Option<VirtualLanIdentifier>, Option<VirtualLanIdentifier>), QinQVirtualLanPacketProcessing<'ethernet_addresses, INPDO>>,
	
	/// Inner 802.1Q Virtual LAN.
	pub inner: HashMap<VirtualLanIdentifier, EthernetPacketProcessing<'ethernet_addresses, INPDO>>,
	
	/// No virtual LANs.
	pub none: EthernetPacketProcessing<'ethernet_addresses, INPDO>,
}

impl<'ethernet_addresses, INPDO: IncomingNetworkPacketDropObserver<DropReason=EthernetIncomingNetworkPacketDropReason<'ethernet_addresses>>> VirtualLanPacketProcessing<'ethernet_addresses, INPDO>
{
	#[inline(always)]
	pub(crate) fn dropped_packet(&self, reason: EthernetIncomingNetworkPacketDropReason<'ethernet_addresses>)
	{
		self.none.dropped_packet(reason)
	}
	
	#[inline(always)]
	pub(crate) fn get_packet_processing_for_outer_virtual_lan(&self, outer_virtual_lan_identifier: Option<VirtualLanIdentifier>, inner_virtual_lan_identifier: Option<VirtualLanIdentifier>) -> Option<&QinQVirtualLanPacketProcessing<'ethernet_addresses, INPDO>>
	{
		self.outer.get(&(inner_virtual_lan_identifier, outer_virtual_lan_identifier))
	}
	
	#[inline(always)]
	pub(crate) fn get_packet_processing_for_inner_virtual_lan(&self, inner_virtual_lan_identifier: Option<VirtualLanIdentifier>) -> Option<&EthernetPacketProcessing<'ethernet_addresses, INPDO>>
	{
		match inner_virtual_lan_identifier
		{
			None => Some(&self.none),
			Some(ref virtual_lan_identifier) => self.inner.get(virtual_lan_identifier),
		}
	}
}
