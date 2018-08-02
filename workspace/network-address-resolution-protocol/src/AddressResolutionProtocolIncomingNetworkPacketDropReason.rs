// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Captures the reason and salient data for dropping a packet earlier than might be expected.
///
/// This reason is reported immediately before the underlying packet is dropped, at which point all referenced data will no longer exist.
///
/// Salient data is by its nature unlikely to always be completely valid, and should be used only as a source of raw bytes.
#[derive(Debug)]
#[derive(Serialize)]
pub enum AddressResolutionProtocolIncomingNetworkPacketDropReason<'header>
{
	/// Temporary reason until support for Address Resolution Protocol replies is implemented.
	ReuseInReply,
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	PacketIsTooShort,
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	NotSupportedForAnythingOtherThanInternetProtocolVersion4,
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	DestinationEthernetAddressIsMulticast
	{
		/// Address Reolution Protocol (ARP) packet header.
		header: &'header AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	OperationIsUnsupported
	{
		/// Address Reolution Protocol (ARP) packet header.
		header: &'header AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	RequestIsMulticast
	{
		/// Address Reolution Protocol (ARP) packet header.
		header: &'header AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	///
	/// This is a violation of RFC 5227; it is only checked for if so configured with the feature `drop-arp-requests-with-non-zero-target-hardware-address`.
	RequestTargetHardwareAddressIsZero
	{
		/// Address Reolution Protocol (ARP) packet header.
		header: &'header AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	HardwareAndPacketSourceEthernetAddressMismatch
	{
		/// Address Reolution Protocol (ARP) packet header.
		header: &'header AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	HardwareAndPacketDestinationEthernetAddressMismatch
	{
		/// Address Reolution Protocol (ARP) packet header.
		header: &'header AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	ProbeIsNotForUs
	{
		/// Address Reolution Protocol (ARP) packet header.
		header: &'header AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	///
	/// This is not evidence of anything untoward.
	BroadcastIsNotForUs
	{
		/// Address Reolution Protocol (ARP) packet header.
		header: &'header AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	RequestIsNotAProbeAndIsNotBroadcast
	{
		/// Address Reolution Protocol (ARP) packet header.
		header: &'header AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	RequestIsNotAProbeAndSenderProtocolAddressIsNotUnicast
	{
		/// Address Reolution Protocol (ARP) packet header.
		header: &'header AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	GratuitousReplyIsNotValidUnicast
	{
		/// Address Reolution Protocol (ARP) packet header.
		header: &'header AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	ReplyTargetHardwareAddressIsNotValidUnicast
	{
		/// Address Reolution Protocol (ARP) packet header.
		header: &'header AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	ReplySourceAndTargetProtocolAddressesAreTheSame
	{
		/// Address Reolution Protocol (ARP) packet header.
		header: &'header AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	ReplySenderProtocolAddressIsNotValidUnicast
	{
		/// Address Reolution Protocol (ARP) packet header.
		header: &'header AddressResolutionProtocolPacketHeader,
	},
	
	/// Occurs during Address Reolution Protocol (ARP) packet processing.
	ReplyTargetProtocolAddressIsNotValidUnicast
	{
		/// Address Reolution Protocol (ARP) packet header.
		header: &'header AddressResolutionProtocolPacketHeader,
	},
}
