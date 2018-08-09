// This file is part of of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/of network/master/COPYRIGHT. No part of of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/of network/master/COPYRIGHT.


/// Configuration.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct QinQVirtualLanPacketProcessingConfiguration<ARP: Layer3PacketProcessingConfiguration, IPV4: Layer3PacketProcessingConfiguration, IPV6: Layer3PacketProcessingConfiguration>
	where ARP::L3PP : Layer3PacketProcessing<CheckSumsValidated=()>, IPV4::L3PP : Layer3PacketProcessing<CheckSumsValidated=(bool, bool)>, IPV6::L3PP : Layer3PacketProcessing<CheckSumsValidated=bool>
{
	/// Outer QinQ Virtual LAN permitted classes of service.
	pub outer_packet_processing: EthernetPacketProcessingConfiguration<ARP, IPV4, IPV6>,
	
	/// Inner packet processing configuration.
	pub inner_packet_processing: EthernetPacketProcessingConfiguration<ARP, IPV4, IPV6>,
}

impl<ARP: Layer3PacketProcessingConfiguration, IPV4: Layer3PacketProcessingConfiguration, IPV6: Layer3PacketProcessingConfiguration> QinQVirtualLanPacketProcessingConfiguration<ARP, IPV4, IPV6>
	where ARP::L3PP : Layer3PacketProcessing<CheckSumsValidated=()>, IPV4::L3PP : Layer3PacketProcessing<CheckSumsValidated=(bool, bool)>, IPV6::L3PP : Layer3PacketProcessing<CheckSumsValidated=bool>
{
	/// Configure.
	#[inline(always)]
	pub fn configure<EINPDO: EthernetIncomingNetworkPacketDropObserver<ARPINPDR=<ARP::L3PP as Layer3PacketProcessing>::DropReason, IPV4INPDR=<IPV4::L3PP as Layer3PacketProcessing>::DropReason, IPV6INPDR=<IPV6::L3PP as Layer3PacketProcessing>::DropReason>>(self, dropped_packet_reporting: &Rc<EINPDO>, our_valid_unicast_ethernet_address: MediaAccessControlAddress) -> QinQVirtualLanPacketProcessing<EINPDO, ARP::L3PP, IPV4::L3PP, IPV6::L3PP>
	{
		QinQVirtualLanPacketProcessing
		{
			outer_packet_processing: self.outer_packet_processing.configure(dropped_packet_reporting, our_valid_unicast_ethernet_address),
			inner_packet_processing: self.inner_packet_processing.configure(dropped_packet_reporting, our_valid_unicast_ethernet_address),
		}
	}
}
