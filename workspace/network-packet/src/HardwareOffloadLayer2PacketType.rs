// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Hardware offload layer 2 packet type.
///
/// If the packet is a tunneled packet, then this is known as the Outer Layer 2 packet type.
///
/// Must DPDK drivers do not categorise and return `Unknown`, making this categorisation close to useless.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub enum HardwareOffloadLayer2PacketType
{
	/// Either:-
	///
	/// * Ignore this packet.
	/// * HardwareOffload categorisation hasn't happened.
	///
	/// It seems possible that some DPDK drivers don't set `RTE_PTYPE_L2_ETHER` even on ethernet packets.
	Unknown,
	
	/// Ether packet; may be further categorised.
	///
	/// Most drivers, excluding Intel's, do not categorise further.
	Ethernet(Option<HardwareOffloadCategorisedLayer2PacketType>),
}
