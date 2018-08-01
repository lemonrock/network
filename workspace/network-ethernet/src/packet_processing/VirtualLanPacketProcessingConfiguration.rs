// This file is part of of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/of network/master/COPYRIGHT. No part of of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/of network/master/COPYRIGHT.


/// Configuration.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct VirtualLanPacketProcessingConfiguration
{
	/// Outer QinQ Virtual LAN.
	pub outer: HashMap<(Option<VirtualLanIdentifier>, Option<VirtualLanIdentifier>), QinQVirtualLanPacketProcessingConfiguration>,
	
	/// Inner 802.1Q Virtual LAN.
	pub inner: HashMap<VirtualLanIdentifier, EthernetPacketProcessingConfiguration>,
	
	/// No virtual LANs.
	pub none: EthernetPacketProcessingConfiguration,
}

impl Display for VirtualLanPacketProcessingConfiguration
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl VirtualLanPacketProcessingConfiguration
{
	/// Configure.
	#[inline(always)]
	pub fn configure<'ethernet_addresses, INPDO: IncomingNetworkPacketDropObserver<DropReason=EthernetIncomingNetworkPacketDropReason<'ethernet_addresses>>>(mut self, dropped_packet_reporting: &Rc<INPDO>, our_valid_unicast_ethernet_address: MediaAccessControlAddress) -> VirtualLanPacketProcessing<'ethernet_addresses, INPDO>
	{
		VirtualLanPacketProcessing
		{
			outer: self.outer.drain().map(|(key, value)| (key, value.configure(dropped_packet_reporting, our_valid_unicast_ethernet_address))).collect(),
			
			inner: self.inner.drain().map(|(key, value)| (key, value.configure(dropped_packet_reporting, our_valid_unicast_ethernet_address))).collect(),
			
			none: self.none.configure(dropped_packet_reporting, our_valid_unicast_ethernet_address),
		}
	}
}
