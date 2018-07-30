// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Tag control information.
#[repr(C, packed)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct TagControlInformation(NetworkEndianU16);

impl Into<NetworkEndianU16> for TagControlInformation
{
	#[inline(always)]
	fn into(self) -> NetworkEndianU16
	{
		self.0
	}
}

impl Into<u16> for TagControlInformation
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0.to_native_endian()
	}
}

impl From<NetworkEndianU16> for TagControlInformation
{
	#[inline(always)]
	fn from(value: NetworkEndianU16) -> Self
	{
		TagControlInformation(value)
	}
}

impl From<u16> for TagControlInformation
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		TagControlInformation(NetworkEndianU16::from_native_endian(value))
	}
}

impl Display for TagControlInformation
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", self.0.to_native_endian())
	}
}

impl Debug for TagControlInformation
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", self.0.to_native_endian())
	}
}

impl TagControlInformation
{
	/// Parse.
	#[inline(always)]
	pub fn parse(self) -> Result<(ClassOfService, DropEligibleIndicator, Option<VirtualLanIdentifier>), ()>
	{
		let value = self.0.to_native_endian();
		
		let virtual_lan_identifier = match value & 0x0FFF
		{
			0 => None,
			0x0FFF => return Err(()),
			valid @ _ => Some(VirtualLanIdentifier(valid))
		};
		
		let class_of_service = unsafe { transmute((value & 0b1110_0000_0000_0000 >> 13) as u8) };
		let drop_eligible_indicator = value & 0b0001_0000_0000_0000 == 0b0001_0000_0000_0000;
		Ok((class_of_service, drop_eligible_indicator, virtual_lan_identifier))
	}
}
