// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Represents common internet control message protocol (ICMP) types.
///
/// Deprecated, unassigned, reserved and experimental types are not provided for.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct InternetControlMessageProtocolVersion4Type(u8);

impl Display for InternetControlMessageProtocolVersion4Type
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl Into<u8> for InternetControlMessageProtocolVersion4Type
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl From<u8> for InternetControlMessageProtocolVersion4Type
{
	#[inline(always)]
	fn from(value: u8) -> Self
	{
		InternetControlMessageProtocolVersion4Type(value)
	}
}

impl InternetControlMessageProtocolVersion4Type
{
	/// Echo Reply ('pong').
	pub const EchoReply: Self = InternetControlMessageProtocolVersion4Type(0);
	
	/// Destination Unreachable
	pub const DestinationUnreachable: Self = InternetControlMessageProtocolVersion4Type(3);
	
	/// Redirect Message.
	pub const RedirectMessage: Self = InternetControlMessageProtocolVersion4Type(5);
	
	/// Echo Request ('ping').
	pub const EchoRequest: Self = InternetControlMessageProtocolVersion4Type(8);
	
	/// Router Advertisement.
	pub const RouterAdvertisement: Self = InternetControlMessageProtocolVersion4Type(9);
	
	/// Router Solicitation (request).
	pub const RouterSolicitation: Self = InternetControlMessageProtocolVersion4Type(10);
	
	/// Time exceeded (TTL).
	pub const TimeExceeded: Self = InternetControlMessageProtocolVersion4Type(11);
	
	/// Bad Internet Protocol (IP) version 4 header.
	pub const BadInternetProtocolVersion4Header: Self = InternetControlMessageProtocolVersion4Type(12);
	
	/// Timestamp Request.
	pub const TimestampRequest: Self = InternetControlMessageProtocolVersion4Type(13);
	
	/// Timestamp Reply.
	pub const TimestampReply: Self = InternetControlMessageProtocolVersion4Type(14);
}
