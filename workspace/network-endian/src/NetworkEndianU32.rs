// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Modelled as a packed 4-byte array rather than an u32 because (a) it is not native endian and (b) its alignment is not necessary 4 bytes (it's actually 1).
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(C, packed)]
pub struct NetworkEndianU32([u8; NetworkEndianU32::Length]);

impl PartialOrd for NetworkEndianU32
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		u32::from_be(u32::from_bytes(self.0)).partial_cmp(&u32::from_be(u32::from_bytes(other.0)))
	}
}

impl Ord for NetworkEndianU32
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		u32::from_be(u32::from_bytes(self.0)).cmp(&u32::from_be(u32::from_bytes(other.0)))
	}
}

impl Into<[u8; NetworkEndianU32::Length]> for NetworkEndianU32
{
	#[inline(always)]
	fn into(self) -> [u8; NetworkEndianU32::Length]
	{
		self.to_bytes()
	}
}

impl From<[u8; NetworkEndianU32::Length]> for NetworkEndianU32
{
	#[inline(always)]
	fn from(bytes: [u8; NetworkEndianU32::Length]) -> Self
	{
		NetworkEndianU32(bytes)
	}
}

impl NetworkEndian for NetworkEndianU32
{
	const Length: usize = 4;
	
	type Bytes = [u8; 4];
	
	#[inline(always)]
	fn to_bytes(self) -> Self::Bytes
	{
		self.0
	}
	
	#[inline(always)]
	fn bytes(&self) -> &[u8]
	{
		&self.0[..]
	}
	
	#[inline(always)]
	fn write_to_hash<H: Hasher>(&self, hasher: &mut H)
	{
		hasher.write_u32(unsafe { transmute_copy(&self.0) })
	}
}

impl Display for NetworkEndianU32
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", self.to_native_endian())
	}
}

impl NetworkEndianU32
{
	/// Zero.
	pub const Zero: Self = NetworkEndianU32([0, 0, 0, 0]);
	
	/// Maximum.
	pub const Maximum: Self = NetworkEndianU32([0xFF, 0xFF, 0xFF, 0xFF]);
	
	/// Top bit set.
	pub const TopBitSetOnly: Self = NetworkEndianU32([128, 0, 0, 0]);
	
	/// From network endian.
	#[inline(always)]
	pub const fn from_network_endian(network_endian: [u8; 4]) -> Self
	{
		NetworkEndianU32(network_endian)
	}
	
	/// To network endian.
	#[inline(always)]
	pub fn to_network_endian(self) -> u32
	{
		self.big_endian_from_bytes()
	}
	
	/// To native endian.
	#[inline(always)]
	pub fn to_native_endian(self) -> u32
	{
		u32::from_be(self.big_endian_from_bytes())
	}
	
	/// From native endian.
	#[inline(always)]
	pub fn from_native_endian(native_endian: u32) -> Self
	{
		NetworkEndianU32(native_endian.to_be().to_bytes())
	}
	
	/// Is not zero.
	#[inline(always)]
	pub fn is_not_zero(self) -> bool
	{
		self != Self::Zero
	}
	
	#[inline(always)]
	fn big_endian_from_bytes(self) -> u32
	{
		u32::from_bytes(self.0)
	}
}
