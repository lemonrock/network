// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
///
/// Note that Internet protocol version 4 packet header checksums are not validated unless done by hardware offload.
#[repr(C, packed)]
pub struct Layer3Packet
{
//	/// An internet protocol (IP) version 4 packet.
//	pub internet_protocol_version_4_packet: InternetProtocolVersion4Packet,
//
//	/// An internet protocol (IP) version 6 packet.
//	pub internet_protocol_version_6_packet: InternetProtocolVersion6Packet,
//
//	/// An address resolution protocol (ARP) packet.
//	pub address_resolution_protocol_packet: AddressResolutionProtocolPacket,
	
	/// Other kinds of layer 3 packets, not differentiated.
	pub other: PhantomData<u8>,
}

impl Display for Layer3Packet
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Debug for Layer3Packet
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "(layer 3 packet)")
	}
}
