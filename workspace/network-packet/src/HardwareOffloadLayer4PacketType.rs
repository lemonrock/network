// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Hardware offload layer 4 packet type.
///
/// Only a small number of DPDK drivers categorise; mostly Intel's, Mellanox's and Cisco's.
///
/// If those that do, not all of these values may be supported by the driver, althought TCP, UDP and Fragmented seem to be supported by all of them.
///
/// In this case, DPDK drivers categorise as `OtherNotAFragment`.
///
/// If the packet is a tunneled packet, then this is known as the Outer Layer 4 packet type.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub enum HardwareOffloadLayer4PacketType
{
	/// Either the driver did not categorise this packet or the layer 4 data structure is absent.
	UncategorisedOrAbsent,
	
	/// Transmission Control Protocol (TCP).
	TransmissionControlProtocol,
	
	/// User Datagram Protocol (UCP).
	UserDatagramProtocol,
	
	/// Stream Control Transmission Protocol (SCTP).
	StreamControlTransmissionProtocol,
	
	/// Internet Control Message Protocol (ICMP).
	///
	/// Only used on networks supporting internet protocol (IP) version 4.
	InternetControlMessageProtocol,
	
	/// A fragmented internet protocol (IP) version 4 or version 6 packet.
	///
	/// Will never be the first fragment.
	///
	/// May not necessarily be a fragment of a TCP, UDP, SCTP or ICMP packet.
	Fragmented,
	
	/// A internet protocol (IP) version 4 or version 6 packet which is:-
	///
	/// * not TCP, UDP (and for Intel DPDK drivers, SCTP or ICMP);
	/// * not a fragment
	OtherNotAFragment,
	
	/// Invalid or introduced after this code was written.
	Other,
}
