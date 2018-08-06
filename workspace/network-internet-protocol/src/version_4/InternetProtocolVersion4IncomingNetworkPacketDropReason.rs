// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Captures the reason and salient data for dropping with a packet earlier than might be expected.
///
/// This reason is reported immediately before the underlying packet is dropped, at which point all referenced data will no longer exist.
///
/// Salient data is by its nature unlikely to always be completely valid, and should be used only as a source of raw bytes.
#[derive(Debug)]
#[derive(Serialize)]
pub enum InternetProtocolVersion4IncomingNetworkPacketDropReason
{
	/// Packet is too short.
	PacketIsTooShort,
	
	/// Header is not version 4.
	HeaderIsNot4
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	},
	
	/// Total Length field is invalid.
	TotalLengthInvalid
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
		
		/// Expected total length.
		layer_3_length: u16,
	},
	
	/// Only occurs if the feature `drop-ipv4-packets-with-do-not-fragment-and-non-zero-identification` is configured.
	InvalidFragmentationFlagsOrIdentification
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	},
	
	/// Internet Control Message Protocol packets sent over Internet Protocol (IP) version 4 should not be fragmented.
	InternetControlMessageProtocolPacketsShouldNotBeFragmented
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	},
	
	/// Unsupported layer 4 protocol.
	UnsupportedLayer4Protocol
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
		
		/// Unsupported layer 4 protocol.
		unsupported_layer_4_protocol: u8,
	},
	
	/// Total length field is less than the length of the header.
	TotalLengthLessThanHeader
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	},
	
	/// Only occurs if the feature `drop-packets-with-ipv4-options` is configured.
	HasOptions
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	},
	
	/// Only occurs if the feature ``drop-packets-with-ipv4-options-lacking-zero-padding` is configured and the feature `drop-packets-with-ipv4-options` is *not* configured.
	///
	/// The end of the options list was not zero-padded.
	OptionsWereNotZeroPadded
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	},
	
	/// Only occurs if the feature `drop-packets-with-ipv4-options` is *not* configured.
	///
	/// These options are considered obsolete by RFC 7126 and so are dropped; there is no good reason for an end-host to receive them.
	///
	/// * Stream Identifier (SID) (Type = 136).
	/// * Probe MTU (MTUP) (Type = 11).
	/// * Reply MTU (MTUR) (Type = 12).
	/// * Traceroute (TR) (Type = 82) (actually as of RFC 6814).
	/// * Experimental Access Control (VISA) (Type = 142) (actually as of RFC 6814).
	/// * Extended Internet Protocol (EIP) (Type = 145) (actually as of RFC 6814).
	/// * Address Extension (ADDEXT) (Type = 147) (actually as of RFC 6814).
	/// * Sender Directed Multi-Destination Delivery (SDB) (Type = 149) (actually as of RFC 6814); also known as Selective Directed Broadcast.
	/// * Dynamic Packet State (DPS) (Type = 151) (actually as of RFC 6814).
	/// * Upstream Multicast Packet (UMP) (Type = 152) (actually as of RFC 6814).
	/// * ENCODE (ENCODE) (Type = 15) (actually as of RFC 6814).
	OptionIsObsoleteAsOfRfc7126
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
		
		option_kind: InternetProtocolVersion4OptionKind,
	},
	
	/// Only occurs if the feature `drop-packets-with-ipv4-options` is *not* configured.
	///
	/// These options are considered threats by RFC 7126 and so are dropped; there is no good reason for an end-host to receive them.
	///
	/// * Loose Source and Record Route (LSRR or LSR) (Type = 131).
	/// * Strict Source and Record Route (SSRR or SSR) (Type = 137).
	/// * Record Route (RR) (Type = 7).
	/// * Internet Timestamp (TS) (Type = 68).
	/// * Router Alert (RTRALT) (Type = 148).
	/// * Quick-Start (QS) (Type = 25).
	OptionIsThreatAsOfRfc7126
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
		
		option_kind: InternetProtocolVersion4OptionKind,
	},
	
	/// Only occurs if the feature `drop-packets-with-ipv4-options` is *not* configured.
	///
	/// The RFC 3692 style Experimental (EXP) options (Type = 30, 94, 158 or 222) as defined in RFC 4727 should not be seen on the internet.
	OptionIsExperimental
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	
		option_kind: InternetProtocolVersion4OptionKind,
	},
	
	/// Only occurs if the feature `drop-packets-with-ipv4-options` is *not* configured.
	///
	/// As of RFC 7126 these options are admissable but highly unlikely in modern scenarios.
	///
	/// * DoD Basic Security (SEC) (Type = 130).
	/// * DoD Extended Security (E-SEC) (Type = 133).
	/// * Commercial IP Security (CIPSO) (Type = 134).
	OptionIsSecurity
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	
		option_kind: InternetProtocolVersion4OptionKind,
	},
	
	/// Only occurs if the feature `drop-packets-with-ipv4-options` is *not* configured.
	///
	/// These are rarely encountered options registered with [IANA](https://www.iana.org/assignments/ip-parameters/ip-parameters.xhtml#ip-parameters-1) but not deprecated or obsoleted as yet.
	///
	/// Strictly speaking, we are not supposed to drop them but given they are extremely obscure it seems wise to do so.
	///
	/// * Experimental Measurement (ZSU) (Type = 10).
	/// * Experimental Flow Control (FINN) (Type = 205).
	/// * IMI Traffic Descriptor (IMITD) (Type = 144).
	/// * (Name not known) (Type = 150) (unassigned but previously in use until 2005).
	OptionIsRarelyEncounteredButRegisteredAtIana
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	
		option_kind: InternetProtocolVersion4OptionKind,
	},
	
	/// Only occurs if the feature `drop-packets-with-ipv4-options` is *not* configured.
	///
	/// Checks to see if an unknown option which should not be copied onto fragments is on a fragment.
	///
	/// There is no good reason to receive data in this situation.
	///
	/// This is rarely-encountered drop reason, as it only occurs after checking to see if an option kind is acceptable.
	OptionShouldNotBePresentOnFragments
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	
		option_kind: InternetProtocolVersion4OptionKind,
	},
	
	/// Only occurs if the feature `drop-packets-with-ipv4-options` is *not* configured.
	///
	/// Checks to see if an unknown option is using a reserved class (1 and 3 are reserved).
	///
	/// There is no good reason to receive data with this option kind.
	///
	/// This is rarely-encountered drop reason, as it only occurs after checking to see if an option kind is acceptable.
	OptionHasReservedClass
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	
		option_kind: InternetProtocolVersion4OptionKind,
	},
	
	/// Only occurs if the feature `drop-packets-with-ipv4-options` is *not* configured.
	///
	/// Checks to see if an unknown option is actually one with an assigned (or previously assigned) number but with different copy or class bits.
	///
	/// There is no good reason to receive data with this option kind.
	///
	/// This is rarely-encountered drop reason, as it only occurs after checking to see if an option kind is acceptable.
	OptionIsAssignedOrPreviouslyAssignedWithDifferentCopyOrClassBits
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	
		option_kind: InternetProtocolVersion4OptionKind,
	},
	
	/// Only occurs if the feature `drop-packets-with-ipv4-options` is *not* configured.
	///
	/// Checks to see if an option is a duplicate.
	///
	/// Duplicates are considered based on the five-bit `number` field of the option_kind.
	///
	/// This is rarely-encountered drop reason, as it only occurs after checking to see if an option kind is acceptable.
	OptionIsDuplicate
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	
		option_kind: InternetProtocolVersion4OptionKind,
	},
	
	/// Only occurs if the feature `drop-packets-with-ipv4-options` is *not* configured.
	///
	/// An option lacks the `length` field.
	///
	/// This is rarely-encountered drop reason, as it only occurs after checking to see if an option kind is acceptable.
	OptionLacksLength
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
		
		option_kind: InternetProtocolVersion4OptionKind,
	},
	
	/// Only occurs if the feature `drop-packets-with-ipv4-options` is *not* configured.
	///
	/// An option has a `length` field which is too short.
	///
	/// This is rarely-encountered drop reason, as it only occurs after checking to see if an option kind is acceptable.
	OptionLengthTooShort
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
		
		option_kind: InternetProtocolVersion4OptionKind,
	},
	
	/// Only occurs if the feature `drop-packets-with-ipv4-options` is *not* configured.
	///
	/// An option has a `length` field which is too long.
	///
	/// This is rarely-encountered drop reason, as it only occurs after checking to see if an option kind is acceptable.
	OptionLengthTooLong
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
		
		option_kind: InternetProtocolVersion4OptionKind,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	///
	/// Received a packet which did not have its Internet Protocol (IP) version 4 check sum validated in hardware and which, when calculated in software, was invalid.
	InternetProtocolCheckSumWhenCalculatedInSoftwareWasInvalid
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	///
	/// Received a packet with a source address that was the same as the destination address.
	SourceAndDestinationAddressAreTheSame
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	///
	/// Received a packet with a source address that was an invalid unicast address when the protocol was Internet Control Message Protocol (ICMP) or Transmission Control Protocol (TCP).
	///
	/// This can include the loopback, unspecified ('any'), broadcast and documentation addresses.
	SourceAddressNotValidUnicast
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	///
	/// Received a packet with a source address that was an invalid unicast address or was not the unspecified (any) address (0.0.0.0) when the protocol was User Datagram Protocol (UDP).
	///
	/// This can include the loopback, unspecified ('any'), broadcast and documentation addresses.
	SourceAddressNotValidUnicastOrUnspecified
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	///
	/// Received a packet with a source address that was denied (eg banned, firewalled).
	SourceAddressDenied
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	///
	/// Received a unicast packet to a destination that isn't us.
	UnicastDestinationIsNotUs
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	///
	/// Received an ethernet broadcast but the packet's destination address was not broadcast.
	EthernetBroadcastNotInternetBroadcast
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	MulticastAddressIsNotMulticast
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	MulticastAddressMismatchesEthernetAddress
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	MulticastAddressDenied
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	DestinationWasLoopbackorDocumentationAddress
	{
		/// Internet Protocol (IP) version 4 packet header.
		#[serde(serialize_with = "InternetProtocolVersion4IncomingNetworkPacketDropReason::serialize_non_null")]
		header: NonNull<InternetProtocolVersion4PacketHeader>,
	},
}

impl IncomingNetworkPacketProcessingDropReason for InternetProtocolVersion4IncomingNetworkPacketDropReason
{
}

impl InternetProtocolVersion4IncomingNetworkPacketDropReason
{
	#[inline(always)]
	fn serialize_non_null<S: Serializer, T: Serialize>(to_serialize: &NonNull<T>, serializer: S) -> Result<S::Ok, S::Error>
	{
		unsafe { to_serialize.as_ref().serialize(serializer) }
	}
}
