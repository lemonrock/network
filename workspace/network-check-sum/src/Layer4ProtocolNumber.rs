// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// A layer 4 protocol number.
/// Overlaps with Internet Protocol Version 6 next header numbers.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Layer4ProtocolNumber(u8);

impl Display for Layer4ProtocolNumber
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl From<u8> for Layer4ProtocolNumber
{
	#[inline(always)]
	fn from(value: u8) -> Self
	{
		Layer4ProtocolNumber(value)
	}
}

impl Into<u8> for Layer4ProtocolNumber
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl Layer4ProtocolNumber
{
	/// Transmission Control Protocol (TCP).
	pub const Tcp: Self = Layer4ProtocolNumber::new(6);
	
	/// User Datagram Protocol (UDP).
	pub const Udp: Self = Layer4ProtocolNumber::new(17);
	
	/// New instance of a layer 4 protocol number.
	#[inline(always)]
	pub const fn new(value: u8) -> Self
	{
		Layer4ProtocolNumber(value)
	}
}
