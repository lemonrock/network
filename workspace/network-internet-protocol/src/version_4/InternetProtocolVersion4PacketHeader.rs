// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct InternetProtocolVersion4PacketHeader
{
	/// Version and internet header length bit fields.
	pub version_and_internet_header_length: u8,
	
	/// Type of service.
	pub type_of_service: u8,
	
	/// Total length.
	pub total_length: NetworkEndianU16,
	
	/// Fragmentation packet identifier.
	pub fragment_identifier: NetworkEndianU16,
	
	/// Fragmentation offset.
	pub fragment_offset: NetworkEndianU16,
	
	/// Hops.
	pub time_to_live: u8,
	
	/// Layer 4 protocol identifier.
	pub next_proto_id: Layer4ProtocolNumber,
	
	/// Check sum.
	pub check_sum: InternetCheckSum,
	
	/// Source address.
	pub source_address: InternetProtocolVersion4HostAddress,
	
	/// Destination address.
	pub destination_address: InternetProtocolVersion4HostAddress,
}

impl Display for InternetProtocolVersion4PacketHeader
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

#[cfg(feature = "dpdk-sys")]
impl Into<ipv4_hdr> for InternetProtocolVersion4PacketHeader
{
	#[inline(always)]
	fn into(self) -> ipv4_hdr
	{
		unsafe { transmute(self) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<&'a ipv4_hdr> for &'a InternetProtocolVersion4PacketHeader
{
	#[inline(always)]
	fn into(self) -> &'a ipv4_hdr
	{
		unsafe { transmute(self) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<NonNull<ipv4_hdr>> for &'a mut InternetProtocolVersion4PacketHeader
{
	#[inline(always)]
	fn into(self) -> NonNull<ipv4_hdr>
	{
		unsafe { NonNull::new_unchecked(self as *mut InternetProtocolVersion4PacketHeader as *mut ipv4_hdr) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<*const ipv4_hdr> for &'a InternetProtocolVersion4PacketHeader
{
	#[inline(always)]
	fn into(self) -> *const ipv4_hdr
	{
		self as *const InternetProtocolVersion4PacketHeader as *const _
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<*mut ipv4_hdr> for &'a mut InternetProtocolVersion4PacketHeader
{
	#[inline(always)]
	fn into(self) -> *mut ipv4_hdr
	{
		self as *mut InternetProtocolVersion4PacketHeader as *mut _
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<&'a mut ipv4_hdr> for &'a mut InternetProtocolVersion4PacketHeader
{
	#[inline(always)]
	fn into(self) -> &'a mut ipv4_hdr
	{
		unsafe { transmute(self) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl From<ipv4_hdr> for InternetProtocolVersion4PacketHeader
{
	#[inline(always)]
	fn from(value: ipv4_hdr) -> Self
	{
		unsafe { transmute(value) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> From<&'a ipv4_hdr> for &'a InternetProtocolVersion4PacketHeader
{
	#[inline(always)]
	fn from(value: &'a ipv4_hdr) -> &'a InternetProtocolVersion4PacketHeader
	{
		unsafe { transmute(value) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> From<&'a mut ipv4_hdr> for &'a mut InternetProtocolVersion4PacketHeader
{
	#[inline(always)]
	fn from(value: &'a mut ipv4_hdr) -> &'a mut InternetProtocolVersion4PacketHeader
	{
		unsafe { transmute(value) }
	}
}

impl InternetProtocolVersion4PacketHeader
{
	/// Header size.
	pub const HeaderSize: usize = size_of::<Self>();
	
	/// Header size (u8).
	pub const HeaderSizeU8: u8 = Self::HeaderSize as u8;
	
	/// Header size (u16).
	pub const HeaderSizeU16: u16 = Self::HeaderSize as u16;
	
	const ReservedFragmentFlag: u16 = 0b1000_0000_0000_0000;
	
	const DoNotFragmentFlag: u16 = 0b0100_0000_0000_0000;
	
	const MoreFragmentsFlag: u16 = 0b0010_0000_0000_0000;
	
	/// Is the version not '4'?
	#[inline(always)]
	pub fn is_version_not_4(&self) -> bool
	{
		self.version_and_internet_header_length & 0xF0 != 4 << 4
	}
	
	/// RFC 6864 Section 4: "Atomic datagrams: (DF==1)&&(MF==0)&&(frag_offset==0)
	/// Non-atomic datagrams: (DF==0)||(MF==1)||(frag_offset>0)".
	#[inline(always)]
	pub fn has_invalid_fragmentation_flags_or_identification(&self) -> bool
	{
		const FragmentFlagMask: u16 = InternetProtocolVersion4PacketHeader::ReservedFragmentFlag | InternetProtocolVersion4PacketHeader::DoNotFragmentFlag | InternetProtocolVersion4PacketHeader::MoreFragmentsFlag;
		
		const UnfragmentedOrLastFragment: u16 = 0;
		
		let r = self.fragment_offset.to_network_endian() & FragmentFlagMask.to_be();
		
		if r == Self::DoNotFragmentFlag.to_be()
		{
			// Strictly speaking, checking the fragment identifier is non-zero VIOLATES RFC 6864 section 4.1 paragraph 5.
			self.fragment_offset_is_not_zero() || if cfg!(feature = "drop-ipv4-packets-with-do-not-fragment-and-non-zero-identification")
			{
				self.fragment_identifier.is_not_zero()
			}
			else
			{
				false
			}
		}
		else if r == UnfragmentedOrLastFragment || r == Self::MoreFragmentsFlag.to_be()
		{
			false
		}
		else
		{
			true
		}
	}
	
	#[inline(always)]
	fn fragment_offset_is_not_zero(&self) -> bool
	{
		self.fragment_offset.is_not_zero()
	}
	
	/// Header length including options.
	#[inline(always)]
	pub fn header_length_including_options(&self) -> u8
	{
		self.version_and_internet_header_length & 0x0F << 2
	}
	
	/// Total length.
	#[inline(always)]
	pub fn total_length(&self) -> u16
	{
		self.total_length.to_native_endian()
	}
	
	/// Hops.
	#[inline(always)]
	pub fn hops(&self) -> u8
	{
		self.time_to_live
	}
	
	/// Layer 4 protocol number.
	#[inline(always)]
	pub fn layer_4(&self) -> Layer4ProtocolNumber
	{
		self.next_proto_id
	}
	
	/// Payload length.
	#[inline(always)]
	pub fn payload_length(&self) -> u16
	{
		self.total_length.to_native_endian() - (self.header_length_including_options() as u16)
	}
	
	/// Checks if an internet protocol (IP) version 4 packet is fragmented.
	#[inline(always)]
	pub fn is_fragmented(&self) -> bool
	{
		const OffsetMask: u16 = InternetProtocolVersion4PacketHeader::MoreFragmentsFlag - 1;
		
		self.fragment_offset.to_network_endian() & (Self::MoreFragmentsFlag | OffsetMask).to_be() != 0
	}
}
