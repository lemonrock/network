// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Ethernet packet processing configuration.
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct EthernetPacketProcessingConfiguration
{
	/// Inner 802.1Q Virtual LAN honour drop eligible.
	#[serde(default = "EthernetPacketProcessingConfiguration::inner_honour_drop_eligible_indicator_default")] pub inner_honour_drop_eligible_indicator: bool,
	
	/// Inner 802.1Q Virtual LAN permitted classes of service.
	#[serde(default)] pub inner_permitted_classes_of_service: PermittedClassesOfService,
	
	/// Blacklist or whitelist of ethernet addresses.
	#[serde(default)] pub source_ethernet_address_blacklist_or_whitelist: MediaAccessControlAddressList,
}

impl EthernetPacketProcessingConfiguration
{
	/// Configure.
	#[inline(always)]
	pub fn configure<'ethernet_addresses, INPDO: IncomingNetworkPacketDropObserver<DropReason=EthernetIncomingNetworkPacketDropReason<'ethernet_addresses>>>(mut self, dropped_packet_reporting: &Rc<INPDO>, our_valid_unicast_ethernet_address: MediaAccessControlAddress) -> EthernetPacketProcessing<'ethernet_addresses, INPDO>
	{
		EthernetPacketProcessing
		{
			dropped_packet_reporting: dropped_packet_reporting.clone(),
			inner_honour_drop_eligible_indicator: self.inner_honour_drop_eligible_indicator,
			inner_permitted_classes_of_service: self.inner_permitted_classes_of_service,
			source_ethernet_address_blacklist_or_whitelist: self.source_ethernet_address_blacklist_or_whitelist,
			our_valid_unicast_ethernet_address,
		}
	}
	
	#[inline(always)]
	fn inner_honour_drop_eligible_indicator_default() -> bool
	{
		true
	}
}
