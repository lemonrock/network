// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Captures the reason and salient data for dropping with a packet earlier than might be expected.
///
/// This reason is reported immediately before the underlying packet is dropped, at which point all referenced data will no longer exist.
///
/// Salient data is by its nature unlikely to always be completely valid, and should be used only as a source of raw bytes.
#[derive(Debug)]
#[derive(Serialize)]
pub enum InternetProtocolVersion6IncomingNetworkPacketDropReason<ICMPV6INPDR: IncomingNetworkPacketProcessingDropReason, TCPINPDR: IncomingNetworkPacketProcessingDropReason, UDPINPDR: IncomingNetworkPacketProcessingDropReason>
{
	/// Packet is too short.
	PacketIsTooShort,
	
	/// Header is not version 6.
	HeaderIsNot6
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Flow label was zero.
	///
	/// Only if the feature `drop-ipv6-packets-with-non-zero-flow-label` is configured.
	FlowLabelIsNonZero
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Hop-by-hop options extension header was not the first extension header.
	HopByHopOptionsIsNotFirstExtensionHeader
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Hop-by-hop options extension header was too short.
	HopByHopOptionsUnderflow
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Hop-by-hop options extension header was too long.
	HopByHopOptionsHeaderExtensionLengthOverflow
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Type-Length-Value option in an extension header was too short.
	TypeLengthValueOptionTypeUnderflow
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Type-Length-Value option in an extension header was too short.
	TypeLengthValueOptionLengthUnderflow
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Type-Length-Value option in an extension header was too short.
	TypeLengthValueOptionDataUnderflow
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Type-Length-Value option in an extension header required discard of packet if not supported.
	TypeLengthValueOptionDiscardPacket
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
		
		/// Option type.
		option_type: u8,
	},
	
	/// Type-Length-Value option in an extension header was experimental or should not be used on the internet.
	TypeLengthValueOptionShouldNotBeUsedOnTheInternet
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
		
		/// Option type.
		option_type: u8,
	},
	
	/// Routing extension header was repeated.
	RoutingExtensionHeaderRepeated
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Routing extension header was too short.
	RoutingExtensionHeaderUnderflow
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// We are not a router, and the routing extension header has route segments left, ie we are not the final destination.
	RoutingExtensionHeaderHasSegmentsLeft
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
		
		/// Routing type.
		routing_type: u8,
		
		/// Segments left.
		segments_left: u8,
	},
	
	/// We are not a router, and the routing extension header had a deprecated, experimental or reserved routing type.
	RoutingExtensionHeaderRoutingTypeIsDeprecatedExperimentalOrReserved
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
		
		/// Routing type.
		routing_type: u8,
		
		/// Segments left.
		segments_left: u8,
	},
	
	/// Fragment extension header was repeated.
	FragmentExtensionHeaderRepeated
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Fragment extension header was too short.
	FragmentExtensionHeaderUnderflow
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Fragment extension header had its first reserved field set to a value which is not zero.
	///
	/// Only if the feature `drop-ipv6-fragments-when-first-reserved-field-is-not-zero` was configured.
	FragmentExtensionHeaderFirstReservedFieldNonZero
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
		
		/// Value of the reserved field.
		reserved: u8,
	},
	
	/// Fragment extension header had its first reserved field set to a value which is not zero.
	///
	/// Only if the feature `drop-ipv6-fragments-when-second-reserved-field-is-not-zero` was configured.
	FragmentExtensionHeaderSecondReservedFieldNonZero
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
		
		/// Value of the reserved field.
		reserved: u8,
	},
	
	/// Fragment extension header implies that there is only one fragment, or that the last fragment is the first.
	///
	/// Whilst strictly possible, there is no good reason to send fragmented packets when there is only one fragment!
	FragmentExtensionHeaderOnlyOneFragmentOrLastFragmentIsFirst
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Packet fragment, apart from the last fragment, has a length which is not a multiple of eight (8).
	PacketFragmentNotAMultipleOfEight
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Reassembling using this fragment would make the packet including payload larger than 65,535 bytes.
	PacketFragmentWouldMakeReassembledPacketWouldTooLarge
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// A packet has overly small fragments.
	///
	/// This is usually indicative of an attack.
	PacketFragmentTooSmall
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// The encapsulating security payload (ESP) extension header is unsupported.
	///
	/// ESP is part of IPsec.
	EncapulatingSecurityPayloadExtensionHeaderUnsupported
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// The authentication header (AH) extension header is unsupported.
	///
	/// AH is part of IPsec.
	AuthenticationHeaderExtensionHeaderUnsupported
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// The no-next-header 'psuedo' extension header is unsupported.
	NoNextHeaderIsUnsupported
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// The destination options extension header was repeated more than twice.
	MoreThanTwoDestinationOptionsExtensionHeaders
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// The mobility extension header is unsupported.
	MobilityExtensionHeaderUnsupported
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// The host identity protocol (HIP) extension header is unsupported.
	HostIdentityProtocolExtensionHeaderUnsupported
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// The SHIM6 extension header is unsupported.
	Shim6ProtocolExtensionHeaderUnsupported
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Experimentation extension headers are unsupported.
	ExperimentationExtensionHeaderUnsupported
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// An unrecognised extension header or unsupported layer 4 protocol was specified.
	///
	/// Only a very small number of layer 4 protocols are supported.
	UnrecognisedExtensionHeaderOrLayer4Protocol
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
		
		/// Next header (extension header type) or layer 4 protocol number.
		next_header: u8,
	},
	
	/// Occurs during Internet Protocol (IP) version 4 packet processing.
	///
	/// Received a packet with a source address that was the same as the destination address.
	SourceAndDestinationAddressAreTheSame
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Received a packet with a source address that was an invalid unicast address.
	///
	/// This can include the loopback, unspecified ('any'), broadcast and documentation addresses.
	SourceAddressNotValidUnicast
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Received a packet with a source address that was denied (eg banned, firewalled).
	SourceAddressDenied
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Received a packet with a destination address that was reserved for documentation.
	DestinationAddressDocumentation
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Received a packet with a destination address that was reserved for loopback (ie is should never be received by a network card).
	DestinationAddressLoopback
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Received a packet with a destination address that was reserved for multicast 'loopback' (interface-local) (ie is should never be received by a network card).
	DestinationAddressInterfaceLocal
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Received a unicast packet to a destination that isn't us.
	UnicastDestinationIsNotUs
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Occurs during Internet Protocol (IP) version 6 packet processing.
	MulticastAddressIsNotMulticast
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Occurs during Internet Protocol (IP) version 6 packet processing.
	MulticastAddressIsNotValidMulticast
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
		
		/// Parsing error.
		parsing_error: InternetProtocolVersion6MulticastAddressParseError,
	},
	
	/// Occurs during Internet Protocol (IP) version 6 packet processing.
	MulticastAddressMismatchesEthernetAddress
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Occurs during Internet Protocol (IP) version 6 packet processing.
	EthernetBroadcastShouldNotOccur
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Occurs during Internet Protocol (IP) version 6 packet processing.
	MulticastAddressDenied
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Occurs during Internet Protocol (IP) version 6 packet processing.
	DestinationWasLoopbackOrDocumentationAddress
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Occurs during Internet Protocol (IP) version 6 packet processing.
	TransmissionControlProtocolPacketsShouldOnlyBeUnicast
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Occurs during Internet Protocol (IP) version 6 packet processing.
	UserDatagramProtocolPacketsMustHaveACheckSumSet
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Internet Control Message Protocol (ICMP) version 6 packets sent over Internet Protocol (IP) version 6 should not be fragmented.
	InternetControlMessageProtocolVersion6PacketsShouldNotBeFragmented
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
	},
	
	/// Wrapper around a problematic Internet Control Message Protocol (ICMP) version 6 packet.
	ProblematicInternetControlMessageProtocolVersion6Packet
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
		
		/// Reason
		reason: ICMPV6INPDR,
	},
	
	/// Wrapper around a problematic Transmission Control Protocol (TCP) packet.
	ProblematicTransmissionControlProtocolPacket
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
		
		/// Reason
		reason: TCPINPDR,
	},
	
	/// Wrapper around a problematic User Datagram Protocol (UDP) packet.
	ProblematicUserDatagramProtocolPacket
	{
		/// Internet Protocol (IP) version 6 packet header.
		#[serde(serialize_with = "InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null")]
		header: NonNull<InternetProtocolVersion6PacketHeader>,
		
		/// Reason
		reason: UDPINPDR,
	},
}

impl<ICMPV6INPDR: IncomingNetworkPacketProcessingDropReason, TCPINPDR: IncomingNetworkPacketProcessingDropReason, UDPINPDR: IncomingNetworkPacketProcessingDropReason> IncomingNetworkPacketProcessingDropReason for InternetProtocolVersion6IncomingNetworkPacketDropReason<ICMPV6INPDR, TCPINPDR, UDPINPDR>
{
}

#[allow(non_snake_case)]
#[inline(always)]
fn InternetProtocolVersion6IncomingNetworkPacketDropReason_serialize_non_null<S: Serializer>(to_serialize: &NonNull<InternetProtocolVersion6PacketHeader>, serializer: S) -> Result<S::Ok, S::Error>
{
	unsafe { to_serialize.as_ref().serialize(serializer) }
}
