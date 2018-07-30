// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a Media Access Control Address.
///
/// Also known as an Ethernet Address.
///
/// A universally administered address is uniquely assigned to a device by its manufacturer. The first three octets (in network order) contain the Organizationally Unique Identifier (OUI). The following three (MAC-48 and EUI-48) octets are assigned by that organization with the only constraint of uniqueness.
///
/// A locally administered address is assigned to a device by a network administrator and does not contain OUIs.
///
/// See [IEEE Explanation](http://standards.ieee.org/regauth/groupmac/tutorial.html) for more details.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, packed)]
pub struct MediaAccessControlAddress([u8; MediaAccessControlAddress::Size]);

impl Display for MediaAccessControlAddress
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Debug for MediaAccessControlAddress
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		let bytes = &self.0;
		write!(f, "{:02X}-{:02X}-{:02X}-{:02X}-{:02X}-{:02X}", bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5])
	}
}

impl Serialize for MediaAccessControlAddress
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		serializer.serialize_str(&format!("{}", self))
	}
}

impl<'deserialize> Deserialize<'deserialize> for MediaAccessControlAddress
{
	#[inline(always)]
	fn deserialize<D: Deserializer<'deserialize>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct FromStr;
		
		impl<'deserialize> Visitor<'deserialize> for FromStr
		{
			type Value = MediaAccessControlAddress;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
			{
				formatter.write_str("string of 6 2-byte hexadecimal values separated by hyphens, eg 00-AA-BB-CC-DD-EE")
			}
			
			#[inline(always)]
			fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E>
			{
				#[inline(always)]
				fn next<'a, E: de::Error>(splits: &mut SplitN<'a, char>) -> Result<u8, E>
				{
					if let Some(hexadecimal_byte_without_prefix) = splits.next()
					{
						match u8::from_str_radix(hexadecimal_byte_without_prefix, 16)
						{
							Ok(value) => Ok(value),
							Err(_) => Err(E::custom("Could not convert hexadecimal byte in MediaAccessControlAddress")),
						}
					}
					else
					{
						Err(E::custom("Less than 6 hexadecimal bytes in MediaAccessControlAddress"))
					}
				}
				
				let splits = &mut value.splitn(6, '-');
				
				let bytes =
				[
					next(splits)?,
					next(splits)?,
					next(splits)?,
					next(splits)?,
					next(splits)?,
					next(splits)?,
				];
				
				if splits.next().is_some()
				{
					Err(E::custom("More than 6 hexadecimal bytes"))
				}
				else
				{
					Ok(MediaAccessControlAddress(bytes))
				}
			}
		}
		
		deserializer.deserialize_str(FromStr)
	}
}

impl MediaAccessControlAddress
{
	/// Size of an Media Access Control Address in bytes.
	pub const Size: usize = 6;
	
	/// Size of an Media Access Control Address in bytes (as an u8).
	pub const SizeU8: u8 = Self::Size as u8;
	
	/// Size of an Media Access Control Address in bytes (as an u32).
	pub const SizeU32: u32 = Self::Size as u32;
	
	/// An address that is all zeros.
	pub const Zero: Self = MediaAccessControlAddress([0; Self::Size]);
	
	// Also known as a Multicast or Broadcast address.
	pub(crate) const GroupAddressBitFlag: u8 = 0x01;
	
	pub(crate) const LocallyAdministeredAddressBitFlag: u8 = 0x02;
	
	/// Alternative formatting to debug and display format.
	///
	/// As per IEEE standard 802 (2001), ISBN 0-7381-2941-0.
	#[inline(always)]
	pub fn ibm_token_ring_bit_reversed_format(&self, f: &mut Formatter) -> fmt::Result
	{
		let bytes = &self.0;
		write!(f, "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}", bytes[5].reverse_bits(), bytes[4].reverse_bits(), bytes[3].reverse_bits(), bytes[2].reverse_bits(), bytes[1].reverse_bits(), bytes[0].reverse_bits())
	}
	
	/// From DPDK type.
	#[cfg(feature = "dpdk-sys")]
	#[inline(always)]
	pub fn from_ether_addr(dpdk_type: ether_addr) -> Self
	{
		MediaAccessControlAddress(dpdk_type.addr_bytes)
	}
	
	/// To DPDK type.
	#[cfg(feature = "dpdk-sys")]
	#[inline(always)]
	pub fn to_ether_addr(self) -> ether_addr
	{
		ether_addr
		{
			addr_bytes: self.0
		}
	}
	
	/// To DPDK type.
	#[cfg(feature = "dpdk-sys")]
	#[inline(always)]
	pub fn to_ether_addr_reference(&self) -> &ether_addr
	{
		unsafe { transmute(self) }
	}
	
	/// To DPDK type.
	#[cfg(feature = "dpdk-sys")]
	#[inline(always)]
	pub fn to_ether_addr_mutable_reference(&mut self) -> &mut ether_addr
	{
		unsafe { transmute(self) }
	}
	
	/// From octets.
	#[inline(always)]
	pub fn from_octets(octets: [u8; Self::Size]) -> Self
	{
		MediaAccessControlAddress(octets)
	}
	
	/// To octets.
	#[inline(always)]
	pub fn to_octets(self) -> [u8; Self::Size]
	{
		self.0
	}
	
	/// To octets.
	#[inline(always)]
	pub fn to_octets_reference(&self) -> &[u8; Self::Size]
	{
		&self.0
	}
	
	/// To octets.
	#[inline(always)]
	pub fn to_octets_mutable_reference(&mut self) -> &mut [u8; Self::Size]
	{
		&mut self.0
	}
	
	/// Size (in bytes) of an Organizationally Unique Identifier (OUI).
	pub const OrganizationallyUniqueIdentifierSize: usize = 3;
	
	/// Random locally administered unicast address.
	#[inline(always)]
	pub fn random_unicast_address() -> Self
	{
		let mut this: Self = unsafe { uninitialized() };
		
		let rand: [u8; 8] = unsafe { transmute(generate_hyper_thread_safe_random_u64()) };
		unsafe { copy_nonoverlapping(rand.as_ptr(), this.0.as_mut_ptr(), Self::Size) };
		
		{
			let first_byte = unsafe { this.0.get_unchecked_mut(0) };
			let mut first_byte_copy = *first_byte;
			first_byte_copy &= !Self::GroupAddressBitFlag;
			first_byte_copy |= Self::LocallyAdministeredAddressBitFlag;
			*first_byte = first_byte_copy;
		}
		
		this
	}
	
	/// Is this address all zeros?
	#[inline(always)]
	pub fn is_zero(&self) -> bool
	{
		self.get_first_four_bytes_network_endian() == 0x0000_0000 && self.get_last_two_bytes_network_endian() == 0x0000
	}
	
	/// Is this address not all zeros?
	#[inline(always)]
	pub fn is_not_zero(&self) -> bool
	{
		self.get_first_four_bytes_network_endian() != 0x0000_0000 || self.get_last_two_bytes_network_endian() != 0x0000
	}
	
	/// Is this a unicast address?
	///
	/// This can include addresses for which `self.is_zero()` is true. These, however, are unspecified. To check for a valid unicast address, use `self.is_valid_unicast()`.
	#[inline(always)]
	pub fn is_unicast(&self) -> bool
	{
		self.get_first_byte() & Self::GroupAddressBitFlag == 0
	}
	
	/// If this is an assigned address, is it valid?
	///
	/// This means that it is a valid unicast address that is not all zeros.
	#[inline(always)]
	pub fn is_valid_unicast(&self) -> bool
	{
		return self.is_unicast() && self.is_not_zero()
	}
	
	/// If this is an assigned address, is it invalid?
	#[inline(always)]
	pub fn is_not_valid_unicast(&self) -> bool
	{
		!self.is_valid_unicast()
	}
	
	/// Is this a multicast (or broadcast, considered a sub type of multicast in Ethernet) address?
	///
	/// Note that multicast and broadcast addresses will also be true for `self.is_locally_administered()` and false for `self.is_universally_administered()`.
	#[inline(always)]
	pub fn is_multicast_or_broadcast(&self) -> bool
	{
		self.get_first_byte() & Self::GroupAddressBitFlag == Self::GroupAddressBitFlag
	}
	
	/// Is this a multicast address?
	#[inline(always)]
	pub fn is_multicast(&self) -> bool
	{
		self.is_multicast_or_broadcast() && self.is_not_broadcast()
	}
	
	/// Is this not a multicast address?
	///
	/// Same as `is_unicast()`.
	#[inline(always)]
	pub fn is_not_multicast(&self) -> bool
	{
		self.is_unicast()
	}
	
	/// Is this an internet protocol (IP) version 6 multicast address?
	///
	/// Will only return 3 bytes (23 bits) if the top bit of them is not set.
	pub fn internet_protocol_version_4_multicast_23_bits(&self) -> Option<&[u8; 3]>
	{
		// Sanitized OUIs are available from https://linuxnet.ca/ieee/oui/
		
		const IanaSelf: [u8; 3] = [0x01, 0x00, 0x5E];
		
		match self.universally_administered_organizationally_unique_identifier()
		{
			None => None,
			Some((organizationally_unique_identifier, lower_3_bytes)) =>
			{
				if organizationally_unique_identifier == &IanaSelf
				{
					const IsNotForMulticastBitFlag: u8 = 0b1000_0000;
					if lower_3_bytes[0] & IsNotForMulticastBitFlag == IsNotForMulticastBitFlag
					{
						None
					}
					else
					{
						Some(lower_3_bytes)
					}
				}
				else
				{
					None
				}
			}
		}
	}
	
	/// Is this an internet protocol (IP) version 6 multicast address?
	///
	/// Same as `is_unicast()`.
	#[inline(always)]
	pub fn is_internet_protocol_version_6_multicast(&self) -> bool
	{
		self.get_first_two_bytes_network_endian() == 0x3333
	}
	
	/// Is this an internet protocol (IP) version 6 multicast address?
	///
	/// See [RFC 2464|https://tools.ietf.org/html/rfc2464], section 7.
	#[inline(always)]
	pub fn internet_protocol_version_6_multicast_32_bits(&self) -> Option<&[u8; 4]>
	{
		if self.is_internet_protocol_version_6_multicast()
		{
			Some(array_ref!(self.0, 2, 4))
		}
		else
		{
			None
		}
	}
	
	/// Is this a broadcast address?
	#[inline(always)]
	pub fn is_broadcast(&self) -> bool
	{
		self.get_first_four_bytes_network_endian() == 0xFFFF_FFFF && self.get_last_two_bytes_network_endian() == 0xFFFF
	}
	
	/// Is this not a broadcast address?
	#[inline(always)]
	pub fn is_not_broadcast(&self) -> bool
	{
		self.get_first_four_bytes_network_endian() != 0xFFFF_FFFF || self.get_last_two_bytes_network_endian() != 0xFFFF
	}
	
	/// Is this an universally administered address?
	#[inline(always)]
	pub fn is_universally_administered(&self) -> bool
	{
		self.get_first_byte() & Self::LocallyAdministeredAddressBitFlag == 0
	}
	
	/// Is this address one that is locally administered?
	///
	/// This includes multicast and broadcast addresses.
	#[inline(always)]
	pub fn is_locally_administered(&self) -> bool
	{
		self.get_first_byte() & Self::LocallyAdministeredAddressBitFlag == Self::LocallyAdministeredAddressBitFlag
	}
	
	/// Organizationally Unique Identifier (OUI) and MAC-48 (EUI-48) as a tuple.
	///
	/// Only present if this is an universally administered address.
	///
	/// From Wikipedia: "To convert a MAC-48 into an EUI-64, copy the OUI, append the two octets FF-FF and then copy the organization-specified extension identifier".
	#[inline(always)]
	pub fn universally_administered_organizationally_unique_identifier(&self) -> Option<(&[u8; Self::OrganizationallyUniqueIdentifierSize], &[u8; 3])>
	{
		if self.is_universally_administered()
		{
			Some((array_ref!(self.0, 0, MediaAccessControlAddress::OrganizationallyUniqueIdentifierSize), array_ref!(self.0, 3, 3)))
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	fn get_first_byte(&self) -> u8
	{
		*unsafe { self.0.get_unchecked(0) }
	}
	
	#[inline(always)]
	fn get_first_two_bytes_network_endian(&self) -> u16
	{
		unsafe { *(self.0.get_unchecked(0) as *const u8 as *const u16) }
	}
	
	#[inline(always)]
	fn get_first_four_bytes_network_endian(&self) -> u32
	{
		unsafe { *(self.0.get_unchecked(0) as *const u8 as *const u32) }
	}
	
	#[inline(always)]
	fn get_last_two_bytes_network_endian(&self) -> u16
	{
		unsafe { *(self.0.get_unchecked(3) as *const u8 as *const u16) }
	}
}
