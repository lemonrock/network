// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Represents one variant of the rest of the header.
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct PathMaximumTransmissionUnit
{
	/// Unused.
	pub unused: NetworkEndianU16,
	
	/// Maximum transmission unit (access via accessor `self.next_hop_maximum_transmission_unit()` to get in native endian form).
	pub next_hop_maximum_transmission_unit: NetworkEndianU16,
}

impl Display for PathMaximumTransmissionUnit
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl PathMaximumTransmissionUnit
{
	/// Tries to get maximum transmission unit size of next hop.
	#[inline(always)]
	pub fn next_hop_maximum_transmission_unit(&self) -> Result<MaximumTransmissionUnitSize, ()>
	{
		MaximumTransmissionUnitSize::try_from(self.next_hop_maximum_transmission_unit)
	}
}
