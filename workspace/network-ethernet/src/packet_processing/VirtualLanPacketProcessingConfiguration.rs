// This file is part of of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/of network/master/COPYRIGHT. No part of of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/of network/master/COPYRIGHT.


/// Configuration.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct VirtualLanPacketProcessingConfiguration<L3PPC: Layer3PacketProcessingConfiguration>
{
	/// Outer QinQ Virtual LAN.
	pub outer: HashMap<(Option<VirtualLanIdentifier>, Option<VirtualLanIdentifier>), QinQVirtualLanPacketProcessingConfiguration<L3PPC>>,
	
	/// Inner 802.1Q Virtual LAN.
	pub inner: HashMap<VirtualLanIdentifier, EthernetPacketProcessingConfiguration<L3PPC>>,
	
	/// No virtual LANs.
	pub none: EthernetPacketProcessingConfiguration<L3PPC>,
}

impl<L3PPC: Layer3PacketProcessingConfiguration> VirtualLanPacketProcessingConfiguration<L3PPC>
{
	/// Configure.
	#[inline(always)]
	pub fn configure<EINPDO: EthernetIncomingNetworkPacketDropObserver>(mut self, dropped_packet_reporting: &Rc<EINPDO>, our_valid_unicast_ethernet_address: MediaAccessControlAddress) -> VirtualLanPacketProcessing<EINPDO, L3PPC::L3PP>
	{
		VirtualLanPacketProcessing
		{
			outer: self.outer.drain().map(|(key, value)| (key, value.configure(dropped_packet_reporting, our_valid_unicast_ethernet_address))).collect(),
			
			inner: self.inner.drain().map(|(key, value)| (key, value.configure(dropped_packet_reporting, our_valid_unicast_ethernet_address))).collect(),
			
			none: self.none.configure(dropped_packet_reporting, our_valid_unicast_ethernet_address),
		}
	}
}
