// This file is part of of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/of network/master/COPYRIGHT. No part of of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/of network/master/COPYRIGHT.


/// Captures the reason and salient data for dropping a packet earlier than might be expected.
///
/// This reason is reported immediately before the underlying packet is dropped, at which point all referenced data will no longer exist.
///
/// Salient data is by its nature unlikely to always be completely valid, and should be used only as a source of raw bytes.
///
/// * `IPV4INPDR` is the Internet Protocol (IP) version 4 incoming network packet drop reason type.
/// * `IPV6INPDR` is the Internet Protocol (IP) version 6 incoming network packet drop reason type.
/// * `ARPINPDR` is the Address Resolution Protocol (ARP) incoming network packet drop reason type.
#[derive(Debug)]
#[derive(Serialize)]
pub enum EthernetIncomingNetworkPacketDropReason<'ethernet_addresses, IPV4INPDR: IncomingNetworkPacketProcessingDropReason, IPV6INPDR: IncomingNetworkPacketProcessingDropReason, ARPINPDR: IncomingNetworkPacketProcessingDropReason>
{
	/// The packet's length was too short to be an ethernet packet.
	///
	/// This should be unusual, as most network hardware should not even be providing such packets to upper layers to process.
	IsTooShortToBeAnEthernetPacket,
	
	/// The packet had a source address which was not a valid unicast address.
	SourceEthernetAddressIsNotValidUnicast
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
	},
	
	/// The packet's source address was our unicast ethernet address; it is a spoof.
	SourceEthernetAddressIsOurUnicastEthernetAddress
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
	},
	
	/// A packet had a source address which was administratively denied.
	DeniedSourceEthernetAddress
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
	},
	
	/// A packet should never have a destination ethernet address of zero.
	DestinationEthernetAddressIsZero
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
	},
	
	/// This can occur if a link has multiple ethernet addresses or is listening promiscuously.
	DestinationEthernetAddressIsNotOneOfOurs
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
	},
	
	/// Currently recognised and supported EtherTypes are Internet Protocol (IP) Version 4, Internet Protocol (IP) Version 6, Address Resolution Protocol, Virtual LAN tagging (801.1q) and QinQ Virtual LAN tagging (802.1ad).
	///
	/// Ether frame sizes are entirely unsupported.
	UnsupportedEtherType
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
		
		/// EtherType or LegacyEthernetFrameSize.
		unsuspported_ether_type_or_legacy_ethernet_frame_size: EtherTypeOrLegacyEthernetFrameSize,
	},
	
	/// Wrapper around a problematic internet protocol version 4 packet.
	ProblematicInternetProtocolVersion4Packet
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
		
		/// Reason
		reason: IPV4INPDR,
	},
	
	/// Wrapper around a problematic internet protocol version 6 packet.
	ProblematicInternetProtocolVersion6Packet
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
		
		/// Reason
		reason: IPV6INPDR,
	},
	
	/// Wrapper around a problematic internet protocol version 6 packet.
	ProblematicAddressResolutionProtocolPacket
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
		
		/// Reason
		reason: ARPINPDR,
	},
	
	/// The packet's length was too short to be an IEEE 802.1Q (Virtual LAN) packet.
	///
	/// This should not occur when hardware strips IEEE 802.1Q (Virtual LAN) fields.
	IsTooShortToBeA8021QVirtualLanEthernetPacket,
	
	/// Occurs during packet processing of an IEEE 802.1Q (Virtual LAN) packet.
	CouldNotParse8011QVirtualLanTag
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,

		/// Tag Control Information (TCI).
		tag_control_information: TagControlInformation,
	},
	
	/// Occurs during packet processing of an IEEE 802.1Q (Virtual LAN) packet.
	NoConfigurationFor8011QVirtualLan
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,

		/// Tag Control Information (TCI).
		tag_control_information: TagControlInformation,
	},
	
	/// Occurs during packet processing of an IEEE 802.1Q (Virtual LAN) packet.
	DropEligibleFor8011QVirtualLan
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,

		/// Tag Control Information (TCI).
		tag_control_information: TagControlInformation,
	},
	
	/// Occurs during packet processing of an IEEE 802.1Q (Virtual LAN) packet.
	DropThisClassOfServiceFor8011QVirtualLan
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,

		/// Tag Control Information (TCI).
		tag_control_information: TagControlInformation,
	},
	
	/// The packet's length was too short to be an IEEE 802.1ad 'QinQ' (Virtual LAN) packet.
	///
	/// This should not occur when hardware strips IEEE 802.1ad 'QinQ' (Virtual LAN) fields.
	IsTooShortToBeAQinQVirtualLanEthernetPacket,
	
	/// Occurs during packet processing of an IEEE 802.1ad 'QinQ' (Virtual LAN) packet.
	CouldNotParseOuterVirtualLanTag
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,

		/// Outer Tag Control Information (TCI).
		outer_tag_control_information: TagControlInformation,

		/// Inner Tag Control Information (TCI).
		inner_tag_control_information: TagControlInformation,
	},
	
	/// Occurs during packet processing of an IEEE 802.1ad 'QinQ' (Virtual LAN) packet.
	CouldNotParseInnerVirtualLanTag
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,

		/// Outer Tag Control Information (TCI).
		outer_tag_control_information: TagControlInformation,

		/// Inner Tag Control Information (TCI).
		inner_tag_control_information: TagControlInformation,
	},
	
	/// Occurs during packet processing of an IEEE 802.1ad 'QinQ' (Virtual LAN) packet.
	NoConfigurationForQinQVirtualLan
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,

		/// Outer Tag Control Information (TCI).
		outer_tag_control_information: TagControlInformation,

		/// Inner Tag Control Information (TCI).
		inner_tag_control_information: TagControlInformation,
	},
	
	/// Occurs during packet processing of an IEEE 802.1ad 'QinQ' (Virtual LAN) packet.
	DropEligibleForOuterVirtualLan
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,

		/// Outer Tag Control Information (TCI).
		outer_tag_control_information: TagControlInformation,

		/// Inner Tag Control Information (TCI).
		inner_tag_control_information: TagControlInformation,
	},
	
	/// Occurs during packet processing of an IEEE 802.1ad 'QinQ' (Virtual LAN) packet.
	DropEligibleForInnerVirtualLan
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,

		/// Outer Tag Control Information (TCI).
		outer_tag_control_information: TagControlInformation,

		/// Inner Tag Control Information (TCI).
		inner_tag_control_information: TagControlInformation,
	},
	
	/// Occurs during packet processing of an IEEE 802.1ad 'QinQ' (Virtual LAN) packet.
	DropThisClassOfServiceForOuterVirtualLan
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,

		/// Outer Tag Control Information (TCI).
		outer_tag_control_information: TagControlInformation,

		/// Inner Tag Control Information (TCI).
		inner_tag_control_information: TagControlInformation,
	},
	
	/// Occurs during packet processing of an IEEE 802.1ad 'QinQ' (Virtual LAN) packet.
	DropThisClassOfServiceForInnerVirtualLan
	{
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,

		/// Outer Tag Control Information (TCI).
		outer_tag_control_information: TagControlInformation,

		/// Inner Tag Control Information (TCI).
		inner_tag_control_information: TagControlInformation,
	},
}

impl<'ethernet_addresses, IPV4INPDR: Sized + Debug, IPV6INPDR: Sized + Debug, ARPINPDR: Sized + Debug> IncomingNetworkPacketProcessingDropReason for EthernetIncomingNetworkPacketDropReason<'ethernet_addresses, IPV4INPDR, IPV6INPDR, ARPINPDR>
{
}
