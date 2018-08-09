// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Represents known Internet Control Message Protocol (ICMP) Neighbor Discovery Option (RFC 4861) types.
///
/// See [IANA](https://www.iana.org/assignments/icmpv6-parameters/icmpv6-parameters.xhtml#icmpv6-parameters-5) for a complete list.
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union InternetControlMessageProtocolVersion6NeighborDiscoveryOptionType
{
	known: KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType,
	unknown: u8,
}

impl Serialize for InternetControlMessageProtocolVersion6NeighborDiscoveryOptionType
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		(unsafe { self.unknown }).serialize(serializer)
	}
}

impl<'deserialize> Deserialize<'deserialize> for InternetControlMessageProtocolVersion6NeighborDiscoveryOptionType
{
	#[inline(always)]
	fn deserialize<D: Deserializer<'deserialize>>(deserializer: D) -> Result<Self, D::Error>
	{
		Ok
		(
			Self
			{
				unknown: u8::deserialize(deserializer)?,
			}
		)
		
	}
}

impl From<u8> for InternetControlMessageProtocolVersion6NeighborDiscoveryOptionType
{
	#[inline(always)]
	fn from(unknown: u8) -> Self
	{
		InternetControlMessageProtocolVersion6NeighborDiscoveryOptionType
		{
			unknown
		}
	}
}

impl Into<u8> for InternetControlMessageProtocolVersion6NeighborDiscoveryOptionType
{
	#[inline(always)]
	fn into(self) -> u8
	{
		unsafe { self.unknown }
	}
}

impl Display for InternetControlMessageProtocolVersion6NeighborDiscoveryOptionType
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", unsafe { self.unknown })
	}
}

impl Debug for InternetControlMessageProtocolVersion6NeighborDiscoveryOptionType
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", unsafe { self.unknown })
	}
}

impl PartialOrd for InternetControlMessageProtocolVersion6NeighborDiscoveryOptionType
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		unsafe { self.unknown.partial_cmp(&other.unknown) }
	}
}

impl Ord for InternetControlMessageProtocolVersion6NeighborDiscoveryOptionType
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		unsafe { self.unknown.cmp(&other.unknown) }
	}
}

impl PartialEq for InternetControlMessageProtocolVersion6NeighborDiscoveryOptionType
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { self.unknown == other.unknown }
	}
}

impl Eq for InternetControlMessageProtocolVersion6NeighborDiscoveryOptionType
{
}

impl Hash for InternetControlMessageProtocolVersion6NeighborDiscoveryOptionType
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		hasher.write_u8(unsafe { self.unknown })
	}
}
