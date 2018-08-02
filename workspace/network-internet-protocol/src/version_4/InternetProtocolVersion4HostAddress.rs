// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Internet Protocol (IP) version 4 host address.
///
/// Stored internally in network byte order.
///
/// Defaults to `Unspecified` (which is the same as `Any`).
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(C, packed)]
pub struct InternetProtocolVersion4HostAddress(pub(crate) [u8; InternetProtocolVersion4HostAddress::Size]);

impl Display for InternetProtocolVersion4HostAddress
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}.{}.{}.{}", self.get_first_byte(), self.get_second_byte(), self.get_third_byte(), self.get_fourth_byte())
	}
}

impl Debug for InternetProtocolVersion4HostAddress
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}.{}.{}.{}", self.get_first_byte(), self.get_second_byte(), self.get_third_byte(), self.get_fourth_byte())
	}
}

impl Default for InternetProtocolVersion4HostAddress
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Unspecified
	}
}

impl From<[u8; InternetProtocolVersion4HostAddress::Size]> for InternetProtocolVersion4HostAddress
{
	#[inline(always)]
	fn from(octets: [u8; InternetProtocolVersion4HostAddress::Size]) -> Self
	{
		InternetProtocolVersion4HostAddress(octets)
	}
}

impl Into<[u8; InternetProtocolVersion4HostAddress::Size]> for InternetProtocolVersion4HostAddress
{
	#[inline(always)]
	fn into(self) -> [u8; InternetProtocolVersion4HostAddress::Size]
	{
		self.0
	}
}

impl NetworkEndian for InternetProtocolVersion4HostAddress
{
	/// Underlying bytes.
	#[inline(always)]
	fn bytes(&self) -> &[u8]
	{
		&self.0
	}
	
	/// Writes to a hasher creating a hash.
	#[inline(always)]
	fn write_to_hash<H: Hasher>(&self, hasher: &mut H)
	{
		hasher.write_u32(unsafe { transmute_copy(&self.0) })
	}
}

impl TreeBitmapAddress for InternetProtocolVersion4HostAddress
{
	type Nibbles = [u8; Self::NibblesLength];
	
	#[inline(always)]
	fn nibbles(self) -> Self::Nibbles
	{
		self.nibbles_non_destructively()
	}
	
	#[inline(always)]
	fn from_nibbles(nibbles: &[u8]) -> Self
	{
		let mut octets: <Self as InternetProtocolHostAddress>::Octets = unsafe { zeroed() };
		let limit = min(Self::NibblesLength, nibbles.len());
		for (nibble_index, nibble) in nibbles.iter().enumerate().take(limit)
		{
			let nibble = *nibble;
			let remainder = nibble_index % 2;
			let octet_index = nibble_index / 2;
			octets[octet_index] = if remainder == 0
			{
				nibble << 4
			}
			else
			{
				octets[octet_index] | nibble
			};
		}
		
		InternetProtocolVersion4HostAddress(octets)
	}
	
	#[inline(always)]
	fn mask(self, depth: u32) -> Self
	{
		debug_assert!(depth <= 32, "depth exceeds 32");
		
		let mask_bits = <Self as InternetProtocolHostAddress>::MaskBits::from_depth(depth as u8);
		Self::from_network_endian(self.as_network_endian() & mask_bits as <Self as InternetProtocolHostAddress>::BigEndianValue)
	}
}

/// A trait abstracting the similarities between internet protocol (IP) version 4 and version 6 host addresses.
impl InternetProtocolHostAddress for InternetProtocolVersion4HostAddress
{
	type BigEndianValue = u32;
	
	type NativeEndianValue = u32;
	
	type RustAddress = Ipv4Addr;
	
	type LibCAddress = in_addr;
	
	type MaskBits = InternetProtocolVersion4MaskBits;
	
	type Octets = [u8; 4];
	
	const Size: usize = 4;
	
	const SizeU8: u8 = 4;
	
	#[inline(always)]
	fn from_network_endian(big_endian_value: Self::BigEndianValue) -> Self
	{
		Self::from_octets(unsafe { transmute(big_endian_value) })
	}
	
	#[inline(always)]
	fn from_octets(octets: Self::Octets) -> Self
	{
		InternetProtocolVersion4HostAddress(octets)
	}
	
	#[inline(always)]
	fn from_rust_address_to_libc_address(rust_address: &Self::RustAddress) -> Self::LibCAddress
	{
		unsafe { transmute_copy(rust_address) }
	}
	
	#[inline(always)]
	fn from_rust_address(rust_address: &Self::RustAddress) -> Self
	{
		unsafe { transmute_copy(rust_address) }
	}
	
	#[inline(always)]
	fn to_rust_address(&self) -> Self::RustAddress
	{
		unsafe { transmute_copy(self) }
	}
	
	#[inline(always)]
	fn to_libc_address(self) -> Self::LibCAddress
	{
		in_addr
		{
			s_addr: self.as_network_endian(),
		}
	}
	
	#[inline(always)]
	fn as_native_endian(&self) -> Self::NativeEndianValue
	{
		u32::from_be(self.as_network_endian())
	}
	
	#[inline(always)]
	fn as_network_endian(&self) -> Self::BigEndianValue
	{
		u32::from_bytes(self.0)
	}
	
	#[inline(always)]
	fn to_media_access_control_address(&self) -> Result<MediaAccessControlAddress, ()>
	{
		MediaAccessControlAddress::from_private_internet_protocol_version_4_host_address(self)
	}
	
	#[inline(always)]
	fn nibbles_non_destructively(&self) -> <Self as TreeBitmapAddress>::Nibbles
	{
		let octets = &self.0;
		
		let _0 = unsafe { *octets.get_unchecked(0) };
		let _1 = unsafe { *octets.get_unchecked(1) };
		let _2 = unsafe { *octets.get_unchecked(2) };
		let _3 = unsafe { *octets.get_unchecked(3) };
		
		[
			_0 >> 4,
			_0 & 0x0F,
			_1 >> 4,
			_1 & 0x0F,
			_2 >> 4,
			_2 & 0x0F,
			_3 >> 4,
			_3 & 0x0F,
		]
	}
}

impl InternetProtocolVersion4HostAddress
{
	/// Unspecified (Any) address.
	pub const Unspecified: Self = InternetProtocolVersion4HostAddress([0, 0, 0, 0]);
	
	/// Broadcast address.
	pub const Broadcast: Self = InternetProtocolVersion4HostAddress([255, 255, 255, 255]);
	
	/// To an embedded RFC 8215 globally routable (RFC 8215) `InternetProtocolVersion6HostAddress`.
	#[inline(always)]
	pub fn to_embedded_rfc8215_internet_protocol_version_6_host_address(self) -> InternetProtocolVersion6HostAddress
	{
		InternetProtocolVersion6HostAddress::from([0x00, 0x64, 0xFF, 0x9B, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, self.get_first_byte(), self.get_second_byte(), self.get_third_byte(), self.get_fourth_byte()])
	}
	
	/// To an embedded RFC 6052 globally routable (RFC 6052) `InternetProtocolVersion6HostAddress`.
	#[inline(always)]
	pub fn to_embedded_rfc6052_internet_protocol_version_6_host_address(self) -> InternetProtocolVersion6HostAddress
	{
		InternetProtocolVersion6HostAddress::from([0x00, 0x64, 0xFF, 0x9B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, self.get_first_byte(), self.get_second_byte(), self.get_third_byte(), self.get_fourth_byte()])
	}
	
	/// To a mapped (RFC 4291) `InternetProtocolVersion6HostAddress`.
	#[inline(always)]
	pub fn to_mapped_internet_protocol_version_6_host_address(self) -> InternetProtocolVersion6HostAddress
	{
		InternetProtocolVersion6HostAddress::from([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, self.get_first_byte(), self.get_second_byte(), self.get_third_byte(), self.get_fourth_byte()])
	}
	
	/// To a deprecated compatible (RFC 4291) `InternetProtocolVersion6HostAddress`.
	#[inline(always)]
	pub fn to_deprecated_compatible_internet_protocol_version_6_host_address(self) -> InternetProtocolVersion6HostAddress
	{
		InternetProtocolVersion6HostAddress::from([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, self.get_first_byte(), self.get_second_byte(), self.get_third_byte(), self.get_fourth_byte()])
	}
	
	/// Is this a valid unicast address?
	#[inline(always)]
	pub fn is_valid_unicast(self) -> bool
	{
		!self.is_not_valid_unicast()
	}
	
	/// Is this not a valid unicast address?
	#[inline(always)]
	pub fn is_not_valid_unicast(self) -> bool
	{
		self.is_unspecified() ||
		self.is_loopback() ||
		self.is_multicast() ||
		self.is_documentation() ||
		self.is_broadcast()
	}
	
	/// Is this not a globally unicast address?
	#[inline(always)]
	pub fn is_not_globally_unicast_unique(self) -> bool
	{
		self.is_not_valid_unicast() || self.is_link_local() || self.is_private()
	}
	
	/// Is this a private (ie not globally routable) address?
	#[inline(always)]
	pub fn is_private(self) -> bool
	{
		InternetProtocolVersion4NetworkAddress::Private1.contains(self) || InternetProtocolVersion4NetworkAddress::Private2.contains(self) || InternetProtocolVersion4NetworkAddress::Private3.contains(self)
	}
	
	/// Is this a link local address?
	#[inline(always)]
	pub fn is_link_local(self) -> bool
	{
		InternetProtocolVersion4NetworkAddress::LinkLocal.contains(self)
	}
	
	/// Is this the unspecified address?
	#[inline(always)]
	pub fn is_unspecified(self) -> bool
	{
		self == Self::Unspecified
	}
	
	/// Is this the broadcast address?
	#[inline(always)]
	pub fn is_broadcast(self) -> bool
	{
		self == Self::Broadcast
	}
	
	/// Is this not the broadcast address?
	#[inline(always)]
	pub fn is_not_broadcast(self) -> bool
	{
		self != Self::Broadcast
	}
	
	/// Is this a loopback address?
	#[inline(always)]
	pub fn is_loopback(self) -> bool
	{
		InternetProtocolVersion4NetworkAddress::Loopback.contains(self)
	}
	
	/// Is this a multicast address?
	#[inline(always)]
	pub fn is_multicast(self) -> bool
	{
		InternetProtocolVersion4NetworkAddress::Multicast.contains(self)
	}
	
	/// Is this a multicast address?
	#[inline(always)]
	pub fn is_not_multicast(self) -> bool
	{
		!self.is_multicast()
	}
	
	/// Is this an address used for documentation and in examples?
	#[inline(always)]
	pub fn is_documentation(self) -> bool
	{
		InternetProtocolVersion4NetworkAddress::TestNet1.contains(self) || InternetProtocolVersion4NetworkAddress::TestNet2.contains(self) || InternetProtocolVersion4NetworkAddress::TestNet3.contains(self)
	}
	
	/// Are the lower 23 bits a match?
	///
	/// Used for multicast addresses.
	#[inline(always)]
	pub fn has_lower_23_bits(self, lower_23_bits: &[u8; 3]) -> bool
	{
		self.get_second_byte() & 0b0111_1111 == lower_23_bits[0] && self.get_third_byte() == lower_23_bits[1] && self.get_fourth_byte() == lower_23_bits[2]
	}
	
	/// Are the lower 23 bits a match?
	///
	/// Used for multicast addresses.
	#[inline(always)]
	pub fn does_not_have_lower_23_bits(self, lower_23_bits: &[u8; 3]) -> bool
	{
		!self.has_lower_23_bits(lower_23_bits)
	}
	
	#[inline(always)]
	fn get_first_byte(&self) -> u8
	{
		unsafe { * self.0.get_unchecked(0) }
	}
	
	#[inline(always)]
	fn get_second_byte(&self) -> u8
	{
		unsafe { * self.0.get_unchecked(1) }
	}
	
	#[inline(always)]
	fn get_third_byte(&self) -> u8
	{
		unsafe { * self.0.get_unchecked(2) }
	}
	
	#[inline(always)]
	fn get_fourth_byte(&self) -> u8
	{
		unsafe { * self.0.get_unchecked(3) }
	}
}
