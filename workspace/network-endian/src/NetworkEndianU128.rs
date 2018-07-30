// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Modelled as a packed 16-byte array rather than u128 u32 because (a) it is not native endian and (b) its alignment is not necessary 16 bytes (it's actually 1).
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(C, packed)]
pub struct NetworkEndianU128([u8; 16]);

impl PartialOrd for NetworkEndianU128
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		u128::from_be(u128::from_bytes(self.0)).partial_cmp(&u128::from_be(u128::from_bytes(other.0)))
	}
}

impl Ord for NetworkEndianU128
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		u128::from_be(u128::from_bytes(self.0)).cmp(&u128::from_be(u128::from_bytes(other.0)))
	}
}

impl NetworkEndian for NetworkEndianU128
{
	#[inline(always)]
	fn bytes(&self) -> &[u8]
	{
		&self.0[..]
	}
	
	#[inline(always)]
	fn write_to_hash<H: Hasher>(&self, hasher: &mut H)
	{
		hasher.write_u128(unsafe { transmute_copy(&self.0) })
	}
}

impl Display for NetworkEndianU128
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", self.to_native_endian())
	}
}

impl NetworkEndianU128
{
	/// Zero.
	pub const Zero: Self = NetworkEndianU128([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
	
	/// Maximum.
	pub const Maximum: Self = NetworkEndianU128([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
	
	/// From network endian.
	#[inline(always)]
	pub const fn from_network_endian(network_endian: [u8; 16]) -> Self
	{
		NetworkEndianU128(network_endian)
	}
	
	/// To network endian.
	#[inline(always)]
	pub fn to_network_endian(self) -> u128
	{
		self.big_endian_from_bytes()
	}
	
	/// To native endian.
	#[inline(always)]
	pub fn to_native_endian(self) -> u128
	{
		u128::from_be(self.big_endian_from_bytes())
	}
	
	/// From native endian.
	#[inline(always)]
	pub fn from_native_endian(native_endian: u128) -> Self
	{
		NetworkEndianU128(native_endian.to_be().to_bytes())
	}
	
	/// Is not zero.
	#[inline(always)]
	pub fn is_not_zero(self) -> bool
	{
		self.big_endian_from_bytes() != 0
	}

	#[inline(always)]
	fn big_endian_from_bytes(self) -> u128
	{
		u128::from_bytes(self.0)
	}
}
