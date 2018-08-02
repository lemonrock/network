// This file is part of of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/of network/master/COPYRIGHT. No part of of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/of network/master/COPYRIGHT.


/// Configuration.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct QinQVirtualLanPacketProcessingConfiguration<ARPC: AddressResolutionProtocolPacketProcessingConfiguration, IPV4C: InternetProtocolVersion4PacketProcessingConfiguration, IPV6C: InternetProtocolVersion6PacketProcessingConfiguration>
{
	/// Outer QinQ Virtual LAN permitted classes of service.
	pub outer_packet_processing: EthernetPacketProcessingConfiguration<ARPC, IPV4C, IPV6C>,
	
	/// Inner packet processing configuration.
	pub inner_packet_processing: EthernetPacketProcessingConfiguration<ARPC, IPV4C, IPV6C>,
}

impl<ARPC: AddressResolutionProtocolPacketProcessingConfiguration, IPV4C: InternetProtocolVersion4PacketProcessingConfiguration, IPV6C: InternetProtocolVersion6PacketProcessingConfiguration> QinQVirtualLanPacketProcessingConfiguration<ARPC, IPV4C, IPV6C>
{
	/// Configure.
	#[inline(always)]
	pub fn configure<EINPDO: EthernetIncomingNetworkPacketDropObserver>(self, dropped_packet_reporting: &Rc<EINPDO>, our_valid_unicast_ethernet_address: MediaAccessControlAddress) -> QinQVirtualLanPacketProcessing<EINPDO, ARPC::ARP, IPV4C::IPV4, IPV6C::IPV6>
	{
		QinQVirtualLanPacketProcessing
		{
			outer_packet_processing: self.outer_packet_processing.configure(dropped_packet_reporting, our_valid_unicast_ethernet_address),
			inner_packet_processing: self.inner_packet_processing.configure(dropped_packet_reporting, our_valid_unicast_ethernet_address),
		}
	}
}
