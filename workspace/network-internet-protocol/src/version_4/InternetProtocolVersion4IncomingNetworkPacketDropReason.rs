// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Captures the reason and salient data for dropping with a packet earlier than might be expected.
///
/// This reason is reported immediately before the underlying packet is dropped, at which point all referenced data will no longer exist.
///
/// Salient data is by its nature unlikely to always be completely valid, and should be used only as a source of raw bytes.
#[derive(Debug)]
pub enum InternetProtocolVersion4IncomingNetworkPacketDropReason<'header>
{
	/// Packet is too short.
	PacketIsTooShort,
	
	/// Header is not version 4.
	HeaderIsNot4
	{
		/// Internet Protocol (IP) version 4 packet header.
		header: &'header InternetProtocolVersion4PacketHeader,
	},
	
	/// Total Length field is invalid.
	TotalLengthInvalid
	{
		/// Internet Protocol (IP) version 4 packet header.
		header: &'header InternetProtocolVersion4PacketHeader,
	},
	
	/// Only occurs if the feature `drop-ipv4-packets-with-do-not-fragment-and-non-zero-identification` is configured.
	InvalidFragmentationFlagsOrIdentification
	{
		/// Internet Protocol (IP) version 4 packet header.
		header: &'header InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	TotalLengthLessThanHeader
	{
		/// Internet Protocol (IP) version 4 packet header.
		header: &'header InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	///
	/// Only if configured.
	HasOptions
	{
		/// Internet Protocol (IP) version 4 packet header.
		header: &'header InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	OptionLacksKind
	{
		/// Internet Protocol (IP) version 4 packet header.
		header: &'header InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	OptionLengthTooShort
	{
		/// Internet Protocol (IP) version 4 packet header.
		header: &'header InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	OptionLengthTooLong
	{
		/// Internet Protocol (IP) version 4 packet header.
		header: &'header InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	///
	/// Received a packet with a source address that was an invalid unicast address.
	///
	/// This can include the loopback, unspecified ('any'), broadcast and documentation addresses.
	SourceAddressNotValidUnicast
	{
		/// Internet Protocol (IP) version 4 packet header.
		header: &'header InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	///
	/// Received a packet with a source address that was denied (eg banned, firewalled).
	SourceAddressDenied
	{
		/// Internet Protocol (IP) version 4 packet header.
		header: &'header InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	///
	/// Received a unicast packet to a destination that isn't us.
	UnicastDestinationIsNotUs
	{
		/// Internet Protocol (IP) version 4 packet header.
		header: &'header InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	///
	/// Received an ethernet broadcast but the packet's destination address was not broadcast.
	EthernetBroadcastNotInternetBroadcast
	{
		/// Internet Protocol (IP) version 4 packet header.
		header: &'header InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	MulticastAddressIsNotMulticast
	{
		/// Internet Protocol (IP) version 4 packet header.
		header: &'header InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	MulticastAddressMismatchesEthernetAddress
	{
		/// Internet Protocol (IP) version 4 packet header.
		header: &'header InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	MulticastAddressDenied
	{
		/// Internet Protocol (IP) version 4 packet header.
		header: &'header InternetProtocolVersion4PacketHeader,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	DestinationWasLoopbackorDocumentationAddress
	{
		/// Internet Protocol (IP) version 4 packet header.
		header: &'header InternetProtocolVersion4PacketHeader,
	},
}

impl IncomingNetworkPacketProcessingDropReason for InternetProtocolVersion4IncomingNetworkPacketDropReason
{
}
