// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Extension header type or Layer 4 protocol number (the range overlaps; yuck).
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union KnownOrUnknownLayer4ProtocolNumber
{
	/// A known layer 4 protocol number.
	pub layer_4_protocol_number: Layer4ProtocolNumber,
	
	/// An unknown layer 4 protocol number.
	pub unknown: u8,
}

impl Serialize for KnownOrUnknownLayer4ProtocolNumber
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		(unsafe { self.unknown }).serialize(serializer)
	}
}

impl From<u8> for KnownOrUnknownLayer4ProtocolNumber
{
	#[inline(always)]
	fn from(value: u8) -> Self
	{
		Self
		{
			unknown: value,
		}
	}
}

impl Into<u8> for KnownOrUnknownLayer4ProtocolNumber
{
	#[inline(always)]
	fn into(self) -> u8
	{
		unsafe { self.unknown }
	}
}

impl Display for KnownOrUnknownLayer4ProtocolNumber
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", unsafe { self.unknown })
	}
}

impl Debug for KnownOrUnknownLayer4ProtocolNumber
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", unsafe { self.unknown })
	}
}

impl PartialOrd for KnownOrUnknownLayer4ProtocolNumber
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		unsafe { self.unknown.partial_cmp(&other.unknown) }
	}
}

impl Ord for KnownOrUnknownLayer4ProtocolNumber
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		unsafe { self.unknown.cmp(&other.unknown) }
	}
}

impl PartialEq for KnownOrUnknownLayer4ProtocolNumber
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { self.unknown == other.unknown }
	}
}

impl Eq for KnownOrUnknownLayer4ProtocolNumber
{
}

impl Hash for KnownOrUnknownLayer4ProtocolNumber
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		hasher.write_u8(unsafe { self.unknown })
	}
}

impl KnownOrUnknownLayer4ProtocolNumber
{
	/// Internet Control Message Protocol (ICMP).
	///
	/// RFC 792.
	pub const InternetControlMessageProtocol: u8 = 1;
	
	/// Transmission Control Protocol (TCP).
	///
	/// RFC 793.
	pub const TransmissionControlProtocol: u8 = 6;
	
	/// User Datagram Protocol (UDP).
	///
	/// RFC 768.
	pub const UserDatagramProtocol: u8 = 17;
}
