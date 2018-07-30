// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a RFC 1071 internet checksum.
#[repr(C, packed)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct InternetChecksum(NetworkEndianU16);

impl Into<NetworkEndianU16> for InternetChecksum
{
	#[inline(always)]
	fn into(self) -> NetworkEndianU16
	{
		self.0
	}
}

impl Into<u16> for InternetChecksum
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0.to_native_endian()
	}
}

impl From<NetworkEndianU16> for InternetChecksum
{
	#[inline(always)]
	fn from(value: NetworkEndianU16) -> Self
	{
		InternetChecksum(value)
	}
}

impl From<u16> for InternetChecksum
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		InternetChecksum(NetworkEndianU16::from_native_endian(value))
	}
}

impl Display for InternetChecksum
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", self.0.to_native_endian())
	}
}

impl Debug for InternetChecksum
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", self.0.to_native_endian())
	}
}

impl Default for InternetChecksum
{
	#[inline(always)]
	fn default() -> Self
	{
		InternetChecksum(NetworkEndianU16::Zero)
	}
}
