// This file is part of of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/of network/master/COPYRIGHT. No part of of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/of network/master/COPYRIGHT.


/// Configuration.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct VirtualLanPacketProcessingConfiguration<ARPC: AddressResolutionProtocolPacketProcessingConfiguration, IPV4C: InternetProtocolVersion4PacketProcessingConfiguration, IPV6C: InternetProtocolVersion6PacketProcessingConfiguration>
{
	/// Outer QinQ Virtual LAN.
	pub outer: HashMap<(Option<VirtualLanIdentifier>, Option<VirtualLanIdentifier>), QinQVirtualLanPacketProcessingConfiguration<ARPC, IPV4C, IPV6C>>,
	
	/// Inner 802.1Q Virtual LAN.
	pub inner: HashMap<VirtualLanIdentifier, EthernetPacketProcessingConfiguration<ARPC, IPV4C, IPV6C>>,
	
	/// No virtual LANs.
	pub none: EthernetPacketProcessingConfiguration<ARPC, IPV4C, IPV6C>,
}

impl<ARPC: AddressResolutionProtocolPacketProcessingConfiguration, IPV4C: InternetProtocolVersion4PacketProcessingConfiguration, IPV6C: InternetProtocolVersion6PacketProcessingConfiguration> VirtualLanPacketProcessingConfiguration<ARPC, IPV4C, IPV6C>
{
	/// Configure.
	#[inline(always)]
	pub fn configure<EINPDO: EthernetIncomingNetworkPacketDropObserver<ARPINPDR=<ARPC::ARP as AddressResolutionProtocolPacketProcessing>::DropReason, IPV4INPDR=<IPV4C::IPV4 as InternetProtocolVersion4PacketProcessing>::DropReason, IPV6INPDR=<IPV6C::IPV6 as InternetProtocolVersion6PacketProcessing>::DropReason>>(mut self, dropped_packet_reporting: &Rc<EINPDO>, our_valid_unicast_ethernet_address: MediaAccessControlAddress) -> VirtualLanPacketProcessing<EINPDO, ARPC::ARP, IPV4C::IPV4, IPV6C::IPV6>
	{
		VirtualLanPacketProcessing
		{
			outer: self.outer.drain().map(|(key, value)| (key, value.configure(dropped_packet_reporting, our_valid_unicast_ethernet_address))).collect(),
			
			inner: self.inner.drain().map(|(key, value)| (key, value.configure(dropped_packet_reporting, our_valid_unicast_ethernet_address))).collect(),
			
			none: self.none.configure(dropped_packet_reporting, our_valid_unicast_ethernet_address),
		}
	}
}
