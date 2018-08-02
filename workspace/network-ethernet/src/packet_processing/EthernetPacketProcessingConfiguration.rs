// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Ethernet packet processing configuration.
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct EthernetPacketProcessingConfiguration<ARPC: AddressResolutionProtocolPacketProcessingConfiguration, IPV4C: InternetProtocolVersion4PacketProcessingConfiguration, IPV6C: InternetProtocolVersion6PacketProcessingConfiguration>
{
	/// Inner 802.1Q Virtual LAN honour drop eligible.
	#[serde(default = "EthernetPacketProcessingConfiguration::<ARP, IPV4, IPV6>::inner_honour_drop_eligible_indicator_default")] pub inner_honour_drop_eligible_indicator: bool,
	
	/// Inner 802.1Q Virtual LAN permitted classes of service.
	#[serde(default)] pub inner_permitted_classes_of_service: PermittedClassesOfService,
	
	/// Blacklist or whitelist of ethernet addresses.
	#[serde(default)] pub source_ethernet_address_blacklist_or_whitelist: MediaAccessControlAddressList,
	
	/// Address Resolution Protocol (ARP) packet processing configuration.
	#[serde(default)] pub address_resolution_protocol_packet_processing_configuration: ARPC,
	
	/// Internet Protocol (IP) version 4 packet processing configuration.
	#[serde(default)] pub internet_protocol_version_4_packet_processing_configuration: IPV4C,
	
	/// Internet Protocol (IP) version 6 packet processing configuration.
	#[serde(default)] pub internet_protocol_version_6_packet_processing_configuration: IPV6C,
}

impl<ARPC: AddressResolutionProtocolPacketProcessingConfiguration, IPV4C: InternetProtocolVersion4PacketProcessingConfiguration, IPV6C: InternetProtocolVersion6PacketProcessingConfiguration> EthernetPacketProcessingConfiguration<ARPC, IPV4C, IPV6C>
{
	/// Configure.
	#[inline(always)]
	pub fn configure<EINPDO: EthernetIncomingNetworkPacketDropObserver>(self, dropped_packet_reporting: &Rc<EINPDO>, our_valid_unicast_ethernet_address: MediaAccessControlAddress) -> EthernetPacketProcessing<EINPDO, ARPC::ARP, IPV4C::IPV4, IPV6C::IPV6>
	{
		EthernetPacketProcessing
		{
			dropped_packet_reporting: dropped_packet_reporting.clone(),
			inner_honour_drop_eligible_indicator: self.inner_honour_drop_eligible_indicator,
			inner_permitted_classes_of_service: self.inner_permitted_classes_of_service,
			our_valid_unicast_ethernet_address,
			source_ethernet_address_blacklist_or_whitelist: self.source_ethernet_address_blacklist_or_whitelist,
			address_resolution_protocol_packet_processing: self.address_resolution_protocol_packet_processing_configuration.configure(dropped_packet_reporting),
			internet_protocol_version_4_packet_processing: self.internet_protocol_version_4_packet_processing_configuration.configure(dropped_packet_reporting),
			internet_protocol_version_6_packet_processing: self.internet_protocol_version_6_packet_processing_configuration.configure(dropped_packet_reporting),
		}
	}
	
	#[inline(always)]
	fn inner_honour_drop_eligible_indicator_default() -> bool
	{
		true
	}
}
