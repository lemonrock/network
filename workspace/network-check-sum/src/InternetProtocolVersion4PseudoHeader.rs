// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// A pseudo-header for Internet Protocol Version 4.
#[repr(C, packed)]
pub struct InternetProtocolVersion4PseudoHeader
{
	source_internet_protocol_version_4_address: NetworkEndianU32,
	destination_internet_protocol_version_4_address: NetworkEndianU32,
	reserved: u8,
	layer_4_protocol_number: u8,
	layer_4_packet_size: NetworkEndianU16,
}

impl InternetProtocolVersion4PseudoHeader
{
	#[inline(always)]
	pub(crate) fn new(source_internet_protocol_version_4_address: &NetworkEndianU32, destination_internet_protocol_version_4_address: &NetworkEndianU32, layer_4_protocol_number: Layer4ProtocolNumber, layer_4_packet_size: u16) -> Self
	{
		Self
		{
			source_internet_protocol_version_4_address: *source_internet_protocol_version_4_address,
			destination_internet_protocol_version_4_address: *destination_internet_protocol_version_4_address,
			reserved: 0,
			layer_4_protocol_number: layer_4_protocol_number.into(),
			layer_4_packet_size: NetworkEndianU16::from_native_endian(layer_4_packet_size),
		}
	}
	
	/// Computes a secure hash.
	#[inline(always)]
	pub fn secure_hash(digester: &mut impl Digest, source_internet_protocol_version_4_address: &NetworkEndianU32, destination_internet_protocol_version_4_address: &NetworkEndianU32, layer_4_protocol_number: Layer4ProtocolNumber, layer_4_packet_size: u16)
	{
		digester.input(source_internet_protocol_version_4_address.bytes());
		digester.input(destination_internet_protocol_version_4_address.bytes());
		digester.input(&[0]);
		digester.input(&[layer_4_protocol_number.into()]);
		digester.input(NetworkEndianU16::from_native_endian(layer_4_packet_size).bytes());
	}
}
