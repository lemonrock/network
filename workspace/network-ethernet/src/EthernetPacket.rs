// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
#[derive(Debug)]
pub struct EthernetPacket
{
	/// Header.
	pub header: EthernetPacketHeader,

	/// Payload
	pub payload: EthernetPacketPayload,
}

impl Display for EthernetPacket
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

macro_rules! parse_802_1q_virtual_lan_tag_control_information_drop
{
	($reason: tt, $self: ident, $tag_control_information: ident, $packet_processing: ident, $packet: ident) =>
	{
		{
			let reason = $reason
			{
				ethernet_addresses: $self.ethernet_addresses(),
				tag_control_information: $tag_control_information,
			};
		
			drop!(reason, $packet_processing, $packet)
		}
	}
}

macro_rules! parse_802_1q_virtual_lan_tag_control_information
{
	($self: ident, $tag_control_information: ident, $packet: ident, $packet_processing_by_virtual_lan: ident) =>
	{
		match $tag_control_information.parse()
		{
			Err(_) => parse_802_1q_virtual_lan_tag_control_information_drop!(CouldNotParse8011QVirtualLanTag, $self, $tag_control_information, $packet_processing_by_virtual_lan, $packet),

			Ok((class_of_service, drop_eligible_indicator, inner_virtual_lan_identifier)) =>
			{
				match $packet_processing_by_virtual_lan.get_packet_processing_for_inner_virtual_lan(inner_virtual_lan_identifier)
				{
					None => parse_802_1q_virtual_lan_tag_control_information_drop!(NoConfigurationFor8011QVirtualLan, $self, $tag_control_information, $packet_processing_by_virtual_lan, $packet),

					Some(packet_processing) =>
					{
						if unlikely!(packet_processing.honour_drop_eligible_indicator(drop_eligible_indicator))
						{
							parse_802_1q_virtual_lan_tag_control_information_drop!(DropEligibleFor8011QVirtualLan, $self, $tag_control_information, $packet_processing_by_virtual_lan, $packet)
						}

						if unlikely!(packet_processing.drop_packets_of_class_of_service(class_of_service))
						{
							parse_802_1q_virtual_lan_tag_control_information_drop!(DropThisClassOfServiceFor8011QVirtualLan, $self, $tag_control_information, packet_processing, $packet)
						}
						
						packet_processing
					}
				}
			}
		}
	}
}

macro_rules! parse_802_1ad_virtual_lan_tag_control_information_drop
{
	($reason: tt, $self: ident, $outer_tag_control_information: ident, $inner_tag_control_information: ident, $packet_processing: ident, $packet: ident) =>
	{
		{
			let reason = $reason
			{
				ethernet_addresses: $self.ethernet_addresses(),
				outer_tag_control_information: $outer_tag_control_information,
				inner_tag_control_information: $inner_tag_control_information,
			};
		
			drop!(reason, $packet_processing, $packet)
		}
	}
}

macro_rules! parse_802_1ad_virtual_lan_tag_control_information
{
	($self: ident, $outer_tag_control_information: ident, $inner_tag_control_information: ident, $packet: ident, $packet_processing_by_virtual_lan: ident) =>
	{
		{
			let (outer_virtual_lan_identifier, outer_drop_eligible_indicator, outer_class_of_service) = match $outer_tag_control_information.parse()
			{
				Err(_) => parse_802_1ad_virtual_lan_tag_control_information_drop!(CouldNotParseOuterVirtualLanTag, $self, $outer_tag_control_information, $inner_tag_control_information, $packet_processing_by_virtual_lan, $packet),

				Ok((class_of_service, drop_eligible_indicator, outer_virtual_lan_identifier)) =>
				{
					(outer_virtual_lan_identifier, drop_eligible_indicator, class_of_service)
				}
			};

			let (inner_virtual_lan_identifier, inner_drop_eligible_indicator, inner_class_of_service) = match $inner_tag_control_information.parse()
			{
				Err(_) => parse_802_1ad_virtual_lan_tag_control_information_drop!(CouldNotParseInnerVirtualLanTag, $self, $outer_tag_control_information, $inner_tag_control_information, $packet_processing_by_virtual_lan, $packet),

				Ok((class_of_service, drop_eligible_indicator, outer_virtual_lan_identifier)) =>
				{
					(outer_virtual_lan_identifier, drop_eligible_indicator, class_of_service)
				}
			};

			match $packet_processing_by_virtual_lan.get_packet_processing_for_outer_virtual_lan(outer_virtual_lan_identifier, inner_virtual_lan_identifier)
			{
				None => parse_802_1ad_virtual_lan_tag_control_information_drop!(NoConfigurationForQinQVirtualLan, $self, $outer_tag_control_information, $inner_tag_control_information, $packet_processing_by_virtual_lan, $packet),
				
				Some(packet_processing_for_q_in_q_virtual_lan) =>
				{
					if unlikely!(packet_processing_for_q_in_q_virtual_lan.honour_outer_drop_eligible_indicator(outer_drop_eligible_indicator))
					{
						parse_802_1ad_virtual_lan_tag_control_information_drop!(DropEligibleForOuterVirtualLan, $self, $outer_tag_control_information, $inner_tag_control_information, $packet_processing_by_virtual_lan, $packet)
					}
					
					if unlikely!(packet_processing_for_q_in_q_virtual_lan.drop_packets_of_outer_class_of_service(outer_class_of_service))
					{
						parse_802_1ad_virtual_lan_tag_control_information_drop!(DropThisClassOfServiceForOuterVirtualLan, $self, $outer_tag_control_information, $inner_tag_control_information, $packet_processing_by_virtual_lan, $packet)
					}
					
					if unlikely!(packet_processing_for_q_in_q_virtual_lan.honour_inner_drop_eligible_indicator(inner_drop_eligible_indicator))
					{
						parse_802_1ad_virtual_lan_tag_control_information_drop!(DropEligibleForInnerVirtualLan, $self, $outer_tag_control_information, $inner_tag_control_information, $packet_processing_by_virtual_lan, $packet)
					}
					
					if unlikely!(packet_processing_for_q_in_q_virtual_lan.drop_packets_of_inner_class_of_service(inner_class_of_service))
					{
						parse_802_1ad_virtual_lan_tag_control_information_drop!(DropThisClassOfServiceForInnerVirtualLan, $self, $outer_tag_control_information, $outer_tag_control_information, $packet_processing_by_virtual_lan, $packet)
					}
					
					&packet_processing_for_q_in_q_virtual_lan.inner_packet_processing
				}
			}
		}
	}
}

macro_rules! process_802_1ad_virtual_lan_tagging
{
	($self: ident, $packet: ident, $packet_processing_by_virtual_lan: ident) =>
	{
		{
			if unlikely!($packet.is_too_short_to_be_a_qinq_vlan_ethernet_packet())
			{
				drop!(IsTooShortToBeAQinQVirtualLanEthernetPacket, $packet_processing_by_virtual_lan, $packet)
			}

			let qinq_virtual_lan_packet = $self.qinq_virtual_lan_packet();

			let outer_tag_control_information = qinq_virtual_lan_packet.tag_control_information();

			let inner_virtual_lan_packet = qinq_virtual_lan_packet.virtual_lan_packet();

			let inner_tag_control_information = inner_virtual_lan_packet.tag_control_information();

			let packet_processing = parse_802_1ad_virtual_lan_tag_control_information!($self, outer_tag_control_information, inner_tag_control_information, $packet, $packet_processing_by_virtual_lan);

			let layer_3_length = $packet.packet_length_if_contiguous_less_ethernet_packet_header() - (VirtualLanPacketHeader::IEEE_802_1ad_SizeU16 + VirtualLanPacketHeader::IEEE_802_1Q_SizeU16);
			
			let layer_3_packet = inner_virtual_lan_packet.layer_3_packet();
			
			Self::process_layer_3(layer_3_packet, $packet, packet_processing, layer_3_length, inner_virtual_lan_packet.potentially_invalid_ether_type())
		}
	}
}

macro_rules! guard_is_valid_ethernet_packet
{
	($self: ident, $packet_processing_by_virtual_lan: ident, $packet: ident) =>
	{
		{
			if unlikely!($packet.is_too_short_to_be_an_ethernet_packet())
			{
				drop!(IsTooShortToBeAnEthernetPacket, $packet_processing_by_virtual_lan, $packet)
			}
			
			if unlikely!($packet.hardware_offload_tunnel_packet_type() != HardwareOffloadTunnelPacketType::Uncategorised)
			{
				drop!(HardwareOffloadingCategorisationIsTunnelPacket { ethernet_addresses: $self.ethernet_addresses() }, $packet_processing_by_virtual_lan, $packet)
			}
			
			if unlikely!($packet.hardware_offload_categorisation_indicates_an_unwanted_packet())
			{
				drop!(HardwareOffloadingCategorisationIsUnwanted { ethernet_addresses: $self.ethernet_addresses() }, $packet_processing_by_virtual_lan, $packet)
			}
		}
	}
}

macro_rules! guard_ethernet_addresses_drop
{
	($reason: tt, $ethernet_addresses: ident, $packet_processing: ident, $packet: ident) =>
	{
		drop!($reason { ethernet_addresses: $ethernet_addresses }, $packet_processing, $packet)
	}
}

macro_rules! guard_ethernet_addresses
{
	($ethernet_addresses: ident, $packet: ident, $packet_processing: ident) =>
	{
		{
			let (source_ethernet_address, destination_ethernet_address) = $ethernet_addresses.addresses();

			if unlikely!(source_ethernet_address.is_not_valid_unicast())
			{
				guard_ethernet_addresses_drop!(SourceEthernetAddressIsNotValidUnicast, $ethernet_addresses, $packet_processing, $packet)
			}

			let we_do_not_support_sending_to_ourselves = $packet_processing.is_ethernet_address_our_valid_unicast_ethernet_address(source_ethernet_address);
			if unlikely!(we_do_not_support_sending_to_ourselves)
			{
				guard_ethernet_addresses_drop!(SourceEthernetAddressIsOurUnicastEthernetAddress, $ethernet_addresses, $packet_processing, $packet)
			}

			if unlikely!($packet_processing.is_denied_source_ethernet_address(source_ethernet_address))
			{
				guard_ethernet_addresses_drop!(DeniedSourceEthernetAddress, $ethernet_addresses, $packet_processing, $packet)
			}
			
			if unlikely!(destination_ethernet_address.is_zero())
			{
				guard_ethernet_addresses_drop!(DestinationEthernetAddressIsZero, $ethernet_addresses, $packet_processing, $packet)
			}

			if destination_ethernet_address.is_valid_unicast()
			{
				let is_for_multiply_assigned_ethernet_addreses_on_one_link_or_promiscuous_mode_or_defective = $packet_processing.is_ethernet_address_not_our_valid_unicast_ethernet_address(destination_ethernet_address);
				if unlikely!(is_for_multiply_assigned_ethernet_addreses_on_one_link_or_promiscuous_mode_or_defective)
				{
					guard_ethernet_addresses_drop!(DestinationEthernetAddressIsNotOneOfOurs, $ethernet_addresses, $packet_processing, $packet)
				}
			}
		}
	}
}

macro_rules! guard_ethernet_addresses_and_compute_packet_length
{
	($self: ident, $ethernet_addresses: ident, $packet: ident, $packet_processing_by_virtual_lan: ident) =>
	{
		{
			let packet_processing = &$packet_processing_by_virtual_lan.none;

			guard_ethernet_addresses!($ethernet_addresses, $packet, packet_processing);

			let layer_3_length = $packet.packet_length_if_contiguous_less_ethernet_packet_header();
			(packet_processing, layer_3_length, $self.layer_3_packet())
		}
	}
}

macro_rules! validate_internet_protocol_version_4_and_layer_4_check_sums
{
	($ethernet_addresses: ident, $packet: ident, $packet_processing: ident) =>
	{
		{
			use self::HardwareOffloadLayer4PacketType::*;
			use self::HardwareOffloadCheckSumStatus::*;
			
			let internet_protocol_version_4_check_sum_validated_in_hardware = match $packet.hardware_offload_internet_protocol_version_4_check_sum_status()
			{
				NoInformationKnown => false,
				
				Bad => drop!(HardwareOffloadingInternetProtocolVersion4CheckSumBad { ethernet_addresses: $ethernet_addresses }, $packet_processing, $packet),
				
				Good | IncorrectButInternetProtocolHeaderIntegrityVerified => true,
			};
			
			let hardware_offload_layer_4_packet_type = $packet.hardware_offload_layer_4_packet_type();
			let layer_4_check_sum_validated_in_hardware = match hardware_offload_layer_4_packet_type
			{
				UncategorisedOrAbsent => false,
				
				TransmissionControlProtocol | UserDatagramProtocol | Fragmented | InternetControlMessageProtocol =>
				{
					match $packet.hardware_offload_layer_4_check_sum_status()
					{
						NoInformationKnown => false,
						
						Bad => drop!(HardwareOffloadingInternetProtocolVersion4Layer4CheckSumBad { ethernet_addresses: $ethernet_addresses }, $packet_processing, $packet),
						
						Good | IncorrectButInternetProtocolHeaderIntegrityVerified => true,
					}
				},
				
				StreamControlTransmissionProtocol | OtherNotAFragment | Other => drop!(HardwareOffloadingCategorisationUnwantedLayer4ProtocolInInternetProtocolVersion4Packet { ethernet_addresses: $ethernet_addresses, hardware_offload_layer_4_packet_type }, $packet_processing, $packet),
			};
			
			(internet_protocol_version_4_check_sum_validated_in_hardware, layer_4_check_sum_validated_in_hardware)
		}
	}
}

macro_rules! internet_protocol_version_6_validate_layer_4_check_sum
{
	($ethernet_addresses: ident, $packet: ident, $packet_processing: ident) =>
	{
		{
			use self::HardwareOffloadLayer4PacketType::*;
			use self::HardwareOffloadCheckSumStatus::*;
			
			let hardware_offload_layer_4_packet_type = $packet.hardware_offload_layer_4_packet_type();
			match hardware_offload_layer_4_packet_type
			{
				UncategorisedOrAbsent => false,
				
				TransmissionControlProtocol | UserDatagramProtocol | Fragmented =>
				{
					match $packet.hardware_offload_layer_4_check_sum_status()
					{
						NoInformationKnown => false,
						
						Bad => drop!(HardwareOffloadingInternetProtocolVersion6Layer4CheckSumBad { ethernet_addresses: $ethernet_addresses }, $packet_processing, $packet),
						
						Good | IncorrectButInternetProtocolHeaderIntegrityVerified => true,
					}
				},
				
				StreamControlTransmissionProtocol | InternetControlMessageProtocol | OtherNotAFragment | Other => drop!(HardwareOffloadingCategorisationUnwantedLayer4ProtocolInInternetProtocolVersion6Packet { ethernet_addresses: $ethernet_addresses, hardware_offload_layer_4_packet_type }, $packet_processing, $packet),
			}
		}
	}
}

impl EthernetPacket
{
	/// Ethernet addresses.
	#[inline(always)]
	pub fn ethernet_addresses(&self) -> &EthernetAddresses
	{
		self.header.ethernet_addresses()
	}
	
	/// Ether type, potentially invalid as it could be a legacy ethernet frame size.
	#[inline(always)]
	pub fn potentially_invalid_ether_type(&self) -> EtherType
	{
		self.header.potentially_invalid_ether_type()
	}
	
	/// Payload as a layer 3 packet.
	#[inline(always)]
	pub fn layer_3_packet(&self) -> &Layer3Packet
	{
		self.payload.layer_3_packet()
	}
	
	/// Payload as a layer 2 IEEE 801.1Q Virtual LAN packet.
	#[inline(always)]
	pub fn virtual_lan_packet(&self) -> &VirtualLanPacket
	{
		self.payload.virtual_lan_packet()
	}
	
	/// Payload as a layer 2 IEEE 801.1ad Virtual LAN packet.
	#[inline(always)]
	pub fn qinq_virtual_lan_packet(&self) -> &QinQVirtualLanPacket
	{
		self.payload.qinq_virtual_lan_packet()
	}
	
	/// Process assuming the poll mode driver has hardware offloading for IEEE 802.1Q and IEEE 802.1ad QinQ Virtual LANs.
	#[inline(always)]
	pub fn process_poll_mode_driver_offloads_qinq_vlan_tagging_stripping<'ethernet_addresses, EINPDO: EthernetIncomingNetworkPacketDropObserver<ARPINPDR=ARP::DropReason, IPV4INPDR=IPV4::DropReason, IPV6INPDR=IPV6::DropReason>, ARP: Layer3PacketProcessing<CheckSumsValidated=()>, IPV4: Layer3PacketProcessing<CheckSumsValidated=(bool, bool)>, IPV6: Layer3PacketProcessing<CheckSumsValidated=bool>>(&'ethernet_addresses self, packet: impl EthernetIncomingNetworkPacket, packet_processing_by_virtual_lan: &VirtualLanPacketProcessing<EINPDO, ARP, IPV4, IPV6>)
	{
		guard_is_valid_ethernet_packet!(self, packet_processing_by_virtual_lan, packet);

		let packet_processing = if unlikely!(packet.was_vlan_tag_control_information_stripped())
		{
			let tag_control_information = packet.stripped_vlan_tag_control_information();
			parse_802_1q_virtual_lan_tag_control_information!(self, tag_control_information, packet, packet_processing_by_virtual_lan)
		}
		else if unlikely!(packet.was_vlan_qinq_tag_control_information_stripped())
		{
			let (outer_tag_control_information, inner_tag_control_information) = packet.stripped_vlan_qinq_tag_control_information();
			parse_802_1ad_virtual_lan_tag_control_information!(self, outer_tag_control_information, inner_tag_control_information, packet, packet_processing_by_virtual_lan)
		}
		else
		{
			&packet_processing_by_virtual_lan.none
		};
	
		let layer_3_length = packet.packet_length_if_contiguous_less_ethernet_packet_header();
		self.process(packet, packet_processing, layer_3_length, self.potentially_invalid_ether_type())
	}
	
	/// Process assuming the poll mode driver has hardware offloading only for IEEE 802.1Q Virtual LANs but not IEEE 801.1ad QinQ Virtual LANs.
	#[inline(always)]
	pub fn process_poll_mode_driver_offloads_only_vlan_tagging_stripping<'ethernet_addresses, EINPDO: EthernetIncomingNetworkPacketDropObserver<ARPINPDR=ARP::DropReason, IPV4INPDR=IPV4::DropReason, IPV6INPDR=IPV6::DropReason>, ARP: Layer3PacketProcessing<CheckSumsValidated=()>, IPV4: Layer3PacketProcessing<CheckSumsValidated=(bool, bool)>, IPV6: Layer3PacketProcessing<CheckSumsValidated=bool>>(&'ethernet_addresses self, packet: impl EthernetIncomingNetworkPacket, packet_processing_by_virtual_lan: &VirtualLanPacketProcessing<EINPDO, ARP, IPV4, IPV6>)
	{
		guard_is_valid_ethernet_packet!(self, packet_processing_by_virtual_lan, packet);

		if unlikely!(packet.was_vlan_tag_control_information_stripped())
		{
			let tag_control_information = packet.stripped_vlan_tag_control_information();
			let packet_processing = parse_802_1q_virtual_lan_tag_control_information!(self, tag_control_information, packet, packet_processing_by_virtual_lan);

			let layer_3_length = packet.packet_length_if_contiguous_less_ethernet_packet_header();
			self.process(packet, packet_processing, layer_3_length, self.potentially_invalid_ether_type())
		}
		else
		{
			match self.potentially_invalid_ether_type()
			{
				EtherType::InternetProtocolVersion4 =>
				{
					self.process_internet_protocol_version_4(packet, packet_processing_by_virtual_lan)
				}

				EtherType::InternetProtocolVersion6 =>
				{
					self.process_internet_protocol_version_6(packet, packet_processing_by_virtual_lan)
				}

				EtherType::AddressResolutionProtocol =>
				{
					self.process_address_resolution_protocol(packet, packet_processing_by_virtual_lan)
				}

				EtherType::QinQVlanTagging =>
				{
					process_802_1ad_virtual_lan_tagging!(self, packet, packet_processing_by_virtual_lan)
				}
				
				potentially_invalid_ether_type @ _ => drop!(Self::unsupported_ether_type::<EINPDO, ARP, IPV4, IPV6>(self.ethernet_addresses(), potentially_invalid_ether_type), packet_processing_by_virtual_lan, packet),
			}
		}
	}
	
	/// Process assuming the poll mode driver has no hardware offloading for IEEE 802.1Q and IEEE 802.1ad QinQ Virtual LANs.
	#[inline(always)]
	pub fn poll_mode_driver_does_not_offload_any_vlan_stripping<'ethernet_addresses, EINPDO: EthernetIncomingNetworkPacketDropObserver<ARPINPDR=ARP::DropReason, IPV4INPDR=IPV4::DropReason, IPV6INPDR=IPV6::DropReason>, ARP: Layer3PacketProcessing<CheckSumsValidated=()>, IPV4: Layer3PacketProcessing<CheckSumsValidated=(bool, bool)>, IPV6: Layer3PacketProcessing<CheckSumsValidated=bool>>(&'ethernet_addresses self, packet: impl EthernetIncomingNetworkPacket, packet_processing_by_virtual_lan: &VirtualLanPacketProcessing<EINPDO, ARP, IPV4, IPV6>)
	{
		guard_is_valid_ethernet_packet!(self, packet_processing_by_virtual_lan, packet);

		match self.potentially_invalid_ether_type()
		{
			EtherType::InternetProtocolVersion4 =>
			{
				self.process_internet_protocol_version_4(packet, packet_processing_by_virtual_lan)
			}

			EtherType::InternetProtocolVersion6 =>
			{
				self.process_internet_protocol_version_6(packet, packet_processing_by_virtual_lan)
			}

			EtherType::AddressResolutionProtocol =>
			{
				self.process_address_resolution_protocol(packet, packet_processing_by_virtual_lan)
			}

			EtherType::QinQVlanTagging =>
			{
				process_802_1ad_virtual_lan_tagging!(self, packet, packet_processing_by_virtual_lan)
			}

			EtherType::VlanTagging =>
			{
				if unlikely!(packet.is_too_short_to_be_a_vlan_ethernet_packet())
				{
					drop!(IsTooShortToBeA8021QVirtualLanEthernetPacket, packet_processing_by_virtual_lan, packet)
				}
				
				let virtual_lan_packet = self.virtual_lan_packet();

				let tag_control_information = virtual_lan_packet.tag_control_information();
				
				let packet_processing = parse_802_1q_virtual_lan_tag_control_information!(self, tag_control_information, packet, packet_processing_by_virtual_lan);

				let layer_3_length = packet.packet_length_if_contiguous_less_ethernet_packet_header() - VirtualLanPacketHeader::IEEE_802_1Q_SizeU16;
				
				let potentially_invalid_ether_type = virtual_lan_packet.potentially_invalid_ether_type();
				
				let layer_3_packet = virtual_lan_packet.layer_3_packet();
				
				Self::process_layer_3(layer_3_packet, packet, packet_processing, layer_3_length, potentially_invalid_ether_type)
			}
			
			potentially_invalid_ether_type @ _ => drop!(Self::unsupported_ether_type::<EINPDO, ARP, IPV4, IPV6>(self.ethernet_addresses(), potentially_invalid_ether_type), packet_processing_by_virtual_lan, packet),
		}
	}

	#[inline(always)]
	fn process<'ethernet_addresses, EINPDO: EthernetIncomingNetworkPacketDropObserver<ARPINPDR=ARP::DropReason, IPV4INPDR=IPV4::DropReason, IPV6INPDR=IPV6::DropReason>, ARP: Layer3PacketProcessing<CheckSumsValidated=()>, IPV4: Layer3PacketProcessing<CheckSumsValidated=(bool, bool)>, IPV6: Layer3PacketProcessing<CheckSumsValidated=bool>>(&'ethernet_addresses self, packet: impl EthernetIncomingNetworkPacket, packet_processing: &EthernetPacketProcessing<EINPDO, ARP, IPV4, IPV6>, layer_3_length: u16, potentially_invalid_ether_type: EtherType)
	{
		Self::process_layer_3(self.layer_3_packet(), packet, packet_processing, layer_3_length, potentially_invalid_ether_type)
	}

	#[inline(always)]
	fn process_layer_3<'ethernet_addresses, EINPDO: EthernetIncomingNetworkPacketDropObserver<ARPINPDR=ARP::DropReason, IPV4INPDR=IPV4::DropReason, IPV6INPDR=IPV6::DropReason>, ARP: Layer3PacketProcessing<CheckSumsValidated=()>, IPV4: Layer3PacketProcessing<CheckSumsValidated=(bool, bool)>, IPV6: Layer3PacketProcessing<CheckSumsValidated=bool>>(layer_3_packet: &Layer3Packet, packet: impl EthernetIncomingNetworkPacket, packet_processing: &EthernetPacketProcessing<EINPDO, ARP, IPV4, IPV6>, layer_3_length: u16, potentially_invalid_ether_type: EtherType)
	{
		let ethernet_packet = packet.ethernet_packet();
		let ethernet_addresses = ethernet_packet.ethernet_addresses();
		
		match potentially_invalid_ether_type
		{
			EtherType::InternetProtocolVersion4 =>
			{
				let check_sum_validated_in_hardware = validate_internet_protocol_version_4_and_layer_4_check_sums!(ethernet_addresses, packet, packet_processing);
				guard_ethernet_addresses!(ethernet_addresses, packet, packet_processing);
				packet_processing.process_internet_protocol_version_4(packet, layer_3_packet, layer_3_length, ethernet_addresses, check_sum_validated_in_hardware)
			}

			EtherType::InternetProtocolVersion6 =>
			{
				let layer_4_check_sum_validated_in_hardware = internet_protocol_version_6_validate_layer_4_check_sum!(ethernet_addresses, packet, packet_processing);
				guard_ethernet_addresses!(ethernet_addresses, packet, packet_processing);
				packet_processing.process_internet_protocol_version_6(packet, layer_3_packet, layer_3_length, ethernet_addresses, layer_4_check_sum_validated_in_hardware)
			}

			EtherType::AddressResolutionProtocol =>
			{
				guard_ethernet_addresses!(ethernet_addresses, packet, packet_processing);
				packet_processing.process_address_resolution_protocol(packet, layer_3_packet, layer_3_length, ethernet_addresses)
			}

			_ => drop!(Self::unsupported_ether_type::<EINPDO, ARP, IPV4, IPV6>(ethernet_addresses, potentially_invalid_ether_type), packet_processing, packet),
		}
	}

	#[inline(always)]
	fn process_internet_protocol_version_4<'ethernet_addresses, EINPDO: EthernetIncomingNetworkPacketDropObserver<ARPINPDR=ARP::DropReason, IPV4INPDR=IPV4::DropReason, IPV6INPDR=IPV6::DropReason>, ARP: Layer3PacketProcessing<CheckSumsValidated=()>, IPV4: Layer3PacketProcessing<CheckSumsValidated=(bool, bool)>, IPV6: Layer3PacketProcessing<CheckSumsValidated=bool>>(&'ethernet_addresses self, packet: impl EthernetIncomingNetworkPacket, packet_processing_by_virtual_lan: &VirtualLanPacketProcessing<EINPDO, ARP, IPV4, IPV6>)
	{
		let ethernet_addresses = self.ethernet_addresses();
		let check_sum_validated_in_hardware = validate_internet_protocol_version_4_and_layer_4_check_sums!(ethernet_addresses, packet, packet_processing_by_virtual_lan);
		let (packet_processing, layer_3_length, layer_3_packet) = guard_ethernet_addresses_and_compute_packet_length!(self, ethernet_addresses, packet, packet_processing_by_virtual_lan);
		packet_processing.process_internet_protocol_version_4(packet, layer_3_packet, layer_3_length, ethernet_addresses, check_sum_validated_in_hardware)
	}

	#[inline(always)]
	fn process_internet_protocol_version_6<'ethernet_addresses, EINPDO: EthernetIncomingNetworkPacketDropObserver<ARPINPDR=ARP::DropReason, IPV4INPDR=IPV4::DropReason, IPV6INPDR=IPV6::DropReason>, ARP: Layer3PacketProcessing<CheckSumsValidated=()>, IPV4: Layer3PacketProcessing<CheckSumsValidated=(bool, bool)>, IPV6: Layer3PacketProcessing<CheckSumsValidated=bool>>(&'ethernet_addresses self, packet: impl EthernetIncomingNetworkPacket, packet_processing_by_virtual_lan: &VirtualLanPacketProcessing<EINPDO, ARP, IPV4, IPV6>)
	{
		let ethernet_addresses = self.ethernet_addresses();
		let layer_4_check_sum_validated_in_hardware = internet_protocol_version_6_validate_layer_4_check_sum!(ethernet_addresses, packet, packet_processing_by_virtual_lan);
		let (packet_processing, layer_3_length, layer_3_packet) = guard_ethernet_addresses_and_compute_packet_length!(self, ethernet_addresses, packet, packet_processing_by_virtual_lan);
		packet_processing.process_internet_protocol_version_6(packet, layer_3_packet, layer_3_length, ethernet_addresses, layer_4_check_sum_validated_in_hardware)
	}

	#[inline(always)]
	fn process_address_resolution_protocol<'ethernet_addresses, EINPDO: EthernetIncomingNetworkPacketDropObserver<ARPINPDR=ARP::DropReason, IPV4INPDR=IPV4::DropReason, IPV6INPDR=IPV6::DropReason>, ARP: Layer3PacketProcessing<CheckSumsValidated=()>, IPV4: Layer3PacketProcessing<CheckSumsValidated=(bool, bool)>, IPV6: Layer3PacketProcessing<CheckSumsValidated=bool>>(&'ethernet_addresses self, packet: impl EthernetIncomingNetworkPacket, packet_processing_by_virtual_lan: &VirtualLanPacketProcessing<EINPDO, ARP, IPV4, IPV6>)
	{
		let ethernet_addresses = self.ethernet_addresses();
		let (packet_processing, layer_3_length, layer_3_packet) = guard_ethernet_addresses_and_compute_packet_length!(self, ethernet_addresses, packet, packet_processing_by_virtual_lan);
		packet_processing.process_address_resolution_protocol(packet, layer_3_packet, layer_3_length, ethernet_addresses)
	}
	
	#[inline(always)]
	fn unsupported_ether_type<'ethernet_addresses, EINPDO: EthernetIncomingNetworkPacketDropObserver<ARPINPDR=ARP::DropReason, IPV4INPDR=IPV4::DropReason, IPV6INPDR=IPV6::DropReason>, ARP: Layer3PacketProcessing<CheckSumsValidated=()>, IPV4: Layer3PacketProcessing<CheckSumsValidated=(bool, bool)>, IPV6: Layer3PacketProcessing<CheckSumsValidated=bool>>(ethernet_addresses: &'ethernet_addresses EthernetAddresses, potentially_invalid_ether_type: EtherType) -> EthernetIncomingNetworkPacketDropReason<'ethernet_addresses, EINPDO::ARPINPDR, EINPDO::IPV4INPDR, EINPDO::IPV6INPDR>
	{
		UnsupportedEtherType
		{
			ethernet_addresses,
			unsuspported_ether_type_or_legacy_ethernet_frame_size: EtherTypeOrLegacyEthernetFrameSize
			{
				ether_type: potentially_invalid_ether_type,
			}
		}
	}
}
