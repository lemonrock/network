// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Layer 4 packet type.
///
/// If the packet is a tunneled packet, then this is known as the Outer Layer 4 packet type.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum HardwareOffloadTunnelPacketType
{
	/// Probably not a tunnel packet type.
	Uncategorised,
	
	/// Internet protocol (IP) in internet protocol (IP) tunnel.
	InternetProtocol,
	
	/// Generic Routing Encapsulation (GRE).
	GenericRoutingEncapsulation,
	
	/// Virtual eXtensible Local Area Network (VxLAN).
	VirtualExtensibleLocalAreaNetwork,
	
	/// Network Virtualization using Generic Routing Encapsulation (NVGRE).
	NetworkVirtualizationUsingGenericRoutingEncapsulation,
	
	/// Generic Network Virtualization Encapsulation (GENEVE).
	GenericNetworkVirtualizationEncapsulation,
	
	/// ?
	TeredoOrGenericRoutingEncapsulationOrVirtualExtensibleLocalAreaNetwork,
	
	/// GPRS Tunneling Protocol control (GTP-C).
	GprsTunnelingProtocolControl,
	
	/// GPRS Tunneling Protocol user data (GTP-U).
	GprsTunnelingProtocolUserData,
	
	/// IP Encapsulating Security Payload (ESP).
	///
	/// Part of IPsec.
	InternetProtocolEncapsulatingSecurityPayload,
	
	/// Layer 2 Tunneling Protocol (L2TP).
	Layer2TunnelingProtocol,
	
	/// Invalid or introduced after this code was written.
	Other,
}
