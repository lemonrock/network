// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// An untagged union representing codes.
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union InternetControlMessageProtocolVersion6Code
{
	/// Destination Unreachable.
	///
	/// `type_` of the packet header is `InternetControlMessageProtocolVersion6Type::DestinationUnreachable`.
	destination_unreachable: InternetControlMessageProtocolVersion6CodeDestinationUnreachable,
	
	/// Packet too big.
	///
	/// `type_` of the packet header is `InternetControlMessageProtocolVersion6Type::PacketTooBig`.
	packet_too_big: InternetControlMessageProtocolVersion6CodePacketTooBig,
	
	/// Time exceeded.
	///
	/// `type_` of the packet header is `InternetControlMessageProtocolVersion6Type::TimeExceeded`.
	time_exceeded: InternetControlMessageProtocolVersion6CodeTimeExceeded,
	
	/// Parameter problem.
	///
	/// `type_` of the packet header is `InternetControlMessageProtocolVersion6Type::TimeExceeded`.
	parameter_problem: InternetControlMessageProtocolVersion6CodeParameterProblem,
	
	/// Many `type_` values in the packet header are experimental, reserved, obscure or lacking in information.
	///
	/// Or their may be just an unexpected code value present.
	undifferentiated: u8,
}

impl Serialize for InternetControlMessageProtocolVersion6Code
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		(unsafe { self.undifferentiated }).serialize(serializer)
	}
}

impl<'deserialize> Deserialize<'deserialize> for InternetControlMessageProtocolVersion6Code
{
	#[inline(always)]
	fn deserialize<D: Deserializer<'deserialize>>(deserializer: D) -> Result<Self, D::Error>
	{
		Ok
		(
			Self
			{
				undifferentiated: u8::deserialize(deserializer)?,
			}
		)
		
	}
}

impl From<u8> for InternetControlMessageProtocolVersion6Code
{
	#[inline(always)]
	fn from(undifferentiated: u8) -> Self
	{
		InternetControlMessageProtocolVersion6Code
		{
			undifferentiated
		}
	}
}

impl Into<u8> for InternetControlMessageProtocolVersion6Code
{
	#[inline(always)]
	fn into(self) -> u8
	{
		unsafe { self.undifferentiated }
	}
}

impl Display for InternetControlMessageProtocolVersion6Code
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Debug for InternetControlMessageProtocolVersion6Code
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", unsafe { self.undifferentiated })
	}
}

impl PartialOrd for InternetControlMessageProtocolVersion6Code
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		unsafe { self.undifferentiated.partial_cmp(&other.undifferentiated) }
	}
}

impl Ord for InternetControlMessageProtocolVersion6Code
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		unsafe { self.undifferentiated.cmp(&other.undifferentiated) }
	}
}

impl PartialEq for InternetControlMessageProtocolVersion6Code
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { self.undifferentiated == other.undifferentiated }
	}
}

impl Eq for InternetControlMessageProtocolVersion6Code
{
}

impl Hash for InternetControlMessageProtocolVersion6Code
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		hasher.write_u8(unsafe { self.undifferentiated })
	}
}
