// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Layer 3 packet processing configuration.
pub trait Layer3PacketProcessingConfiguration: Debug + Default + Serialize
{
	/// Layer 3 packet processing type.
	type L3PP: Layer3PacketProcessing;
	
	/// Configure.
	#[inline(always)]
	fn configure<EINPDO: EthernetIncomingNetworkPacketDropObserver>(self, dropped_packet_reporting: &Rc<EINPDO>) -> Self::L3PP;
}
