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
pub enum EthernetIncomingNetworkPacketDropReason<'ethernet_addresses, ARPINPDR: IncomingNetworkPacketProcessingDropReason, IPV4INPDR: IncomingNetworkPacketProcessingDropReason, IPV6INPDR: IncomingNetworkPacketProcessingDropReason>
{
	/// The packet's length was too short to be an ethernet packet.
	///
	/// This should be unusual, as most network hardware should not even be providing such packets to upper layers to process.
	IsTooShortToBeAnEthernetPacket
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
	},
	
	/// Hardware offloading categorised this as a tunnel packet.
	HardwareOffloadingCategorisationIsTunnelPacket
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
	},
	
	/// Hardware offloading categorised this as an unwanted packet.
	///
	/// This can include packets related to IEEE 1488 timestamping.
	HardwareOffloadingCategorisationIsUnwanted
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
	},
	
	/// Hardware offloading categorised this packet's layer 4 protocol and it is one we do not want.
	HardwareOffloadingCategorisationUnwantedLayer4ProtocolInInternetProtocolVersion4Packet
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
		
		/// Harware offload layer 4 packet type.
		hardware_offload_layer_4_packet_type: HardwareOffloadLayer4PacketType,
	},
	
	/// Hardware offloading categorised this packet's layer 4 protocol and it is one we do not want.
	HardwareOffloadingCategorisationUnwantedLayer4ProtocolInInternetProtocolVersion6Packet
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
		
		/// Harware offload layer 4 packet type.
		hardware_offload_layer_4_packet_type: HardwareOffloadLayer4PacketType,
	},
	
	/// Hardware offloading determined the Internet Protocol (IP) version 4 check sum was bad.
	HardwareOffloadingInternetProtocolVersion4CheckSumBad
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
	},
	
	/// Hardware offloading determined the Internet Protoocl (IP) version 4 layer 4 packet's check sum was bad.
	HardwareOffloadingInternetProtocolVersion4Layer4CheckSumBad
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
	},
	
	/// Hardware offloading determined the Internet Protoocl (IP) version 6 layer 4 packet's check sum was bad.
	HardwareOffloadingInternetProtocolVersion6Layer4CheckSumBad
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
	},
	
	/// The packet had a source address which was not a valid unicast address.
	SourceEthernetAddressIsNotValidUnicast
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
	},
	
	/// The packet's source address was our unicast ethernet address; it is a spoof.
	SourceEthernetAddressIsOurUnicastEthernetAddress
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
	},
	
	/// A packet had a source address which was administratively denied.
	DeniedSourceEthernetAddress
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
	},
	
	/// A packet should never have a destination ethernet address of zero.
	DestinationEthernetAddressIsZero
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
	},
	
	/// This can occur if a link has multiple ethernet addresses or is listening promiscuously.
	DestinationEthernetAddressIsNotOneOfOurs
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
	},
	
	/// Currently recognised and supported EtherTypes are Internet Protocol (IP) Version 4, Internet Protocol (IP) Version 6, Address Resolution Protocol, Virtual LAN tagging (801.1q) and QinQ Virtual LAN tagging (802.1ad).
	///
	/// Ether frame sizes are entirely unsupported.
	UnsupportedEtherType
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
		
		/// EtherType or LegacyEthernetFrameSize.
		unsuspported_ether_type_or_legacy_ethernet_frame_size: EtherTypeOrLegacyEthernetFrameSize,
	},
	
	/// Wrapper around a problematic internet protocol version 4 packet.
	ProblematicInternetProtocolVersion4Packet
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
		
		/// Reason
		reason: IPV4INPDR,
	},
	
	/// Wrapper around a problematic internet protocol version 6 packet.
	ProblematicInternetProtocolVersion6Packet
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
		
		/// Reason
		reason: IPV6INPDR,
	},
	
	/// Wrapper around a problematic internet protocol version 6 packet.
	ProblematicAddressResolutionProtocolPacket
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,
		
		/// Reason
		reason: ARPINPDR,
	},
	
	/// The packet's length was too short to be an IEEE 802.1Q (Virtual LAN) packet.
	///
	/// This should not occur when hardware strips IEEE 802.1Q (Virtual LAN) fields.
	IsTooShortToBeA8021QVirtualLanEthernetPacket
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
	},
	
	/// Occurs during packet processing of an IEEE 802.1Q (Virtual LAN) packet.
	CouldNotParse8011QVirtualLanTag
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,

		/// Tag Control Information (TCI).
		tag_control_information: TagControlInformation,
	},
	
	/// Occurs during packet processing of an IEEE 802.1Q (Virtual LAN) packet.
	NoConfigurationFor8011QVirtualLan
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,

		/// Tag Control Information (TCI).
		tag_control_information: TagControlInformation,
	},
	
	/// Occurs during packet processing of an IEEE 802.1Q (Virtual LAN) packet.
	DropEligibleFor8011QVirtualLan
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,

		/// Tag Control Information (TCI).
		tag_control_information: TagControlInformation,
	},
	
	/// Occurs during packet processing of an IEEE 802.1Q (Virtual LAN) packet.
	DropThisClassOfServiceFor8011QVirtualLan
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,

		/// Tag Control Information (TCI).
		tag_control_information: TagControlInformation,
	},
	
	/// The packet's length was too short to be an IEEE 802.1ad 'QinQ' (Virtual LAN) packet.
	///
	/// This should not occur when hardware strips IEEE 802.1ad 'QinQ' (Virtual LAN) fields.
	IsTooShortToBeAQinQVirtualLanEthernetPacket
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
	},
	
	/// Occurs during packet processing of an IEEE 802.1ad 'QinQ' (Virtual LAN) packet.
	CouldNotParseOuterVirtualLanTag
	{
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
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
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
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
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
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
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
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
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
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
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
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
		/// Approximate time this packet arrived at.
		now: MonotonicMillisecondTimestamp,
		
		/// Dropped packet's ethernet addresses.
		ethernet_addresses: &'ethernet_addresses EthernetAddresses,

		/// Outer Tag Control Information (TCI).
		outer_tag_control_information: TagControlInformation,

		/// Inner Tag Control Information (TCI).
		inner_tag_control_information: TagControlInformation,
	},
}

impl<'ethernet_addresses, ARPINPDR: IncomingNetworkPacketProcessingDropReason, IPV4INPDR: IncomingNetworkPacketProcessingDropReason, IPV6INPDR: IncomingNetworkPacketProcessingDropReason> IncomingNetworkPacketProcessingDropReason for EthernetIncomingNetworkPacketDropReason<'ethernet_addresses, ARPINPDR, IPV4INPDR, IPV6INPDR>
{
}
