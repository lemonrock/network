// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
#[derive(Debug)]
pub struct InternetProtocolVersion4Packet
{
	/// Header.
	pub header: InternetProtocolVersion4PacketHeader,
	
	/// Options.
	pub options: PhantomData<u8>,
	
	/// Payload.
	pub payload: Layer4Packet,
}

impl Display for InternetProtocolVersion4Packet
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl InternetProtocolVersion4Packet
{
	/// Is the packet length too short?
	#[inline(always)]
	pub fn is_packet_length_too_short(layer_3_length: u16) -> bool
	{
		layer_3_length < InternetProtocolVersion4PacketHeader::HeaderSizeU16
	}
	
	#[inline(always)]
	pub(crate) fn process<'lifetime, EINPDO: EthernetIncomingNetworkPacketDropObserver<IPV4INPDR=InternetProtocolVersion4IncomingNetworkPacketDropReason>>(&'lifetime self, packet: impl EthernetIncomingNetworkPacket, packet_processing: &InternetProtocolVersion4PacketProcessing<EINPDO>, layer_3_length: u16, ethernet_addresses: &'lifetime EthernetAddresses)
	{
		unimplemented!();
	}
}
