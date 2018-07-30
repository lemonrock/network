// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InternetProtocolVersion6PacketHeader
{
	/// Version, traffic class and flow label bit fields.
	pub version_and_traffic_class_and_flow_label: [u8; 4],
	
	/// Payload length
	pub payload_length_including_extension_headers: NetworkEndianU16,
	
	/// Next header.
	pub next_header: ExtensionHeaderTypeOrLayer4ProtocolNumber,
	
	/// Hop limits.
	pub hop_limits: u8,
	
	/// Source address.
	pub source_address: InternetProtocolVersion6HostAddress,
	
	/// Destination address.
	pub destination_address: InternetProtocolVersion6HostAddress,
	
	/// Extension header or payload pointer.
	pub extension_header_or_payload: PhantomData<u8>,
}

impl Display for InternetProtocolVersion6PacketHeader
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

#[cfg(feature = "dpdk-sys")]
impl Into<ipv6_hdr> for InternetProtocolVersion6PacketHeader
{
	#[inline(always)]
	fn into(self) -> ipv6_hdr
	{
		unsafe { transmute(self) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<&'a ipv6_hdr> for &'a InternetProtocolVersion6PacketHeader
{
	#[inline(always)]
	fn into(self) -> &'a ipv6_hdr
	{
		unsafe { transmute(self) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<&'a mut ipv6_hdr> for &'a mut InternetProtocolVersion6PacketHeader
{
	#[inline(always)]
	fn into(self) -> &'a mut ipv6_hdr
	{
		unsafe { transmute(self) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<NonNull<ipv6_hdr>> for &'a mut InternetProtocolVersion6PacketHeader
{
	#[inline(always)]
	fn into(self) -> NonNull<ipv6_hdr>
	{
		unsafe { NonNull::new_unchecked(self as *mut InternetProtocolVersion6PacketHeader as *mut ipv6_hdr) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<*const ipv6_hdr> for &'a InternetProtocolVersion6PacketHeader
{
	#[inline(always)]
	fn into(self) -> *const ipv6_hdr
	{
		self as *const InternetProtocolVersion6PacketHeader as *const _
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<*mut ipv6_hdr> for &'a mut InternetProtocolVersion6PacketHeader
{
	#[inline(always)]
	fn into(self) -> *mut ipv6_hdr
	{
		self as *mut InternetProtocolVersion6PacketHeader as *mut _
	}
}

#[cfg(feature = "dpdk-sys")]
impl From<ipv6_hdr> for InternetProtocolVersion6PacketHeader
{
	#[inline(always)]
	fn from(value: ipv6_hdr) -> Self
	{
		unsafe { transmute(value) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> From<&'a ipv6_hdr> for &'a InternetProtocolVersion6PacketHeader
{
	#[inline(always)]
	fn from(value: &'a ipv6_hdr) -> &'a InternetProtocolVersion6PacketHeader
	{
		unsafe { transmute(value) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> From<&'a mut ipv6_hdr> for &'a mut InternetProtocolVersion6PacketHeader
{
	#[inline(always)]
	fn from(value: &'a mut ipv6_hdr) -> &'a mut InternetProtocolVersion6PacketHeader
	{
		unsafe { transmute(value) }
	}
}

impl InternetProtocolVersion6PacketHeader
{
	pub(crate) const HeaderSize: usize = size_of::<Self>();
	
	pub(crate) const HeaderSizeU16: u16 = Self::HeaderSize as u16;
	
	/// Is the version not '6'?
	#[inline(always)]
	pub fn is_version_not_6(&self) -> bool
	{
		(unsafe { *self.version_and_traffic_class_and_flow_label.get_unchecked(0) }) >> 4 != 6
	}
	
	/// Hops.
	#[inline(always)]
	pub fn hops(&self) -> u8
	{
		self.hop_limits
	}
	
	/// DifferentiatedServiceCodePoint and ExplicitCongestionNotification.
	#[inline(always)]
	pub fn traffic_class(&self) -> (DifferentiatedServiceCodePoint, ExplicitCongestionNotification)
	{
		let first_four_bits = (unsafe { *self.version_and_traffic_class_and_flow_label.get_unchecked(0) }) << 4;
		let second_four_bits = unsafe { *self.version_and_traffic_class_and_flow_label.get_unchecked(1) } & 0b1111_0000;
		let traffic_class = first_four_bits | second_four_bits;
		
		(DifferentiatedServiceCodePoint::from(traffic_class >> 2), unsafe { transmute(traffic_class & 0b11) })
	}
	
	/// 20-bit flow-label.
	#[inline(always)]
	pub fn flow_label(&self) -> u32
	{
		let first_four_bits = (((unsafe { *self.version_and_traffic_class_and_flow_label.get_unchecked(1) }) & 0b0000_1111)  as u32) << 16;
		let second_eight_bits = ((unsafe { *self.version_and_traffic_class_and_flow_label.get_unchecked(2) }) as u32) << 8;
		let third_eight_bits = (unsafe { *self.version_and_traffic_class_and_flow_label.get_unchecked(3) }) as u32;
		
		first_four_bits | second_eight_bits | third_eight_bits
	}
	
	/// Sets the payload length.
	#[inline(always)]
	pub fn set_payload_length_including_extension_headers(&mut self, payload_length: u16)
	{
		self.payload_length_including_extension_headers = NetworkEndianU16::from_native_endian(payload_length)
	}
	
	/// Sets the explicit congestion notification.
	#[inline(always)]
	pub fn set_explicit_congestion_notification(&mut self, explicit_congestion_notification: ExplicitCongestionNotification)
	{
		let bottom_two_traffic_class_bits = unsafe { *self.version_and_traffic_class_and_flow_label.get_unchecked(1) } & 0b1100_0000;
		
		let into: u8 = explicit_congestion_notification.into();
		
		let second_byte = bottom_two_traffic_class_bits | (into << 4);
		
		unsafe { *self.version_and_traffic_class_and_flow_label.get_unchecked_mut(1) = second_byte }
	}
}
