// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Address resolution protocol (ARP) hardware type.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(C, packed)]
#[derive(Serialize, Deserialize)]
pub struct Operation(NetworkEndianU16);

impl Into<NetworkEndianU16> for Operation
{
	#[inline(always)]
	fn into(self) -> NetworkEndianU16
	{
		self.0
	}
}

impl Into<u16> for Operation
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0.to_native_endian()
	}
}

impl From<NetworkEndianU16> for Operation
{
	#[inline(always)]
	fn from(value: NetworkEndianU16) -> Self
	{
		Operation(value)
	}
}

impl From<u16> for Operation
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		Operation(NetworkEndianU16::from_native_endian(value))
	}
}

impl Display for Operation
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", self.0.to_native_endian())
	}
}

impl Debug for Operation
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", self.0.to_native_endian())
	}
}

impl Operation
{
	/// Request.
	pub const Request: Self = Operation(NetworkEndianU16::from_network_endian([0x00, 0x01]));
	
	/// Reply.
	pub const Reply: Self = Operation(NetworkEndianU16::from_network_endian([0x00, 0x02]));
	
	/// To a network endian value.
	#[inline(always)]
	pub fn to_network_endian(self) -> u16
	{
		self.0.to_network_endian()
	}
}
