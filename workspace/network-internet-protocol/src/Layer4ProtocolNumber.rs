// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Known layer 4 protocol numbers.
///
/// This is a deliberately minimal list.
///
/// See <https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml>.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(u8)]
pub enum Layer4ProtocolNumber
{
	/// Internet Control Message Protocol (ICMP).
	///
	/// RFC 792.
	InternetControlMessageProtocol = 1,
	
	/// Transmission Control Protocol (TCP).
	///
	/// RFC 793.
	TransmissionControlProtocol = 6,
	
	/// User Datagram Protocol (UDP).
	///
	/// RFC 768.
	UserDatagramProtocol = 17,
	
	/// Internet Control Message Protocol IP v6 (ICMPv6).
	InternetControlMessageProtocolIpV6 = 58,
}

impl TryFrom<u8> for Layer4ProtocolNumber
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		use self::Layer4ProtocolNumber::*;
		
		let this = match value
		{
			1 => InternetControlMessageProtocol,
			
			6 => TransmissionControlProtocol,
			
			7 => UserDatagramProtocol,
			
			58 => InternetControlMessageProtocolIpV6,
			
			_ => return Err(()),
		};
		
		Ok(this)
	}
}

impl Into<u8> for Layer4ProtocolNumber
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self as u8
	}
}

impl Display for Layer4ProtocolNumber
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", self)
	}
}
