// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Legacy ethernet frame size.
///
/// Rare and of little value for typical DPDK usage.
#[repr(C, packed)]
#[derive(Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct LegacyEthernetFrameSize(NetworkEndianU16);

impl Into<NetworkEndianU16> for LegacyEthernetFrameSize
{
	#[inline(always)]
	fn into(self) -> NetworkEndianU16
	{
		self.0
	}
}

impl Into<u16> for LegacyEthernetFrameSize
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0.to_native_endian()
	}
}

impl TryFrom<u16> for LegacyEthernetFrameSize
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u16) -> Result<Self, Self::Error>
	{
		Self::new(NetworkEndianU16::from_native_endian(value))
	}
}

impl TryFrom<NetworkEndianU16> for LegacyEthernetFrameSize
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: NetworkEndianU16) -> Result<Self, Self::Error>
	{
		Self::new(value)
	}
}

impl Display for LegacyEthernetFrameSize
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", self.0.to_native_endian())
	}
}

impl Debug for LegacyEthernetFrameSize
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", self.0.to_native_endian())
	}
}

impl LegacyEthernetFrameSize
{
	/// Minimum.
	pub const Minimum: Self = LegacyEthernetFrameSize(NetworkEndianU16::Zero);
	
	/// IEEE 802.3x-1997 frame size change over.
	pub const Maximum: Self = LegacyEthernetFrameSize(NetworkEndianU16::from_network_endian([0x05, 0xFF]));
	
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(value: NetworkEndianU16) -> Result<Self, ()>
	{
		if value.bytes()[0] < EtherTypeOrLegacyEthernetFrameSize::FirstByteSwitchOverValue
		{
			Ok(LegacyEthernetFrameSize(value))
		}
		else
		{
			Err(())
		}
	}
	
	/// To a network endian value.
	#[inline(always)]
	pub fn to_network_endian(self) -> u16
	{
		self.0.to_network_endian()
	}
}
