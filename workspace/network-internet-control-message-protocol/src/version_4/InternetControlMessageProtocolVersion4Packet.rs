// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
#[derive(Debug)]
pub struct InternetControlMessageProtocolVersion4Packet
{
	/// Header.
	pub header: InternetControlMessageProtocolVersion4PacketHeader,

	/// Payload.
	pub payload: InternetControlMessageProtocolVersion4PacketPayload,
}

impl Display for InternetControlMessageProtocolVersion4Packet
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl InternetControlMessageProtocolVersion4Packet
{
//	/// Processes this packet.
//	///
//	/// Applies very strict rules to ignore traffic that is not secure.
//	#[inline(always)]
//	pub fn process_path_mtu_discovery_only<PathMtuDiscovery: Fn()>(&mut self, internet_control_message_protocol_packet_length: usize, internet_header_length: InternetHeaderLength, path_mtu_discovery: PathMtuDiscovery)
//	{
//		if internet_control_message_protocol_packet_length < size_of::<InternetControlMessageProtocolPacketHeader>()
//		{
//			return
//		}
//
//		match (self.header.type_, self.header.code)
//		{
//			// (Destination Unreachable for Path MTU Discovery, Fragmentation required, and DF flag set)
//			(InternetControlMessageProtocolType::DestinationUnreachable, 4) =>
//			{
//				if internet_control_message_protocol_packet_length != internet_header_length.internet_control_message_protocol_destination_unreachable_packet_length()
//				{
//					return
//				}
//
//				if self.is_internet_checksum_incorrect(internet_control_message_protocol_packet_length)
//				{
//					return
//				}
//			}
//
//			_ => (),
//		}
//	}
//
//	/// After this has executed, the checksum field will be zero.
//	#[inline(always)]
//	fn is_internet_checksum_incorrect(&mut self, internet_control_message_protocol_packet_length: usize) -> bool
//	{
//		unsupported!()
//		false
//	}
}
