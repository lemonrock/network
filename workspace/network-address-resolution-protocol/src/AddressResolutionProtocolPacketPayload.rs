// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Payload of address resolution protocol (ARP) packet.
#[repr(C, packed)]
pub union AddressResolutionProtocolPacketPayload
{
	/// Internet Protocol Version 4 payload.
	pub internet_protocol_version_4_payload: AddressResolutionProtocolPacketInternetProtocolVersion4Payload,
	
	/// Other kinds of payloads, not differentiated.
	pub other: PhantomData<u8>,
}

impl Display for AddressResolutionProtocolPacketPayload
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Debug for AddressResolutionProtocolPacketPayload
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "(arp payload)")
	}
}

impl AddressResolutionProtocolPacketPayload
{
	#[inline(always)]
	pub(crate) fn internet_protocol_version_4_payload(&self) -> &AddressResolutionProtocolPacketInternetProtocolVersion4Payload
	{
		unsafe { &self.internet_protocol_version_4_payload }
	}
}
