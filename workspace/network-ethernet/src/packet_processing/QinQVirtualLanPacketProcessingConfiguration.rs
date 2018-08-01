// This file is part of of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/of network/master/COPYRIGHT. No part of of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/of network/master/COPYRIGHT.


/// Configuration.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct QinQVirtualLanPacketProcessingConfiguration
{
	/// Outer QinQ Virtual LAN permitted classes of service.
	pub outer_packet_processing: EthernetPacketProcessingConfiguration,
	
	/// Inner packet processing configuration.
	pub inner_packet_processing: EthernetPacketProcessingConfiguration,
}

impl QinQVirtualLanPacketProcessingConfiguration
{
	/// Configure.
	#[inline(always)]
	pub fn configure<'ethernet_addresses, INPDO: IncomingNetworkPacketDropObserver<DropReason=EthernetIncomingNetworkPacketDropReason<'ethernet_addresses>>>(mut self, dropped_packet_reporting: &Rc<INPDO>, our_valid_unicast_ethernet_address: MediaAccessControlAddress) -> QinQVirtualLanPacketProcessing<'ethernet_addresses, INPDO>
	{
		QinQVirtualLanPacketProcessing
		{
			outer_packet_processing: self.outer_packet_processing.configure(dropped_packet_reporting, our_valid_unicast_ethernet_address),
			inner_packet_processing: self.inner_packet_processing.configure(dropped_packet_reporting, our_valid_unicast_ethernet_address),
		}
	}
}
