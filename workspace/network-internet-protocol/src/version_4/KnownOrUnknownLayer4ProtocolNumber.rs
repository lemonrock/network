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

impl<'deserialize> Deserialize<'deserialize> for KnownOrUnknownLayer4ProtocolNumber
{
	#[inline(always)]
	fn deserialize<D: Deserializer<'deserialize>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct SomeVisitor;
		
		impl<'deserialize> Visitor<'deserialize> for SomeVisitor
		{
			type Value = KnownOrUnknownLayer4ProtocolNumber;
			
			#[inline(always)]
			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
			{
				formatter.write_str("an unsigned 8-bit integer between 0 and 255")
			}
			
			#[inline(always)]
			fn visit_u8<E: de::Error>(self, value: u8) -> Result<Self::Value, E>
			{
				Ok
				(
					KnownOrUnknownLayer4ProtocolNumber
					{
						unknown: value,
					}
				)
			}
			
			#[inline(always)]
			fn visit_u16<E: de::Error>(self, value: u16) -> Result<Self::Value, E>
			{
				if value > ::std::u8::MAX as u16
				{
					return Err(E::custom("too large"))
				}
				
				self.visit_u8(value as u8)
			}
			
			#[inline(always)]
			fn visit_u32<E: de::Error>(self, value: u32) -> Result<Self::Value, E>
			{
				if value > ::std::u8::MAX as u32
				{
					return Err(E::custom("too large"))
				}
				
				self.visit_u8(value as u8)
			}
			
			#[inline(always)]
			fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E>
			{
				if value > ::std::u8::MAX as u64
				{
					return Err(E::custom("too large"))
				}
				
				self.visit_u8(value as u8)
			}
			
			#[inline(always)]
			fn visit_u128<E: de::Error>(self, value: u128) -> Result<Self::Value, E>
			{
				if value > ::std::u8::MAX as u128
				{
					return Err(E::custom("too large"))
				}
				
				self.visit_u8(value as u8)
			}
			
			#[inline(always)]
			fn visit_i8<E: de::Error>(self, value: i8) -> Result<Self::Value, E>
			{
				if value > ::std::i8::MAX as i8
				{
					return Err(E::custom("too large"))
				}
				
				if value < 0
				{
					return Err(E::custom("too small"))
				}
				
				self.visit_u8(value as u8)
			}
			
			#[inline(always)]
			fn visit_i16<E: de::Error>(self, value: i16) -> Result<Self::Value, E>
			{
				if value > ::std::u8::MAX as i16
				{
					return Err(E::custom("too large"))
				}
				
				if value < 0
				{
					return Err(E::custom("too small"))
				}
				
				self.visit_u8(value as u8)
			}
			
			#[inline(always)]
			fn visit_i32<E: de::Error>(self, value: i32) -> Result<Self::Value, E>
			{
				if value > ::std::u8::MAX as i32
				{
					return Err(E::custom("too large"))
				}
				
				if value < 0
				{
					return Err(E::custom("too small"))
				}
				
				self.visit_u8(value as u8)
			}
			
			#[inline(always)]
			fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E>
			{
				if value > ::std::u8::MAX as i64
				{
					return Err(E::custom("too large"))
				}
				
				if value < 0
				{
					return Err(E::custom("too small"))
				}
				
				self.visit_u8(value as u8)
			}
			
			#[inline(always)]
			fn visit_i128<E: de::Error>(self, value: i128) -> Result<Self::Value, E>
			{
				if value > ::std::u8::MAX as i128
				{
					return Err(E::custom("too large"))
				}
				
				if value < 0
				{
					return Err(E::custom("too small"))
				}
				
				self.visit_u8(value as u8)
			}
		}
		
		deserializer.deserialize_u8(SomeVisitor)
	}
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
