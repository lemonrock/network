// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// An Internet Control Message Protocol (ICMP) version 6 Neighbor Discovery option.
#[derive(Debug)]
#[repr(C, packed)]
pub struct InternetControlMessageProtocolVersion6NeighborDiscoveryOption
{
	/// 'header'.
	pub header: InternetControlMessageProtocolVersion6NeighborDiscoveryOptionHeader,
	
	/// 'value'.
	pub value: PhantomData<u8>,
}

impl Display for InternetControlMessageProtocolVersion6NeighborDiscoveryOption
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}
