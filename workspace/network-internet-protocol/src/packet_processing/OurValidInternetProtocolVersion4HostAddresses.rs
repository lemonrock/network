// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Our unicast internet protocol (IP) version 4 host addresses valid for this network interface.
///
/// Used for both Internet Protocol (IP) version 4 packet processing and Address Resolution Protocol (ARP) packet processing `network-address-resolution-protocol` crate.
///
/// No sender packet should be received from this address; if it was, it implies loopback on this interface, which is daft.
#[derive(Debug)]
pub struct OurValidInternetProtocolVersion4HostAddresses(HashSet<InternetProtocolVersion4HostAddress>);

impl OurValidInternetProtocolVersion4HostAddresses
{
	/// Used for both Internet Protocol (IP) version 4 packet processing and Address Resolution Protocol (ARP) packet processing  `network-address-resolution-protocol` crate.
	#[inline(always)]
	pub fn is_internet_protocol_version_4_host_address_one_of_ours(&self, internet_protocol_version_4_host_address: InternetProtocolVersion4HostAddress) -> bool
	{
		debug_assert!(internet_protocol_version_4_host_address.is_valid_unicast(), "internet_protocol_version_4_host_address '{:?}' is not valid unicast", internet_protocol_version_4_host_address);
		
		self.0.contains(&internet_protocol_version_4_host_address)
	}
}
