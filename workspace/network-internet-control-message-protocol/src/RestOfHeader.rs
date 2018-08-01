// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Represents variants of the rest of the header.
#[repr(C, packed)]
pub union RestOfHeader
{
	/// Ident and sequence.
	pub ident_and_sequence: IdentAndSequence,
	
	/// Gateway.
	pub gateway: InternetProtocolVersion4HostAddress,

	/// Path Maximum Transmission Unit (Path MTU or PMTU).
	pub path_maximum_transmission_unit: PathMaximumTransmissionUnit,

	/// Router advertisement.
	pub router_advertisement: RouterAdvertisement,
	
	/// Other.
	pub unused: NetworkEndianU32,
}

impl Clone for RestOfHeader
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self
		{
			unused: unsafe { self.unused },
		}
	}
}

impl Copy for RestOfHeader
{
}

impl PartialEq for RestOfHeader
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { self.unused == other.unused }
	}
}

impl Eq for RestOfHeader
{
}

impl PartialOrd for RestOfHeader
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		unsafe { self.unused.partial_cmp(&other.unused) }
	}
}

impl Ord for RestOfHeader
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		unsafe { self.unused.cmp(&other.unused) }
	}
}

impl Hash for RestOfHeader
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		(unsafe { self.unused }).hash(hasher)
	}
}

impl Display for RestOfHeader
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Debug for RestOfHeader
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", unsafe { self.unused })
	}
}
