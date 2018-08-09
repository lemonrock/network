// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Hardware offload categorised layer 2 packet type.
///
/// Most DPDK drivers, excluding Intel's, do not categorise.
///
/// If those that do, not all of these values may be supported by the driver.
///
/// As such, that makes this categorisation close to useless.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub enum HardwareOffloadCategorisedLayer2PacketType
{
	/// IEEE1588 (802.1AS) timestamp.
	///
	/// Not present in tunneled (inner) layer 2 packets.
	///
	/// EtherType 0x88F7.
	Ieee1588TimeSync,
	
	/// Address Resolution Protocol (ARP).
	///
	/// Not present in tunneled (inner) layer 2 packets.
	///
	/// EtherType 0x0806.
	AddressResolutionProtocol,
	
	/// Link Layer Discovery Protocol (LLDP).
	///
	/// Not present in tunneled (inner) layer 2 packets.
	///
	/// EtherType 0x88CC.
	LinkLayerDiscoveryProtocol,
	
	/// Network Service Header (NSH).
	///
	/// Not present in tunneled (inner) layer 2 packets.
	///
	/// EtherType 0x894F.
	NetworkServiceHeader,
	
	/// Virtual LAN.
	///
	/// Only valid if Virtual LAN stripping is disabled for Intel drivers.
	///
	/// EtherType 0x8100F.
	VirtualLan,
	
	/// QinQ Virtual LAN.
	///
	/// ?Only valid if Virtual LAN stripping is disabled for Intel drivers?
	///
	/// EtherType 0x88A8.
	QinQVirtualLan,
	
	/// PPPoE.
	///
	/// Not present in tunneled (inner) layer 2 packets.
	///
	/// EtherType 0x8863 or EtherType 0x8864.
	PPPoE,
	
	/// Invalid or introduced after this code was written.
	Other,
}
