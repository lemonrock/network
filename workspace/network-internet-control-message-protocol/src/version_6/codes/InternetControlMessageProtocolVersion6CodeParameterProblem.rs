// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Parameter problem.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct InternetControlMessageProtocolVersion6CodeParameterProblem(u8);

impl InternetControlMessageProtocolVersion6CodeParameterProblem
{
	/// Erroneous header field encountered.
	pub const ErroneousHeaderFieldEncountered: Self = InternetControlMessageProtocolVersion6CodeParameterProblem(0);
	
	/// Unrecognized next header type encountered.
	pub const UnrecognizedNextHeaderTypeEncountered: Self = InternetControlMessageProtocolVersion6CodeParameterProblem(1);
	
	/// Unrecognized Internet Protocol (IP) version 6 option encountered.
	pub const UnrecognizedInternetProtocolVersion6OptionEncountered: Self = InternetControlMessageProtocolVersion6CodeParameterProblem(2);
	
	/// Internet Protocol (IP) version 6 first fragment has incomplete header chain (RFC 7112).
	pub const InternetProtocolVersion6FirstFragmentHasIncompleteHeaderChain: Self = InternetControlMessageProtocolVersion6CodeParameterProblem(3);
}

impl TryFrom<u8> for InternetControlMessageProtocolVersion6CodeParameterProblem
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if value <= 3
		{
			Ok(InternetControlMessageProtocolVersion6CodeParameterProblem(value))
		}
		else
		{
			Err(())
		}
	}
}

impl Into<u8> for InternetControlMessageProtocolVersion6CodeParameterProblem
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl Display for InternetControlMessageProtocolVersion6CodeParameterProblem
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}
