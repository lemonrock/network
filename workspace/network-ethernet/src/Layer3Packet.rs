// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
pub struct Layer3Packet
{
	/// Layer 3 packet.
	pub other: PhantomData<u8>,
}

impl Display for Layer3Packet
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Debug for Layer3Packet
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "(layer 3 packet)")
	}
}

impl Layer3Packet
{
	/// Horrible method to cast to a specific implementation of a layer 3 packet.
	#[inline(always)]
	pub fn as_type<P>(&self) -> &P
	{
		unsafe { transmute(&self.other) }
	}
}
