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

macro_rules! process_options
{
	($now: ident, $header: ident, $header_length_including_options: ident, $ethernet_addresses: ident, $packet_processing: ident, $packet: ident) =>
	{
		{
			let duplicate_options = InternetProtocolVersion4OptionsBitSet::new();
		
			let header_pointer = unsafe { $header as *const InternetProtocolVersion4PacketHeader as usize };
			let mut options_pointer = header_pointer + InternetProtocolVersion4PacketHeader::HeaderSize;
			let end_of_options_pointer = header_pointer + ($header_length_including_options as usize);
			while options_pointer != end_of_options_pointer
			{
				let option_kind = InternetProtocolVersion4OptionKind(unsafe { *(options_pointer as *const u8) });
				
				let increment = match option_kind
				{
					InternetProtocolVersion4OptionNumber::EndOfOptionsList =>
					{
						if cfg!(feature = "drop-packets-with-ipv4-options-lacking-zero-padding")
						{
							options_pointer += 1;
							while options_pointer != end_of_options_pointer
							{
								if unlikely!(unsafe { *(options_pointer as *const u8) } != 0x00)
								{
									drop!($now, InternetProtocolVersion4IncomingNetworkPacketDropReason::OptionsWereNotZeroPadded { header: $header.non_null() }, $ethernet_addresses, $packet_processing, $packet)
								}
							
								options_pointer += 1
							}
						}
					
						break
					},
					
					InternetProtocolVersion4OptionNumber::NoOperation => 1,
					
					// RFC 7126 obsolete.
					InternetProtocolVersion4OptionNumber::StreamIdentifier
					| InternetProtocolVersion4OptionNumber::ProbeMaximumTransmissionUnit
					| InternetProtocolVersion4OptionNumber::ReplyMaximumTransmissionUnit
					| InternetProtocolVersion4OptionNumber::Traceroute
					| InternetProtocolVersion4OptionNumber::ExperimentalAccessControl
					| InternetProtocolVersion4OptionNumber::ExtendedInternetProtocol
					| InternetProtocolVersion4OptionNumber::AddressExtension
					| InternetProtocolVersion4OptionNumber::SenderDirectedMultiDestinationDelivery
					| InternetProtocolVersion4OptionNumber::DynamicPacketState
					| InternetProtocolVersion4OptionNumber::UpstreamMulticastPacket
					| InternetProtocolVersion4OptionNumber::ENCODE => drop!($now, InternetProtocolVersion4IncomingNetworkPacketDropReason::OptionIsObsoleteAsOfRfc7126 { header: $header.non_null(), option_kind }, $ethernet_addresses, $packet_processing, $packet),
					
					// RFC 7126 threat.
					//
					// Loose Source and Record Route (LSRR) (Type = 131).
					// Strict Source and Record Route (SSRR) (Type = 137).
					// Record Route (Type = 7).
					// Internet Timestamp (Type = 68).
					// Router Alert (Type = 148).
					// Quick-Start (QS) (Type = 25).
					InternetProtocolVersion4OptionNumber::LooseSourceRouteAndRecordRoute
					| InternetProtocolVersion4OptionNumber::StrictSourceRouteAndRecordRoute
					| InternetProtocolVersion4OptionNumber::RecordRoute
					| InternetProtocolVersion4OptionNumber::InternetTimestamp
					| InternetProtocolVersion4OptionNumber::RouterAlert
					| InternetProtocolVersion4OptionNumber::QuickStart => drop!($now, InternetProtocolVersion4IncomingNetworkPacketDropReason::OptionIsThreatAsOfRfc7126 { header: $header.non_null(), option_kind }, $ethernet_addresses, $packet_processing, $packet),
					
					// RFC 3692 style Experiment (EXP) defined in RFC 4727.
					InternetProtocolVersion4OptionNumber::Rfc3692StyleExperiment1
					| InternetProtocolVersion4OptionNumber::Rfc3692StyleExperiment2
					| InternetProtocolVersion4OptionNumber::Rfc3692StyleExperiment3
					| InternetProtocolVersion4OptionNumber::Rfc3692StyleExperiment4 => drop!($now, InternetProtocolVersion4IncomingNetworkPacketDropReason::OptionIsExperimental { header: $header.non_null(), option_kind }, $ethernet_addresses, $packet_processing, $packet),
					
					// RFC 7126 security.
					//
					// DoD Basic Security Option (Type = 130).
					// DoD Extended Security Option (Type = 133).
					// Commercial IP Security Option (CIPSO) (Type = 134).
					InternetProtocolVersion4OptionNumber::BasicSecurity
					| InternetProtocolVersion4OptionNumber::ExtendedSecurity
					| InternetProtocolVersion4OptionNumber::CommercialSecurity => drop!($now, InternetProtocolVersion4IncomingNetworkPacketDropReason::OptionIsSecurity { header: $header.non_null(), option_kind }, $ethernet_addresses, $packet_processing, $packet),
					
					// Options not dealt with above that are registered at IANA but rarely encountered.
					//
					// Experimental Measurement (ZSU) (Type = 10).
					// Experimental Flow Control (FINN) (Type = 205).
					// IMI Traffic Descriptor (IMITD) (Type = 144).
					// Type = 150 (unassigned but previously in use until 2005).
					InternetProtocolVersion4OptionNumber::ExperimentalMeasurement
					| InternetProtocolVersion4OptionNumber::ExperimentalFlowControl
					| InternetProtocolVersion4OptionNumber::ImiTrafficDescriptor
					| InternetProtocolVersion4OptionNumber::_150 => drop!($now, InternetProtocolVersion4IncomingNetworkPacketDropReason::OptionIsRarelyEncounteredButRegisteredAtIana { header: $header.non_null(), option_kind }, $ethernet_addresses, $packet_processing, $packet),
					
					// Unknown.
					option_kind @ _ =>
					{
						let option_kind = InternetProtocolVersion4OptionKind(option_kind) ;
						
						let class = option_kind.class();
						if unlikely!(class.is_reserved())
						{
							drop!($now, InternetProtocolVersion4IncomingNetworkPacketDropReason::OptionHasReservedClass { header: $header.non_null(), option_kind }, $ethernet_addresses, $packet_processing, $packet)
						}
						
						if unlikely!($header.is_fragment() && option_kind.should_not_be_copied_onto_fragments())
						{
							drop!($now, InternetProtocolVersion4IncomingNetworkPacketDropReason::OptionShouldNotBePresentOnFragments { header: $header.non_null(), option_kind }, $ethernet_addresses, $packet_processing, $packet)
						}
						
						let number = option_kind.number();
						
						if unlikely!(number.is_assigned_or_previously_assigned())
						{
							drop!($now, InternetProtocolVersion4IncomingNetworkPacketDropReason::OptionIsAssignedOrPreviouslyAssignedWithDifferentCopyOrClassBits { header: $header.non_null(), option_kind }, $ethernet_addresses, $packet_processing, $packet)
						}
						
						if unlikely!(duplicate_options.contains(number.into()))
						{
							drop!($now, InternetProtocolVersion4IncomingNetworkPacketDropReason::OptionIsDuplicate { header: $header.non_null(), option_kind }, $ethernet_addresses, $packet_processing, $packet)
						}
						duplicate_options.insert(number.into());
					
						let length_pointer = options_pointer + 1;
						
						if unlikely!(length_pointer + 1 == end_of_options_pointer)
						{
							drop!($now, InternetProtocolVersion4IncomingNetworkPacketDropReason::OptionLacksLength { header: $header.non_null(), option_kind }, $ethernet_addresses, $packet_processing, $packet)
						}
						
						let length_including_option_kind_and_length_field = unsafe { *(length_pointer as *const u8) };
						
						if unlikely!(length_including_option_kind_and_length_field < 2)
						{
							drop!($now, InternetProtocolVersion4IncomingNetworkPacketDropReason::OptionLengthTooShort { header: $header.non_null(), option_kind }, $ethernet_addresses, $packet_processing, $packet)
						}
						
						let length_including_option_kind_and_length_field = length_including_option_kind_and_length_field as usize;
						
						if unlikely!(options_pointer + length_including_option_kind_and_length_field > end_of_options_pointer)
						{
							drop!($now, InternetProtocolVersion4IncomingNetworkPacketDropReason::OptionLengthTooLong { header: $header.non_null(), option_kind }, $ethernet_addresses, $packet_processing, $packet)
						}
						
						length_including_option_kind_and_length_field
					}
				};
				
				options_pointer += increment;
			}
		}
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
	pub(crate) fn process<'lifetime, EINPDO: EthernetIncomingNetworkPacketDropObserver<IPV4INPDR=InternetProtocolVersion4IncomingNetworkPacketDropReason>>(&'lifetime self, packet: impl EthernetIncomingNetworkPacket, packet_processing: &InternetProtocolVersion4PacketProcessing<EINPDO>, layer_3_length: u16, ethernet_addresses: &'lifetime EthernetAddresses, internet_protocol_version_4_check_sum_validated_in_hardware: bool, layer_4_check_sum_validated_in_hardware: bool, now: MonotonicMillisecondTimestamp)
	{
		macro_rules! more_header_validation
		{
			($now: ident, $header: ident, $ethernet_addresses: ident, $packet_processing: ident, $packet: ident, $total_length: ident, $internet_protocol_version_4_check_sum_validated_in_hardware: ident, $layer_3_length: ident) =>
			{
				{
					let header_length_including_options = $header.header_length_including_options();
					let header_length_including_options_as_u16 = header_length_including_options as u16;
					
					if unlikely!($total_length < header_length_including_options_as_u16)
					{
						drop!($now, InternetProtocolVersion4IncomingNetworkPacketDropReason::TotalLengthLessThanHeader { header: $header.non_null() }, $ethernet_addresses, $packet_processing, $packet)
					}
					
					let header_has_ipv4_options = header_length_including_options != InternetProtocolVersion4PacketHeader::HeaderSizeU8;
					if likely!(header_has_ipv4_options)
					{
						if cfg!(feature = "drop-packets-with-ipv4-options")
						{
							drop!($now, InternetProtocolVersion4IncomingNetworkPacketDropReason::HasOptions { header: $header.non_null() }, $ethernet_addresses, $packet_processing, $packet)
						}
						else
						{
							process_options!($now, $header, header_length_including_options, $ethernet_addresses, $packet_processing, $packet)
						}
					}
					
					if unlikely!(!$internet_protocol_version_4_check_sum_validated_in_hardware)
					{
						if unlikely!($header.check_sum_is_invalid())
						{
							drop!($now, InternetProtocolVersion4IncomingNetworkPacketDropReason::InternetProtocolCheckSumWhenCalculatedInSoftwareWasInvalid { header: $header.non_null() }, $ethernet_addresses, $packet_processing, $packet)
						}
					}
					
					if unlikely!($header.source_address_is_same_as_destination_address())
					{
						drop!($now, InternetProtocolVersion4IncomingNetworkPacketDropReason::SourceAndDestinationAddressAreTheSame { header: $header.non_null() }, $ethernet_addresses, $packet_processing, $packet)
					}
					
					// TODO: IPv4 packet reassembly and RSS logic.
					// TODO: Overly small fragments, eg fragments smaller than MSS / MTU minima (eg 1280 for IPv6).
					let packet = match reassemble_fragmented_internet_protocol_version_4_packet($packet, $now, $header, header_length_including_options_as_u16)
					{
						None => return,
						Some(packet) => packet,
					};
					
					$layer_3_length - header_length_including_options_as_u16
				}
			}
		}
		
		let header = &self.header;
		
		if unlikely!(header.is_version_not_4())
		{
			drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::HeaderIsNot4 { header: header.non_null() }, ethernet_addresses, packet_processing, packet)
		}
		
		let total_length = header.total_length();
		
		if unlikely!(total_length != layer_3_length)
		{
			drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::TotalLengthInvalid { header: header.non_null(), layer_3_length }, ethernet_addresses, packet_processing, packet)
		}
		
		if unlikely!(header.has_invalid_fragmentation_flags_or_identification())
		{
			drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::InvalidFragmentationFlagsOrIdentification { header: header.non_null() }, ethernet_addresses, packet_processing, packet)
		}
		
		let is_fragment = header.is_fragment();
		
		fn reassemble_fragmented_internet_protocol_version_4_packet()
		{
			panic!();
		}
		
		match unsafe { header.next_proto_id.unknown }
		{
			KnownOrUnknownLayer4ProtocolNumber::InternetControlMessageProtocol =>
			{
				if is_fragment
				{
					drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::InternetControlMessageProtocolPacketsShouldNotBeFragmented { header: header.non_null() }, ethernet_addresses, packet_processing, packet)
				}
				
				let source_address = header.source_address;
				let destination_address = header.destination_address;
				let (_, destination_ethernet_address) = ethernet_addresses.addresses();
				
				if unlikely!(source_address.is_not_valid_unicast())
				{
					drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::SourceAddressNotValidUnicast { header: header.non_null() }, ethernet_addresses, packet_processing, packet)
				}
				
				let layer_4_length = more_header_validation!(now, header, ethernet_addresses, packet_processing, packet, total_length, internet_protocol_version_4_check_sum_validated_in_hardware, layer_3_length);
				
				if unlikely!(packet_processing.is_source_internet_protocol_version_4_address_denied(&source_address))
				{
					drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::SourceAddressDenied { header: header.non_null() }, ethernet_addresses, packet_processing, packet)
				}
				
				if unlikely!(packet_processing.is_internet_protocol_version_4_host_address_not_one_of_ours(destination_address))
				{
					drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::UnicastDestinationIsNotUs { header: header.non_null() }, ethernet_addresses, packet_processing, packet)
				}
				
				if unlikely!(!destination_ethernet_address.is_valid_unicast())
				{
					drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::EthernetAddressWasNotUnicast { header: header.non_null() }, ethernet_addresses, packet_processing, packet)
				}
				
				packet_processing.process_internet_control_message_protocol_version_4(now, packet, &self.payload, layer_4_length, ethernet_addresses, layer_4_check_sum_validated_in_hardware)
			}
			
			KnownOrUnknownLayer4ProtocolNumber::TransmissionControlProtocol =>
			{
				let source_address = header.source_address;
				let destination_address = header.destination_address;
				let (_, destination_ethernet_address) = ethernet_addresses.addresses();
				
				if unlikely!(source_address.is_not_valid_unicast())
				{
					drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::SourceAddressNotValidUnicast { header: header.non_null() }, ethernet_addresses, packet_processing, packet)
				}
				
				let layer_4_length = more_header_validation!(now, header, ethernet_addresses, packet_processing, packet, total_length, internet_protocol_version_4_check_sum_validated_in_hardware, layer_3_length);
				
				if unlikely!(packet_processing.is_source_internet_protocol_version_4_address_denied(&source_address))
				{
					drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::SourceAddressDenied { header: header.non_null() }, ethernet_addresses, packet_processing, packet)
				}
				
				if unlikely!(!destination_ethernet_address.is_valid_unicast())
				{
					drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::EthernetAddressWasNotUnicast { header: header.non_null() }, ethernet_addresses, packet_processing, packet)
				}
				
				if unlikely!(packet_processing.is_internet_protocol_version_4_host_address_not_one_of_ours(destination_address))
				{
					drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::UnicastDestinationIsNotUs { header: header.non_null() }, ethernet_addresses, packet_processing, packet)
				}
				
				packet_processing.process_transmission_control_protocol(now, packet, &self.payload, layer_4_length, ethernet_addresses, layer_4_check_sum_validated_in_hardware)
			},
			
			KnownOrUnknownLayer4ProtocolNumber::UserDatagramProtocol =>
			{
				let source_address = header.source_address;
				let destination_address = header.destination_address;
				let (_, destination_ethernet_address) = ethernet_addresses.addresses();
				
				if unlikely!(source_address.is_not_valid_unicast() || !source_address.is_unspecified())
				{
					drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::SourceAddressNotValidUnicastOrUnspecified { header: header.non_null() }, ethernet_addresses, packet_processing, packet)
				}
				
				let layer_4_length = more_header_validation!(now, header, ethernet_addresses, packet_processing, packet, total_length, internet_protocol_version_4_check_sum_validated_in_hardware, layer_3_length);
				
				if unlikely!(packet_processing.is_source_internet_protocol_version_4_address_denied(&source_address))
				{
					drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::SourceAddressDenied { header: header.non_null() }, ethernet_addresses, packet_processing, packet)
				}
				
				let (_source_ethernet_address, destination_ethernet_address) = ethernet_addresses.addresses();
				
				let destination_address = header.destination_address;
				
				if destination_ethernet_address.is_unicast()
				{
					if unlikely!(!destination_ethernet_address.is_valid_unicast())
					{
						drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::EthernetAddressWasNotUnicast { header: header.non_null() }, ethernet_addresses, packet_processing, packet)
					}
					
					if unlikely!(packet_processing.is_internet_protocol_version_4_host_address_not_one_of_ours(destination_address))
					{
						drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::UnicastDestinationIsNotUs { header: header.non_null() }, ethernet_addresses, packet_processing, packet)
					}
				}
				else if destination_ethernet_address.is_broadcast()
				{
					if unlikely!(destination_address.is_not_broadcast())
					{
						drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::EthernetBroadcastNotInternetBroadcast { header: header.non_null() }, ethernet_addresses, packet_processing, packet)
					}
				}
				else if let Some(lower_23_bits) = destination_ethernet_address.internet_protocol_version_4_multicast_23_bits()
				{
					if unlikely!(destination_address.is_not_multicast())
					{
						drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::MulticastAddressIsNotMulticast { header: header.non_null() }, ethernet_addresses, packet_processing, packet)
					}
					
					if unlikely!(destination_address.does_not_have_lower_23_bits(lower_23_bits))
					{
						drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::MulticastAddressMismatchesEthernetAddress { header: header.non_null() }, ethernet_addresses, packet_processing, packet)
					}
					
					if packet_processing.is_internet_protocol_version_4_multicast_address_not_one_of_ours(destination_address)
					{
						drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::MulticastAddressDenied { header: header.non_null() }, ethernet_addresses, packet_processing, packet)
					}
				}
				else
				{
					drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::DestinationWasLoopbackOrDocumentationAddress { header: header.non_null() }, ethernet_addresses, packet_processing, packet)
				}
				
				packet_processing.process_user_datagram_protocol(now, packet, &self.payload, layer_4_length, ethernet_addresses, layer_4_check_sum_validated_in_hardware)
			},
			
			unsupported_layer_4_protocol @ _ => drop!(now, InternetProtocolVersion4IncomingNetworkPacketDropReason::UnsupportedLayer4Protocol { header: header.non_null(), unsupported_layer_4_protocol }, ethernet_addresses, packet_processing, packet)
		}
	}
}
