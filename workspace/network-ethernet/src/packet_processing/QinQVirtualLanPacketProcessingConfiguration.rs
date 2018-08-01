// This file is part of of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/of network/master/COPYRIGHT. No part of of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/of network/master/COPYRIGHT.


/// Configuration.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct QinQVirtualLanPacketProcessingConfiguration<L3PPC: Layer3PacketProcessingConfiguration>
{
	/// Outer QinQ Virtual LAN permitted classes of service.
	pub outer_packet_processing: EthernetPacketProcessingConfiguration<L3PPC>,
	
	/// Inner packet processing configuration.
	pub inner_packet_processing: EthernetPacketProcessingConfiguration<L3PPC>,
}

impl<L3PPC: Layer3PacketProcessingConfiguration> QinQVirtualLanPacketProcessingConfiguration<L3PPC>
{
	/// Configure.
	#[inline(always)]
	pub fn configure<EINPDO: EthernetIncomingNetworkPacketDropObserver>(self, dropped_packet_reporting: &Rc<EINPDO>, our_valid_unicast_ethernet_address: MediaAccessControlAddress) -> QinQVirtualLanPacketProcessing<EINPDO, L3PPC::L3PP>
	{
		QinQVirtualLanPacketProcessing
		{
			outer_packet_processing: self.outer_packet_processing.configure(dropped_packet_reporting, our_valid_unicast_ethernet_address),
			inner_packet_processing: self.inner_packet_processing.configure(dropped_packet_reporting, our_valid_unicast_ethernet_address),
		}
	}
}
