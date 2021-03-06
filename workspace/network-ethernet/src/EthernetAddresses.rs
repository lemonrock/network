// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// The source and destination ethernet addresses (MACs) of a packet.
///
/// Depending on the PacketProcessingDropReason, these may be invalid, inappropriate, not for our interface, etc.
#[repr(C, packed)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct EthernetAddresses
{
	/// Destination ethernet address.
	pub destination: MediaAccessControlAddress,
	
	/// Source ethernet address.
	pub source: MediaAccessControlAddress,
}

impl Display for EthernetAddresses
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}, {}", self.source, self.destination)
	}
}

impl EthernetAddresses
{
	/// Addresses.
	#[inline(always)]
	pub fn addresses(&self) -> (&MediaAccessControlAddress, &MediaAccessControlAddress)
	{
		(&self.source, &self.destination)
	}
}
