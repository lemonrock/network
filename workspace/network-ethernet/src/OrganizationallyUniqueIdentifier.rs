// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// An Organizationally Unique Identifier (OUI).
///
/// Sanitized Organizationally Unique Identifiers (OUIs) are available from https://linuxnet.ca/ieee/oui/
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, packed)]
pub struct OrganizationallyUniqueIdentifier([u8; OrganizationallyUniqueIdentifier::Size]);

impl Display for OrganizationallyUniqueIdentifier
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		let bytes = self.0;
		write!(f, "{:02X}:{:02X}:{:02X}", bytes[0], bytes[1], bytes[2])
	}
}

impl Serialize for OrganizationallyUniqueIdentifier
{
	#[inline(always)]
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		serializer.serialize_str(&format!("{}", self))
	}
}

impl<'deserialize> Deserialize<'deserialize> for OrganizationallyUniqueIdentifier
{
	#[inline(always)]
	fn deserialize<D: Deserializer<'deserialize>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct FromStr;
		
		impl<'deserialize> Visitor<'deserialize> for FromStr
		{
			type Value = OrganizationallyUniqueIdentifier;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
			{
				formatter.write_str("string of 3 2-byte hexadecimal values separated by colons, eg 00:AA:BB")
			}
			
			#[inline(always)]
			fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E>
			{
				fn next<'a, E: de::Error>(splits: &mut SplitN<'a, char>) -> Result<u8, E>
				{
					if let Some(hexadecimal_byte_without_prefix) = splits.next()
					{
						match u8::from_str_radix(hexadecimal_byte_without_prefix, 16)
						{
							Ok(value) => Ok(value),
							Err(_) => Err(E::custom("Could not convert hexadecimal byte in OrganizationallyUniqueIdentifier")),
						}
					}
					else
					{
						Err(E::custom("Less than 3 hexadecimal bytes in OrganizationallyUniqueIdentifier"))
					}
				}
				
				let splits = &mut value.splitn(6, ':');
				
				let bytes =
				[
					next(splits)?,
					next(splits)?,
					next(splits)?,
				];
				
				if splits.next().is_some()
				{
					Err(E::custom("More than 3 hexadecimal bytes"))
				}
				else
				{
					Ok(OrganizationallyUniqueIdentifier(bytes))
				}
			}
		}
		
		deserializer.deserialize_str(FromStr)
	}
}

impl From<[u8; OrganizationallyUniqueIdentifier::Size]> for OrganizationallyUniqueIdentifier
{
	#[inline(always)]
	fn from(value: [u8; OrganizationallyUniqueIdentifier::Size]) -> Self
	{
		OrganizationallyUniqueIdentifier(value)
	}
}

impl Into<[u8; OrganizationallyUniqueIdentifier::Size]> for OrganizationallyUniqueIdentifier
{
	#[inline(always)]
	fn into(self) -> [u8; OrganizationallyUniqueIdentifier::Size]
	{
		self.0
	}
}

impl OrganizationallyUniqueIdentifier
{
	/// Size (in bytes) of an Organizationally Unique Identifier (OUI).
	pub const Size: usize = 3;
	
	/// Also known as a Multicast or Broadcast address.
	pub const GroupAddressBitFlag: u8 = 0x01;
	
	/// Locally administered, ie uses an Organizationally Unique Identifier (OUI) assigned by IANA.
	pub const LocallyAdministeredAddressBitFlag: u8 = 0x02;
	
	/// IANA self.
	pub const IanaSelf: Self = OrganizationallyUniqueIdentifier([0x01, 0x00, 0x5E]);
	
	/// Is this an internet protocol (IP) version 6 multicast Organizationally Unique Identifier (OUI)?
	///
	/// Same as `is_unicast()`.
	#[inline(always)]
	pub fn is_internet_protocol_version_6_multicast(&self) -> bool
	{
		self.get_first_two_bytes_network_endian() == 0x3333
	}
	
	/// Is this a multicast (or broadcast, considered a sub type of multicast in Ethernet) Organizationally Unique Identifier (OUI)?
	///
	/// Otherwise known as a 'group' Organizationally Unique Identifier (OUI).
	///
	/// Note that multicast and broadcast Organizationally Unique Identifier (OUI) will also be true for `self.is_locally_administered()` and false for `self.is_universally_administered()`.
	#[inline(always)]
	pub fn is_multicast_or_broadcast(&self) -> bool
	{
		self.get_first_byte() & Self::GroupAddressBitFlag != 0
	}
	
	/// Is this an universally administered Organizationally Unique Identifier (OUI)?
	#[inline(always)]
	pub fn is_universally_administered(&self) -> bool
	{
		self.get_first_byte() & Self::LocallyAdministeredAddressBitFlag == 0
	}
	
	/// Is this address one that is locally administered?
	///
	/// This includes multicast and broadcast Organizationally Unique Identifier (OUI).
	#[inline(always)]
	pub fn is_locally_administered(&self) -> bool
	{
		self.get_first_byte() & Self::LocallyAdministeredAddressBitFlag != 0
	}
	
	/// Alternative formatting to debug and display format.
	///
	/// As per IEEE standard 802 (2001), ISBN 0-7381-2941-0.
	#[inline(always)]
	pub fn ibm_token_ring_bit_reversed_format(&self, f: &mut Formatter) -> fmt::Result
	{
		let bytes = &self.0;
		write!(f, "{:02X}:{:02X}:{:02X}", bytes[2].reverse_bits(), bytes[1].reverse_bits(), bytes[0].reverse_bits())
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
}
