// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Categorised internet protocol header options and extensions for a categorised layer 3 packet type.
///
/// All drivers support this level of categorisation if they categorise internet protocol (IP) version 4 or version 6 packets.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum HardwareOffloadCategorisedInternetProtocolHeaderOptionsAndExtensionsLayer3PacketType
{
	/// No header options or extensions.
	NotPresent,
	
	/// Has header options or extensions.
	Present,
	
	/// Has header options or extensions, some of which are unrecognised (by hardware).
	PresentAndUnrecognised,
}
