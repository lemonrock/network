// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Hardware offload categorised internet protocol header options and extensions for a categorised layer 3 packet type.
///
/// All DPDK drivers support this level of categorisation if they categorise Internet Protocol (IP) version 4 or version 6 packets.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub enum HardwareOffloadCategorisedInternetProtocolHeaderOptionsAndExtensionsLayer3PacketType
{
	/// No Internet Protocol (IP) version 4 options or Internet Protocol (IP) version 6 extensions present.
	NotPresent,
	
	/// Has Internet Protocol (IP) version 4 options or Internet Protocol (IP) version 6 extensions.
	Present,
	
	/// Has Internet Protocol (IP) version 4 options or Internet Protocol (IP) version 6 extensions, some of which are unrecognised (by hardware).
	PresentAndUnrecognised,
}
