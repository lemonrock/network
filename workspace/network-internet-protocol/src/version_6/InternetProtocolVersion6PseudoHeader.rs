// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// A pseudo-header for Internet Protocol Version 6.
#[repr(C, packed)]
#[derive(Deserialize, Serialize)]
pub struct InternetProtocolVersion6PseudoHeader
{
	source_internet_protocol_version_6_address: InternetProtocolVersion6HostAddress,
	destination_internet_protocol_version_6_address: InternetProtocolVersion6HostAddress,
	layer_4_packet_size: NetworkEndianU32,
	reserved: [u8; 3],
	layer_4_protocol_number: Layer4ProtocolNumber,
}

impl InternetProtocolVersion6PseudoHeader
{
	#[inline(always)]
	pub(crate) fn new(source_internet_protocol_version_6_address: &InternetProtocolVersion6HostAddress, destination_internet_protocol_version_6_address: &InternetProtocolVersion6HostAddress, layer_4_protocol_number: Layer4ProtocolNumber, layer_4_packet_size: u32) -> Self
	{
		Self
		{
			source_internet_protocol_version_6_address: *source_internet_protocol_version_6_address,
			destination_internet_protocol_version_6_address: *destination_internet_protocol_version_6_address,
			layer_4_packet_size: NetworkEndianU32::from_native_endian(layer_4_packet_size),
			reserved: unsafe { zeroed() },
			layer_4_protocol_number,
		}
	}
	
	/// Computes a secure hash.
	#[inline(always)]
	pub fn secure_hash(digester: &mut impl Digest, source_internet_protocol_version_6_address: &InternetProtocolVersion6HostAddress, destination_internet_protocol_version_6_address: &InternetProtocolVersion6HostAddress, layer_4_protocol_number: Layer4ProtocolNumber, layer_4_packet_size: u32)
	{
		digester.input(source_internet_protocol_version_6_address.bytes());
		digester.input(destination_internet_protocol_version_6_address.bytes());
		digester.input(NetworkEndianU32::from_native_endian(layer_4_packet_size).bytes());
		digester.input(&[0, 0, 0]);
		digester.input(&[layer_4_protocol_number.into()]);
	}
	
	/// Internet Protocol (IP) version 6 check sum for Transmission Control Protocol (TCP) segments.
	#[inline(always)]
	pub fn internet_protocol_version_6_tcp_check_sum(source_internet_protocol_version_6_address: &InternetProtocolVersion6HostAddress, destination_internet_protocol_version_6_address: &InternetProtocolVersion6HostAddress, internet_packet_payload_pointer: NonNull<u8>, layer_4_packet_size: usize) -> Rfc1141CompliantCheckSum
	{
		Self::internet_protocol_version_6_layer_4_check_sum(source_internet_protocol_version_6_address, destination_internet_protocol_version_6_address, internet_packet_payload_pointer, layer_4_packet_size, Layer4ProtocolNumber::TransmissionControlProtocol)
	}
	
	/// Internet Protocol (IP) version 6 check sum.
	#[inline(always)]
	pub fn internet_protocol_version_6_layer_4_check_sum(source_internet_protocol_version_6_address: &InternetProtocolVersion6HostAddress, destination_internet_protocol_version_6_address: &InternetProtocolVersion6HostAddress, internet_packet_payload_pointer: NonNull<u8>, layer_4_packet_size: usize, layer_4_protocol_number: Layer4ProtocolNumber) -> Rfc1141CompliantCheckSum
	{
		let sum = Self::internet_protocol_version_6_pseudo_header_check_sum_partial(source_internet_protocol_version_6_address, destination_internet_protocol_version_6_address, layer_4_packet_size, layer_4_protocol_number);
		let sum = Rfc1141CompliantCheckSum::from_data_check_sum_partial(internet_packet_payload_pointer, layer_4_packet_size, sum);
		Rfc1141CompliantCheckSum::finalize(sum)
	}
	
	#[inline(always)]
	fn internet_protocol_version_6_pseudo_header_check_sum_partial(source_internet_protocol_version_6_address: &InternetProtocolVersion6HostAddress, destination_internet_protocol_version_6_address: &InternetProtocolVersion6HostAddress, layer_4_packet_size: usize, layer_4_protocol_number: Layer4ProtocolNumber) -> u32
	{
		let pseudo_header = Self::new(source_internet_protocol_version_6_address, destination_internet_protocol_version_6_address, layer_4_protocol_number, layer_4_packet_size as u32);
		
		Rfc1141CompliantCheckSum::from_struct_check_sum_partial(&pseudo_header, 0)
	}
}
