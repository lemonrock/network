// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Hardware offload layer 3 packet type.
///
/// If the packet is a tunnelled packet, then this is known as the Outer Layer 3 packet type.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub enum HardwareOffloadLayer3PacketType
{
	/// Either the driver did not categorise this packet or the layer 3 data structure is absent.
	UncategorisedOrAbsent,
	
	/// Internet Protocol (IP) version 4.
	///
	/// EtherType 0x0800.
	InternetProtocolVersion4(HardwareOffloadCategorisedInternetProtocolHeaderOptionsAndExtensionsLayer3PacketType),
	
	/// Internet Protocol (IP) version 6.
	///
	/// EtherType 0x86DD.
	InternetProtocolVersion6(HardwareOffloadCategorisedInternetProtocolHeaderOptionsAndExtensionsLayer3PacketType),
	
	/// Invalid or introduced after this code was written.
	Other,
}
