// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Address resolution protocol (ARP) hardware type.
#[repr(C, packed)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct HardwareType(NetworkEndianU16);

impl Into<NetworkEndianU16> for HardwareType
{
	#[inline(always)]
	fn into(self) -> NetworkEndianU16
	{
		self.0
	}
}

impl Into<u16> for HardwareType
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0.to_native_endian()
	}
}

impl From<NetworkEndianU16> for HardwareType
{
	#[inline(always)]
	fn from(value: NetworkEndianU16) -> Self
	{
		HardwareType(value)
	}
}

impl From<u16> for HardwareType
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		HardwareType(NetworkEndianU16::from_native_endian(value))
	}
}

impl Display for HardwareType
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", self.0.to_native_endian())
	}
}

impl Debug for HardwareType
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", self.0.to_native_endian())
	}
}

impl Default for HardwareType
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Ethernet2
	}
}

impl HardwareType
{
	/// Ethernet 2 (also known as Ethernet II).
	pub const Ethernet2: Self = HardwareType(NetworkEndianU16::from_network_endian([0x00, 0x01]));
	
	/// Use this to eliminate unwanted ARP traffic.
	#[inline(always)]
	pub fn is_not_ethernet_2(self) -> bool
	{
		self.0 != Self::Ethernet2.0
	}
}
