// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Cause of failure to parse.
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
pub enum InternetProtocolVersion6MulticastAddressParseError
{
	#[allow(missing_docs)]
	ReservedHighOrderFlag,
	
	#[allow(missing_docs)]
	ReservedOrUnassignedScope(u8),
}

impl Display for InternetProtocolVersion6MulticastAddressParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl ::std::error::Error for InternetProtocolVersion6MulticastAddressParseError
{
}
