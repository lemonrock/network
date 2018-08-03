// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Option class.
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(u8)]
pub enum InternetProtocolVersion4OptionClass
{
	/// Control.
	Control = 0,
	
	/// Reserved for future use.
	ReservedForFutureUse1 = 1,
	
	/// Debugging and measurement.
	DebuggingAndMeasurement = 2,
	
	/// Reserved for future use.
	ReservedForFutureUse2 = 3,
}

impl Into<u8> for InternetProtocolVersion4OptionClass
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self as u8
	}
}
