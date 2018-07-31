// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// A pseudo-header for Internet Protocol Version 4.
#[repr(C, packed)]
pub struct InternetProtocolVersion4PseudoHeader
{
	source_internet_protocol_version_4_address: InternetProtocolVersion4HostAddress,
	destination_internet_protocol_version_4_address: InternetProtocolVersion4HostAddress,
	reserved: u8,
	layer_4_protocol_number: Layer4ProtocolNumber,
	layer_4_packet_size: NetworkEndianU16,
}

impl InternetProtocolVersion4PseudoHeader
{
	#[inline(always)]
	pub(crate) fn new(source_internet_protocol_version_4_address: &InternetProtocolVersion4HostAddress, destination_internet_protocol_version_4_address: &InternetProtocolVersion4HostAddress, layer_4_protocol_number: Layer4ProtocolNumber, layer_4_packet_size: u16) -> Self
	{
		Self
		{
			source_internet_protocol_version_4_address: *source_internet_protocol_version_4_address,
			destination_internet_protocol_version_4_address: *destination_internet_protocol_version_4_address,
			reserved: 0,
			layer_4_protocol_number,
			layer_4_packet_size: NetworkEndianU16::from_native_endian(layer_4_packet_size),
		}
	}
	
	/// Computes a secure hash.
	#[inline(always)]
	pub fn secure_hash(digester: &mut impl Digest, source_internet_protocol_version_4_address: &InternetProtocolVersion4HostAddress, destination_internet_protocol_version_4_address: &InternetProtocolVersion4HostAddress, layer_4_protocol_number: Layer4ProtocolNumber, layer_4_packet_size: u16)
	{
		digester.input(source_internet_protocol_version_4_address.bytes());
		digester.input(destination_internet_protocol_version_4_address.bytes());
		digester.input(&[0]);
		digester.input(&[layer_4_protocol_number.into()]);
		digester.input(NetworkEndianU16::from_native_endian(layer_4_packet_size).bytes());
	}
	
	/// Internet Protocol (IP) version 4 check sum for Transmission Control Protocol (TCP) segments.
	#[inline(always)]
	pub fn internet_protocol_version_4_tcp_check_sum(source_internet_protocol_version_4_address: &NetworkEndianU32, destination_internet_protocol_version_4_address: &NetworkEndianU32, internet_packet_payload_pointer: NonNull<u8>, layer_4_packet_size: usize) -> Self
	{
		Self::internet_protocol_version_4_layer_4_check_sum(source_internet_protocol_version_4_address, destination_internet_protocol_version_4_address, internet_packet_payload_pointer, layer_4_packet_size, Layer4ProtocolNumber::TransmissionControlProtocol)
	}
	
	/// Internet Protocol (IP) version 4 check sum.
	#[inline(always)]
	pub fn internet_protocol_version_4_layer_4_check_sum(source_internet_protocol_version_4_address: &NetworkEndianU32, destination_internet_protocol_version_4_address: &NetworkEndianU32, internet_packet_payload_pointer: NonNull<u8>, layer_4_packet_size: usize, layer_4_protocol_number: Layer4ProtocolNumber) -> Self
	{
		let sum = Self::internet_protocol_version_4_pseudo_header_check_sum_partial(source_internet_protocol_version_4_address, destination_internet_protocol_version_4_address, layer_4_packet_size, layer_4_protocol_number);
		let sum = Self::from_data_check_sum_partial(internet_packet_payload_pointer, layer_4_packet_size, sum);
		Rfc1141CompliantCheckSum::finalize(sum)
	}
	
	#[inline(always)]
	fn internet_protocol_version_4_pseudo_header_check_sum_partial(source_internet_protocol_version_4_address: &NetworkEndianU32, destination_internet_protocol_version_4_address: &NetworkEndianU32, layer_4_packet_size: usize, layer_4_protocol_number: Layer4ProtocolNumber) -> u32
	{
		let pseudo_header = InternetProtocolVersion4PseudoHeader::new(source_internet_protocol_version_4_address, destination_internet_protocol_version_4_address, layer_4_protocol_number, layer_4_packet_size as u16);
		
		Rfc1141CompliantCheckSum::from_struct_check_sum_partial(&pseudo_header, 0)
	}
}
