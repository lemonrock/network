// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// This is a RFC 1071 internet checksum.
#[repr(C, packed)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct InternetCheckSum(NetworkEndianU16);

impl Into<NetworkEndianU16> for InternetCheckSum
{
	#[inline(always)]
	fn into(self) -> NetworkEndianU16
	{
		self.0
	}
}

impl Into<u16> for InternetCheckSum
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0.to_native_endian()
	}
}

impl From<NetworkEndianU16> for InternetCheckSum
{
	#[inline(always)]
	fn from(value: NetworkEndianU16) -> Self
	{
		InternetCheckSum(value)
	}
}

impl From<u16> for InternetCheckSum
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		InternetCheckSum(NetworkEndianU16::from_native_endian(value))
	}
}

impl Display for InternetCheckSum
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", self.0.to_native_endian())
	}
}

impl Debug for InternetCheckSum
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", self.0.to_native_endian())
	}
}

impl Default for InternetCheckSum
{
	#[inline(always)]
	fn default() -> Self
	{
		InternetCheckSum(NetworkEndianU16::Zero)
	}
}
