// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Ethernet packet processing configuration.
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct EthernetPacketProcessingConfiguration<L3PPC: Layer3PacketProcessingConfiguration>
{
	/// Inner 802.1Q Virtual LAN honour drop eligible.
	#[serde(default = "EthernetPacketProcessingConfiguration::<L3PPC>::inner_honour_drop_eligible_indicator_default")] pub inner_honour_drop_eligible_indicator: bool,
	
	/// Inner 802.1Q Virtual LAN permitted classes of service.
	#[serde(default)] pub inner_permitted_classes_of_service: PermittedClassesOfService,
	
	/// Blacklist or whitelist of ethernet addresses.
	#[serde(default)] pub source_ethernet_address_blacklist_or_whitelist: MediaAccessControlAddressList,
	
	/// Layer 3 packet processing configuration
	#[serde(default)] pub layer_3_packet_processing_configuration: L3PPC,
}

impl<'deserialize, L3PPC: Layer3PacketProcessingConfiguration> EthernetPacketProcessingConfiguration<L3PPC>
{
	/// Configure.
	#[inline(always)]
	pub fn configure<EINPDO: EthernetIncomingNetworkPacketDropObserver>(self, dropped_packet_reporting: &Rc<EINPDO>, our_valid_unicast_ethernet_address: MediaAccessControlAddress) -> EthernetPacketProcessing<EINPDO, L3PPC::L3PP>
	{
		EthernetPacketProcessing
		{
			dropped_packet_reporting: dropped_packet_reporting.clone(),
			inner_honour_drop_eligible_indicator: self.inner_honour_drop_eligible_indicator,
			inner_permitted_classes_of_service: self.inner_permitted_classes_of_service,
			our_valid_unicast_ethernet_address,
			source_ethernet_address_blacklist_or_whitelist: self.source_ethernet_address_blacklist_or_whitelist,
			layer_3_packet_processing: self.layer_3_packet_processing_configuration.configure(dropped_packet_reporting),
		}
	}
	
	#[inline(always)]
	fn inner_honour_drop_eligible_indicator_default() -> bool
	{
		true
	}
}
