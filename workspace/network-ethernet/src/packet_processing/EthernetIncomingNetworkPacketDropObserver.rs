// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Allows for notification of dropped incoming network packets.
pub trait EthernetIncomingNetworkPacketDropObserver: Debug
{
	/// Address Resolution Protocol (ARP) incoming network packet drop reason type.
	type ARPINPDR: IncomingNetworkPacketProcessingDropReason;
	
	/// Internet Protocol (IP) version 4 incoming network packet drop reason type.
	type IPV4INPDR: IncomingNetworkPacketProcessingDropReason;
	
	/// Internet Protocol (IP) version 6 incoming network packet drop reason type.
	type IPV6INPDR: IncomingNetworkPacketProcessingDropReason;
	
	/// Implement this to observe dropped packets.
	#[inline(always)]
	fn dropped_packet<'ethernet_addresses>(&self, reason: EthernetIncomingNetworkPacketDropReason<'ethernet_addresses, Self::ARPINPDR, Self::IPV4INPDR, Self::IPV6INPDR>);
}
