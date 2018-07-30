// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
#[derive(Debug)]
pub struct AddressResolutionProtocolPacket
{
	/// Header.
	pub header: AddressResolutionProtocolPacketHeader,

	/// Payload.
	pub payload: AddressResolutionProtocolPacketPayload,
}

impl Display for AddressResolutionProtocolPacket
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

#[cfg(feature = "dpdk-sys")]
impl Into<arp_hdr> for AddressResolutionProtocolPacket
{
	#[inline(always)]
	fn into(self) -> arp_hdr
	{
		unsafe { transmute(self) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<&'a arp_hdr> for &'a AddressResolutionProtocolPacket
{
	#[inline(always)]
	fn into(self) -> &'a arp_hdr
	{
		unsafe { transmute(self) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<&'a mut arp_hdr> for &'a mut AddressResolutionProtocolPacket
{
	#[inline(always)]
	fn into(self) -> &'a mut arp_hdr
	{
		unsafe { transmute(self) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<NonNull<arp_hdr>> for &'a mut AddressResolutionProtocolPacket
{
	#[inline(always)]
	fn into(self) -> NonNull<arp_hdr>
	{
		unsafe { NonNull::new_unchecked(self as *mut AddressResolutionProtocolPacket as *mut arp_hdr) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<*const arp_hdr> for &'a AddressResolutionProtocolPacket
{
	#[inline(always)]
	fn into(self) -> *const arp_hdr
	{
		self as *const AddressResolutionProtocolPacket as *const _
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<*mut arp_hdr> for &'a mut AddressResolutionProtocolPacket
{
	#[inline(always)]
	fn into(self) -> *mut arp_hdr
	{
		self as *mut AddressResolutionProtocolPacket as *mut _
	}
}

#[cfg(feature = "dpdk-sys")]
impl From<arp_hdr> for AddressResolutionProtocolPacket
{
	#[inline(always)]
	fn from(value: arp_hdr) -> Self
	{
		unsafe { transmute(value) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> From<&'a arp_hdr> for &'a AddressResolutionProtocolPacket
{
	#[inline(always)]
	fn from(value: &'a arp_hdr) -> &'a AddressResolutionProtocolPacket
	{
		unsafe { transmute(value) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> From<&'a mut arp_hdr> for &'a mut AddressResolutionProtocolPacket
{
	#[inline(always)]
	fn from(value: &'a mut arp_hdr) -> &'a mut AddressResolutionProtocolPacket
	{
		unsafe { transmute(value) }
	}
}

impl AddressResolutionProtocolPacket
{
	/// Use this to eliminate invalid traffic.
	#[inline(always)]
	pub fn is_packet_length_too_short(layer_3_length: u16) -> bool
	{
		layer_3_length < AddressResolutionProtocolPacketHeader::HeaderSizeU16
	}

	/// Use this to eliminate obsolete ARP traffic.
	#[inline(always)]
	pub fn is_invalid_for_internet_protocol_version_4(&self, layer_3_length: u16) -> bool
	{
		self.is_layer_3_length_invalid_for_internet_protocol_version_4(layer_3_length) || self.header.is_header_invalid_for_internet_protocol_version_4()
	}


	/// Payload.
	#[inline(always)]
	pub fn payload(&mut self) -> &mut AddressResolutionProtocolPacketInternetProtocolVersion4Payload
	{
		unsafe { &mut self.payload.internet_protocol_version_4_payload }
	}
	
	/// Is the the layer 3 length invalid for an internet protocol version 4 ARP message?
	#[inline(always)]
	pub fn is_layer_3_length_invalid_for_internet_protocol_version_4(&self, layer_3_length: u16) -> bool
	{
		const PayloadSizeU16: u16 = size_of::<AddressResolutionProtocolPacketInternetProtocolVersion4Payload>() as u16;
		
		layer_3_length != AddressResolutionProtocolPacketHeader::HeaderSizeU16 + PayloadSizeU16
	}
}
