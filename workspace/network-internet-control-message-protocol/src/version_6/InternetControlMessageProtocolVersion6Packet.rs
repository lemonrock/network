// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
#[derive(Debug)]
pub struct InternetControlMessageProtocolVersion6Packet
{
	/// Header.
	pub header: InternetControlMessageProtocolVersion6PacketHeader,

	/// Payload.
	pub payload: InternetControlMessageProtocolVersion6PacketPayload,
}

impl Display for InternetControlMessageProtocolVersion6Packet
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl InternetControlMessageProtocolVersion6Packet
{
	/// Is this an error message or an informational message?
	#[inline(always)]
	pub fn message_kind(self) -> InternetControlMessageProtocolVersion6MessageKind
	{
		self.header.message_kind()
	}
}
